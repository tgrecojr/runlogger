use super::render;
use crate::app::{App, Screen};
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    style::{Color, Modifier},
    Terminal,
};

fn draw(app: &App) -> Buffer {
    let mut terminal = Terminal::new(TestBackend::new(100, 50)).unwrap();
    terminal.draw(|f| render(f, app)).unwrap();
    terminal.backend().buffer().clone()
}

fn text(buf: &Buffer) -> String {
    (0..buf.area.height)
        .map(|y| {
            (0..buf.area.width)
                .map(|x| buf[(x, y)].symbol())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Asserts `needle` is on screen and its first cell is bold red.
fn assert_bold_red(buf: &Buffer, needle: &str) {
    let screen = text(buf);
    let line_len = buf.area.width as usize + 1;
    let at = screen
        .find(needle)
        .unwrap_or_else(|| panic!("{needle:?} not on screen:\n{screen}"));
    let prefix_chars = screen[..at].chars().count();
    let (x, y) = (prefix_chars % line_len, prefix_chars / line_len);
    let cell = &buf[(x as u16, y as u16)];
    assert_eq!(cell.fg, Color::Red, "{needle:?} is not red");
    assert!(
        cell.modifier.contains(Modifier::BOLD),
        "{needle:?} is not bold"
    );
}

// @spec RL-012, RL-004
#[test]
fn run_list_load_error_shows_in_bold_red_without_empty_history_message() {
    let mut app = App::new();
    app.switch_to_screen(Screen::RunList);
    app.run_list_state.error_message = Some("Could not load runs: boom".into());
    let buf = draw(&app);

    assert_bold_red(&buf, "Could not load runs: boom");
    assert!(!text(&buf).contains("No runs logged yet"));
}

// @spec RL-004
#[test]
fn run_list_empty_history_message_shows_after_successful_load() {
    let mut app = App::new();
    app.switch_to_screen(Screen::RunList);
    assert!(text(&draw(&app)).contains("No runs logged yet"));
}

// @spec AN-014, AN-013
#[test]
fn analytics_error_replaces_all_panels() {
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);
    app.analytics_state.error_message = Some("Could not load analytics: boom".into());
    let buf = draw(&app);
    let screen = text(&buf);

    assert_bold_red(&buf, "Could not load analytics: boom");
    for panel in [
        "Streaks",
        "Totals",
        "Daily Mileage",
        "Mileage by Month",
        "No data to display",
    ] {
        assert!(!screen.contains(panel), "{panel:?} still shown:\n{screen}");
    }
}
