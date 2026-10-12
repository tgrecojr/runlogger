use crate::models::run::Run;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, Timelike, Utc};
use rusqlite::{params, types::Value, Connection, Params};

/// Storage failures callers need to tell apart from generic SQLite errors.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Another run already starts at {date} {time}")]
    DuplicateStart { date: NaiveDate, time: NaiveTime },
    #[error("This database was written by a newer version of Run Logger (schema {found}; this version supports {supported})")]
    NewerSchema { found: i64, supported: i64 },
}

const DATE_FORMAT: &str = "%Y-%m-%d";
/// Start times are stored to the whole second, whatever precision the caller supplies.
const TIME_FORMAT: &str = "%H:%M:%S";

// @spec STORE-006, STORE-007, STORE-008, STORE-009
pub fn insert_run(conn: &Connection, run: &Run) -> Result<i64> {
    conn.execute(
        "INSERT INTO runs (date, time_started, distance_miles, note, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            run.date.to_string(),
            run.time_started.format(TIME_FORMAT).to_string(),
            run.distance_miles,
            run.note,
            run.created_at.to_rfc3339(),
        ],
    )
    .map_err(|e| write_error(e, run, "Failed to insert run"))?;

    Ok(conn.last_insert_rowid())
}

// @spec STORE-012
pub fn get_all_runs(conn: &Connection) -> Result<Vec<Run>> {
    query_runs(
        conn,
        "SELECT id, date, time_started, distance_miles, note, created_at
         FROM runs
         ORDER BY date DESC, time_started DESC",
        [],
    )
}

/// A uniqueness violation becomes a typed `DuplicateStart` naming the
/// conflicting date and start time; anything else keeps its SQLite cause.
// @spec STORE-005
fn write_error(e: rusqlite::Error, run: &Run, context: &'static str) -> anyhow::Error {
    let duplicate = matches!(
        &e,
        rusqlite::Error::SqliteFailure(f, _)
            if f.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE
    );
    if duplicate {
        let time = run
            .time_started
            .with_nanosecond(0)
            .unwrap_or(run.time_started);
        anyhow::Error::new(StorageError::DuplicateStart {
            date: run.date,
            time,
        })
    } else {
        anyhow::Error::new(e).context(context)
    }
}

/// A `runs` row as stored, before any text column is parsed.
struct StoredRun {
    id: i64,
    date: String,
    time_started: String,
    distance_miles: Value,
    note: Option<String>,
    created_at: String,
}

fn query_runs(conn: &Connection, sql: &str, params: impl Params) -> Result<Vec<Run>> {
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt
        .query_map(params, |row| {
            Ok(StoredRun {
                id: row.get(0)?,
                date: row.get(1)?,
                time_started: row.get(2)?,
                distance_miles: row.get(3)?,
                note: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    rows.into_iter().map(to_run).collect()
}

// @spec STORE-013, STORE-015
fn to_run(row: StoredRun) -> Result<Run> {
    let id = row.id;
    let unreadable =
        |field: &str, value: &str| anyhow!("Run {id} has an unreadable {field}: {value:?}");

    let date = NaiveDate::parse_from_str(&row.date, DATE_FORMAT)
        .map_err(|_| unreadable("date", &row.date))?;
    // Older versions stored fractional seconds; accept them and drop the fraction.
    let time_started = NaiveTime::parse_from_str(&row.time_started, "%H:%M:%S%.f")
        .ok()
        .and_then(|t| t.with_nanosecond(0))
        .ok_or_else(|| unreadable("start time", &row.time_started))?;
    let created_at = DateTime::parse_from_rfc3339(&row.created_at)
        .map_err(|_| unreadable("creation timestamp", &row.created_at))?
        .with_timezone(&Utc);
    let distance_miles = match &row.distance_miles {
        Value::Real(miles) => *miles,
        Value::Integer(miles) => *miles as f64,
        Value::Text(text) => return Err(unreadable("distance", text)),
        Value::Null => return Err(unreadable("distance", "NULL")),
        Value::Blob(bytes) => {
            return Err(unreadable("distance", &format!("<{} bytes>", bytes.len())))
        }
    };

    Ok(Run {
        id: Some(id),
        date,
        time_started,
        distance_miles,
        note: row.note,
        created_at,
    })
}

// @spec STORE-006, STORE-007, STORE-010
pub fn update_run(conn: &Connection, run: &Run) -> Result<()> {
    let id = run.id.context("Run must have an id to be updated")?;
    conn.execute(
        "UPDATE runs SET date = ?1, time_started = ?2, distance_miles = ?3, note = ?4 WHERE id = ?5",
        params![
            run.date.to_string(),
            run.time_started.format(TIME_FORMAT).to_string(),
            run.distance_miles,
            run.note,
            id,
        ],
    )
    .map_err(|e| write_error(e, run, "Failed to update run"))?;
    Ok(())
}

// @spec STORE-011
pub fn delete_run(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM runs WHERE id = ?1", params![id])
        .context("Failed to delete run")?;
    Ok(())
}

#[cfg(test)]
mod tests;
