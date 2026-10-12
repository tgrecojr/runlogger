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
    let help = screen_on(Screen::Help, 100);
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
        (
            "macOS database location",
            &["~/Library/Application Support/runlogger/runs.db (macOS)"],
        ),
    ];
    for (topic, lines) in topics {
        for line in lines {
            assert!(help.contains(line), "{topic}: {line:?} missing:\n{help}");
        }
    }
}
