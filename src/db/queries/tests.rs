use super::*;
use crate::db::migrations::init_database;
use crate::test_support::{block_writes, insert_raw, memory_db, raw_time};
use chrono::Timelike;

fn run_at(time: NaiveTime) -> Run {
    Run::new(
        NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
        time,
        2.0,
        None,
    )
    .unwrap()
}

// @spec STORE-007
#[test]
fn insert_and_update_store_whole_second_start_times() {
    let conn = memory_db();
    let precise = NaiveTime::from_hms_nano_opt(7, 15, 30, 165_846_000).unwrap();
    let id = insert_run(&conn, &run_at(precise)).unwrap();
    assert_eq!(raw_time(&conn, id), "07:15:30");

    let mut edited = run_at(NaiveTime::from_hms_nano_opt(8, 0, 1, 5).unwrap());
    edited.id = Some(id);
    update_run(&conn, &edited).unwrap();
    assert_eq!(raw_time(&conn, id), "08:00:01");
}

// @spec STORE-013
#[test]
fn read_accepts_fractional_start_times_and_truncates_them() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "11:51:04.165846", 1.5);
    let runs = get_all_runs(&conn).unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(
        runs[0].time_started,
        NaiveTime::from_hms_opt(11, 51, 4).unwrap()
    );
    assert_eq!(runs[0].time_started.nanosecond(), 0);
}

// @spec STORE-014
#[test]
fn startup_repairs_fractional_start_times() {
    let conn = memory_db();
    let id = insert_raw(&conn, "2026-10-01", "11:51:04.165846", 1.5);
    init_database(&conn).unwrap();
    assert_eq!(raw_time(&conn, id), "11:51:04");
}

// @spec STORE-014
#[test]
fn startup_repair_skips_rows_that_would_duplicate_another_run() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "07:00:00", 1.0);
    let id = insert_raw(&conn, "2026-10-01", "07:00:00.500", 1.0);
    init_database(&conn).unwrap();
    assert_eq!(raw_time(&conn, id), "07:00:00.500");
}

// @spec STORE-014
#[test]
fn startup_continues_when_the_repair_fails() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "11:51:04.165846", 1.5);
    block_writes(&conn, "UPDATE");
    assert!(init_database(&conn).is_ok());
}

// @spec STORE-015
#[test]
fn unreadable_row_fails_the_read_naming_id_field_and_value() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "07:00:00", 1.0);
    let bad = insert_raw(&conn, "2026-13-45", "07:00:00", 1.0);
    let err = format!("{:#}", get_all_runs(&conn).unwrap_err());
    assert!(err.contains(&format!("Run {bad}")), "{err}");
    assert!(err.contains("date"), "{err}");
    assert!(err.contains("\"2026-13-45\""), "{err}");
}

// @spec STORE-015
#[test]
fn unreadable_value_is_quoted_with_control_characters_escaped() {
    let conn = memory_db();
    insert_raw(&conn, "2026-10-01", "bad\nvalue", 1.0);
    let err = format!("{:#}", get_all_runs(&conn).unwrap_err());
    assert!(err.contains("start time"), "{err}");
    assert!(err.contains("\"bad\\nvalue\""), "{err}");
    assert!(!err.contains('\n'), "{err}");
}
