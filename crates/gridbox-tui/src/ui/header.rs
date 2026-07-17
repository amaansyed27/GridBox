use crate::{app::App, ui::theme};
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let titles = crate::app::Tab::ALL
        .iter()
        .enumerate()
        .map(|(index, tab)| {
            Line::from(vec![
                Span::styled(format!("F{} ", index + 1), theme::muted()),
                Span::raw(tab.title()),
            ])
        })
        .collect::<Vec<_>>();
    let selected = crate::app::Tab::ALL
        .iter()
        .position(|tab| *tab == app.active_tab)
        .unwrap_or_default();

    let title = if let Some(snapshot) = &app.live {
        format!(" GRIDBOX · {} ", snapshot.session.title())
    } else {
        " GRIDBOX · LOCAL F1 ENGINEERING ".to_string()
    };

    let tabs = Tabs::new(titles)
        .select(selected)
        .style(theme::muted())
        .highlight_style(theme::active())
        .divider("  ")
        .block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(tabs, area);
}
