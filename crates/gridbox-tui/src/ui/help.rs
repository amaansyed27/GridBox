use crate::ui::theme;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame<'_>, area: Rect) {
    let commands = [
        (
            "/driver <number>",
            "Focus a driver in the local demo analysis",
        ),
        ("/schedule <year>", "Load a Jolpica season schedule"),
        (
            "/session <year> <event> <session>",
            "Load a completed FastF1 session summary",
        ),
        (
            "/compare <year> <event> <session> <drivers…>",
            "Compare fastest laps from FastF1",
        ),
        ("/model <ollama-model>", "Change the local Ollama model"),
        ("/clear", "Clear the engineer conversation"),
        ("/quit", "Exit GridBox"),
    ];

    let mut lines = vec![Line::styled("Commands", theme::accent()), Line::raw("")];
    for (command, detail) in commands {
        lines.push(Line::from(vec![
            Span::styled(format!("{command:<48}"), theme::accent()),
            Span::raw(detail),
        ]));
    }
    lines.extend([
        Line::raw(""),
        Line::styled("Keyboard", theme::accent()),
        Line::raw("F1–F5 switch views · Tab cycles views · Ctrl+L clears input · Ctrl+Q exits"),
        Line::raw(""),
        Line::styled("Notes", theme::accent()),
        Line::raw("Run `gridbox demo-live` to exercise the moving timing and strategy views."),
        Line::raw("Event names in slash commands currently use one token, e.g. AbuDhabi."),
        Line::raw("FastF1 is used for completed-session timing and telemetry analysis."),
        Line::raw("All AI inference is sent only to the configured localhost model server."),
    ]);

    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(Block::default().title(" Help ").borders(Borders::ALL)),
        area,
    );
}
