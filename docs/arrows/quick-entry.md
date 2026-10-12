# Arrow: quick-entry

The run-logging form: prefilled fields, text capture, parsing and run rules, insert and edit-save, inline messages.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. Four gaps: future dates accepted (QE-024), Time not refreshed after an insert (QE-016), post-edit form reset and Run List selection (QE-017), and sticky edit mode (QE-023).

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/quick-entry/quick-entry-design.md

### EARS
- docs/intent/quick-entry/quick-entry-specs.md (24 specs)

### Tests
- src/logic/validation.rs (inline; QE-010)
- src/input/tests.rs (QE-018)

### Code
- src/app.rs (`QuickEntryState`, `InputField`)
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
| Parsing and validation | QE-008 to QE-014, QE-024 | 7 | 0 | 1 |
| Submit | QE-015 to QE-021 | 6 | 0 | 1 |
| Edit mode | QE-022 to QE-023 | 1 | 0 | 1 |

**Summary:** 20 of 24 specs implemented; 4 gaps (QE-016, QE-017, QE-023, QE-024). Tests cite QE-010 and QE-018 only.

## Key Findings

1. **Time not refreshed after insert** — `submit` in `src/input/quick_entry.rs` keeps Time, so a second run collides with STORE-005 uniqueness (QE-016).
2. **Sticky edit mode** — `editing_run_id` survives navigating away (`handle_key_event` never clears it); the next Enter overwrites the old run (QE-023).
3. **Post-edit landing** — the form resets and Run List opens, but the edited run is not selected (QE-017, RL-011).
4. **Future dates accepted** — `parse_date` has no upper bound (QE-024).
5. **NaN distance passes app validation** — `Run::new` comparisons are false for NaN; storage outcome unverified.

## Work Required

### Must Fix
1. Refresh Time after an insert (QE-016).
2. Cancel edit mode when leaving Quick Entry (QE-023).
3. Select the edited run on the post-edit return (QE-017; RL owns selection in RL-011).
4. Reject future dates (QE-024).

### Should Fix
5. Decide whether Date refreshes across midnight.
6. Reject non-finite distances explicitly.
7. Add tests for the remaining QE specs (validation and `Run::new` are pure and easy to cover first).

### Nice to Have
8. Make labels and error messages list every accepted format; treat whitespace-only Distance as empty.
