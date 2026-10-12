# Arrow: run-list

The history view: full run list newest first, selection, edit hand-off and immediate delete.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. Load and delete failures now show on screen. Three gaps: no scrolling (RL-007), jump to top after delete (RL-010), edited run not selected after an edit (RL-011).

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/run-list/run-list-design.md

### EARS
- docs/intent/run-list/run-list-specs.md (13 specs)

### Tests
- src/input/tests.rs (RL-012, RL-013)
- src/ui/tests.rs (RL-004, RL-012)

### Code
- src/app.rs (`RunListState`)
- src/input/run_list.rs (`handle_run_list_input`)
- src/input/mod.rs (`load_runs`)
- src/ui/screens/run_list.rs

## Architecture

**Purpose:** Let the user browse past runs and pick one to edit or delete.

**Key Components:**
1. `RunListState` — loaded runs, selected index, (unused) scroll offset, error message
2. `load_runs` — full reload from Storage, selection reset; records load failures
3. `handle_run_list_input` — ↑/↓, `e` hand-off to Quick Entry, `d` delete
4. `screens/run_list.rs` — error line, table, empty state, Actions panel

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps |
|----------|----------|-------------|----------|------|
| Loading | RL-001 | 1 | 0 | 0 |
| Display | RL-002 to RL-005 | 4 | 0 | 0 |
| Selection | RL-006 to RL-007 | 1 | 0 | 1 |
| Edit and delete | RL-008 to RL-011 | 2 | 0 | 2 |
| Errors | RL-012 to RL-013 | 2 | 0 | 0 |

**Summary:** 10 of 13 specs implemented; 3 gaps (RL-007, RL-010, RL-011). Tests cite RL-004, RL-012 and RL-013.

## Key Findings

1. **No scrolling** — `scroll_offset` is maintained in `handle_run_list_input` (hard-coded 10-row page) but never read by `src/ui/screens/run_list.rs`, which renders the `Table` without a `TableState` (RL-007).
2. **Dead clamp after delete** — the clamp after the delete reload runs after `load_runs` has already reset the selection to 0 (RL-010).

## Work Required

### Must Fix
1. Render with a `TableState` (or equivalent) so the selection stays visible (RL-007).
2. Preserve selection position across delete reloads (RL-010) and select the edited run after a Quick Entry edit (RL-011, paired with QE-017).

### Should Fix
3. Add tests for the remaining RL specs (selection logic can be tested on `RunListState` without a terminal).

### Nice to Have
4. Make the footer and Actions panel list the same complete set of keys.
