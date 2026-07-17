mod dashboard;
mod engineer;
mod header;
mod help;
mod history;
mod input;
mod live;
mod logo;
mod theme;

use crate::app::{App, Tab};
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let [header_area, content_area, input_area, footer_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(8),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    header::render(frame, header_area, app);
    match app.active_tab {
        Tab::Dashboard => dashboard::render(frame, content_area, app),
        Tab::Live => live::render(frame, content_area, app),
        Tab::History => history::render(frame, content_area, app),
        Tab::Engineer => engineer::render(frame, content_area, app),
        Tab::Help => help::render(frame, content_area),
    }
    input::render(frame, input_area, app);
    input::render_footer(frame, footer_area, app);
}
