use super::*;

// @spec QE-008
#[test]
fn submitted_slash_date_is_saved_as_that_date() {
    let conn = memory_db();
    let mut app = App::new();
    fill(&mut app, "03/07/2020", "07:15", "3", "");
    press(&mut app, &conn, &[key(KeyCode::Enter)]);

    assert_eq!(app.quick_entry_state.error_message, None);
    let runs = db::queries::get_all_runs(&conn).unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].date, NaiveDate::from_ymd_opt(2020, 3, 7).unwrap());
    assert_eq!(
        runs[0].time_started,
        NaiveTime::from_hms_opt(7, 15, 0).unwrap()
    );
}

// @spec QE-015
#[test]
fn enter_removes_previous_messages_before_validating() {
    let conn = memory_db();
    let mut app = App::new();
    fill(&mut app, "2020-01-01", "07:00:00", "abc", "");
    app.quick_entry_state.success_message = Some("Run logged successfully!".into());
    press(&mut app, &conn, &[key(KeyCode::Enter)]);
    assert_eq!(app.quick_entry_state.success_message, None);
    assert!(app.quick_entry_state.error_message.is_some());

    app.quick_entry_state.distance = "2".into();
    press(&mut app, &conn, &[key(KeyCode::Enter)]);
    assert_eq!(app.quick_entry_state.error_message, None);
    assert_eq!(
        app.quick_entry_state.success_message.as_deref(),
        Some("Run logged successfully!")
    );
}

// @spec QE-019, QE-011
#[test]
fn invalid_submission_shows_message_and_keeps_fields() {
    let cases = [
        (
            "2020/01/01",
            "07:00",
            "3",
            "Invalid date format. Use YYYY-MM-DD",
        ),
        (
            "2020-01-01",
            "7pm",
            "3",
            "Invalid time format. Use HH:MM or HH:MM:SS",
        ),
        ("2020-01-01", "07:00", "", "Distance is required"),
        ("2020-01-01", "07:00", "0", "Distance must be positive"),
    ];
    for (date, time, distance, message) in cases {
        let conn = memory_db();
        let mut app = App::new();
        fill(&mut app, date, time, distance, "keep me");
        let before = snapshot(&app.quick_entry_state);
        press(&mut app, &conn, &[key(KeyCode::Enter)]);

        let state = &app.quick_entry_state;
        assert_eq!(state.error_message.as_deref(), Some(message));
        assert_eq!(snapshot(state), before);
        assert_eq!(app.screen, Screen::QuickEntry);
        assert!(db::queries::get_all_runs(&conn).unwrap().is_empty());
    }
}

// @spec QE-019
#[test]
fn invalid_edit_submission_keeps_fields_and_edit_mode() {
    let conn = memory_db();
    let id = insert_raw(&conn, "2020-01-01", "07:00:00", 2.0);
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2'), ch('e')]);
    assert_eq!(app.quick_entry_state.editing_run_id, Some(id));
    app.quick_entry_state.distance = "abc".into();
    let before = snapshot(&app.quick_entry_state);
    press(&mut app, &conn, &[key(KeyCode::Enter)]);

    let state = &app.quick_entry_state;
    assert_eq!(
        state.error_message.as_deref(),
        Some("Invalid distance. Enter a number (e.g., 3.5)")
    );
    assert_eq!(snapshot(state), before);
    assert_eq!(app.screen, Screen::QuickEntry);
    let runs = db::queries::get_all_runs(&conn).unwrap();
    assert_eq!(runs[0].distance_miles, 2.0);
}

// @spec QE-022
#[test]
fn edit_key_loads_the_selected_run_into_the_form() {
    let conn = memory_db();
    let date = NaiveDate::from_ymd_opt(2020, 5, 9).unwrap();
    let time = NaiveTime::from_hms_opt(6, 4, 3).unwrap();
    let run = Run::new(date, time, 3.5, Some("hilly".into())).unwrap();
    let id = db::queries::insert_run(&conn, &run).unwrap();
    let mut app = App::new();
    app.quick_entry_state.focused_field = InputField::Note;
    app.quick_entry_state.error_message = Some("old".into());
    app.quick_entry_state.success_message = Some("older".into());
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2'), ch('e')]);

    assert_eq!(app.screen, Screen::QuickEntry);
    let state = &app.quick_entry_state;
    assert_eq!(
        snapshot(state),
        (
            "2020-05-09".into(),
            "06:04:03".into(),
            "3.5".into(),
            "hilly".into(),
            Some(id)
        )
    );
    assert_eq!(state.focused_field, InputField::Distance);
    assert_eq!(state.error_message, None);
    assert_eq!(state.success_message, None);
    assert!(state.is_editing());
}

// @spec QE-022
#[test]
fn editing_a_run_without_note_leaves_note_empty() {
    let conn = memory_db();
    let id = insert_raw(&conn, "2020-05-09", "06:04:03", 12.0);
    let mut app = App::new();
    app.quick_entry_state.note = "stale".into();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2'), ch('e')]);

    let state = &app.quick_entry_state;
    assert_eq!(state.note, "");
    assert_eq!(state.distance, "12");
    assert_eq!(state.editing_run_id, Some(id));
}

// @spec QE-020
#[test]
fn duplicate_start_time_says_which_run_conflicts_and_how_to_fix_it() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    let mut app = App::new();
    fill(&mut app, "2026-10-01", "07:00:00", "3", "second run");
    press(&mut app, &conn, &[key(KeyCode::Enter)]);

    let state = &app.quick_entry_state;
    assert_eq!(
        state.error_message.as_deref(),
        Some("Another run already starts at 2026-10-01 07:00:00 — change the Time and press Enter to save.")
    );
    assert_eq!(
        (state.date.as_str(), state.time.as_str()),
        ("2026-10-01", "07:00:00")
    );
    assert_eq!(
        (state.distance.as_str(), state.note.as_str()),
        ("3", "second run")
    );
}

// @spec QE-020
#[test]
fn other_save_failures_give_the_cause_and_keep_the_edit() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "07:00:00", 2.0);
    crate::test_support::block_writes(&conn, "UPDATE");
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2'), ch('e')]);
    press(&mut app, &conn, &[key(KeyCode::Enter)]);

    let state = &app.quick_entry_state;
    let msg = state.error_message.as_deref().unwrap_or("");
    assert!(msg.starts_with("Could not save run: "), "{msg}");
    assert!(
        msg.contains("UPDATE blocked"),
        "underlying cause missing: {msg}"
    );
    assert!(
        msg.ends_with("Your entry is still here — press Enter to try again."),
        "{msg}"
    );
    assert!(state.is_editing());
    assert_eq!(app.screen, Screen::QuickEntry);
}
