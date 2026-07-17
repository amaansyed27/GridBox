use crate::{
    commands::{parse_command, UserCommand},
    events::{AppAction, AppEvent},
};
use chrono::Utc;
use gridbox_models::{ChatMessage, LiveSnapshot, RaceEvent};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchMode {
    Auto,
    Live,
    Demo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Live,
    History,
    Engineer,
    Help,
}

impl Tab {
    pub const ALL: [Self; 5] = [
        Self::Dashboard,
        Self::Live,
        Self::History,
        Self::Engineer,
        Self::Help,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Live => "Live",
            Self::History => "History",
            Self::Engineer => "Engineer",
            Self::Help => "Help",
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub launch_mode: LaunchMode,
    pub active_tab: Tab,
    pub input: String,
    pub status: String,
    pub live: Option<LiveSnapshot>,
    pub schedule: Vec<RaceEvent>,
    pub chat: Vec<ChatMessage>,
    pub selected_driver: Option<u32>,
    pub model: String,
    pub busy: bool,
    pub should_quit: bool,
    pub last_fastf1: Option<Value>,
    pub compact_logo: bool,
}

impl App {
    pub fn new(launch_mode: LaunchMode, model: impl Into<String>, compact_logo: bool) -> Self {
        let active_tab = if launch_mode == LaunchMode::Auto {
            Tab::Dashboard
        } else {
            Tab::Live
        };
        Self {
            launch_mode,
            active_tab,
            input: String::new(),
            status: if launch_mode == LaunchMode::Demo {
                "Starting fully local live demo…".to_string()
            } else {
                "Ready. /help lists commands.".to_string()
            },
            live: None,
            schedule: Vec::new(),
            chat: vec![ChatMessage::assistant(
                "GridBox Engineer is local. Load live data with /live or historical data with /session.",
            )],
            selected_driver: None,
            model: model.into(),
            busy: false,
            should_quit: false,
            last_fastf1: None,
            compact_logo,
        }
    }

    pub fn next_tab(&mut self) {
        let index = Tab::ALL
            .iter()
            .position(|tab| *tab == self.active_tab)
            .unwrap_or_default();
        self.active_tab = Tab::ALL[(index + 1) % Tab::ALL.len()];
    }

    pub fn previous_tab(&mut self) {
        let index = Tab::ALL
            .iter()
            .position(|tab| *tab == self.active_tab)
            .unwrap_or_default();
        self.active_tab = Tab::ALL[(index + Tab::ALL.len() - 1) % Tab::ALL.len()];
    }

    pub fn set_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
    }

    pub fn submit_input(&mut self) -> AppAction {
        let input = std::mem::take(&mut self.input);
        match parse_command(&input) {
            Ok(command) => self.apply_command(command),
            Err(error) if error == "empty command" => AppAction::None,
            Err(error) => {
                self.status = error;
                AppAction::None
            }
        }
    }

    pub fn apply_event(&mut self, event: AppEvent) {
        self.busy = false;
        match event {
            AppEvent::LiveLoaded(Ok(snapshot)) => {
                let is_live = snapshot.session.is_live_at(Utc::now());
                self.status = if self.launch_mode == LaunchMode::Demo {
                    format!(
                        "DEMO LIVE · {} · {} drivers · fully local",
                        snapshot.session.title(),
                        snapshot.drivers.len()
                    )
                } else if is_live {
                    format!(
                        "LIVE · {} · {} drivers · {}",
                        snapshot.session.title(),
                        snapshot.drivers.len(),
                        snapshot.source
                    )
                } else {
                    format!(
                        "Latest session: {} · currently not active · {}",
                        snapshot.session.title(),
                        snapshot.source
                    )
                };
                self.live = Some(*snapshot);
                if is_live || self.launch_mode != LaunchMode::Auto {
                    self.active_tab = Tab::Live;
                }
            }
            AppEvent::LiveLoaded(Err(error)) => {
                self.status = format!("Live data unavailable: {error}");
            }
            AppEvent::AiCompleted(Ok(answer)) => {
                self.chat.push(ChatMessage::assistant(answer));
                self.status = format!("{} answered locally", self.model);
                self.active_tab = Tab::Engineer;
            }
            AppEvent::AiCompleted(Err(error)) => {
                self.chat.push(ChatMessage::assistant(format!(
                    "Local model error: {error}"
                )));
                self.status = "Local model request failed".to_string();
                self.active_tab = Tab::Engineer;
            }
            AppEvent::ScheduleLoaded(Ok(schedule)) => {
                self.status = format!("Loaded {} races", schedule.len());
                self.schedule = schedule;
                self.active_tab = Tab::History;
            }
            AppEvent::ScheduleLoaded(Err(error)) => {
                self.status = format!("Schedule unavailable: {error}");
            }
            AppEvent::FastF1Completed(Ok(result)) => {
                let pretty =
                    serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string());
                self.chat.push(ChatMessage::assistant(format!(
                    "FastF1 analysis result:\n{pretty}"
                )));
                self.last_fastf1 = Some(result);
                self.status = "FastF1 analysis complete".to_string();
                self.active_tab = Tab::Engineer;
            }
            AppEvent::FastF1Completed(Err(error)) => {
                self.chat.push(ChatMessage::assistant(format!(
                    "FastF1 worker error: {error}"
                )));
                self.status = "FastF1 analysis failed".to_string();
                self.active_tab = Tab::Engineer;
            }
        }
    }

    fn apply_command(&mut self, command: UserCommand) -> AppAction {
        match command {
            UserCommand::Help => {
                self.active_tab = Tab::Help;
                self.status = "Command reference".to_string();
                AppAction::None
            }
            UserCommand::Quit => AppAction::Quit,
            UserCommand::Clear => {
                self.chat.clear();
                self.chat
                    .push(ChatMessage::assistant("Conversation cleared."));
                self.status = "Engineer conversation cleared".to_string();
                AppAction::None
            }
            UserCommand::Live | UserCommand::Refresh if self.launch_mode == LaunchMode::Demo => {
                self.active_tab = Tab::Live;
                self.status = "Demo live stream runs continuously and needs no refresh".to_string();
                AppAction::None
            }
            UserCommand::Live | UserCommand::Refresh => {
                self.active_tab = Tab::Live;
                self.busy = true;
                self.status = "Refreshing OpenF1…".to_string();
                AppAction::RefreshLive
            }
            UserCommand::Model(model) => {
                self.model = model;
                self.status = format!("Local model set to {}", self.model);
                AppAction::None
            }
            UserCommand::Driver(number) => {
                self.selected_driver = Some(number);
                self.status = format!("Focused car #{number}");
                AppAction::None
            }
            UserCommand::Schedule(year) => {
                self.busy = true;
                self.status = format!("Loading {year} schedule…");
                AppAction::LoadSchedule(year)
            }
            UserCommand::Session {
                year,
                event,
                session,
            } => {
                self.busy = true;
                self.chat.push(ChatMessage::user(format!(
                    "/session {year} {event} {session}"
                )));
                self.active_tab = Tab::Engineer;
                AppAction::LoadSession {
                    year,
                    event,
                    session,
                }
            }
            UserCommand::Compare {
                year,
                event,
                session,
                drivers,
            } => {
                self.busy = true;
                self.chat.push(ChatMessage::user(format!(
                    "/compare {year} {event} {session} {}",
                    drivers.join(" ")
                )));
                self.active_tab = Tab::Engineer;
                AppAction::Compare {
                    year,
                    event,
                    session,
                    drivers,
                }
            }
            UserCommand::Ask(question) => {
                self.chat.push(ChatMessage::user(question.clone()));
                self.active_tab = Tab::Engineer;
                self.busy = true;
                self.status = format!("{} is reasoning locally…", self.model);
                AppAction::AskAi(question)
            }
        }
    }
}
