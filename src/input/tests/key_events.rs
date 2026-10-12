use super::*;

fn with_kind(code: KeyCode, kind: crossterm::event::KeyEventKind) -> KeyEvent {
    KeyEvent::new_with_kind(code, crossterm::event::KeyModifiers::NONE, kind)
}

// @spec NAV-019
#[test]
fn key_releases_are_ignored_on_every_screen() {
    use crossterm::event::KeyEventKind::Release;
    let conn = memory_db();
    for screen in ALL_SCREENS {
        let mut app = app_on(screen);
        let before = qe(&app).distance.clone();
        press(
            &mut app,
            &conn,
            &[
                with_kind(KeyCode::Char('q'), Release),
                with_kind(KeyCode::Char('2'), Release),
                with_kind(KeyCode::Esc, Release),
            ],
        );
        assert!(!app.should_quit, "release of q quit on {screen:?}");
        assert_eq!(app.screen, screen, "release switched screens on {screen:?}");
        assert!(
            !app.waiting_for_nav,
            "release of Esc armed the prefix on {screen:?}"
        );
        assert_eq!(
            qe(&app).distance,
            before,
            "release typed into the form on {screen:?}"
        );
    }
}

// @spec NAV-019
#[test]
fn key_repeats_still_act() {
    use crossterm::event::KeyEventKind::Repeat;
    let conn = memory_db();
    let mut app = app_on(Screen::QuickEntry);
    press(
        &mut app,
        &conn,
        &[
            with_kind(KeyCode::Char('5'), Repeat),
            with_kind(KeyCode::Char('5'), Repeat),
        ],
    );
    assert_eq!(qe(&app).distance, "55");
}
