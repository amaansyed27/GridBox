use crate::{
    app::{App, LaunchMode, Tab},
    events::{AppAction, AppEvent},
    ui,
};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use gridbox_agent::OllamaAgent;
use gridbox_analysis::snapshot_context;
use gridbox_fastf1_client::FastF1Client;
use gridbox_jolpica::JolpicaClient;
use gridbox_openf1::OpenF1Client;
use gridbox_storage::{Config, LiveRecorder};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};
use tokio::{sync::mpsc, time::Instant};

#[derive(Clone)]
pub struct AppServices {
    pub openf1: OpenF1Client,
    pub jolpica: JolpicaClient,
    pub agent: OllamaAgent,
    pub fastf1: FastF1Client,
    pub recorder: LiveRecorder,
    pub record_live_sessions: bool,
}

pub async fn run_tui(config: Config, services: AppServices, mode: LaunchMode) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, config, services, mode).await;

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    result
}

async fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    config: Config,
    services: AppServices,
    mode: LaunchMode,
) -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut app = App::new(mode, config.llm.model.clone(), config.ui.compact_logo);
    let tick_rate = Duration::from_millis(config.ui.tick_rate_ms.max(30));
    let live_poll = Duration::from_secs(config.openf1.poll_interval_secs.max(2));
    let mut last_live_refresh = Instant::now() - live_poll;

    if mode == LaunchMode::Live || config.openf1.auto_detect {
        dispatch_action(AppAction::RefreshLive, &app, services.clone(), tx.clone());
        last_live_refresh = Instant::now();
    }

    while !app.should_quit {
        while let Ok(event) = rx.try_recv() {
            app.apply_event(event);
        }

        if should_auto_refresh(&app, config.openf1.auto_detect)
            && last_live_refresh.elapsed() >= live_poll
            && !app.busy
        {
            app.busy = true;
            dispatch_action(AppAction::RefreshLive, &app, services.clone(), tx.clone());
            last_live_refresh = Instant::now();
        }

        terminal.draw(|frame| ui::render(frame, &app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = handle_key(&mut app, key);
                    if matches!(action, AppAction::Quit) {
                        app.should_quit = true;
                        continue;
                    }
                    if !matches!(action, AppAction::None) {
                        dispatch_action(action, &app, services.clone(), tx.clone());
                    }
                }
            }
        }
    }

    Ok(())
}

fn should_auto_refresh(app: &App, auto_detect: bool) -> bool {
    app.launch_mode == LaunchMode::Live
        || (auto_detect
            && app
                .live
                .as_ref()
                .map(|snapshot| snapshot.session.is_live_at(chrono::Utc::now()))
                .unwrap_or(true))
}

fn handle_key(app: &mut App, key: KeyEvent) -> AppAction {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c') | KeyCode::Char('q') => return AppAction::Quit,
            KeyCode::Char('l') => {
                app.input.clear();
                return AppAction::None;
            }
            _ => {}
        }
    }

    match key.code {
        KeyCode::F(1) => app.set_tab(Tab::Dashboard),
        KeyCode::F(2) => app.set_tab(Tab::Live),
        KeyCode::F(3) => app.set_tab(Tab::History),
        KeyCode::F(4) => app.set_tab(Tab::Engineer),
        KeyCode::F(5) => app.set_tab(Tab::Help),
        KeyCode::BackTab => app.previous_tab(),
        KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => app.previous_tab(),
        KeyCode::Tab => app.next_tab(),
        KeyCode::Enter => return app.submit_input(),
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Esc => app.input.clear(),
        KeyCode::Char(character) => app.input.push(character),
        _ => {}
    }
    AppAction::None
}

fn dispatch_action(
    action: AppAction,
    app: &App,
    services: AppServices,
    tx: mpsc::UnboundedSender<AppEvent>,
) {
    match action {
        AppAction::None | AppAction::Quit => {}
        AppAction::RefreshLive => {
            tokio::spawn(async move {
                let result = services
                    .openf1
                    .snapshot_latest()
                    .await
                    .map(Box::new)
                    .map_err(|error| error.to_string());
                if let Ok(snapshot) = &result {
                    if services.record_live_sessions
                        && snapshot.session.is_live_at(chrono::Utc::now())
                    {
                        let _ = services.recorder.record_snapshot(snapshot);
                    }
                }
                let _ = tx.send(AppEvent::LiveLoaded(result));
            });
        }
        AppAction::AskAi(question) => {
            let history = app.chat.clone();
            let context = snapshot_context(app.live.as_ref(), app.selected_driver);
            let agent = services.agent.with_model(app.model.clone());
            tokio::spawn(async move {
                let result = agent
                    .chat(&history, &context, &question)
                    .await
                    .map_err(|error| error.to_string());
                let _ = tx.send(AppEvent::AiCompleted(result));
            });
        }
        AppAction::LoadSchedule(year) => {
            tokio::spawn(async move {
                let result = services
                    .jolpica
                    .schedule(year)
                    .await
                    .map_err(|error| error.to_string());
                let _ = tx.send(AppEvent::ScheduleLoaded(result));
            });
        }
        AppAction::LoadSession {
            year,
            event,
            session,
        } => {
            tokio::spawn(async move {
                let result = services
                    .fastf1
                    .session_summary(year, &event, &session)
                    .await
                    .map_err(|error| error.to_string());
                let _ = tx.send(AppEvent::FastF1Completed(result));
            });
        }
        AppAction::Compare {
            year,
            event,
            session,
            drivers,
        } => {
            tokio::spawn(async move {
                let result = services
                    .fastf1
                    .compare_laps(year, &event, &session, &drivers)
                    .await
                    .map_err(|error| error.to_string());
                let _ = tx.send(AppEvent::FastF1Completed(result));
            });
        }
    }
}
