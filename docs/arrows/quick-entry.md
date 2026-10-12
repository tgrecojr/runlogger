# Arrow: quick-entry

The run-logging form: prefilled fields, text capture, parsing and run rules, insert and edit-save, inline messages.

## Status

**AUDITED** — audited 2026-10-12 (git SHA `88de5b9`); since then QE-012, QE-013, QE-020 and QE-025 were changed and tested. All 25 specs implemented and tested.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/quick-entry/quick-entry-design.md

### EARS
- docs/intent/quick-entry/quick-entry-specs.md (25 specs)

### Tests
- src/input/quick_entry/tests/form.rs (QE-001, QE-002, QE-003, QE-004, QE-007, QE-025)
- src/input/quick_entry/tests/submit.rs (QE-008, QE-011, QE-015, QE-019, QE-020, QE-022)
- src/input/tests/edits.rs (QE-014, QE-016, QE-017, QE-023, QE-024)
- src/input/tests/errors.rs (QE-018)
- src/logic/validation.rs (QE-008, QE-009, QE-010, QE-011, QE-012, QE-024)
- src/models/run.rs (QE-013)
- src/ui/screens/quick_entry/tests.rs (QE-005, QE-006, QE-021)

### Code
- src/app.rs (QE-001, QE-007, QE-022, QE-023, QE-025)
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
4. `Run::new` — distance must be finite and 0 < d ≤ 200
5. `screens/quick_entry.rs` — form rendering, focus styling, messages

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps | Tested |
|----------|----------|-------------|----------|------|--------|
| Form | QE-001, QE-002, QE-003, QE-004, QE-005, QE-006, QE-007, QE-025 | 8 | 0 | 0 | 8 |
| Parsing and Validation | QE-008, QE-009, QE-010, QE-011, QE-012, QE-013, QE-024, QE-014 | 8 | 0 | 0 | 8 |
| Submit | QE-015, QE-016, QE-017, QE-018, QE-019, QE-020, QE-021 | 7 | 0 | 0 | 7 |
| Edit Mode | QE-022, QE-023 | 2 | 0 | 0 | 2 |

**Summary:** 25 of 25 specs marked implemented; 25 of 25 cited by a test. Untested: none.

## Key Findings

1. **Date not normalized after a save** — Date is kept exactly as typed (e.g. `10/12/2026`); Date and Time are not trimmed, so surrounding spaces make them invalid. Not covered by a spec.

## Work Required

### Nice to Have
1. Make labels and error messages list every accepted format; trim Date/Time; treat whitespace-only Distance as empty.
