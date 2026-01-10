use anyhow::{Context, Result};
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

pub fn get_db_path() -> Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .context("Failed to get data directory")?;

    let app_dir = data_dir.join("runlogger");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .context("Failed to create application directory")?;
    }

    Ok(app_dir.join("runs.db"))
}

pub fn open_connection(db_path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)
        .context("Failed to open database connection")?;

    Ok(conn)
}

pub fn init_db(db_path: &PathBuf) -> Result<Connection> {
    let conn = open_connection(db_path)?;
    crate::db::migrations::init_database(&conn)
        .context("Failed to initialize database")?;
    Ok(conn)
}
