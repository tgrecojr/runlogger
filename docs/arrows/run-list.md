# Arrow: run-list

The history view: full run list newest first, selection, edit hand-off and immediate delete.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. One gap: the selection jumps to the top after a delete (RL-010).

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/run-list/run-list-design.md

### EARS
- docs/intent/run-list/run-list-specs.md (13 specs)

### Tests
- src/input/tests.rs (RL-011, RL-012, RL-013)
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
| Edit and delete | RL-008 to RL-011 | 3 | 0 | 1 |
| Errors | RL-012 to RL-013 | 2 | 0 | 0 |

**Summary:** 12 of 13 specs implemented; 1 gap (RL-010). Tests cite RL-004, RL-007, RL-011, RL-012 and RL-013.

## Key Findings

1. **Selection jumps to the top after a delete** — the clamp after the delete reload runs after `load_runs` has already reset the selection to 0 (RL-010).

## Work Required

### Must Fix
1. Keep the selection's row position across a delete reload (RL-010).

### Should Fix
2. Add tests for the remaining RL specs.

### Nice to Have
3. Make the footer and Actions panel list the same complete set of keys.
