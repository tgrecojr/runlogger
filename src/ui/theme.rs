use ratatui::style::{Color, Modifier, Style};

pub fn focused_input_style() -> Style {
    Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD)
}

pub fn unfocused_input_style() -> Style {
    Style::default().fg(Color::White)
}

pub fn error_style() -> Style {
    Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::BOLD)
}

pub fn success_style() -> Style {
    Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD)
}

pub fn stat_style() -> Style {
    Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}

pub fn goal_met_style() -> Style {
    Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD)
}

pub fn goal_not_met_style() -> Style {
    Style::default()
        .fg(Color::Red)
}
