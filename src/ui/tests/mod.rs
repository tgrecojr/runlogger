use super::render;
use crate::app::{App, Screen};
use crate::models::analytics::{Analytics, DailyData, MonthlyData};
use crate::models::run::Run;
use crate::test_support::{buffer_text, draw_app, locate};
use chrono::{NaiveDate, NaiveTime};
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    style::{Color, Modifier},
    Terminal,
};

mod analytics;
mod chrome;
mod run_list;

fn draw(app: &mut App) -> Buffer {
    draw_app(app, 100, 50)
}

fn text(buf: &Buffer) -> String {
    buffer_text(buf)
}

/// Asserts `needle` is on screen and its first cell is bold red.
fn assert_bold_red(buf: &Buffer, needle: &str) {
    let cell = &buf[locate(buf, needle)];
    assert_eq!(cell.fg, Color::Red, "{needle:?} is not red");
    assert!(
        cell.modifier.contains(Modifier::BOLD),
        "{needle:?} is not bold"
    );
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

fn analytics_screen(today_goal_met: bool) -> App {
    let first = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
    let mut recent_trend: Vec<DailyData> = (0..14)
        .map(|i| DailyData {
            date: first + chrono::Duration::days(i),
            distance: 0.0,
        })
        .collect();
    recent_trend[13].distance = 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1; // 06/15
    recent_trend[12].distance = 0.004; // 06/14
    recent_trend[11].distance = 0.5; // 06/13
    recent_trend[10].distance = 0.995; // 06/12: shown and judged at the same precision
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);
    app.analytics_state.analytics = Analytics {
        current_streak: 3,
        today_goal_met,
        recent_trend,
        monthly_breakdown: vec![MonthlyData {
            year: 2025,
            month: 6,
            total_distance: 42.3,
            run_count: 14,
            average_distance: 3.0,
        }],
        ..Analytics::empty()
    };
    app
}

/// Style of a day's distance on the Last 14 Days chart (`MM/DD` + 7 columns).
fn day_distance_cell(buf: &Buffer, day: &str) -> ratatui::buffer::Cell {
    let (x, y) = locate(buf, day);
    buf[(x + 7, y)].clone()
}

fn screen_on(screen: Screen, width: u16) -> String {
    let mut app = App::new();
    app.switch_to_screen(screen);
    let mut terminal = Terminal::new(TestBackend::new(width, 40)).unwrap();
    terminal.draw(|f| render(f, &mut app)).unwrap();
    text(terminal.backend().buffer())
}

const FOOTERS: [(Screen, &str); 4] = [
    (
        Screen::QuickEntry,
        "[Tab] Field  [Enter] Save  [Esc Esc] Clear  [Esc 1-3/h] Screen/Help  [^Q] Quit",
    ),
    (
        Screen::RunList,
        "[↑↓] Navigate  [e] Edit  [d] Delete  [1-3] Screen  [h] Help  [q] Quit",
    ),
    (Screen::Analytics, "[1-3] Screen  [h] Help  [q] Quit"),
    (Screen::Help, "[1-3] Screen  [q] Quit"),
];
