use super::render;
use crate::app::{App, Screen};
use crate::models::analytics::{Analytics, DailyData, MonthlyData};
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

/// Screen position of the first cell of `needle`.
fn locate(buf: &Buffer, needle: &str) -> (u16, u16) {
    let screen = text(buf);
    let line_len = buf.area.width as usize + 1;
    let at = screen
        .find(needle)
        .unwrap_or_else(|| panic!("{needle:?} not on screen:\n{screen}"));
    let prefix_chars = screen[..at].chars().count();
    (
        (prefix_chars % line_len) as u16,
        (prefix_chars / line_len) as u16,
    )
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

// @spec AN-011
#[test]
fn chart_shows_fourteen_days_colored_by_rounded_total() {
    let mut app = analytics_screen(true);
    let buf = draw(&mut app);
    let screen = text(&buf);

    assert!(screen.contains("Last 14 Days"), "{screen}");
    for day in 2..=15 {
        let label = format!("06/{day:02}");
        assert!(screen.contains(&label), "{label} missing:\n{screen}");
    }
    let met = day_distance_cell(&buf, "06/15");
    assert_eq!(
        (met.fg, met.modifier.contains(Modifier::BOLD)),
        (Color::Green, true)
    );
    assert_eq!(day_distance_cell(&buf, "06/13").fg, Color::Yellow);
    assert_eq!(day_distance_cell(&buf, "06/14").fg, Color::Red);
}

// @spec AN-011, AN-012
#[test]
fn chart_sits_beside_a_46_column_monthly_panel() {
    let mut app = analytics_screen(true);
    let buf = draw(&mut app);
    let (chart_x, chart_y) = locate(&buf, "Last 14 Days");
    let (month_x, month_y) = locate(&buf, "Mileage by Month");
    assert_eq!(chart_y, month_y, "panels are not side by side");
    assert!(month_x > chart_x);

    let line = text(&buf)
        .lines()
        .nth(month_y as usize)
        .unwrap()
        .to_string();
    let chars: Vec<char> = line.chars().collect();
    let left = (month_x - 1) as usize; // the corner before the title
    let right = (left..chars.len()).find(|&i| chars[i] == '┐').unwrap();
    assert_eq!(right - left + 1, 46, "monthly panel width");
}

// @spec AN-016
#[test]
fn streak_shows_run_today_hint_until_today_is_goal_met() {
    let mut pending = analytics_screen(false);
    let buf = draw(&mut pending);
    assert_eq!(buf[locate(&buf, "(run today to extend)")].fg, Color::Yellow);

    let mut done = analytics_screen(true);
    assert!(!text(&draw(&mut done)).contains("(run today to extend)"));
}

// @spec AN-011, AN-012
#[test]
fn short_terminal_keeps_today_and_the_full_monthly_header() {
    let mut app = analytics_screen(true);
    let mut terminal = Terminal::new(TestBackend::new(80, 30)).unwrap();
    terminal.draw(|f| render(f, &mut app)).unwrap();
    let screen = text(terminal.backend().buffer());

    assert!(screen.contains("06/15"), "today missing:\n{screen}");
    assert!(
        !screen.contains("06/02"),
        "oldest day shown instead of newest:\n{screen}"
    );
    assert!(
        screen.contains("Monthly Totals (Green ≥50 mi, Yellow ≥25 mi)"),
        "{screen}"
    );
}
