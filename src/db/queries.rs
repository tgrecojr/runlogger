use crate::models::run::Run;
use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveTime};
use rusqlite::{params, Connection};

pub fn insert_run(conn: &Connection, run: &Run) -> Result<i64> {
    conn.execute(
        "INSERT INTO runs (date, time_started, distance_miles, note, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            run.date.to_string(),
            run.time_started.to_string(),
            run.distance_miles,
            run.note,
            run.created_at.to_rfc3339(),
        ],
    )
    .context("Failed to insert run")?;

    Ok(conn.last_insert_rowid())
}

pub fn get_all_runs(conn: &Connection) -> Result<Vec<Run>> {
    let mut stmt = conn.prepare(
        "SELECT id, date, time_started, distance_miles, note, created_at
         FROM runs
         ORDER BY date DESC, time_started DESC",
    )?;

    let runs = stmt
        .query_map([], |row| {
            let date_str: String = row.get(1)?;
            let time_str: String = row.get(2)?;
            let created_str: String = row.get(5)?;

            Ok(Run {
                id: Some(row.get(0)?),
                date: NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
                time_started: NaiveTime::parse_from_str(&time_str, "%H:%M:%S")
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
                distance_miles: row.get(3)?,
                note: row.get(4)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?
                    .with_timezone(&chrono::Utc),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(runs)
}

#[allow(dead_code)]
pub fn get_runs_by_date_range(
    conn: &Connection,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<Run>> {
    let mut stmt = conn.prepare(
        "SELECT id, date, time_started, distance_miles, note, created_at
         FROM runs
         WHERE date >= ?1 AND date <= ?2
         ORDER BY date DESC, time_started DESC",
    )?;

    let runs = stmt
        .query_map(
            params![start_date.to_string(), end_date.to_string()],
            |row| {
                let date_str: String = row.get(1)?;
                let time_str: String = row.get(2)?;
                let created_str: String = row.get(5)?;

                Ok(Run {
                    id: Some(row.get(0)?),
                    date: NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                        .map_err(|_| rusqlite::Error::InvalidQuery)?,
                    time_started: NaiveTime::parse_from_str(&time_str, "%H:%M:%S")
                        .map_err(|_| rusqlite::Error::InvalidQuery)?,
                    distance_miles: row.get(3)?,
                    note: row.get(4)?,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                        .map_err(|_| rusqlite::Error::InvalidQuery)?
                        .with_timezone(&chrono::Utc),
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(runs)
}

pub fn update_run(conn: &Connection, run: &Run) -> Result<()> {
    let id = run.id.context("Run must have an id to be updated")?;
    conn.execute(
        "UPDATE runs SET date = ?1, time_started = ?2, distance_miles = ?3, note = ?4 WHERE id = ?5",
        params![
            run.date.to_string(),
            run.time_started.to_string(),
            run.distance_miles,
            run.note,
            id,
        ],
    )
    .context("Failed to update run")?;
    Ok(())
}

pub fn delete_run(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM runs WHERE id = ?1", params![id])
        .context("Failed to delete run")?;
    Ok(())
}
