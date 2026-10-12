use super::load_runs;
use crate::app::{App, InputField, QuickEntryState, Screen};
use crate::db::queries::StorageError;
use crate::logic::validation::{current_time, format_date, format_time};
use crate::models::run::Run;
use crate::{db, logic};
use anyhow::Result;
use chrono::Local;
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
        KeyCode::Char(c) => state.type_char(c),
        KeyCode::Backspace => state.delete_char(),
        _ => {}
    }
}

// @spec QE-015, QE-016, QE-017, QE-018, QE-019
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
                    app.run_list_state.select_run(id);
                    app.switch_to_screen(Screen::RunList);
                }
                Err(e) => state.error_message = Some(save_error_message(&e)),
            }
        }
        None => match db::queries::insert_run(conn, &run) {
            Ok(_) => {
                // A fresh start time keeps the next run from duplicating this one.
                state.date = format_date(&run.date);
                state.time = format_time(&current_time());
                state.distance.clear();
                state.note.clear();
                state.focused_field = InputField::Distance;
                state.success_message = Some("Run logged successfully!".to_string());
            }
            Err(e) => state.error_message = Some(save_error_message(&e)),
        },
    }
}

/// Says what went wrong with a save and what the user can do about it.
// @spec QE-020
fn save_error_message(e: &anyhow::Error) -> String {
    match e.downcast_ref::<StorageError>() {
        Some(StorageError::DuplicateStart { date, time }) => format!(
            "Another run already starts at {} {} — change the Time and press Enter to save.",
            format_date(date),
            format_time(time)
        ),
        _ => format!(
            "Could not save run: {e:#}. Your entry is still here — press Enter to try again."
        ),
    }
}

/// Parses the form in the order Date, Time, not-in-the-future, Distance, then
/// applies the run rules; the first failure is returned.
// @spec QE-014, QE-024
fn build_run(state: &QuickEntryState) -> Result<Run> {
    let date = logic::validation::parse_date(&state.date)?;
    let time = logic::validation::parse_time(&state.time)?;
    logic::validation::check_not_future(date, time, Local::now().naive_local())?;
    let distance = logic::validation::parse_distance(&state.distance)?;
    let note = (!state.note.is_empty()).then(|| state.note.clone());
    Run::new(date, time, distance, note)
}

#[cfg(test)]
mod tests;
