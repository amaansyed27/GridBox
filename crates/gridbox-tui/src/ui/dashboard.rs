use crate::{
    app::App,
    ui::{logo, theme},
};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let [logo_area, body_area] = Layout::vertical([
        Constraint::Percentage(48),
        Constraint::Percentage(52),
    ])
    .areas(area);

    let logo_text = if app.compact_logo || logo_area.width < 70 || logo_area.height < 8 {
        logo::COMPACT
    } else {
        logo::LARGE
    };
    frame.render_widget(
        Paragraph::new(logo_text)
            .style(theme::accent())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE)),
        logo_area,
    );

    let data_state = match &app.live {
        Some(snapshot) if snapshot.session.is_live_at(chrono::Utc::now()) => {
            format!("LIVE: {}", snapshot.session.title())
        }
        Some(snapshot) => format!("Latest: {}", snapshot.session.title()),
        None => "No session loaded".to_string(),
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("Data  ", theme::accent()),
            Span::raw(data_state),
        ]),
        Line::from(vec![
            Span::styled("Model ", theme::accent()),
            Span::raw(format!("{} through Ollama", app.model)),
        ]),
        Line::from(vec![
            Span::styled("Local ", theme::accent()),
            Span::raw("analysis, chat history, cache and recordings stay on this machine"),
        ]),
        Line::raw(""),
        Line::styled("Start with one of these:", theme::muted()),
        Line::raw("  /live"),
        Line::raw("  /schedule 2026"),
        Line::raw("  /session 2025 Monaco Q"),
        Line::raw("  /compare 2025 Monaco Q NOR VER"),
        Line::raw("  Ask: Who is currently inside an undercut or DRS window?"),
    ];

    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(Block::default().title(" Workspace ").borders(Borders::ALL)),
        body_area,
    );
}
