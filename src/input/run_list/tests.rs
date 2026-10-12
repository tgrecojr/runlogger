//! Tests for Run List key handling.

use crate::app::{App, Screen};
use crate::input::handle_key_event;
use crate::test_support::{ch, insert_raw, key, memory_db};
use crossterm::event::{KeyCode, KeyEvent};
use rusqlite::Connection;

fn press(app: &mut App, conn: &Connection, keys: &[KeyEvent]) {
    for k in keys {
        handle_key_event(app, *k, conn);
    }
}

fn listed_ids(app: &App) -> Vec<Option<i64>> {
    app.run_list_state.runs.iter().map(|r| r.id).collect()
}

fn selected_id(app: &App) -> Option<i64> {
    let state = &app.run_list_state;
    state.runs.get(state.selected_index).and_then(|r| r.id)
}

fn stored_ids(conn: &Connection) -> Vec<i64> {
    let mut stmt = conn.prepare("SELECT id FROM runs ORDER BY id").unwrap();
    stmt.query_map([], |row| row.get(0))
        .unwrap()
        .map(Result::unwrap)
        .collect()
}

/// Runs inserted out of order; returns ids in expected newest-first order.
fn unordered_history(conn: &Connection) -> Vec<Option<i64>> {
    let mid_morning = insert_raw(conn, "2026-10-02", "09:30:00", 2.0);
    let oldest = insert_raw(conn, "2026-09-30", "18:00:00", 1.0);
    let newest = insert_raw(conn, "2026-10-03", "06:00:00", 3.0);
    let mid_evening = insert_raw(conn, "2026-10-02", "19:15:00", 4.0);
    let mid_early = insert_raw(conn, "2026-10-02", "05:45:00", 5.0);
    vec![newest, mid_evening, mid_morning, mid_early, oldest]
        .into_iter()
        .map(Some)
        .collect()
}

/// Opens Run List with `2` from Analytics and moves the selection down `down` rows.
fn run_list_at(conn: &Connection, down: usize) -> App {
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);
    press(&mut app, conn, &[ch('2')]);
    for _ in 0..down {
        press(&mut app, conn, &[key(KeyCode::Down)]);
    }
    app
}

// @spec RL-001
#[test]
fn pressing_2_loads_full_history_newest_first_and_selects_newest() {
    let conn = memory_db();
    let expected = unordered_history(&conn);
    let mut app = App::new();
    app.switch_to_screen(Screen::Analytics);

    press(&mut app, &conn, &[ch('2')]);

    assert_eq!(app.screen, Screen::RunList);
    assert_eq!(listed_ids(&app), expected);
    assert_eq!(app.run_list_state.selected_index, 0);
    assert_eq!(selected_id(&app), expected[0]);
}

// @spec RL-001
#[test]
fn esc_then_2_from_another_screen_reloads_history_and_selects_newest() {
    let conn = memory_db();
    let mut expected = unordered_history(&conn);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2')]);
    assert_eq!(listed_ids(&app), expected);

    // Move off the newest run, leave for Analytics, add a newer run, reopen.
    press(&mut app, &conn, &[key(KeyCode::Down), key(KeyCode::Down)]);
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('3')]);
    let newer = insert_raw(&conn, "2026-10-03", "07:00:00", 1.5);
    expected.insert(0, Some(newer));
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2')]);

    assert_eq!(app.screen, Screen::RunList);
    assert_eq!(listed_ids(&app), expected);
    assert_eq!(app.run_list_state.selected_index, 0);
    assert_eq!(selected_id(&app), Some(newer));
}

// @spec RL-006
#[test]
fn down_and_up_move_the_selection_one_run_and_stop_at_the_ends() {
    let conn = memory_db();
    let ids = unordered_history(&conn);
    let mut app = run_list_at(&conn, 0);
    let last = ids.len() - 1;

    press(&mut app, &conn, &[key(KeyCode::Up)]);
    assert_eq!(
        app.run_list_state.selected_index, 0,
        "stops at the first run"
    );

    for (i, id) in ids.iter().enumerate().skip(1) {
        press(&mut app, &conn, &[key(KeyCode::Down)]);
        assert_eq!(app.run_list_state.selected_index, i);
        assert_eq!(selected_id(&app), *id);
    }
    press(&mut app, &conn, &[key(KeyCode::Down)]);
    assert_eq!(
        app.run_list_state.selected_index, last,
        "stops at the last run"
    );

    for i in (0..last).rev() {
        press(&mut app, &conn, &[key(KeyCode::Up)]);
        assert_eq!(app.run_list_state.selected_index, i);
    }
    press(&mut app, &conn, &[key(KeyCode::Up)]);
    assert_eq!(app.run_list_state.selected_index, 0);
}

// @spec RL-008
#[test]
fn e_hands_the_selected_run_to_quick_entry_and_switches_screen() {
    let conn = memory_db();
    let ids = unordered_history(&conn);
    let mut app = run_list_at(&conn, 2); // 2026-10-02 09:30:00, 2.0 mi

    press(&mut app, &conn, &[ch('e')]);

    assert_eq!(app.screen, Screen::QuickEntry);
    let qe = &app.quick_entry_state;
    assert_eq!(qe.editing_run_id, ids[2]);
    assert_eq!(qe.date, "2026-10-02");
    assert_eq!(qe.time, "09:30:00");
    assert_eq!(qe.distance, "2");
    assert_eq!(
        stored_ids(&conn).len(),
        ids.len(),
        "nothing changed in storage"
    );
}

// @spec RL-008
#[test]
fn e_does_nothing_when_the_list_is_empty() {
    let conn = memory_db();
    let mut app = run_list_at(&conn, 0);

    press(&mut app, &conn, &[ch('e')]);

    assert_eq!(app.screen, Screen::RunList);
    assert!(!app.quick_entry_state.is_editing());
}

// @spec RL-009
#[test]
fn d_deletes_the_selected_run_immediately_and_reloads() {
    let conn = memory_db();
    let ids = unordered_history(&conn);
    let mut app = run_list_at(&conn, 1);
    let doomed = ids[1].unwrap();
    // A run added after opening only appears if the list is reloaded.
    let added = insert_raw(&conn, "2026-09-01", "07:00:00", 1.0);

    press(&mut app, &conn, &[ch('d')]);

    assert_eq!(app.screen, Screen::RunList, "no confirmation step");
    assert!(!stored_ids(&conn).contains(&doomed), "deleted from storage");
    let mut expected: Vec<_> = ids
        .iter()
        .copied()
        .filter(|&id| id != Some(doomed))
        .collect();
    expected.push(Some(added));
    assert_eq!(listed_ids(&app), expected, "history reloaded");
    assert_eq!(app.run_list_state.error_message, None);
}

// @spec RL-009
#[test]
fn d_does_nothing_when_the_list_is_empty() {
    let conn = memory_db();
    let mut app = run_list_at(&conn, 0);
    // Stored after opening, so the (empty) list does not show it.
    let hidden = insert_raw(&conn, "2026-10-01", "07:00:00", 1.0);

    press(&mut app, &conn, &[ch('d')]);

    assert_eq!(stored_ids(&conn), vec![hidden]);
    assert!(app.run_list_state.runs.is_empty());
    assert_eq!(app.run_list_state.error_message, None);
}

// @spec RL-014
#[test]
fn pressing_2_on_run_list_keeps_the_selected_run() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-03", "07:00:00", 3.0);
    let middle = insert_raw(&conn, "2026-10-02", "07:00:00", 2.0);
    insert_raw(&conn, "2026-10-01", "07:00:00", 1.0);
    let mut app = App::new();
    press(
        &mut app,
        &conn,
        &[key(KeyCode::Esc), ch('2'), key(KeyCode::Down)],
    );
    insert_raw(&conn, "2026-10-04", "07:00:00", 4.0); // a newer run appears above it
    for keys in [vec![ch('2')], vec![key(KeyCode::Esc), ch('2')]] {
        press(&mut app, &conn, &keys);
        let state = &app.run_list_state;
        assert_eq!(state.runs.len(), 4);
        assert_eq!(state.runs[state.selected_index].id, Some(middle));
    }
}

// @spec RL-014
#[test]
fn pressing_2_selects_newest_when_the_selected_run_is_gone() {
    let conn = memory_db();
    let newest = insert_raw(&conn, "2026-10-03", "07:00:00", 3.0);
    let doomed = insert_raw(&conn, "2026-10-02", "07:00:00", 2.0);
    let mut app = App::new();
    press(
        &mut app,
        &conn,
        &[key(KeyCode::Esc), ch('2'), key(KeyCode::Down)],
    );
    conn.execute("DELETE FROM runs WHERE id = ?1", [doomed])
        .unwrap();
    press(&mut app, &conn, &[ch('2')]);
    let state = &app.run_list_state;
    assert_eq!(state.runs[state.selected_index].id, Some(newest));
}
