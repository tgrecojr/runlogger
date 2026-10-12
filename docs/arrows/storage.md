# Arrow: storage

Local SQLite persistence of the run history — location, schema, stored formats, start-time repair, and insert/read/update/delete.

## Status

**AUDITED** — audited 2026-10-12 (git SHA `88de5b9`); since then schema versioning (STORE-016, STORE-017), the unreadable-distance error (STORE-015) and earlier fixes landed with tests. All 17 specs implemented and tested.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/storage/storage-design.md

### EARS
- docs/intent/storage/storage-specs.md (17 specs)

### Tests
- src/db/connection/tests.rs (STORE-001, STORE-002, STORE-003)
- src/db/queries/tests/reads.rs (STORE-012, STORE-013, STORE-015)
- src/db/queries/tests/startup.rs (STORE-003, STORE-014, STORE-016, STORE-017)
- src/db/queries/tests/writes.rs (STORE-004, STORE-005, STORE-006, STORE-007, STORE-008, STORE-009, STORE-010, STORE-011)

### Code
- src/db/connection.rs (STORE-001, STORE-002, STORE-003)
- src/db/migrations.rs (STORE-003, STORE-004, STORE-005, STORE-014, STORE-016, STORE-017)
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
| Location and Startup | STORE-001, STORE-002, STORE-003, STORE-016, STORE-017, STORE-014 | 6 | 0 | 0 | 6 |
| Persisted Shape | STORE-004, STORE-005, STORE-006, STORE-007, STORE-008 | 5 | 0 | 0 | 5 |
| Operations | STORE-009, STORE-010, STORE-011, STORE-012, STORE-013, STORE-015 | 6 | 0 | 0 | 6 |

**Summary:** 17 of 17 specs marked implemented; 17 of 17 cited by a test. Untested: none.

## Key Findings

1. **Missing rows on update/delete** — updating or deleting an id that no longer exists succeeds silently (unreachable from the UI). Not covered by a spec.
2. **Unused surface** — `get_runs_by_date_range` and `idx_runs_created_at` have no consumers.

## Work Required

### Nice to Have
1. Remove or use `get_runs_by_date_range` and `idx_runs_created_at` (the index removal would be the first new migration).
