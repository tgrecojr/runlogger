use super::*;

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

// @spec STORE-014
#[test]
fn startup_repair_leaves_other_malformed_values_alone() {
    let conn = memory_db();
    let garbage = insert_raw(&conn, "2026-10-01", "bad\nvalue", 1.0);
    let suffixed = insert_raw(&conn, "2026-10-02", "07:00:00Z", 1.0);
    let fractional = insert_raw(&conn, "2026-10-03", "11:51:04.165846", 1.0);
    init_database(&conn).unwrap();

    assert_eq!(raw_time(&conn, garbage), "bad\nvalue");
    assert_eq!(raw_time(&conn, suffixed), "07:00:00Z");
    assert_eq!(raw_time(&conn, fractional), "11:51:04");
}

// @spec STORE-003
#[test]
fn startup_creates_runs_table_and_indexes() {
    let conn = memory_db();
    let objects = scalar(
        &conn,
        "SELECT group_concat(type || ' ' || name, ', ') FROM (SELECT type, name
         FROM sqlite_master WHERE tbl_name = 'runs' AND sql IS NOT NULL ORDER BY name)",
    );
    assert_eq!(objects, "index idx_runs_date, table runs");
}

// @spec STORE-003
#[test]
fn startup_on_existing_database_leaves_run_data_unchanged() {
    let conn = memory_db();
    let id = insert_run(&conn, &run_on((2026, 9, 3), (6, 30, 0), 4.25, Some("easy"))).unwrap();
    let before = raw_row(&conn, id);
    init_database(&conn).unwrap();
    assert_eq!(raw_row(&conn, id), before);
    assert_eq!(history_ids(&conn), vec![id]);
}

fn user_version(conn: &rusqlite::Connection) -> i64 {
    conn.query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap()
}

// @spec STORE-016
#[test]
fn new_database_is_at_the_current_schema_version() {
    let conn = memory_db();
    assert_eq!(user_version(&conn), crate::db::migrations::SCHEMA_VERSION);
}

// @spec STORE-016
#[test]
fn unversioned_database_is_migrated_without_losing_runs() {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "CREATE TABLE runs (id INTEGER PRIMARY KEY AUTOINCREMENT, date TEXT NOT NULL,
             time_started TEXT NOT NULL, distance_miles REAL NOT NULL, note TEXT,
             created_at TEXT NOT NULL, UNIQUE(date, time_started));
         INSERT INTO runs (date, time_started, distance_miles, note, created_at)
             VALUES ('2026-10-01', '07:00:00', 2.0, 'old', '2026-01-01T00:00:00+00:00');",
    )
    .unwrap();
    assert_eq!(user_version(&conn), 0);

    init_database(&conn).unwrap();
    assert_eq!(user_version(&conn), crate::db::migrations::SCHEMA_VERSION);
    let runs = get_all_runs(&conn).unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].note.as_deref(), Some("old"));

    init_database(&conn).unwrap(); // a current database is left as is
    assert_eq!(get_all_runs(&conn).unwrap().len(), 1);
}

// @spec STORE-017
#[test]
fn database_from_a_newer_app_is_refused() {
    let conn = memory_db();
    conn.execute_batch("PRAGMA user_version = 99").unwrap();
    let err = init_database(&conn).unwrap_err();
    assert!(
        err.to_string()
            .contains("written by a newer version of Run Logger"),
        "{err:#}"
    );
    assert_eq!(user_version(&conn), 99);
}

// @spec STORE-016
#[test]
fn version_1_database_drops_the_unused_index_and_keeps_its_runs() {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "CREATE TABLE runs (id INTEGER PRIMARY KEY AUTOINCREMENT, date TEXT NOT NULL,
             time_started TEXT NOT NULL, distance_miles REAL NOT NULL, note TEXT,
             created_at TEXT NOT NULL, UNIQUE(date, time_started));
         CREATE INDEX idx_runs_date ON runs(date DESC);
         CREATE INDEX idx_runs_created_at ON runs(created_at DESC);
         INSERT INTO runs (date, time_started, distance_miles, note, created_at)
             VALUES ('2026-10-01', '07:00:00', 2.0, NULL, '2026-01-01T00:00:00+00:00');
         PRAGMA user_version = 1;",
    )
    .unwrap();

    init_database(&conn).unwrap();
    assert_eq!(user_version(&conn), 2);
    let created_at_index = scalar(
        &conn,
        "SELECT CAST(count(*) AS TEXT) FROM sqlite_master WHERE name = 'idx_runs_created_at'",
    );
    assert_eq!(created_at_index, "0");
    assert_eq!(get_all_runs(&conn).unwrap().len(), 1);
}
