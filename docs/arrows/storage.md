# Arrow: storage

Local SQLite persistence of the run history — location, schema, stored formats, start-time repair, and insert/read/update/delete.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. All 15 specs implemented. Start times are stored and read in whole seconds, older fractional rows are repaired at startup, and unreadable rows produce a precise error.

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
- src/db/connection.rs
- src/db/migrations.rs (`init_database`, `repair_start_times`)
- src/db/queries.rs (`insert_run`, `get_all_runs`, `update_run`, `delete_run`, `query_runs`, `to_run`)
- src/models/run.rs (`Run` struct layout)

## Architecture

**Purpose:** Keep the user's run history in one embedded SQLite file and return it to the rest of the app.

**Key Components:**
1. `connection.rs` — resolves `<data_dir>/runlogger/runs.db`, opens it, runs schema setup
2. `migrations.rs` — idempotent DDL plus the fractional-start-time repair
3. `queries.rs` — writes with whole-second times; reads raw rows and converts them in `to_run`

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps |
|----------|----------|-------------|----------|------|
| Location and startup | STORE-001 to STORE-003, STORE-014 | 4 | 0 | 0 |
| Persisted shape | STORE-004 to STORE-008 | 5 | 0 | 0 |
| Operations | STORE-009 to STORE-013, STORE-015 | 6 | 0 | 0 |

**Summary:** 15 of 15 specs implemented. Tests cite STORE-007, STORE-013, STORE-014, STORE-015; STORE-001 to STORE-006 and STORE-008 to STORE-012 have no test yet.

## Key Findings

1. **No schema versioning** — `src/db/migrations.rs` has only `IF NOT EXISTS` DDL and the idempotent repair.
2. **Unused surface** — `get_runs_by_date_range` and `idx_runs_created_at` have no consumers.

## Work Required

### Should Fix
1. Add tests for STORE-001 to STORE-006 and STORE-008 to STORE-012.

### Nice to Have
2. Introduce schema versioning before the next schema change.
3. Remove or use `get_runs_by_date_range` and `idx_runs_created_at`.
