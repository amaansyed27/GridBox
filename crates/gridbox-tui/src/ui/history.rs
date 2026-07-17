use crate::{app::App, ui::theme};
use ratatui::{
    layout::{Constraint, Rect},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    if app.schedule.is_empty() {
        frame.render_widget(
            Paragraph::new("No schedule loaded. Use /schedule <year>.").block(
                Block::default()
                    .title(" Historical schedule ")
                    .borders(Borders::ALL),
            ),
            area,
        );
        return;
    }

    let header = Row::new(["Rnd", "Race", "Circuit", "Location", "Date"]).style(theme::accent());
    let rows = app.schedule.iter().map(|race| {
        Row::new(vec![
            Cell::from(race.round.to_string()),
            Cell::from(race.race_name.clone()),
            Cell::from(race.circuit_name.clone()),
            Cell::from(format!("{}, {}", race.locality, race.country)),
            Cell::from(race.display_time()),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Percentage(24),
            Constraint::Percentage(24),
            Constraint::Percentage(24),
            Constraint::Percentage(23),
        ],
    )
    .header(header)
    .column_spacing(1)
    .block(
        Block::default()
            .title(" Season schedule ")
            .borders(Borders::ALL),
    );
    frame.render_widget(table, area);
}
