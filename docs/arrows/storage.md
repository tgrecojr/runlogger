# Arrow: storage

Local SQLite persistence of the run history — location, schema, stored formats, start-time repair, and insert/read/update/delete.

## Status

**AUDITED** — audited 2026-10-12 (git SHA `88de5b9`); since then STORE-005 (typed duplicate error) and STORE-014 (repair scope) were changed and tested. All 15 specs implemented and tested.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/storage/storage-design.md

### EARS
- docs/intent/storage/storage-specs.md (15 specs)

### Tests
- src/db/connection/tests.rs (STORE-001, STORE-002, STORE-003)
- src/db/queries/tests/reads.rs (STORE-012, STORE-013, STORE-015)
- src/db/queries/tests/startup.rs (STORE-003, STORE-014)
- src/db/queries/tests/writes.rs (STORE-004, STORE-005, STORE-006, STORE-007, STORE-008, STORE-009, STORE-010, STORE-011)

### Code
- src/db/connection.rs (STORE-001, STORE-002, STORE-003)
- src/db/migrations.rs (STORE-003, STORE-004, STORE-005, STORE-014)
- src/db/queries.rs (STORE-005, STORE-006, STORE-007, STORE-008, STORE-009, STORE-010, STORE-011, STORE-012, STORE-013, STORE-015)

## Architecture

**Purpose:** Keep the user's run history in one embedded SQLite file and return it to the rest of the app.

**Key Components:**
1. `connection.rs` — resolves `<data_dir>/runlogger/runs.db`, opens it, runs schema setup
2. `migrations.rs` — idempotent DDL plus the fractional-start-time repair
3. `queries.rs` — writes with whole-second times; reads raw rows and converts them in `to_run`

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps | Tested |
|----------|----------|-------------|----------|------|--------|
| Location and Startup | STORE-001, STORE-002, STORE-003, STORE-014 | 4 | 0 | 0 | 4 |
| Persisted Shape | STORE-004, STORE-005, STORE-006, STORE-007, STORE-008 | 5 | 0 | 0 | 5 |
| Operations | STORE-009, STORE-010, STORE-011, STORE-012, STORE-013, STORE-015 | 6 | 0 | 0 | 6 |

**Summary:** 15 of 15 specs marked implemented; 15 of 15 cited by a test. Untested: none.

## Key Findings

1. **Unreadable distance not named** — a non-numeric `distance_miles` fails `get_all_runs` with a raw rusqlite error that names no run, unlike STORE-015's date/time/timestamp errors. Not covered by a spec.
2. **Missing rows on update/delete** — updating or deleting an id that no longer exists succeeds silently (unreachable from the UI today). Not covered by a spec.
3. **No schema versioning**; **unused surface** (`get_runs_by_date_range`, `idx_runs_created_at`).

## Work Required

### Should Fix
1. Decide whether an unreadable distance should get a STORE-015-style error.

### Nice to Have
2. Schema versioning before the next schema change; remove or use `get_runs_by_date_range` and `idx_runs_created_at`.
