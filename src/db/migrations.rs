use crate::db::queries::StorageError;
use anyhow::Result;
use rusqlite::Connection;

/// Schema migrations in order; applying entry *n* brings a database to
/// version *n*+1. Append new migrations; never edit a released one.
const MIGRATIONS: &[&str] = &[
    // 1: the original schema. `IF NOT EXISTS` lets databases created before
    // versioning (version 0, tables present) pass through unchanged.
    "CREATE TABLE IF NOT EXISTS runs (
         id INTEGER PRIMARY KEY AUTOINCREMENT,
         date TEXT NOT NULL,
         time_started TEXT NOT NULL,
         distance_miles REAL NOT NULL,
         note TEXT,
         created_at TEXT NOT NULL,
         UNIQUE(date, time_started)
     );
     CREATE INDEX IF NOT EXISTS idx_runs_date ON runs(date DESC);
     CREATE INDEX IF NOT EXISTS idx_runs_created_at ON runs(created_at DESC);",
    // 2: drop an index no query uses.
    "DROP INDEX IF EXISTS idx_runs_created_at;",
];

/// The schema version this app writes and understands.
pub const SCHEMA_VERSION: i64 = MIGRATIONS.len() as i64;

/// Brings the database to `SCHEMA_VERSION`, then repairs legacy start times.
/// A database from a newer app is refused rather than misread.
// @spec STORE-003, STORE-004, STORE-005, STORE-014, STORE-016, STORE-017
pub fn init_database(conn: &Connection) -> Result<()> {
    let version: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if version > SCHEMA_VERSION {
        return Err(StorageError::NewerSchema {
            found: version,
            supported: SCHEMA_VERSION,
        }
        .into());
    }
    for (applied, sql) in MIGRATIONS.iter().enumerate().skip(version as usize) {
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", applied as i64 + 1)?;
        tx.commit()?;
    }
    repair_start_times(conn);
    Ok(())
}

/// Rewrites start times stored as `HH:MM:SS.<digits>` (by older versions) to
/// whole seconds; any other malformed value is left for the read to report.
/// A row whose rewrite would duplicate another run is left as is; a failed
/// repair is ignored because reads tolerate fractional times.
// @spec STORE-014
fn repair_start_times(conn: &Connection) {
    let _ = conn.execute(
        "UPDATE OR IGNORE runs SET time_started = substr(time_started, 1, 8)
         WHERE time_started GLOB '[0-2][0-9]:[0-5][0-9]:[0-5][0-9].[0-9]*'
           AND substr(time_started, 10) NOT GLOB '*[^0-9]*'",
        [],
    );
}
