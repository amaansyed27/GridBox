use crate::{app::App, ui::theme};
use gridbox_analysis::{analyze_live_strategy, StrategySeverity};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let Some(snapshot) = &app.live else {
        frame.render_widget(
            Paragraph::new(
                "No live snapshot loaded. Enter /live to detect the latest or active session.",
            )
            .wrap(Wrap { trim: true })
            .block(Block::default().title(" Live ").borders(Borders::ALL)),
            area,
        );
        return;
    };

    let [timing_area, side_area] =
        Layout::horizontal([Constraint::Percentage(68), Constraint::Percentage(32)]).areas(area);

    let header =
        Row::new(["P", "DRV", "GAP", "INT", "LAP", "LAST", "TYRE", "AGE"]).style(theme::accent());
    let rows = snapshot.sorted_drivers().into_iter().map(|driver| {
        let selected = app.selected_driver == Some(driver.driver_number);
        let style = if selected {
            theme::active()
        } else {
            Default::default()
        };
        Row::new(vec![
            Cell::from(
                driver
                    .position
                    .map_or("-".into(), |value| value.to_string()),
            ),
            Cell::from(driver.display_name().to_string()),
            Cell::from(driver.gap_to_leader.clone().unwrap_or_else(|| "-".into())),
            Cell::from(driver.interval.clone().unwrap_or_else(|| "-".into())),
            Cell::from(
                driver
                    .lap_number
                    .map_or("-".into(), |value| value.to_string()),
            ),
            Cell::from(
                driver
                    .last_lap_duration
                    .map(format_lap)
                    .unwrap_or_else(|| "-".into()),
            ),
            Cell::from(driver.compound.clone().unwrap_or_else(|| "-".into())),
            Cell::from(
                driver
                    .tyre_age
                    .map_or("-".into(), |value| value.to_string()),
            ),
        ])
        .style(style)
    });
    let table = Table::new(
        rows,
        [
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Length(9),
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(4),
        ],
    )
    .header(header)
    .column_spacing(1)
    .block(
        Block::default()
            .title(format!(
                " {} · fetched {} ",
                snapshot.session.title(),
                snapshot.fetched_at.format("%H:%M:%S UTC")
            ))
            .borders(Borders::ALL),
    );
    frame.render_widget(table, timing_area);

    let [strategy_area, control_area, weather_area] = Layout::vertical([
        Constraint::Percentage(44),
        Constraint::Percentage(40),
        Constraint::Percentage(16),
    ])
    .areas(side_area);

    let strategy_lines = analyze_live_strategy(snapshot)
        .into_iter()
        .map(|insight| {
            let style = match insight.severity {
                StrategySeverity::Info => theme::muted(),
                StrategySeverity::Watch => theme::warning(),
                StrategySeverity::Critical => theme::critical(),
            };
            Line::from(vec![
                Span::styled(format!("{}: ", insight.title), style),
                Span::raw(insight.detail),
            ])
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(strategy_lines)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(" Strategy signals ")
                    .borders(Borders::ALL),
            ),
        strategy_area,
    );

    let control_lines = snapshot
        .race_control
        .iter()
        .rev()
        .take(8)
        .map(|event| {
            Line::from(vec![
                Span::styled(event.date.format("%H:%M:%S ").to_string(), theme::muted()),
                Span::raw(event.flag.as_deref().unwrap_or(&event.category)),
                Span::raw(" · "),
                Span::raw(event.message.clone()),
            ])
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(control_lines)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(" Race control ")
                    .borders(Borders::ALL),
            ),
        control_area,
    );

    let weather = snapshot.weather.as_ref().map_or_else(
        || "Weather unavailable".to_string(),
        |weather| {
            format!(
                "Air {}°C · Track {}°C · Rain {}",
                weather
                    .air_temperature
                    .map_or("-".into(), |value| format!("{value:.1}")),
                weather
                    .track_temperature
                    .map_or("-".into(), |value| format!("{value:.1}")),
                weather.rainfall.map_or_else(
                    || "-".to_string(),
                    |value| if value { "yes".to_string() } else { "no".to_string() },
                )
            )
        },
    );
    frame.render_widget(
        Paragraph::new(weather).block(Block::default().title(" Weather ").borders(Borders::ALL)),
        weather_area,
    );
}

fn format_lap(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor() as u32;
    let remainder = seconds - (minutes as f64 * 60.0);
    format!("{minutes}:{remainder:06.3}")
}
