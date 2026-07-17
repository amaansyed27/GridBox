use crate::{app::App, ui::theme};
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let prompt = if app.busy { "… " } else { "› " };
    let paragraph = Paragraph::new(Line::from(vec![
        Span::styled(prompt, theme::accent()),
        Span::raw(app.input.as_str()),
    ]))
    .block(
        Block::default()
            .title(" Command / question ")
            .borders(Borders::ALL),
    );
    frame.render_widget(paragraph, area);

    let cursor_x = area
        .x
        .saturating_add(2)
        .saturating_add(prompt.chars().count() as u16)
        .saturating_add(app.input.chars().count() as u16)
        .min(area.right().saturating_sub(2));
    frame.set_cursor_position((cursor_x, area.y.saturating_add(1)));
}

pub fn render_footer(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let status = Line::from(vec![
        Span::styled("STATUS ", theme::muted()),
        Span::raw(&app.status),
        Span::styled("  ·  Ctrl+Q exit", theme::muted()),
    ]);
    frame.render_widget(Paragraph::new(status), area);
}
