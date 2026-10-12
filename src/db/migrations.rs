use rusqlite::{Connection, Result};

// @spec STORE-003, STORE-004, STORE-005, STORE-014
pub fn init_database(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS runs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            time_started TEXT NOT NULL,
            distance_miles REAL NOT NULL,
            note TEXT,
            created_at TEXT NOT NULL,
            UNIQUE(date, time_started)
        );

        CREATE INDEX IF NOT EXISTS idx_runs_date ON runs(date DESC);
        CREATE INDEX IF NOT EXISTS idx_runs_created_at ON runs(created_at DESC);
        ",
    )?;
    repair_start_times(conn);
    Ok(())
}

/// Rewrites start times stored as `HH:MM:SS.<digits>` (by older versions) to
/// whole seconds; any other malformed value is left for the read to report. A row whose rewrite would duplicate another run is left as is;
/// a failed repair is ignored because reads tolerate fractional times.
// @spec STORE-014
fn repair_start_times(conn: &Connection) {
    let _ = conn.execute(
        "UPDATE OR IGNORE runs SET time_started = substr(time_started, 1, 8)
         WHERE time_started GLOB '[0-2][0-9]:[0-5][0-9]:[0-5][0-9].[0-9]*'
           AND substr(time_started, 10) NOT GLOB '*[^0-9]*'",
        [],
    );
}
