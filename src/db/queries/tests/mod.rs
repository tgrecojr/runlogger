use super::*;
use crate::db::migrations::init_database;
use crate::test_support::{block_writes, insert_raw, memory_db, raw_time};
use chrono::Timelike;

mod reads;
mod startup;
mod writes;

fn run_at(time: NaiveTime) -> Run {
    Run::new(
        NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
        time,
        2.0,
        None,
    )
    .unwrap()
}

fn run_on(date: (i32, u32, u32), time: (u32, u32, u32), miles: f64, note: Option<&str>) -> Run {
    Run::new(
        NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap(),
        NaiveTime::from_hms_opt(time.0, time.1, time.2).unwrap(),
        miles,
        note.map(str::to_string),
    )
    .unwrap()
}

fn scalar(conn: &Connection, sql: &str) -> String {
    conn.query_row(sql, [], |row| row.get(0)).unwrap()
}

fn raw_text(conn: &Connection, id: i64, column: &str) -> String {
    scalar(conn, &format!("SELECT {column} FROM runs WHERE id = {id}"))
}

/// The stored date, start time, distance, note and creation timestamp, space-separated.
fn raw_row(conn: &Connection, id: i64) -> String {
    raw_text(
        conn,
        id,
        "date || ' ' || time_started || ' ' || distance_miles || ' ' || \
         coalesce(note, '-') || ' ' || created_at",
    )
}

/// Ids of every stored run, in the order the full history returns them.
fn history_ids(conn: &Connection) -> Vec<i64> {
    let runs = get_all_runs(conn).unwrap();
    runs.iter().map(|r| r.id.unwrap()).collect()
}
