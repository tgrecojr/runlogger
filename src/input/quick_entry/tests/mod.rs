//! Tests for Quick Entry key handling and submit flow.

use super::*;

use crate::input::handle_key_event;
use crate::test_support::{ch, insert_raw, key, memory_db};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

mod form;
mod submit;

fn press(app: &mut App, conn: &Connection, keys: &[KeyEvent]) {
    for k in keys {
        handle_key_event(app, *k, conn);
    }
}

fn type_text(app: &mut App, conn: &Connection, text: &str) {
    for c in text.chars() {
        handle_key_event(app, ch(c), conn);
    }
}

/// Asserts `date`/`time` are strictly `YYYY-MM-DD`/`HH:MM:SS` and within a few
/// seconds of the local clock (tolerant of midnight and second boundaries).
fn assert_is_now(date: &str, time: &str) {
    assert_eq!(date.len(), 10, "{date:?}");
    assert_eq!(time.len(), 8, "{time:?}");
    let shown = NaiveDateTime::parse_from_str(&format!("{date} {time}"), "%Y-%m-%d %H:%M:%S")
        .unwrap_or_else(|e| panic!("{date:?} {time:?}: {e}"));
    let now = Local::now().naive_local();
    let gap = (now - shown).num_seconds().abs();
    assert!(gap <= 5, "{shown} is {gap}s away from {now}");
}

fn snapshot(state: &QuickEntryState) -> (String, String, String, String, Option<i64>) {
    (
        state.date.clone(),
        state.time.clone(),
        state.distance.clone(),
        state.note.clone(),
        state.editing_run_id,
    )
}

fn fill(app: &mut App, date: &str, time: &str, distance: &str, note: &str) {
    let state = &mut app.quick_entry_state;
    state.date = date.to_string();
    state.time = time.to_string();
    state.distance = distance.to_string();
    state.note = note.to_string();
}
