use super::load_runs;
use crate::app::{App, InputField, QuickEntryState, Screen};
use crate::models::run::Run;
use crate::{db, logic};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rusqlite::Connection;

// @spec QE-002, QE-003, QE-004
pub(super) fn handle_quick_entry_input(app: &mut App, key: KeyEvent, conn: &Connection) {
    let state = &mut app.quick_entry_state;

    match key.code {
        KeyCode::Tab => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                state.prev_field();
            } else {
                state.next_field();
            }
        }
        KeyCode::BackTab => {
            state.prev_field();
        }
        KeyCode::Enter => submit(app, conn),
        KeyCode::Char(c) => {
            state.current_input_mut().push(c);
        }
        KeyCode::Backspace => {
            state.current_input_mut().pop();
        }
        _ => {}
    }
}

// @spec QE-015, QE-016, QE-017, QE-018, QE-019, QE-020
fn submit(app: &mut App, conn: &Connection) {
    let state = &mut app.quick_entry_state;
    state.error_message = None;
    state.success_message = None;

    let mut run = match build_run(state) {
        Ok(run) => run,
        Err(e) => {
            state.error_message = Some(e.to_string());
            return;
        }
    };

    match state.editing_run_id {
        Some(id) => {
            run.id = Some(id);
            match db::queries::update_run(conn, &run) {
                Ok(()) => {
                    // The edit is saved: reset the form, then show the history
                    // (Run List shows its own error if the reload fails).
                    state.clear();
                    load_runs(app, conn);
                    app.switch_to_screen(Screen::RunList);
                }
                Err(e) => state.error_message = Some(format!("Database error: {}", e)),
            }
        }
        None => match db::queries::insert_run(conn, &run) {
            Ok(_) => {
                state.distance.clear();
                state.note.clear();
                state.focused_field = InputField::Distance;
                state.success_message = Some("Run logged successfully!".to_string());
            }
            Err(e) => state.error_message = Some(format!("Database error: {}", e)),
        },
    }
}

/// Parses the form in the order Date, Time, Distance, then applies the run
/// rules; the first failure is returned.
// @spec QE-014
fn build_run(state: &QuickEntryState) -> Result<Run> {
    let date = logic::validation::parse_date(&state.date)?;
    let time = logic::validation::parse_time(&state.time)?;
    let distance = logic::validation::parse_distance(&state.distance)?;
    let note = (!state.note.is_empty()).then(|| state.note.clone());
    Run::new(date, time, distance, note)
}
