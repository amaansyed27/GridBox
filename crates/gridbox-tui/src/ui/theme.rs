use ratatui::style::{Color, Modifier, Style};

pub fn accent() -> Style {
    Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}

pub fn active() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}

pub fn muted() -> Style {
    Style::default().fg(Color::DarkGray)
}

pub fn warning() -> Style {
    Style::default().fg(Color::Yellow)
}

pub fn critical() -> Style {
    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
}
