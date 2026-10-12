# Arrow: run-list

The history view: full run list newest first, selection, edit hand-off and immediate delete.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. All 13 specs implemented.

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
- src/app.rs (`RunListState`, `select_run`)
- src/input/run_list.rs (`handle_run_list_input`)
- src/input/mod.rs (`load_runs`)
- src/ui/screens/run_list.rs

## Architecture

**Purpose:** Let the user browse past runs and pick one to edit or delete.

**Key Components:**
1. `RunListState` — loaded runs, selected index, `TableState` scroll position, error message; `select_run`
2. `load_runs` — full reload from Storage, selection reset; records load failures
3. `handle_run_list_input` — ↑/↓, `e` hand-off to Quick Entry, `d` delete
4. `screens/run_list.rs` — error line, table, empty state, Actions panel

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps |
|----------|----------|-------------|----------|------|
| Loading | RL-001 | 1 | 0 | 0 |
| Display | RL-002 to RL-005 | 4 | 0 | 0 |
| Selection | RL-006 to RL-007 | 2 | 0 | 0 |
| Edit and delete | RL-008 to RL-011 | 4 | 0 | 0 |
| Errors | RL-012 to RL-013 | 2 | 0 | 0 |

**Summary:** 13 of 13 specs implemented. Tests cite RL-004, RL-007 and RL-010 to RL-013.

## Key Findings

None outstanding.

## Work Required

### Should Fix
1. Add tests for RL-001 to RL-003, RL-005, RL-006, RL-008 and RL-009.
