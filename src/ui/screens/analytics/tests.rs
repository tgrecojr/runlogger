//! Tests for Analytics screen rendering.

use crate::app::{App, Screen};
use crate::input::handle_key_event;
use crate::models::analytics::{Analytics, MonthlyData};
use crate::test_support::{buffer_text, ch, draw_app, insert_raw, key, locate, memory_db};
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    style::{Color, Modifier},
};

/// Renders the Analytics screen showing `analytics`.
fn draw_analytics(analytics: Analytics) -> Buffer {
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);
    app.analytics_state.analytics = analytics;
    draw_app(&mut app, 100, 50)
}

/// Foreground color and boldness of the first cell of `value`, found right
/// after `label` so equal values elsewhere on screen cannot be picked up.
fn style_after(buf: &Buffer, label: &str, value: &str) -> (Color, bool) {
    let (x, y) = locate(buf, &format!("{label}{value}"));
    let cell = &buf[(x + label.chars().count() as u16, y)];
    (cell.fg, cell.modifier.contains(Modifier::BOLD))
}

const CURRENT: &str = "Current Streak: ";
const YEAR_GOAL: &str = "Year Goal (365 days @ 1mi): ";

// @spec AN-009
#[test]
fn current_streak_above_zero_is_bold_green_and_longest_streak_is_in_days() {
    let buf = draw_analytics(Analytics {
        current_streak: 3,
        longest_streak: 17,
        today_goal_met: true,
        ..Analytics::empty()
    });
    assert_eq!(style_after(&buf, CURRENT, "3 days"), (Color::Green, true));
    assert!(buffer_text(&buf).contains("Longest Streak: 17 days"));
}

// @spec AN-009
#[test]
fn current_streak_of_zero_is_red() {
    let buf = draw_analytics(Analytics {
        longest_streak: 4,
        ..Analytics::empty()
    });
    assert_eq!(style_after(&buf, CURRENT, "0 days").0, Color::Red);
}

// @spec AN-009
#[test]
fn year_goal_shows_days_remaining_and_percentage() {
    let buf = draw_analytics(Analytics {
        days_remaining_to_year_goal: 100,
        year_goal_completion_percentage: 72.6,
        ..Analytics::empty()
    });
    let (fg, _) = style_after(&buf, YEAR_GOAL, "100 days (72.6%)");
    assert_ne!(fg, Color::Yellow, "100 days remaining is not yet below 100");
    assert_ne!(fg, Color::Green);
}

// @spec AN-009
#[test]
fn year_goal_is_bold_yellow_when_fewer_than_100_days_remain() {
    let buf = draw_analytics(Analytics {
        days_remaining_to_year_goal: 99,
        year_goal_completion_percentage: 72.9,
        ..Analytics::empty()
    });
    assert_eq!(
        style_after(&buf, YEAR_GOAL, "99 days (72.9%)"),
        (Color::Yellow, true)
    );
}

// @spec AN-009
#[test]
fn year_goal_with_no_days_remaining_is_complete_in_bold_green() {
    let buf = draw_analytics(Analytics {
        days_remaining_to_year_goal: 0,
        year_goal_completion_percentage: 100.0,
        ..Analytics::empty()
    });
    let screen = buffer_text(&buf);
    let line = screen
        .lines()
        .find(|l| l.contains(YEAR_GOAL))
        .expect("year goal line");
    let compact: String = line.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        compact.contains("Goal Complete! 🎉 (100.0%)"),
        "year goal line: {line:?}"
    );
    assert_eq!(
        style_after(&buf, YEAR_GOAL, "Goal Complete!"),
        (Color::Green, true)
    );
    let (x, y) = locate(&buf, "(100.0%)");
    assert_eq!(buf[(x, y)].fg, Color::Green);
    assert!(buf[(x, y)].modifier.contains(Modifier::BOLD));
}

/// Asserts `needle` lies inside the bordered panel titled `title`.
fn assert_in_panel(buf: &Buffer, title: &str, needle: &str) {
    let (tx, ty) = locate(buf, title);
    let (nx, ny) = locate(buf, needle);
    assert!(ny > ty, "{needle:?} is not below the {title:?} title");
    // The panel's left border is the nearest `│` left of the title.
    let left = (0..tx)
        .rev()
        .find(|&x| buf[(x, ny)].symbol() == "│")
        .unwrap_or_else(|| panic!("no border left of {title:?}"));
    let right = (nx..buf.area.width)
        .find(|&x| buf[(x, ny)].symbol() == "│")
        .unwrap_or_else(|| panic!("no border right of {needle:?}"));
    let title_right = (tx..buf.area.width)
        .find(|&x| buf[(x, ty)].symbol() == "┐")
        .unwrap_or_else(|| panic!("no corner right of {title:?}"));
    assert!(
        nx > left && right == title_right,
        "{needle:?} is not inside the {title:?} panel"
    );
}

// @spec AN-010
#[test]
fn totals_week_and_period_panels_show_counts_and_two_decimal_distances() {
    let buf = draw_analytics(Analytics {
        total_runs: 42,
        total_distance: 123.456,
        average_distance: 2.9394,
        runs_this_week: 5,
        average_distance_this_week: 1.234,
        runs_this_month: 12,
        average_distance_this_month: 3.0,
        runs_this_year: 101,
        average_distance_this_year: 4.5678,
        ..Analytics::empty()
    });

    for needle in [
        "Total Runs: 42",
        "Total Distance: 123.46 mi",
        "Average: 2.94 mi",
    ] {
        assert_in_panel(&buf, "Totals", needle);
    }
    for needle in ["This Week: 5 runs", "Avg Distance: 1.23 mi"] {
        assert_in_panel(&buf, "Week", needle);
    }
    for needle in [
        "This Month: 12 runs",
        "Avg: 3.00 mi",
        "This Year: 101 runs",
        "Avg: 4.57 mi",
    ] {
        assert_in_panel(&buf, "Period", needle);
    }
    // Last 30 days come before the current year in the Period panel.
    assert!(locate(&buf, "This Month: ").1 < locate(&buf, "This Year: ").1);
}

/// A run date `days` from the real local today, as stored in the database.
fn local_date(days: i64) -> String {
    (chrono::Local::now().date_naive() + chrono::Duration::days(days))
        .format("%Y-%m-%d")
        .to_string()
}

// @spec AN-008
#[test]
fn opening_analytics_recomputes_from_the_full_history_as_of_today() {
    let conn = memory_db();
    insert_raw(&conn, "2001-03-04", "07:00:00", 2.0);
    // Far enough ahead to stay in the future even if midnight passes mid-test.
    insert_raw(&conn, &local_date(30), "07:00:00", 50.0);
    let mut app = App::new();

    // Esc → 3 from Quick Entry, where a bare `3` would be typed into the form.
    handle_key_event(&mut app, key(KeyCode::Esc), &conn);
    handle_key_event(&mut app, ch('3'), &conn);
    assert_eq!(app.screen, Screen::Analytics);
    assert_eq!(app.analytics_state.error_message, None);
    let a = &app.analytics_state.analytics;
    assert_eq!(
        a.total_runs, 1,
        "future runs are not part of today's history"
    );
    assert_eq!(a.total_distance, 2.0);
    let today_before = chrono::Local::now().date_naive();

    // A run added later is picked up when the screen is opened again with `3`.
    insert_raw(&conn, "2001-03-05", "07:00:00", 3.0);
    handle_key_event(&mut app, ch('2'), &conn);
    assert_eq!(app.screen, Screen::RunList);
    handle_key_event(&mut app, ch('3'), &conn);
    assert_eq!(app.screen, Screen::Analytics);
    let a = &app.analytics_state.analytics;
    assert_eq!(a.total_runs, 2);
    assert_eq!(a.total_distance, 5.0);
    assert_eq!(a.longest_streak, 2);
    let trend_end = a.recent_trend.last().expect("trend").date;
    let today_after = chrono::Local::now().date_naive();
    assert!(
        trend_end == today_before || trend_end == today_after,
        "trend ends {trend_end}, not today"
    );
    assert!(buffer_text(&draw_app(&mut app, 100, 50)).contains("Total Runs: 2"));
}

fn month(month: u32, total: f64) -> MonthlyData {
    MonthlyData {
        year: 2025,
        month,
        total_distance: total,
        run_count: 10,
        average_distance: total / 10.0,
    }
}

// @spec AN-012
#[test]
fn month_colors_follow_the_displayed_total() {
    let buf = draw_analytics(Analytics {
        monthly_breakdown: vec![month(6, 49.96), month(5, 24.96), month(4, 24.94)],
        ..Analytics::empty()
    });
    let screen = buffer_text(&buf);
    let total_cell = |label: &str| {
        let (x, y) = locate(&buf, label);
        let line: Vec<char> = screen.lines().nth(y as usize).unwrap().chars().collect();
        let offset = (x as usize..line.len())
            .find(|&i| line[i].is_ascii_digit() && i > x as usize + 8)
            .unwrap();
        (
            line[offset..offset + 4].iter().collect::<String>(),
            buf[(offset as u16, y)].clone(),
        )
    };

    let (june, june_cell) = total_cell("Jun 2025");
    assert_eq!(june, "50.0");
    assert_eq!(
        (june_cell.fg, june_cell.modifier.contains(Modifier::BOLD)),
        (Color::Green, true)
    );
    let (may, may_cell) = total_cell("May 2025");
    assert_eq!(may, "25.0");
    assert_eq!(may_cell.fg, Color::Yellow);
    let (april, april_cell) = total_cell("Apr 2025");
    assert_eq!(april, "24.9");
    assert_eq!(april_cell.fg, Color::White);
}
