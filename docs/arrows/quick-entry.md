# Arrow: quick-entry

The run-logging form: prefilled fields, text capture, parsing and run rules, insert and edit-save, inline messages.

## Status

**PARTIAL** — audited 2026-10-12 (git SHA `88de5b9`). 21 of 24 specs verified against code; "NaN" is accepted as a distance (QE-012, QE-013) and database errors omit their cause (QE-020).

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/quick-entry/quick-entry-design.md

### EARS
- docs/intent/quick-entry/quick-entry-specs.md (24 specs)

### Tests
- src/input/tests.rs (QE-014, QE-016, QE-017, QE-018, QE-023, QE-024)
- src/logic/validation.rs (QE-010, QE-024)

### Code
- src/app.rs (QE-001, QE-007, QE-022, QE-023)
- src/input/quick_entry.rs (QE-002, QE-003, QE-004, QE-014, QE-015, QE-016, QE-017, QE-018, QE-019, QE-020, QE-024)
- src/logic/validation.rs (QE-008, QE-009, QE-010, QE-011, QE-012, QE-024)
- src/models/run.rs (QE-013)
- src/ui/screens/quick_entry.rs (QE-005, QE-006, QE-021)

## Architecture

**Purpose:** Turn typed fields into a valid saved run, for both new runs and edits of existing ones.

**Key Components:**
1. `QuickEntryState` — raw field text, focus, messages, edit-mode id; `new`/`clear`/`load_run`
2. `handle_quick_entry_input` / `submit` / `build_run` — Tab/Backspace/character capture and the Enter submit flow
3. `validation.rs` — date/time/distance parsing with multiple accepted formats
4. `Run::new` — distance bounds (0 < d ≤ 200)
5. `screens/quick_entry.rs` — form rendering, focus styling, messages

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps | Tested |
|----------|----------|-------------|----------|------|--------|
| Form | QE-001, QE-002, QE-003, QE-004, QE-005, QE-006, QE-007 | 7 | 0 | 0 | 0 |
| Parsing and Validation | QE-008, QE-009, QE-010, QE-011, QE-012, QE-013, QE-024, QE-014 | 8 | 0 | 0 | 3 |
| Submit | QE-015, QE-016, QE-017, QE-018, QE-019, QE-020, QE-021 | 7 | 0 | 0 | 3 |
| Edit Mode | QE-022, QE-023 | 2 | 0 | 0 | 1 |

**Summary:** 24 of 24 specs marked implemented; 7 of 24 cited by a test. Untested: QE-001, QE-002, QE-003, QE-004, QE-005, QE-006, QE-007, QE-008, QE-009, QE-011, QE-012, QE-013, QE-015, QE-019, QE-020, QE-021, QE-022.

## Key Findings

1. **"NaN" accepted as a distance** — `parse_distance` uses `str::parse::<f64>`, which accepts `NaN`; it passes both `Run::new` bounds (NaN comparisons are false) and is then rejected by the database's NOT NULL rule, so the user sees "Database error: Failed to insert run" instead of "Invalid distance…" (QE-012, QE-013).
2. **Database errors omit the cause** — `submit` formats storage errors with `{}`, showing only "Failed to insert run"/"Failed to update run", while Run List and Analytics use `{:#}` to include the underlying cause (QE-020; contrast RL-012, AN-014).
3. **Date not normalized after a save** — Date is kept exactly as typed (e.g. `10/12/2026`); Date and Time are not trimmed, so surrounding spaces make them invalid. Not covered by a spec.

## Work Required

### Must Fix
1. Reject non-finite distances with "Invalid distance. Enter a number (e.g., 3.5)" (QE-012, QE-013).

### Should Fix
2. Decide whether QE-020's message should include the underlying cause, as RL-012 and AN-014 do.
3. Decide whether Date refreshes across midnight.
4. Add tests for the 17 untested QE specs.

### Nice to Have
5. Make labels and error messages list every accepted format; trim Date/Time; treat whitespace-only Distance as empty.
