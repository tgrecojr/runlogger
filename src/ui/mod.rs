pub mod components;
pub mod screens;
pub mod theme;

use crate::app::{App, Screen};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_header(f, chunks[0], app.screen);
    render_screen(f, chunks[1], app);
    render_footer(f, chunks[2], app.screen);
}

fn render_header(f: &mut Frame, area: Rect, current_screen: Screen) {
    let tabs = vec![
        ("1", "Quick Entry", current_screen == Screen::QuickEntry),
        ("2", "Run List", current_screen == Screen::RunList),
        ("3", "Analytics", current_screen == Screen::Analytics),
        ("h", "Help", current_screen == Screen::Help),
    ];

    let spans: Vec<Span> = tabs
        .iter()
        .flat_map(|(key, label, active)| {
            let style = if *active {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            vec![
                Span::styled(format!("[{}]", key), style),
                Span::raw(" "),
                Span::styled(*label, style),
                Span::raw("  "),
            ]
        })
        .collect();

    let header = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title("Run Logger"));

    f.render_widget(header, area);
}

fn render_screen(f: &mut Frame, area: Rect, app: &App) {
    match app.screen {
        Screen::QuickEntry => screens::quick_entry::render(f, area, &app.quick_entry_state),
        Screen::RunList => screens::run_list::render(f, area, &app.run_list_state),
        Screen::Analytics => screens::analytics::render(f, area, &app.analytics_state),
        Screen::Help => screens::help::render(f, area),
    }
}

fn render_footer(f: &mut Frame, area: Rect, current_screen: Screen) {
    let footer_text = match current_screen {
        Screen::QuickEntry => "[Tab] Next  [Shift+Tab] Prev  [Enter] Submit  [Esc] Clear  [h] Help  [Ctrl+Q] Quit",
        Screen::RunList => "[↑↓] Navigate  [1-3] Switch screen  [h] Help  [q] Quit",
        Screen::Analytics => "[1-3] Switch screen  [h] Help  [q] Quit",
        Screen::Help => "[1-3] Switch screen  [q] Quit",
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
