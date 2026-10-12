use super::*;

// @spec NAV-014, NAV-016
#[test]
fn each_footer_is_accurate_and_fits_an_80_column_terminal() {
    for (screen, footer) in FOOTERS {
        let shown = screen_on(screen, 80);
        let last_lines: Vec<&str> = shown.lines().rev().take(3).collect();
        assert!(
            last_lines.iter().any(|l| l.contains(footer)),
            "{screen:?} footer should read {footer:?}:\n{}",
            last_lines.join("\n")
        );
    }
}

// @spec NAV-013, NAV-016
#[test]
fn header_shows_esc_keys_on_quick_entry_and_direct_keys_elsewhere() {
    let quick_entry = screen_on(Screen::QuickEntry, 100);
    let header: String = quick_entry.lines().take(3).collect();
    assert!(
        header.contains("[Esc 1] Quick Entry  [Esc 2] Run List  [Esc 3] Analytics  [Esc h] Help"),
        "{header}"
    );

    let run_list = screen_on(Screen::RunList, 100);
    let header: String = run_list.lines().take(3).collect();
    assert!(
        header.contains("[1] Quick Entry  [2] Run List  [3] Analytics  [h] Help"),
        "{header}"
    );
}

// @spec NAV-016
#[test]
fn help_describes_esc_commands_and_where_direct_keys_work() {
    let help = screen_on(Screen::Help, 100);
    assert!(help.contains("[Esc] then [h] or [?]"), "{help}");
    assert!(help.contains("not in Quick Entry"), "{help}");
    assert!(!help.contains("works from any screen"), "{help}");
}

// @spec NAV-015
#[test]
fn help_covers_navigation_quick_entry_run_list_goal_and_data_location() {
    let mut app = App::new();
    app.db_location = Some("/scratch/runlogger/runs.db".into());
    app.switch_to_screen(Screen::Help);
    let help = buffer_text(&draw_app(&mut app, 100, 40));
    let topics: [(&str, &[&str]); 5] = [
        (
            "navigation keys",
            &[
                "Navigation:",
                "[Esc] then [1/2/3] - Quick Entry / Run List / Analytics",
                "[Esc] then [h] or [?] - This help screen",
                "[1-3], [h], [?] - Same, directly",
                "[q] - Quit (not in Quick Entry)",
                "[Ctrl+Q] or [Ctrl+C] - Quit from anywhere",
            ],
        ),
        (
            "Quick Entry keys and behavior",
            &[
                "Quick Entry Screen:",
                "[Tab] / [Shift+Tab] - Next / previous field",
                "[Enter] - Save the run",
                "[Backspace] - Delete last character",
                "[Esc] [Esc] - Clear the form",
                "Date and time are pre-filled; Time resets to now after each save",
                "Runs cannot be dated or timed in the future",
            ],
        ),
        (
            "Run List keys",
            &[
                "Run List Screen:",
                "[Up/Down] - Select a run",
                "[e] - Edit the selected run",
                "[d] - Delete the selected run",
            ],
        ),
        (
            "1-mile daily goal and streak rule",
            &[
                "Your Goal:",
                "Run at least 1 mile every day; runs on the same day add up.",
                "Your streak counts through yesterday until today's mile is logged.",
            ],
        ),
        ("database location in use", &["/scratch/runlogger/runs.db"]),
    ];
    for (topic, lines) in topics {
        for line in lines {
            assert!(help.contains(line), "{topic}: {line:?} missing:\n{help}");
        }
    }
}

// @spec NAV-020
#[test]
fn footer_becomes_the_esc_menu_while_a_command_key_is_awaited() {
    let conn = crate::test_support::memory_db();
    for (screen, menu) in [
        (
            Screen::QuickEntry,
            "Esc → [1] Quick Entry  [2] Run List  [3] Analytics  [h] Help  [Esc] Clear",
        ),
        (
            Screen::RunList,
            "Esc → [1] Quick Entry  [2] Run List  [3] Analytics  [h] Help",
        ),
        (
            Screen::Analytics,
            "Esc → [1] Quick Entry  [2] Run List  [3] Analytics  [h] Help",
        ),
        (
            Screen::Help,
            "Esc → [1] Quick Entry  [2] Run List  [3] Analytics  [h] Help",
        ),
    ] {
        let mut app = App::new();
        app.switch_to_screen(screen);
        crate::input::handle_key_event(
            &mut app,
            crate::test_support::key(crossterm::event::KeyCode::Esc),
            &conn,
        );
        let buf = draw_app(&mut app, 80, 30);
        let footer: Vec<String> = buffer_text(&buf)
            .lines()
            .rev()
            .take(3)
            .map(String::from)
            .collect();
        let line = footer
            .iter()
            .find(|l| l.contains("Esc →"))
            .unwrap_or_else(|| panic!("{screen:?}: no Esc menu in {footer:?}"));
        assert!(line.contains(menu), "{screen:?}: {line}");
        if screen != Screen::QuickEntry {
            assert!(!line.contains("[Esc] Clear"), "{screen:?}: {line}");
        }
        // The footer's text row is the second-to-last row.
        let footer_y = buf.area.height - 2;
        let footer_row: Vec<char> = buffer_text(&buf)
            .lines()
            .nth(footer_y as usize)
            .unwrap()
            .chars()
            .collect();
        let x = (0..footer_row.len())
            .find(|&i| footer_row[i..].starts_with(&['E', 's', 'c', ' ', '→']))
            .unwrap();
        let cell = &buf[(x as u16, footer_y)];
        assert_eq!(
            (
                cell.fg,
                cell.modifier.contains(ratatui::style::Modifier::BOLD)
            ),
            (Color::Cyan, true)
        );

        // The next key ends the wait and restores the normal footer.
        crate::input::handle_key_event(
            &mut app,
            crate::test_support::key(crossterm::event::KeyCode::Char('x')),
            &conn,
        );
        let after = buffer_text(&draw_app(&mut app, 80, 30));
        let footer_after: Vec<&str> = after.lines().rev().take(3).collect();
        assert!(
            footer_after.iter().all(|l| !l.contains("Esc →")),
            "{screen:?}: menu still shown: {footer_after:?}"
        );
    }
}
