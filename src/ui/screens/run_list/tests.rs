//! Tests for Run List screen rendering.

use super::render;
use crate::app::RunListState;
use crate::models::run::Run;
use crate::test_support::{buffer_text, locate};
use chrono::{NaiveDate, NaiveTime};
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    style::{Color, Modifier},
    Terminal,
};

/// Renders only the Run List screen (no app header/footer) at 100x30.
fn draw(state: &mut RunListState) -> Buffer {
    let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
    terminal.draw(|f| render(f, f.area(), state)).unwrap();
    terminal.backend().buffer().clone()
}

fn run(
    id: i64,
    date: (i32, u32, u32),
    time: (u32, u32, u32),
    miles: f64,
    note: Option<&str>,
) -> Run {
    let date = NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap();
    let time = NaiveTime::from_hms_opt(time.0, time.1, time.2).unwrap();
    let mut run = Run::new(date, time, miles, note.map(String::from)).unwrap();
    run.id = Some(id);
    run
}

fn three_runs() -> RunListState {
    let mut state = RunListState::new();
    state.runs = vec![
        run(3, (2026, 10, 3), (6, 5, 9), 3.1, Some("tempo run")),
        run(2, (2026, 10, 2), (19, 45, 0), 2.456, None),
        run(1, (2026, 9, 30), (7, 0, 0), 10.0, Some("long")),
    ];
    state
}

/// The text of screen row `y`, trimmed of the table borders and padding.
fn row_text(buf: &Buffer, y: u16) -> String {
    let line = buffer_text(buf)
        .lines()
        .nth(y as usize)
        .unwrap()
        .to_string();
    line.trim_matches(|c: char| c == '│' || c.is_whitespace())
        .to_string()
}

/// Splits a row on runs of 2+ spaces into its visible cells.
fn cells(row: &str) -> Vec<String> {
    row.split("  ")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

fn assert_style(buf: &Buffer, needle: &str, fg: Color, bold: bool) {
    let cell = &buf[locate(buf, needle)];
    assert_eq!(cell.fg, fg, "{needle:?} colour");
    assert_eq!(
        cell.modifier.contains(Modifier::BOLD),
        bold,
        "{needle:?} bold"
    );
}

// @spec RL-002
#[test]
fn table_is_titled_with_run_count() {
    let mut state = three_runs();
    let screen = buffer_text(&draw(&mut state));
    assert!(screen.contains("Run List (3 runs)"), "{screen}");

    state.runs.truncate(1);
    let screen = buffer_text(&draw(&mut state));
    assert!(screen.contains("Run List (1 runs)"), "{screen}");
}

// @spec RL-002
#[test]
fn table_has_date_time_distance_and_note_columns() {
    let mut state = three_runs();
    let buf = draw(&mut state);

    let (_, header_y) = locate(&buf, "Date");
    assert_eq!(
        cells(&row_text(&buf, header_y)),
        ["Date", "Time", "Distance (mi)", "Note"]
    );

    let (_, y) = locate(&buf, "2026-10-03");
    assert_eq!(
        cells(&row_text(&buf, y)),
        ["2026-10-03", "06:05:09", "3.10", "tempo run"]
    );
    let (_, y) = locate(&buf, "2026-09-30");
    assert_eq!(
        cells(&row_text(&buf, y)),
        ["2026-09-30", "07:00:00", "10.00", "long"]
    );
}

// @spec RL-002
#[test]
fn distance_rounds_to_two_places_and_missing_note_is_blank() {
    let mut state = three_runs();
    let buf = draw(&mut state);

    let (_, y) = locate(&buf, "2026-10-02");
    assert_eq!(
        cells(&row_text(&buf, y)),
        ["2026-10-02", "19:45:00", "2.46"],
        "note column must be blank"
    );
}

// @spec RL-003
#[test]
fn selected_row_is_bold_yellow_and_others_are_not() {
    let mut state = three_runs();
    state.selected_index = 1;
    let buf = draw(&mut state);

    for needle in ["2026-10-02", "19:45:00", "2.46"] {
        assert_style(&buf, needle, Color::Yellow, true);
    }
    for needle in ["2026-10-03", "tempo run", "2026-09-30", "10.00"] {
        assert_style(&buf, needle, Color::Reset, false);
    }

    state.selected_index = 0;
    let buf = draw(&mut state);
    for needle in ["2026-10-03", "06:05:09", "3.10", "tempo run"] {
        assert_style(&buf, needle, Color::Yellow, true);
    }
    assert_style(&buf, "2026-10-02", Color::Reset, false);
}

const HINTS: &str = "[↑↓] Navigate  [e] Edit  [d] Delete  [Esc→1/2/3] Switch screens";

// @spec RL-005
#[test]
fn actions_panel_lists_hints_below_the_table() {
    let mut state = three_runs();
    let buf = draw(&mut state);

    let (_, table_y) = locate(&buf, "Run List (3 runs)");
    let (_, last_row_y) = locate(&buf, "2026-09-30");
    let (_, actions_y) = locate(&buf, "Actions");
    let (_, hints_y) = locate(&buf, HINTS);
    assert!(actions_y > table_y && actions_y > last_row_y);
    assert_eq!(hints_y, actions_y + 1, "hints sit inside the Actions panel");
}

// @spec RL-005
#[test]
fn actions_panel_is_shown_when_the_history_is_empty() {
    let mut state = RunListState::new();
    let buf = draw(&mut state);

    let (_, empty_y) = locate(&buf, "No runs logged yet.");
    let (_, actions_y) = locate(&buf, "Actions");
    let (_, hints_y) = locate(&buf, HINTS);
    assert!(actions_y > empty_y);
    assert_eq!(hints_y, actions_y + 1);
}
