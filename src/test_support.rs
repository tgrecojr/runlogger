//! Shared helpers for unit tests: an in-memory database with the real schema,
//! raw row insertion (to simulate rows written by older versions or by hand),
//! and key-event constructors.

use crate::db::migrations::init_database;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rusqlite::{params, Connection};

pub fn memory_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    init_database(&conn).unwrap();
    conn
}

/// Inserts a row exactly as given, bypassing `insert_run`'s formatting.
pub fn insert_raw(conn: &Connection, date: &str, time: &str, miles: f64) -> i64 {
    conn.execute(
        "INSERT INTO runs (date, time_started, distance_miles, note, created_at)
         VALUES (?1, ?2, ?3, NULL, '2026-01-01T00:00:00+00:00')",
        params![date, time, miles],
    )
    .unwrap();
    conn.last_insert_rowid()
}

pub fn raw_time(conn: &Connection, id: i64) -> String {
    conn.query_row(
        "SELECT time_started FROM runs WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )
    .unwrap()
}

/// Makes every UPDATE or DELETE on `runs` fail, to simulate a storage write failure.
pub fn block_writes(conn: &Connection, op: &str) {
    conn.execute_batch(&format!(
        "CREATE TRIGGER block_{op} BEFORE {op} ON runs BEGIN SELECT RAISE(ABORT, '{op} blocked'); END;"
    ))
    .unwrap();
}

pub fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

pub fn ch(c: char) -> KeyEvent {
    key(KeyCode::Char(c))
}
