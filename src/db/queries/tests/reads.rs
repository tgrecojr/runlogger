use super::*;

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

// @spec STORE-012
#[test]
fn full_history_is_ordered_by_date_then_start_time_descending() {
    let conn = memory_db();
    let a = insert_run(&conn, &run_on((2026, 10, 1), (18, 0, 0), 1.0, None)).unwrap();
    let b = insert_run(&conn, &run_on((2026, 10, 3), (6, 0, 0), 1.0, None)).unwrap();
    let c = insert_run(&conn, &run_on((2026, 10, 1), (6, 0, 0), 1.0, None)).unwrap();
    let d = insert_run(&conn, &run_on((2026, 10, 3), (19, 30, 0), 1.0, None)).unwrap();
    let e = insert_run(&conn, &run_on((2025, 12, 31), (23, 0, 0), 1.0, None)).unwrap();
    assert_eq!(history_ids(&conn), vec![d, b, a, c, e]);
}

// @spec STORE-015
#[test]
fn unreadable_distance_fails_the_read_naming_the_run() {
    let conn = memory_db();
    conn.execute(
        "INSERT INTO runs (date, time_started, distance_miles, note, created_at)
         VALUES ('2026-10-01', '07:00:00', 'abc', NULL, '2026-01-01T00:00:00+00:00')",
        [],
    )
    .unwrap();
    let id = conn.last_insert_rowid();
    let err = format!("{:#}", get_all_runs(&conn).unwrap_err());
    assert!(
        err.contains(&format!("Run {id} has an unreadable distance")),
        "{err}"
    );
    assert!(err.contains("\"abc\""), "{err}");
}
