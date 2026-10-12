use super::*;

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
