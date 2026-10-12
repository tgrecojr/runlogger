use super::render;
use crate::app::{App, Screen};
use crate::models::run::Run;
use chrono::{NaiveDate, NaiveTime};
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    style::{Color, Modifier},
    Terminal,
};

fn draw(app: &mut App) -> Buffer {
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
    let buf = draw(&mut app);

    assert_bold_red(&buf, "Could not load runs: boom");
    assert!(!text(&buf).contains("No runs logged yet"));
}

// @spec RL-004
#[test]
fn run_list_empty_history_message_shows_after_successful_load() {
    let mut app = App::new();
    app.switch_to_screen(Screen::RunList);
    assert!(text(&draw(&mut app)).contains("No runs logged yet"));
}

// @spec AN-014, AN-013
#[test]
fn analytics_error_replaces_all_panels() {
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);
    app.analytics_state.error_message = Some("Could not load analytics: boom".into());
    let buf = draw(&mut app);
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

fn many_runs(n: i64) -> Vec<Run> {
    let first = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    (0..n)
        .map(|i| {
            let date = first + chrono::Duration::days(i);
            let time = NaiveTime::from_hms_opt(7, 0, 0).unwrap();
            let mut run = Run::new(date, time, 1.0, None).unwrap();
            run.id = Some(i + 1);
            run
        })
        .collect()
}

// @spec RL-007
#[test]
fn run_list_scrolls_to_keep_the_selection_visible() {
    let mut app = App::new();
    app.switch_to_screen(Screen::RunList);
    app.run_list_state.runs = many_runs(40);
    let date_of = |app: &App, i: usize| {
        app.run_list_state.runs[i]
            .date
            .format("%Y-%m-%d")
            .to_string()
    };

    for i in [35, 39, 20, 0] {
        app.run_list_state.selected_index = i;
        let expected = date_of(&app, i);
        let screen = text(&draw(&mut app));
        assert!(
            screen.contains(&expected),
            "selected run {i} ({expected}) not visible:\n{screen}"
        );
    }
}
