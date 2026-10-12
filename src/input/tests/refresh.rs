use super::*;
use chrono::NaiveDate;

fn day(d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(2025, 6, d).unwrap()
}

// @spec AN-008
#[test]
fn analytics_recompute_when_the_date_changes_while_shown() {
    let conn = memory_db();
    insert_raw(&conn, "2025-06-15", "07:00:00", 2.0);
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);
    load_analytics_for(&mut app, &conn, day(15));
    assert_eq!(app.analytics_state.analytics.current_streak, 1);

    // Same day: nothing to do.
    refresh_analytics_if_stale(&mut app, &conn, day(15));
    assert_eq!(app.analytics_state.computed_for, Some(day(15)));

    // Two days later the streak has lapsed.
    refresh_analytics_if_stale(&mut app, &conn, day(17));
    assert_eq!(app.analytics_state.computed_for, Some(day(17)));
    assert_eq!(app.analytics_state.analytics.current_streak, 0);
}

// @spec AN-008
#[test]
fn analytics_are_not_recomputed_while_another_screen_is_shown() {
    let conn = memory_db();
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);
    load_analytics_for(&mut app, &conn, day(15));
    app.switch_to_screen(Screen::RunList);
    refresh_analytics_if_stale(&mut app, &conn, day(17));
    assert_eq!(app.analytics_state.computed_for, Some(day(15)));
}
