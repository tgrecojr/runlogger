# Arrow: storage

Local SQLite persistence of the run history — location, schema, stored formats, start-time repair, and insert/read/update/delete.

## Status

**PARTIAL** — audited 2026-10-12 (git SHA `88de5b9`). 14 of 15 specs verified against code; STORE-014's startup repair is broader than specified.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/storage/storage-design.md

### EARS
- docs/intent/storage/storage-specs.md (15 specs)

### Tests
- src/db/queries/tests.rs (STORE-007, STORE-013, STORE-014, STORE-015)

### Code
- src/db/connection.rs (STORE-001, STORE-002, STORE-003)
- src/db/migrations.rs (STORE-003, STORE-004, STORE-005, STORE-014)
- src/db/queries.rs (STORE-006, STORE-007, STORE-008, STORE-009, STORE-010, STORE-011, STORE-012, STORE-013, STORE-015)

## Architecture

**Purpose:** Keep the user's run history in one embedded SQLite file and return it to the rest of the app.

**Key Components:**
1. `connection.rs` — resolves `<data_dir>/runlogger/runs.db`, opens it, runs schema setup
2. `migrations.rs` — idempotent DDL plus the fractional-start-time repair
3. `queries.rs` — writes with whole-second times; reads raw rows and converts them in `to_run`

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps | Tested |
|----------|----------|-------------|----------|------|--------|
| Location and Startup | STORE-001, STORE-002, STORE-003, STORE-014 | 4 | 0 | 0 | 1 |
| Persisted Shape | STORE-004, STORE-005, STORE-006, STORE-007, STORE-008 | 5 | 0 | 0 | 1 |
| Operations | STORE-009, STORE-010, STORE-011, STORE-012, STORE-013, STORE-015 | 6 | 0 | 0 | 2 |

**Summary:** 15 of 15 specs marked implemented; 4 of 15 cited by a test. Untested: STORE-001, STORE-002, STORE-003, STORE-004, STORE-005, STORE-006, STORE-008, STORE-009, STORE-010, STORE-011, STORE-012.

## Key Findings

1. **STORE-014 repair too broad** — `repair_start_times` (`src/db/migrations.rs`) truncates every `time_started` longer than 8 characters, not only fractional-second values; a malformed hand-edited value (e.g. `"bad\nvalue"`) is silently cut instead of left for STORE-015's error. Values written by the app (`HH:MM:SS`, `HH:MM:SS.ffffff`) are unaffected.
2. **Unreadable distance not named** — a non-numeric `distance_miles` fails `get_all_runs` with a raw rusqlite error that names no run (`query_runs`), unlike STORE-015's date/time/timestamp errors. Not covered by a spec.
3. **Missing rows on update/delete** — updating or deleting an id that no longer exists succeeds silently. Not covered by a spec.
4. **No schema versioning**; **unused surface** (`get_runs_by_date_range`, `idx_runs_created_at`).

## Work Required

### Must Fix
1. Restrict the startup repair to `HH:MM:SS.<digits>` values (STORE-014).

### Should Fix
2. Decide whether an unreadable distance should get a STORE-015-style error.
3. Add tests for STORE-001 to STORE-006 and STORE-008 to STORE-012.

### Nice to Have
4. Schema versioning before the next schema change; remove or use `get_runs_by_date_range` and `idx_runs_created_at`.
