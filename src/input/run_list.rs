use super::load_runs;
use crate::app::{App, Screen};
use crate::db;
use crossterm::event::{KeyCode, KeyEvent};
use rusqlite::Connection;

// @spec RL-006, RL-008, RL-009, RL-010, RL-013
pub(super) fn handle_run_list_input(app: &mut App, key: KeyEvent, conn: &Connection) {
    match key.code {
        KeyCode::Up => {
            let state = &mut app.run_list_state;
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
        }
        KeyCode::Down => {
            let state = &mut app.run_list_state;
            if state.selected_index + 1 < state.runs.len() {
                state.selected_index += 1;
            }
        }
        KeyCode::Char('e') => {
            // Edit selected run
            let state = &app.run_list_state;
            if !state.runs.is_empty() && state.selected_index < state.runs.len() {
                let run = state.runs[state.selected_index].clone();
                app.quick_entry_state.load_run(&run);
                app.switch_to_screen(Screen::QuickEntry);
            }
        }
        KeyCode::Char('d') => {
            // Delete selected run
            let run_id = {
                let state = &app.run_list_state;
                if !state.runs.is_empty() && state.selected_index < state.runs.len() {
                    state.runs[state.selected_index].id
                } else {
                    None
                }
            };

            if let Some(id) = run_id {
                if let Err(e) = db::queries::delete_run(conn, id) {
                    app.run_list_state.error_message = Some(format!("Could not delete run: {e:#}"));
                    return;
                }
                load_runs(app, conn);
                // Adjust selected index if needed
                let state = &mut app.run_list_state;
                if state.selected_index >= state.runs.len() && !state.runs.is_empty() {
                    state.selected_index = state.runs.len() - 1;
                }
            }
        }
        _ => {}
    }
}
