use super::*;
use chrono::NaiveTime;

/// A fixed "today", deliberately not the real current date.
fn today() -> NaiveDate {
    NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()
}

fn days_ago(n: i64) -> NaiveDate {
    today() - chrono::Duration::days(n)
}

fn run(date: NaiveDate, miles: f64) -> Run {
    Run::new(date, NaiveTime::from_hms_opt(7, 0, 0).unwrap(), miles, None).unwrap()
}

// @spec AN-001
#[test]
fn goal_is_judged_on_the_total_rounded_to_hundredths() {
    assert!(goal_met(
        0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
    ));
    assert!(goal_met(1.0));
    assert!(goal_met(0.996));
    assert!(!goal_met(0.994));
    assert!(!goal_met(0.99));
}

// @spec AN-001
#[test]
fn ten_tenth_mile_runs_meet_the_goal_for_the_day() {
    let runs: Vec<Run> = (0..10).map(|_| run(today(), 0.1)).collect();
    let a = calculate_analytics(&runs, today());
    assert_eq!(a.current_streak, 1);
    assert_eq!(a.longest_streak, 1);
}

// @spec AN-002
#[test]
fn unfinished_today_does_not_break_the_streak() {
    let runs = vec![
        run(days_ago(1), 2.0),
        run(days_ago(2), 2.0),
        run(today(), 0.5),
    ];
    let a = calculate_analytics(&runs, today());
    assert_eq!(a.current_streak, 2);
    assert!(!a.today_goal_met);
}

// @spec AN-002
#[test]
fn finished_today_extends_the_streak() {
    let runs = vec![run(days_ago(1), 2.0), run(today(), 1.0)];
    let a = calculate_analytics(&runs, today());
    assert_eq!(a.current_streak, 2);
    assert!(a.today_goal_met);
}

// @spec AN-002
#[test]
fn a_missed_yesterday_breaks_the_streak() {
    let runs = vec![run(days_ago(2), 2.0), run(days_ago(3), 2.0)];
    assert_eq!(calculate_analytics(&runs, today()).current_streak, 0);
}

// @spec AN-003, AN-015
#[test]
fn longest_streak_ignores_future_runs() {
    let future: Vec<Run> = (1..=5)
        .map(|n| run(today() + chrono::Duration::days(n), 2.0))
        .collect();
    let mut runs = vec![run(days_ago(1), 2.0), run(days_ago(2), 2.0)];
    runs.extend(future);
    assert_eq!(calculate_analytics(&runs, today()).longest_streak, 2);
}

// @spec AN-004, AN-015
#[test]
fn year_goal_counts_goal_met_days_this_year_through_today() {
    let runs = vec![
        run(today(), 1.0),
        run(days_ago(1), 1.0),
        run(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(), 5.0),
        run(today() + chrono::Duration::days(1), 5.0),
    ];
    let a = calculate_analytics(&runs, today());
    assert_eq!(a.days_remaining_to_year_goal, 363);
}

// @spec AN-005, AN-015
#[test]
fn totals_ignore_future_runs() {
    let runs = vec![
        run(today(), 2.0),
        run(today() + chrono::Duration::days(3), 10.0),
    ];
    let a = calculate_analytics(&runs, today());
    assert_eq!(a.total_runs, 1);
    assert_eq!(a.total_distance, 2.0);
}

// @spec AN-006, AN-015
#[test]
fn period_windows_end_today() {
    let runs = vec![
        run(today(), 2.0),
        run(days_ago(6), 4.0),
        run(days_ago(7), 8.0),
        run(today() + chrono::Duration::days(2), 100.0),
    ];
    let a = calculate_analytics(&runs, today());
    assert_eq!(a.runs_this_week, 2);
    assert_eq!(a.average_distance_this_week, 3.0);
    assert_eq!(a.runs_this_month, 3);
    assert_eq!(a.runs_this_year, 3);
}

// @spec AN-011
#[test]
fn recent_trend_is_the_fourteen_dates_ending_today() {
    let a = calculate_analytics(&[run(days_ago(20), 1.0)], today());
    assert_eq!(a.recent_trend.len(), 14);
    assert_eq!(a.recent_trend.first().unwrap().date, days_ago(13));
    assert_eq!(a.recent_trend.last().unwrap().date, today());
}

// @spec AN-013, AN-015
#[test]
fn only_future_runs_is_treated_as_an_empty_history() {
    let runs = vec![run(today() + chrono::Duration::days(1), 3.0)];
    let a = calculate_analytics(&runs, today());
    assert_eq!(a.total_runs, 0);
    assert_eq!(a.days_remaining_to_year_goal, 365);
    assert!(a.recent_trend.is_empty());
    assert!(a.monthly_breakdown.is_empty());
}
