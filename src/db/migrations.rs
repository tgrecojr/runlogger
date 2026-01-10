use rusqlite::{Connection, Result};

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
    Ok(())
}
