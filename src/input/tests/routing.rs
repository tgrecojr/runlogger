use super::*;

// @spec NAV-005
#[test]
fn ctrl_q_and_ctrl_c_quit_from_every_screen() {
    let conn = memory_db();
    for quit_key in ['q', 'c'] {
        for screen in ALL_SCREENS {
            let mut app = app_on(screen);
            press(&mut app, &conn, &[ctrl(quit_key)]);
            assert!(app.should_quit, "Ctrl+{quit_key} on {screen:?}");
        }
    }
}

// @spec NAV-005, NAV-012
#[test]
fn ctrl_q_on_quick_entry_quits_instead_of_typing() {
    let conn = memory_db();
    let mut app = App::new();
    press(&mut app, &conn, &[ctrl('q')]);
    assert!(app.should_quit);
    assert!(qe(&app).distance.is_empty());
}

// @spec NAV-006
#[test]
fn q_quits_from_every_screen_except_quick_entry() {
    let conn = memory_db();
    for screen in OTHER_SCREENS {
        let mut app = app_on(screen);
        press(&mut app, &conn, &[ch('q')]);
        assert!(app.should_quit, "q on {screen:?}");
    }

    let mut app = App::new();
    press(&mut app, &conn, &[ch('q')]);
    assert!(!app.should_quit);
    assert_eq!(app.screen, Screen::QuickEntry);
}

// @spec NAV-007
#[test]
fn esc_keeps_the_screen_and_makes_the_next_key_a_command() {
    let conn = memory_db();
    for screen in ALL_SCREENS {
        let mut app = app_on(screen);
        press(&mut app, &conn, &[key(KeyCode::Esc)]);
        assert_eq!(app.screen, screen, "Esc changed {screen:?}");
        assert!(!app.should_quit);
        assert!(qe(&app).distance.is_empty());

        let target = if screen == Screen::Analytics {
            ('2', Screen::RunList)
        } else {
            ('3', Screen::Analytics)
        };
        press(&mut app, &conn, &[ch(target.0)]);
        assert_eq!(app.screen, target.1, "Esc {} from {screen:?}", target.0);
        assert!(qe(&app).distance.is_empty(), "command typed into the form");
    }
}

// @spec NAV-007
#[test]
fn only_the_one_key_after_esc_is_a_command() {
    let conn = memory_db();
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('x'), ch('2')]);
    assert_eq!(app.screen, Screen::QuickEntry);
    assert_eq!(qe(&app).distance, "2");
}

// @spec NAV-008
#[test]
fn esc_digit_switches_screens_from_every_screen_and_loads_data() {
    for screen in ALL_SCREENS {
        let conn = memory_db();
        insert_raw(&conn, "2026-10-01", "07:00:00", 2.5);

        let mut app = app_on(screen);
        press(&mut app, &conn, &[key(KeyCode::Esc), ch('2')]);
        assert_eq!(app.screen, Screen::RunList, "Esc 2 from {screen:?}");
        assert_eq!(app.run_list_state.runs.len(), 1, "from {screen:?}");
        assert_eq!(app.run_list_state.runs[0].distance_miles, 2.5);

        let mut app = app_on(screen);
        press(&mut app, &conn, &[key(KeyCode::Esc), ch('3')]);
        assert_eq!(app.screen, Screen::Analytics, "Esc 3 from {screen:?}");
        assert_eq!(app.analytics_state.analytics.total_runs, 1);
        assert_eq!(app.analytics_state.analytics.total_distance, 2.5);

        let mut app = app_on(screen);
        press(&mut app, &conn, &[key(KeyCode::Esc), ch('1')]);
        assert_eq!(app.screen, Screen::QuickEntry, "Esc 1 from {screen:?}");
    }
}

// @spec NAV-008
#[test]
fn esc_2_reloads_runs_added_since_the_last_visit() {
    let conn = memory_db();
    let mut app = App::new();
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2')]);
    assert!(app.run_list_state.runs.is_empty());

    insert_raw(&conn, "2026-10-01", "07:00:00", 1.0);
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('3')]);
    assert_eq!(app.analytics_state.analytics.total_runs, 1);
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('2')]);
    assert_eq!(app.run_list_state.runs.len(), 1);
}

// @spec NAV-009
#[test]
fn esc_esc_on_quick_entry_clears_the_form() {
    let conn = memory_db();
    let mut app = App::new();
    type_text(&mut app, &conn, "3.5");
    press(&mut app, &conn, &[key(KeyCode::Tab)]);
    type_text(&mut app, &conn, "hills");
    app.quick_entry_state.date = "2020-01-01".into();
    app.quick_entry_state.error_message = Some("oops".into());
    press(&mut app, &conn, &[key(KeyCode::Esc), key(KeyCode::Esc)]);

    let state = qe(&app);
    assert_eq!(app.screen, Screen::QuickEntry);
    assert!(state.distance.is_empty());
    assert!(state.note.is_empty());
    assert_ne!(state.date, "2020-01-01");
    assert_eq!(state.focused_field, crate::app::InputField::Distance);
    assert_eq!(state.error_message, None);
    assert!(!app.should_quit);
}

// @spec NAV-010
#[test]
fn other_keys_after_esc_are_ignored_on_every_screen() {
    let ignored = [
        ch('x'),
        ch('q'),
        ch('d'),
        ch('e'),
        ch('4'),
        key(KeyCode::Enter),
        key(KeyCode::Tab),
        key(KeyCode::Down),
        key(KeyCode::Backspace),
    ];
    for screen in ALL_SCREENS {
        let conn = memory_db();
        insert_raw(&conn, "2026-10-02", "07:00:00", 2.0);
        insert_raw(&conn, "2026-10-01", "07:00:00", 1.0);
        let mut app = app_on(screen);
        if screen == Screen::RunList {
            load_runs(&mut app, &conn);
        }
        for k in ignored {
            press(&mut app, &conn, &[key(KeyCode::Esc), k]);
            assert_eq!(app.screen, screen, "Esc {k:?} on {screen:?}");
            assert!(!app.should_quit, "Esc {k:?} quit on {screen:?}");
        }
        assert_eq!(run_count(&conn), 2, "a run was deleted on {screen:?}");
        assert_eq!(app.run_list_state.selected_index, 0);
        assert!(!qe(&app).is_editing());
        let state = qe(&app);
        assert!(state.distance.is_empty() && state.note.is_empty());
        assert_eq!(state.focused_field, crate::app::InputField::Distance);
        assert_eq!(state.error_message, None);
        assert_eq!(state.success_message, None);
    }
}

// @spec NAV-010, NAV-005
#[test]
fn ctrl_q_and_ctrl_c_still_quit_after_esc() {
    let conn = memory_db();
    for quit_key in ['q', 'c'] {
        for screen in ALL_SCREENS {
            let mut app = app_on(screen);
            press(&mut app, &conn, &[key(KeyCode::Esc), ctrl(quit_key)]);
            assert!(app.should_quit, "Esc Ctrl+{quit_key} on {screen:?}");
        }
    }
}

// @spec NAV-011
#[test]
fn digits_switch_screens_directly_off_quick_entry_and_load_data() {
    for screen in OTHER_SCREENS {
        let conn = memory_db();
        insert_raw(&conn, "2026-10-01", "07:00:00", 2.5);

        let mut app = app_on(screen);
        press(&mut app, &conn, &[ch('2')]);
        assert_eq!(app.screen, Screen::RunList, "2 from {screen:?}");
        assert_eq!(app.run_list_state.runs.len(), 1, "from {screen:?}");

        let mut app = app_on(screen);
        press(&mut app, &conn, &[ch('3')]);
        assert_eq!(app.screen, Screen::Analytics, "3 from {screen:?}");
        assert_eq!(app.analytics_state.analytics.total_runs, 1);
        assert_eq!(app.analytics_state.analytics.total_distance, 2.5);

        let mut app = app_on(screen);
        press(&mut app, &conn, &[ch('1')]);
        assert_eq!(app.screen, Screen::QuickEntry, "1 from {screen:?}");
        assert!(qe(&app).distance.is_empty());
    }
}

// @spec NAV-011
#[test]
fn h_or_question_mark_shows_help_directly_off_quick_entry() {
    let conn = memory_db();
    for help_key in ['h', '?'] {
        for screen in OTHER_SCREENS {
            let mut app = app_on(screen);
            press(&mut app, &conn, &[ch(help_key)]);
            assert_eq!(app.screen, Screen::Help, "{help_key} from {screen:?}");
            assert!(!app.should_quit);
        }
    }
}

// @spec NAV-012
#[test]
fn quick_entry_types_navigation_digits_and_letters_into_the_form() {
    let conn = memory_db();
    let mut app = App::new();
    type_text(&mut app, &conn, "123h?q");
    assert_eq!(app.screen, Screen::QuickEntry);
    assert!(!app.should_quit);
    assert_eq!(qe(&app).distance, "123h?q");

    press(&mut app, &conn, &[key(KeyCode::Tab)]);
    assert_eq!(qe(&app).focused_field, crate::app::InputField::Note);
    type_text(&mut app, &conn, "q 2 h 3?1");
    press(&mut app, &conn, &[key(KeyCode::Backspace)]);
    assert_eq!(app.screen, Screen::QuickEntry);
    assert!(!app.should_quit);
    assert_eq!(qe(&app).note, "q 2 h 3?");
    assert_eq!(qe(&app).distance, "123h?q");
}

// @spec NAV-012, NAV-007
#[test]
fn quick_entry_does_not_type_esc_or_the_key_after_it() {
    let conn = memory_db();
    let mut app = App::new();
    type_text(&mut app, &conn, "1");
    press(&mut app, &conn, &[key(KeyCode::Esc), ch('x')]);
    type_text(&mut app, &conn, "2");
    assert_eq!(qe(&app).distance, "12");
    assert_eq!(app.screen, Screen::QuickEntry);
}
