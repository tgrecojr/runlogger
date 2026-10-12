# Arrow: run-list

The history view: full run list newest first, selection, edit hand-off and immediate delete.

## Status

**AUDITED** — audited 2026-10-12 (git SHA `88de5b9`). All 13 specs verified against code.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/run-list/run-list-design.md

### EARS
- docs/intent/run-list/run-list-specs.md (13 specs)

### Tests
- src/input/tests.rs (RL-010, RL-011, RL-012, RL-013)
- src/ui/tests.rs (RL-004, RL-007, RL-012)

### Code
- src/app.rs (RL-011)
- src/input/mod.rs (RL-001, RL-012)
- src/input/run_list.rs (RL-006, RL-008, RL-009, RL-010, RL-013)
- src/ui/screens/run_list.rs (RL-002, RL-003, RL-004, RL-005, RL-007, RL-012, RL-013)

## Architecture

**Purpose:** Let the user browse past runs and pick one to edit or delete.

**Key Components:**
1. `RunListState` — loaded runs, selected index, `TableState` scroll position, error message; `select_run`
2. `load_runs` — full reload from Storage, selection reset; records load failures
3. `handle_run_list_input` — ↑/↓, `e` hand-off to Quick Entry, `d` delete
4. `screens/run_list.rs` — error line, table, empty state, Actions panel

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps | Tested |
|----------|----------|-------------|----------|------|--------|
| Loading | RL-001 | 1 | 0 | 0 | 0 |
| Display | RL-002, RL-003, RL-004, RL-005 | 4 | 0 | 0 | 1 |
| Selection | RL-006, RL-007 | 2 | 0 | 0 | 1 |
| Edit and Delete | RL-008, RL-009, RL-010, RL-011 | 4 | 0 | 0 | 2 |
| Errors | RL-012, RL-013 | 2 | 0 | 0 | 2 |

**Summary:** 13 of 13 specs marked implemented; 6 of 13 cited by a test. Untested: RL-001, RL-002, RL-003, RL-005, RL-006, RL-008, RL-009.

## Key Findings

1. **Long errors are cut off** — the RL-012/RL-013 error line is one row, so a long cause chain is truncated rather than wrapped.
2. **"1 runs"** — the table title does not singularize.
3. **Reload on re-open** — pressing `2` while on Run List reloads and moves the selection to the top.

## Work Required

### Should Fix
1. Add tests for RL-001 to RL-003, RL-005, RL-006, RL-008 and RL-009.

### Nice to Have
2. Wrap long error messages; singular "1 run"; keep the selection when `2` is pressed on Run List.
