use crate::app::{App, Screen};
use crate::{db, logic};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::TableState;
use rusqlite::Connection;

mod quick_entry;
mod run_list;
#[cfg(test)]
mod tests;

/// Applies one key press. Storage failures are recorded on the screen that owns
/// them, so no key press can end the app except a quit command.
// @spec NAV-005, NAV-006, NAV-007, NAV-008, NAV-009, NAV-010, NAV-011, NAV-012, NAV-017
pub fn handle_key_event(app: &mut App, key: KeyEvent, conn: &Connection) {
    // Ctrl+Q or Ctrl+C to quit from anywhere
    if key.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(key.code, KeyCode::Char('q') | KeyCode::Char('c'))
    {
        app.quit();
        return;
    }

    // If waiting for navigation command after Escape
    if app.waiting_for_nav {
        app.waiting_for_nav = false;
        match key.code {
            KeyCode::Char('1') => {
                app.switch_to_screen(Screen::QuickEntry);
                return;
            }
            KeyCode::Char('2') => {
                load_runs(app, conn);
                app.switch_to_screen(Screen::RunList);
                return;
            }
            KeyCode::Char('3') => {
                load_analytics(app, conn);
                app.switch_to_screen(Screen::Analytics);
                return;
            }
            KeyCode::Esc => {
                // Second Escape press - clear Quick Entry fields if on that screen
                if app.screen == Screen::QuickEntry {
                    app.quick_entry_state.clear();
                }
                return;
            }
            _ => {
                // Any other key cancels navigation mode
                return;
            }
        }
    }

    // Escape enters navigation mode (instead of clearing fields)
    if key.code == KeyCode::Esc {
        app.waiting_for_nav = true;
        return;
    }

    // Regular 'q' to quit when not in Quick Entry
    if key.code == KeyCode::Char('q') && app.screen != Screen::QuickEntry {
        app.quit();
        return;
    }

    // Handle screen-specific input first for Quick Entry to allow typing numbers and letters
    if app.screen == Screen::QuickEntry {
        quick_entry::handle_quick_entry_input(app, key, conn);
        return;
    }

    // Global navigation keys (only when not in Quick Entry)
    match key.code {
        KeyCode::Char('1') => app.switch_to_screen(Screen::QuickEntry),
        KeyCode::Char('2') => {
            load_runs(app, conn);
            app.switch_to_screen(Screen::RunList);
        }
        KeyCode::Char('3') => {
            load_analytics(app, conn);
            app.switch_to_screen(Screen::Analytics);
        }
        KeyCode::Char('h') | KeyCode::Char('?') => app.switch_to_screen(Screen::Help),
        _ => {}
    }

    // Handle other screens
    match app.screen {
        Screen::QuickEntry => {} // Already handled above
        Screen::RunList => run_list::handle_run_list_input(app, key, conn),
        Screen::Analytics => {}
        Screen::Help => {}
    }
}

/// Reloads the full history into Run List. On failure the list is emptied and
/// the error is shown on Run List instead.
// @spec RL-001, RL-012
pub(super) fn load_runs(app: &mut App, conn: &Connection) {
    let state = &mut app.run_list_state;
    state.selected_index = 0;
    state.table_state = TableState::default();
    match db::queries::get_all_runs(conn) {
        Ok(runs) => {
            state.runs = runs;
            state.error_message = None;
        }
        Err(e) => {
            state.runs.clear();
            state.error_message = Some(format!("Could not load runs: {e:#}"));
        }
    }
}

// @spec AN-008, AN-014
pub(super) fn load_analytics(app: &mut App, conn: &Connection) {
    let state = &mut app.analytics_state;
    match db::queries::get_all_runs(conn) {
        Ok(runs) => {
            state.analytics = logic::streak::calculate_analytics(&runs);
            state.error_message = None;
        }
        Err(e) => {
            state.error_message = Some(format!("Could not load analytics: {e:#}"));
        }
    }
}
