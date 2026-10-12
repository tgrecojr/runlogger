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
    assert_eq!(
        objects,
        "index idx_runs_created_at, index idx_runs_date, table runs"
    );
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
