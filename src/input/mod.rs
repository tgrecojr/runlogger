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
// @spec NAV-005, NAV-007, NAV-012, NAV-017
pub fn handle_key_event(app: &mut App, key: KeyEvent, conn: &Connection) {
    if is_quit_chord(key) {
        app.quit();
        return;
    }
    if app.waiting_for_nav {
        app.waiting_for_nav = false;
        handle_esc_command(app, key, conn);
        return;
    }
    if key.code == KeyCode::Esc {
        app.waiting_for_nav = true;
        return;
    }
    // On Quick Entry every other key types into the form.
    if app.screen == Screen::QuickEntry {
        quick_entry::handle_quick_entry_input(app, key, conn);
        return;
    }
    handle_direct_keys(app, key, conn);
}

/// Ctrl+Q or Ctrl+C quits from anywhere.
fn is_quit_chord(key: KeyEvent) -> bool {
    key.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(key.code, KeyCode::Char('q') | KeyCode::Char('c'))
}

/// The key after Esc: a screen command (works on every screen), or a second Esc
/// that clears Quick Entry. Any other key is ignored.
// @spec NAV-008, NAV-009, NAV-010, NAV-018
fn handle_esc_command(app: &mut App, key: KeyEvent, conn: &Connection) {
    if key.code == KeyCode::Esc {
        if app.screen == Screen::QuickEntry {
            app.quick_entry_state.clear();
        }
    } else if let Some(screen) = screen_for_key(key.code) {
        open_screen(app, screen, conn);
    }
}

/// Single-key commands on every screen except Quick Entry.
// @spec NAV-006, NAV-011
fn handle_direct_keys(app: &mut App, key: KeyEvent, conn: &Connection) {
    if key.code == KeyCode::Char('q') {
        app.quit();
    } else if let Some(screen) = screen_for_key(key.code) {
        open_screen(app, screen, conn);
    } else if app.screen == Screen::RunList {
        run_list::handle_run_list_input(app, key, conn);
    }
}

fn screen_for_key(code: KeyCode) -> Option<Screen> {
    match code {
        KeyCode::Char('1') => Some(Screen::QuickEntry),
        KeyCode::Char('2') => Some(Screen::RunList),
        KeyCode::Char('3') => Some(Screen::Analytics),
        KeyCode::Char('h') | KeyCode::Char('?') => Some(Screen::Help),
        _ => None,
    }
}

/// Switches screens, loading Run List or Analytics data first.
fn open_screen(app: &mut App, screen: Screen, conn: &Connection) {
    match screen {
        Screen::RunList => load_runs(app, conn),
        Screen::Analytics => load_analytics(app, conn),
        Screen::QuickEntry | Screen::Help => {}
    }
    app.switch_to_screen(screen);
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
            state.analytics =
                logic::streak::calculate_analytics(&runs, chrono::Local::now().date_naive());
            state.error_message = None;
        }
        Err(e) => {
            state.error_message = Some(format!("Could not load analytics: {e:#}"));
        }
    }
}
