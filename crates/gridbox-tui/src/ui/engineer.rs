use crate::{app::App, ui::theme};
use gridbox_models::ChatRole;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let mut lines = Vec::new();
    for message in app.chat.iter().rev().take(18).rev() {
        let (label, style) = match message.role {
            ChatRole::User => ("YOU", theme::accent()),
            ChatRole::Assistant => ("ENGINEER", Default::default()),
            ChatRole::System => ("SYSTEM", theme::muted()),
            ChatRole::Tool => ("TOOL", theme::warning()),
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{label}  "), style),
            Span::raw(message.content.clone()),
        ]));
        lines.push(Line::raw(""));
    }
    if app.busy {
        lines.push(Line::styled("Processing locally…", theme::warning()));
    }

    frame.render_widget(
        Paragraph::new(lines).wrap(Wrap { trim: false }).block(
            Block::default()
                .title(format!(" Local engineer · {} ", app.model))
                .borders(Borders::ALL),
        ),
        area,
    );
}
