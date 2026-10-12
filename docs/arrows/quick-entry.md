# Arrow: quick-entry

The run-logging form: prefilled fields, text capture, parsing and run rules, insert and edit-save, inline messages.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. All 24 specs implemented.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/quick-entry/quick-entry-design.md

### EARS
- docs/intent/quick-entry/quick-entry-specs.md (24 specs)

### Tests
- src/logic/validation.rs (inline; QE-010, QE-024)
- src/input/tests.rs (QE-014, QE-016, QE-017, QE-018, QE-023, QE-024)

### Code
- src/app.rs (`QuickEntryState`, `InputField`; edit cancel in `App::switch_to_screen`)
- src/input/quick_entry.rs (`handle_quick_entry_input`, `submit`, `build_run`)
- src/logic/validation.rs
- src/models/run.rs (`Run::new`)
- src/ui/screens/quick_entry.rs

## Architecture

**Purpose:** Turn typed fields into a valid saved run, for both new runs and edits of existing ones.

**Key Components:**
1. `QuickEntryState` — raw field text, focus, messages, edit-mode id; `new`/`clear`/`load_run`
2. `handle_quick_entry_input` / `submit` / `build_run` — Tab/Backspace/character capture and the Enter submit flow
3. `validation.rs` — date/time/distance parsing with multiple accepted formats
4. `Run::new` — distance bounds (0 < d ≤ 200)
5. `screens/quick_entry.rs` — form rendering, focus styling, messages

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps |
|----------|----------|-------------|----------|------|
| Form | QE-001 to QE-007 | 7 | 0 | 0 |
| Parsing and validation | QE-008 to QE-014, QE-024 | 8 | 0 | 0 |
| Submit | QE-015 to QE-021 | 7 | 0 | 0 |
| Edit mode | QE-022 to QE-023 | 2 | 0 | 0 |

**Summary:** 24 of 24 specs implemented. Tests cite QE-010, QE-014, QE-016, QE-017, QE-018, QE-023, QE-024.

## Key Findings

1. **NaN distance passes app validation** — `Run::new` comparisons are false for NaN; storage outcome unverified.
2. **Date prefill across midnight** — Date is captured at startup and on clear, so a run logged after midnight without editing Date lands on the previous day.

## Work Required

### Should Fix
1. Decide whether Date refreshes across midnight.
2. Reject non-finite distances explicitly.
3. Add tests for the remaining QE specs (QE-001 to QE-009, QE-011 to QE-013, QE-015, QE-019 to QE-022).

### Nice to Have
4. Make labels and error messages list every accepted format; treat whitespace-only Distance as empty.
