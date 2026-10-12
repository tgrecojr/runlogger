pub mod components;
pub mod screens;
/// While Esc waits for its command key, the footer lists the commands.
// @spec NAV-020
fn render_esc_menu(f: &mut Frame, area: Rect, current_screen: Screen) {
    let mut menu = String::from("Esc → [1] Quick Entry  [2] Run List  [3] Analytics  [h] Help");
    if current_screen == Screen::QuickEntry {
        menu.push_str("  [Esc] Clear");
    }
    let footer = Paragraph::new(menu)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}

#[cfg(test)]
mod tests;
pub mod theme;

use crate::app::{App, Screen};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
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
    render_footer(f, chunks[2], app.screen, app.waiting_for_nav);
}

// @spec NAV-013, NAV-016
fn render_header(f: &mut Frame, area: Rect, current_screen: Screen) {
    // On Quick Entry single keys type text, so only the Esc form is true there.
    let prefix = if current_screen == Screen::QuickEntry {
        "Esc "
    } else {
        ""
    };
    let tabs = [
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
                Span::styled(format!("[{prefix}{key}]"), style),
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

fn render_screen(f: &mut Frame, area: Rect, app: &mut App) {
    match app.screen {
        Screen::QuickEntry => screens::quick_entry::render(f, area, &app.quick_entry_state),
        Screen::RunList => screens::run_list::render(f, area, &mut app.run_list_state),
        Screen::Analytics => screens::analytics::render(f, area, &app.analytics_state),
        Screen::Help => screens::help::render(f, area, app.db_location.as_deref()),
    }
}

// @spec NAV-014, NAV-016
fn render_footer(f: &mut Frame, area: Rect, current_screen: Screen, waiting_for_nav: bool) {
    if waiting_for_nav {
        render_esc_menu(f, area, current_screen);
        return;
    }
    let footer_text = match current_screen {
        Screen::QuickEntry => {
            "[Tab] Field  [Enter] Save  [Esc Esc] Clear  [Esc 1-3/h] Screen/Help  [^Q] Quit"
        }
        Screen::RunList => "[↑↓] Navigate  [e] Edit  [d] Delete  [1-3] Screen  [h] Help  [q] Quit",
        Screen::Analytics => "[1-3] Screen  [h] Help  [q] Quit",
        Screen::Help => "[1-3] Screen  [q] Quit",
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
