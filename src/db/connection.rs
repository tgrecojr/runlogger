use anyhow::{Context, Result};
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

// @spec STORE-001, STORE-002
pub fn get_db_path() -> Result<PathBuf> {
    db_path_in(dirs::data_dir())
}

/// `runlogger/runs.db` inside the per-user data directory, creating
/// `runlogger` if needed.
fn db_path_in(data_dir: Option<PathBuf>) -> Result<PathBuf> {
    let data_dir = data_dir.context("Failed to get data directory")?;

    let app_dir = data_dir.join("runlogger");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).context("Failed to create application directory")?;
    }

    Ok(app_dir.join("runs.db"))
}

fn open_connection(db_path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path).context("Failed to open database connection")?;

    Ok(conn)
}

// @spec STORE-001, STORE-002, STORE-003
pub fn init_db(db_path: &PathBuf) -> Result<Connection> {
    let conn = open_connection(db_path)?;
    crate::db::migrations::init_database(&conn).map_err(|e| {
        // A newer-schema refusal already says what happened; keep it on top.
        if e.is::<crate::db::queries::StorageError>() {
            e
        } else {
            e.context("Failed to initialize database")
        }
    })?;
    Ok(conn)
}

#[cfg(test)]
mod tests;
