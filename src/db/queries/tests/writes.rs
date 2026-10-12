use super::*;

/// The write was rejected as a duplicate of an existing run's date and start time.
fn assert_duplicate(
    err: &anyhow::Error,
    (y, m, d): (i32, u32, u32),
    (h, min, sec): (u32, u32, u32),
) {
    match err.downcast_ref::<StorageError>() {
        Some(StorageError::DuplicateStart { date, time }) => {
            assert_eq!(*date, NaiveDate::from_ymd_opt(y, m, d).unwrap());
            assert_eq!(*time, NaiveTime::from_hms_opt(h, min, sec).unwrap());
        }
        _ => panic!("not a DuplicateStart error: {err:#}"),
    }
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

// @spec STORE-004
#[test]
fn schema_requires_every_field_except_the_note() {
    let conn = memory_db();
    let columns = scalar(
        &conn,
        "SELECT group_concat(name || ' ' || type || ' ' || \"notnull\" || ' ' || pk, ', ')
         FROM pragma_table_info('runs')",
    );
    assert_eq!(
        columns,
        "id INTEGER 0 1, date TEXT 1 0, time_started TEXT 1 0, \
         distance_miles REAL 1 0, note TEXT 0 0, created_at TEXT 1 0"
    );

    let sql = "INSERT INTO runs (date, time_started, distance_miles, created_at) VALUES";
    for values in [
        "NULL, 't', 1, 'c'",
        "'d', NULL, 1, 'c'",
        "'d', 't', NULL, 'c'",
        "'d', 't', 1, NULL",
    ] {
        let result = conn.execute(&format!("{sql} ({values})"), []);
        assert!(result.is_err(), "accepted {values}");
    }
}

// @spec STORE-005
#[test]
fn insert_with_duplicate_date_and_start_time_is_rejected() {
    let conn = memory_db();
    insert_run(&conn, &run_on((2026, 10, 1), (7, 0, 0), 1.0, None)).unwrap();
    let dup = run_on((2026, 10, 1), (7, 0, 0), 5.0, Some("again"));
    let err = insert_run(&conn, &dup).unwrap_err();
    assert_duplicate(&err, (2026, 10, 1), (7, 0, 0));
    assert_eq!(history_ids(&conn).len(), 1);
}

// @spec STORE-005
#[test]
fn update_onto_another_runs_date_and_start_time_is_rejected() {
    let conn = memory_db();
    insert_run(&conn, &run_on((2026, 10, 1), (7, 0, 0), 1.0, None)).unwrap();
    let id = insert_run(&conn, &run_on((2026, 10, 2), (8, 0, 0), 2.0, None)).unwrap();
    let mut edited = run_on((2026, 10, 1), (7, 0, 0), 2.0, None);
    edited.id = Some(id);
    let err = update_run(&conn, &edited).unwrap_err();
    assert_duplicate(&err, (2026, 10, 1), (7, 0, 0));
    assert_eq!(raw_text(&conn, id, "date"), "2026-10-02");
    assert_eq!(raw_time(&conn, id), "08:00:00");
}

// @spec STORE-006
#[test]
fn date_is_stored_as_zero_padded_local_calendar_text() {
    let conn = memory_db();
    let id = insert_run(&conn, &run_on((2026, 3, 5), (23, 59, 59), 1.0, None)).unwrap();
    assert_eq!(raw_text(&conn, id, "date"), "2026-03-05");

    let mut edited = run_on((2027, 1, 9), (0, 0, 0), 1.0, None);
    edited.id = Some(id);
    update_run(&conn, &edited).unwrap();
    assert_eq!(raw_text(&conn, id, "date"), "2027-01-09");
}

// @spec STORE-008
#[test]
fn creation_timestamp_is_stored_as_utc_rfc3339_at_creation() {
    let conn = memory_db();
    let before = Utc::now();
    let run = run_on((2026, 10, 1), (7, 0, 0), 1.0, None);
    let after = Utc::now();
    let id = insert_run(&conn, &run).unwrap();

    let stored = raw_text(&conn, id, "created_at");
    let parsed = DateTime::parse_from_rfc3339(&stored).unwrap();
    assert_eq!(parsed.offset().local_minus_utc(), 0, "{stored}");
    assert_eq!(parsed.with_timezone(&Utc), run.created_at);
    assert!(before <= run.created_at && run.created_at <= after);
}

// @spec STORE-009
#[test]
fn saving_a_new_run_inserts_it_with_a_new_id() {
    let conn = memory_db();
    let first = insert_run(&conn, &run_on((2026, 10, 1), (7, 0, 0), 1.0, None)).unwrap();
    let second = insert_run(&conn, &run_on((2026, 10, 2), (7, 0, 0), 2.0, None)).unwrap();
    assert_ne!(first, second);
    assert_eq!(history_ids(&conn), vec![second, first]);
}

// @spec STORE-010
#[test]
fn update_changes_fields_by_id_and_keeps_id_and_creation_timestamp() {
    let conn = memory_db();
    let other = insert_run(&conn, &run_on((2026, 9, 1), (6, 0, 0), 1.0, Some("keep"))).unwrap();
    let id = insert_run(&conn, &run_on((2026, 10, 1), (7, 0, 0), 1.0, None)).unwrap();
    let other_before = raw_row(&conn, other);
    let created = raw_text(&conn, id, "created_at");

    let mut edited = run_on((2026, 10, 4), (9, 15, 30), 6.5, Some("tempo"));
    edited.id = Some(id);
    edited.created_at = DateTime::parse_from_rfc3339("2030-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    update_run(&conn, &edited).unwrap();

    let expected = format!("2026-10-04 09:15:30 6.5 tempo {created}");
    assert_eq!(raw_row(&conn, id), expected);
    assert_eq!(raw_row(&conn, other), other_before);
    assert_eq!(history_ids(&conn), vec![id, other]);
}

// @spec STORE-011
#[test]
fn delete_removes_only_the_run_with_that_id() {
    let conn = memory_db();
    let keep = insert_run(&conn, &run_on((2026, 10, 1), (7, 0, 0), 1.0, None)).unwrap();
    let gone = insert_run(&conn, &run_on((2026, 10, 2), (7, 0, 0), 2.0, None)).unwrap();
    delete_run(&conn, gone).unwrap();
    assert_eq!(history_ids(&conn), vec![keep]);
}
