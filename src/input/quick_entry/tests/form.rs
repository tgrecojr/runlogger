use super::*;

// @spec QE-001
#[test]
fn new_form_prefills_now_and_focuses_empty_distance() {
    let app = App::new();
    let state = &app.quick_entry_state;
    assert_is_now(&state.date, &state.time);
    assert_eq!(state.distance, "");
    assert_eq!(state.note, "");
    assert_eq!(state.focused_field, InputField::Distance);
}

// @spec QE-002
#[test]
fn tab_cycles_focus_forward_through_all_fields() {
    let conn = memory_db();
    let mut app = App::new();
    app.quick_entry_state.focused_field = InputField::Date;
    let mut seen = Vec::new();
    for _ in 0..4 {
        press(&mut app, &conn, &[key(KeyCode::Tab)]);
        seen.push(app.quick_entry_state.focused_field);
    }
    assert_eq!(
        seen,
        [
            InputField::Time,
            InputField::Distance,
            InputField::Note,
            InputField::Date
        ]
    );
}

// @spec QE-002
#[test]
fn shift_tab_cycles_focus_backward_through_all_fields() {
    let shift_tab = KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT);
    let back_tab = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
    for k in [shift_tab, back_tab] {
        let conn = memory_db();
        let mut app = App::new();
        app.quick_entry_state.focused_field = InputField::Date;
        let mut seen = Vec::new();
        for _ in 0..4 {
            press(&mut app, &conn, &[k]);
            seen.push(app.quick_entry_state.focused_field);
        }
        assert_eq!(
            seen,
            [
                InputField::Note,
                InputField::Distance,
                InputField::Time,
                InputField::Date
            ],
            "{k:?}"
        );
    }
}

// @spec QE-003
#[test]
fn typed_characters_append_to_the_focused_field_even_command_keys() {
    let conn = memory_db();
    let mut app = App::new();
    let typed = "123hq?ed x";
    for field in [
        InputField::Date,
        InputField::Time,
        InputField::Distance,
        InputField::Note,
    ] {
        fill(&mut app, "D", "T", "M", "N");
        app.quick_entry_state.focused_field = field;
        type_text(&mut app, &conn, typed);

        assert_eq!(app.screen, Screen::QuickEntry);
        assert!(!app.should_quit);
        let state = &app.quick_entry_state;
        let expect = |base: &str, f: InputField| {
            if f == field {
                format!("{base}{typed}")
            } else {
                base.to_string()
            }
        };
        assert_eq!(state.date, expect("D", InputField::Date));
        assert_eq!(state.time, expect("T", InputField::Time));
        assert_eq!(state.distance, expect("M", InputField::Distance));
        assert_eq!(state.note, expect("N", InputField::Note));
    }
}

// @spec QE-004
#[test]
fn backspace_removes_last_character_of_focused_field_only() {
    let conn = memory_db();
    let mut app = App::new();
    fill(&mut app, "2026-10-01", "07:00:00", "3.5", "easy");
    app.quick_entry_state.focused_field = InputField::Note;
    press(&mut app, &conn, &[key(KeyCode::Backspace)]);
    assert_eq!(
        snapshot(&app.quick_entry_state),
        (
            "2026-10-01".into(),
            "07:00:00".into(),
            "3.5".into(),
            "eas".into(),
            None
        )
    );

    app.quick_entry_state.focused_field = InputField::Date;
    press(&mut app, &conn, &[key(KeyCode::Backspace)]);
    assert_eq!(app.quick_entry_state.date, "2026-10-0");
    assert_eq!(app.quick_entry_state.note, "eas");
}

// @spec QE-007
#[test]
fn esc_esc_clears_the_form_and_leaves_edit_mode() {
    let conn = memory_db();
    let mut app = App::new();
    fill(&mut app, "2020-01-01", "01:02:03", "4.2", "note");
    let state = &mut app.quick_entry_state;
    state.focused_field = InputField::Note;
    state.error_message = Some("boom".into());
    state.success_message = Some("yay".into());
    state.editing_run_id = Some(7);

    press(&mut app, &conn, &[key(KeyCode::Esc), key(KeyCode::Esc)]);

    let state = &app.quick_entry_state;
    assert_eq!(app.screen, Screen::QuickEntry);
    assert_is_now(&state.date, &state.time);
    assert_eq!(state.distance, "");
    assert_eq!(state.note, "");
    assert_eq!(state.focused_field, InputField::Distance);
    assert_eq!(state.error_message, None);
    assert_eq!(state.success_message, None);
    assert_eq!(state.editing_run_id, None);
}
