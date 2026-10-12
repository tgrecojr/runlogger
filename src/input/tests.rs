use super::*;
use crate::test_support::{block_writes, ch, insert_raw, key, memory_db};

fn press(app: &mut App, conn: &Connection, keys: &[KeyEvent]) {
    for k in keys {
        handle_key_event(app, *k, conn);
    }
}

fn run_count(conn: &Connection) -> i64 {
    conn.query_row("SELECT COUNT(*) FROM runs", [], |r| r.get(0))
        .unwrap()
}

fn rl_error(app: &App) -> &str {
    app.run_list_state.error_message.as_deref().unwrap_or("")
}

// @spec RL-012, NAV-017
#[test]
fn opening_run_list_with_unreadable_history_shows_error_and_keeps_running() {
    let conn = memory_db();
    insert_raw(&conn, "2026-13-45", "07:00:00", 1.0);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2')]);

    assert_eq!(app.screen, Screen::RunList);
    assert!(app.run_list_state.runs.is_empty());
    assert!(
        rl_error(&app).starts_with("Could not load runs: "),
        "{}",
        rl_error(&app)
    );
    assert!(rl_error(&app).contains("2026-13-45"), "{}", rl_error(&app));
    assert!(!app.should_quit);
}

// @spec RL-012, NAV-017
#[test]
fn direct_switch_to_run_list_with_unreadable_history_shows_error() {
    let conn = memory_db();
    insert_raw(&conn, "2026-13-45", "07:00:00", 1.0);
    let mut app = App::new();
    app.switch_to_screen(Screen::Help);
    press(&mut app, &conn, &[ch('2')]);

    assert_eq!(app.screen, Screen::RunList);
    assert!(rl_error(&app).starts_with("Could not load runs: "));
}

// @spec RL-012
#[test]
fn run_list_error_clears_on_next_successful_load() {
    let conn = memory_db();
    let bad = insert_raw(&conn, "2026-13-45", "07:00:00", 1.0);
    insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2')]);
    conn.execute("DELETE FROM runs WHERE id = ?1", [bad])
        .unwrap();
    press(&mut app, &conn, &[ch('2')]);

    assert_eq!(app.run_list_state.error_message, None);
    assert_eq!(app.run_list_state.runs.len(), 1);
}

// @spec RL-012, QE-018
#[test]
fn failed_reload_after_edit_switches_to_run_list_with_form_reset() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2'), ch('e')]);
    assert!(app.quick_entry_state.is_editing());
    insert_raw(&conn, "2026-13-45", "07:00:00", 1.0);
    press(&mut app, &conn, &[key(KeyCode::Enter)]);

    assert_eq!(app.screen, Screen::RunList);
    assert!(
        rl_error(&app).starts_with("Could not load runs: "),
        "{}",
        rl_error(&app)
    );
    assert!(!app.quick_entry_state.is_editing());
    assert!(app.quick_entry_state.distance.is_empty());
}

// @spec RL-012
#[test]
fn failed_reload_after_successful_delete_shows_load_error() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-02", "07:00:00", 2.0);
    insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2')]);
    insert_raw(&conn, "2026-13-45", "07:00:00", 1.0);
    press(&mut app, &conn, &[ch('d')]);

    assert_eq!(run_count(&conn), 2);
    assert!(app.run_list_state.runs.is_empty());
    assert!(
        rl_error(&app).starts_with("Could not load runs: "),
        "{}",
        rl_error(&app)
    );
    assert!(!app.should_quit);
}

// @spec RL-013, NAV-017
#[test]
fn failed_delete_keeps_list_and_selection_and_shows_error() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-02", "07:00:00", 2.0);
    insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    let mut app = App::new();
    press(
        &mut app,
        &conn,
        &[key(KeyCode::Esc), ch('2'), key(KeyCode::Down)],
    );
    block_writes(&conn, "DELETE");
    press(&mut app, &conn, &[ch('d')]);

    assert_eq!(app.run_list_state.runs.len(), 2);
    assert_eq!(app.run_list_state.selected_index, 1);
    assert!(
        rl_error(&app).starts_with("Could not delete run: "),
        "{}",
        rl_error(&app)
    );
    assert!(
        rl_error(&app).contains("DELETE blocked"),
        "{}",
        rl_error(&app)
    );
    assert!(!app.should_quit);
}

// @spec AN-014, NAV-017
#[test]
fn opening_analytics_with_unreadable_history_shows_error_and_keeps_running() {
    let conn = memory_db();
    let bad = insert_raw(&conn, "2026-13-45", "07:00:00", 1.0);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('3')]);

    assert_eq!(app.screen, Screen::Analytics);
    let msg = app
        .analytics_state
        .error_message
        .clone()
        .unwrap_or_default();
    assert!(msg.starts_with("Could not load analytics: "), "{msg}");
    assert!(!app.should_quit);

    conn.execute("DELETE FROM runs WHERE id = ?1", [bad])
        .unwrap();
    press(&mut app, &conn, &[ch('3')]);
    assert_eq!(app.analytics_state.error_message, None);
}

fn qe(app: &App) -> &crate::app::QuickEntryState {
    &app.quick_entry_state
}

fn type_text(app: &mut App, conn: &Connection, text: &str) {
    for c in text.chars() {
        handle_key_event(app, ch(c), conn);
    }
}

// @spec QE-016
#[test]
fn time_refreshes_to_now_after_a_new_run_is_saved() {
    let conn = memory_db();
    let mut app = App::new();
    app.quick_entry_state.date = "2026-10-01".into();
    app.quick_entry_state.time = "00:00:01".into();
    type_text(&mut app, &conn, "2");
    press(&mut app, &conn, &[key(KeyCode::Enter)]);

    assert_eq!(run_count(&conn), 1);
    assert_eq!(qe(&app).date, "2026-10-01");
    let refreshed = chrono::NaiveTime::parse_from_str(&qe(&app).time, "%H:%M:%S")
        .unwrap_or_else(|_| panic!("not HH:MM:SS: {:?}", qe(&app).time));
    let now = chrono::Local::now().naive_local().time();
    assert!(
        (now - refreshed).num_seconds().abs() <= 2,
        "{refreshed} vs {now}"
    );
}

// @spec QE-024, QE-014
#[test]
fn future_date_is_rejected_before_distance_and_nothing_is_saved() {
    let conn = memory_db();
    let mut app = App::new();
    let tomorrow = chrono::Local::now().date_naive() + chrono::Duration::days(1);
    app.quick_entry_state.date = tomorrow.format("%Y-%m-%d").to_string();
    press(&mut app, &conn, &[key(KeyCode::Enter)]);

    assert_eq!(
        qe(&app).error_message.as_deref(),
        Some("Date cannot be in the future")
    );
    assert_eq!(run_count(&conn), 0);
}

// @spec QE-023
#[test]
fn switching_to_another_screen_mid_edit_cancels_the_edit() {
    let conn = memory_db();
    let id = insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2'), ch('e')]);
    assert!(qe(&app).is_editing());
    press(
        &mut app,
        &conn,
        &[key(KeyCode::Esc), ch('3'), key(KeyCode::Esc), ch('1')],
    );

    assert!(!qe(&app).is_editing());
    assert!(qe(&app).distance.is_empty());
    type_text(&mut app, &conn, "3");
    press(&mut app, &conn, &[key(KeyCode::Enter)]);
    assert_eq!(run_count(&conn), 2);
    let original: f64 = conn
        .query_row("SELECT distance_miles FROM runs WHERE id = ?1", [id], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(original, 2.0);
}

// @spec QE-023
#[test]
fn help_keeps_the_edit_but_a_later_switch_cancels_it() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2'), ch('e')]);
    app.switch_to_screen(Screen::Help);
    assert!(qe(&app).is_editing());
    app.switch_to_screen(Screen::QuickEntry);
    assert!(qe(&app).is_editing());
    app.switch_to_screen(Screen::Help);
    app.switch_to_screen(Screen::Analytics);
    assert!(!qe(&app).is_editing());
}

// @spec RL-011, QE-017
#[test]
fn saving_an_edit_returns_to_run_list_with_the_edited_run_selected() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-03", "07:00:00", 1.0);
    insert_raw(&conn, "2026-10-02", "07:00:00", 1.0);
    let edited = insert_raw(&conn, "2026-10-01", "07:00:00", 1.0);
    let mut app = App::new();
    press(
        &mut app,
        &conn,
        &[
            key(KeyCode::Esc),
            ch('2'),
            key(KeyCode::Down),
            key(KeyCode::Down),
            ch('e'),
        ],
    );
    press(&mut app, &conn, &[key(KeyCode::Backspace)]);
    type_text(&mut app, &conn, "4");
    press(&mut app, &conn, &[key(KeyCode::Enter)]);

    assert_eq!(app.screen, Screen::RunList);
    let state = &app.run_list_state;
    let selected = &state.runs[state.selected_index];
    assert_eq!(selected.id, Some(edited));
    assert_eq!(selected.distance_miles, 4.0);
}

fn three_runs(conn: &Connection) -> [i64; 3] {
    [
        insert_raw(conn, "2026-10-03", "07:00:00", 3.0),
        insert_raw(conn, "2026-10-02", "07:00:00", 2.0),
        insert_raw(conn, "2026-10-01", "07:00:00", 1.0),
    ]
}

fn selected_id(app: &App) -> Option<i64> {
    let state = &app.run_list_state;
    state.runs.get(state.selected_index).and_then(|r| r.id)
}

// @spec RL-010
#[test]
fn delete_keeps_the_selection_at_the_same_row() {
    let conn = memory_db();
    let [_, middle, oldest] = three_runs(&conn);
    let mut app = App::new();
    press(
        &mut app,
        &conn,
        &[key(KeyCode::Esc), ch('2'), key(KeyCode::Down)],
    );
    assert_eq!(selected_id(&app), Some(middle));
    press(&mut app, &conn, &[ch('d')]);

    assert_eq!(app.run_list_state.selected_index, 1);
    assert_eq!(selected_id(&app), Some(oldest));
}

// @spec RL-010
#[test]
fn deleting_the_last_row_selects_the_new_last_row() {
    let conn = memory_db();
    let [_, middle, _] = three_runs(&conn);
    let mut app = App::new();
    press(
        &mut app,
        &conn,
        &[
            key(KeyCode::Esc),
            ch('2'),
            key(KeyCode::Down),
            key(KeyCode::Down),
            ch('d'),
        ],
    );

    assert_eq!(app.run_list_state.selected_index, 1);
    assert_eq!(selected_id(&app), Some(middle));
}

// @spec NAV-018
#[test]
fn esc_then_h_or_question_mark_opens_help_from_any_screen() {
    let conn = memory_db();
    for help_key in ['h', '?'] {
        for start in [Screen::QuickEntry, Screen::RunList, Screen::Analytics] {
            let mut app = App::new();
            app.switch_to_screen(start);
            press(&mut app, &conn, &[key(KeyCode::Esc), ch(help_key)]);
            assert_eq!(app.screen, Screen::Help, "Esc {help_key} from {start:?}");
        }
    }
}

// @spec NAV-018, QE-023
#[test]
fn esc_h_mid_edit_keeps_the_edit() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    let mut app = App::new();
    press(
        &mut app,
        &conn,
        &[
            key(KeyCode::Esc),
            ch('2'),
            ch('e'),
            key(KeyCode::Esc),
            ch('h'),
        ],
    );
    assert_eq!(app.screen, Screen::Help);
    assert!(qe(&app).is_editing());
    assert!(!qe(&app).distance.is_empty());
}
