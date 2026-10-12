---
parent: high-level-design
prefix: QE
---

# Quick Entry

## Context and Design Philosophy

Quick Entry is where runs are logged, and it is the screen the app opens on. Its job is to make logging today's run as fast as possible: date and time are prefilled, focus starts on Distance, so the common path is *type a number, press Enter*.

The same form doubles as the editor for existing runs. When Run List (`RL`) hands over a run with `e`, Quick Entry loads it, enters edit mode, and on save updates that run instead of inserting a new one. One form means one set of parsing and validation rules for both new and edited runs.

Quick Entry owns: the form's state and field focus, text capture while the screen is shown, parsing typed text into a date, time and distance, the rules a run must satisfy (`Run::new`), the submit flow for insert and update, and the inline success/error messages. It does not own which keys leave the screen or clear it (Navigation, `NAV`), how runs are stored (Storage, `STORE`), or how a run is chosen for editing (Run List, `RL`).

Code: `QuickEntryState` and `InputField` in `src/app.rs`; `handle_quick_entry_input`, `submit` and `build_run` in `src/input/quick_entry.rs`; `src/logic/validation.rs`; `Run::new` in `src/models/run.rs`; `src/ui/screens/quick_entry.rs`.

## Form State

`QuickEntryState` holds four raw text fields — `date`, `time`, `distance`, `note` — plus `focused_field`, an optional `error_message`, an optional `success_message`, and `editing_run_id: Option<i64>`. Edit mode is exactly `editing_run_id.is_some()`.

| Entry point | Effect |
|---|---|
| `QuickEntryState::new()` | Called once from `App::new()` at startup. Date = today `YYYY-MM-DD`, Time = now `HH:MM:SS`, Distance/Note empty, focus Distance, no messages, not editing. |
| `clear()` | Same reset as `new()` with a fresh clock reading; also leaves edit mode. Triggered by Esc Esc on this screen (`NAV`). |
| `load_run(&Run)` | Fills fields from a stored run (time as `HH:MM:SS`, distance via `f64::to_string`, note or empty), focus Distance, clears messages, sets `editing_run_id`. Triggered by `e` on Run List (`RL`). |

The prefill is a snapshot taken at startup or on clear — it does not track the clock while the screen sits open.

## Text Capture and Focus

While Quick Entry is the active screen, Navigation routes every key except Ctrl+Q/Ctrl+C and Esc to `handle_quick_entry_input`, so digits and letters that act as commands elsewhere (`1`–`3`, `q`, `h`) are typed into the form.

- **Tab** cycles focus Date → Time → Distance → Note → Date; **Shift+Tab** or **BackTab** goes the other way.
- **Any character key** is appended to the focused field. Modifiers are not inspected, so Ctrl/Alt + letter also inserts the letter.
- **Backspace** removes the focused field's last character.

## Parsing and Validation

On Enter, both messages are cleared, then fields are parsed in order Date → Time → Distance → `Run::new`; the first failure becomes `error_message` and processing stops (`build_run`).

| Field | Accepted | Empty field | Error text |
|---|---|---|---|
| Date (`parse_date`) | `%Y-%m-%d`, `%m/%d/%Y`, `%m-%d-%Y` | today | "Invalid date format. Use YYYY-MM-DD" |
| Time (`parse_time`) | `%H:%M:%S`, `%H:%M`, `%I:%M %p`, `%I:%M:%S %p` | current clock time, truncated to whole seconds | "Invalid time format. Use HH:MM or HH:MM:SS" |
| Distance (`parse_distance`) | anything `f64::from_str` accepts after trimming | error | "Distance is required" / "Invalid distance. Enter a number (e.g., 3.5)" |

A parsed Date after today's local date is rejected with "Date cannot be in the future" (`QE-024`); runs record what has happened, and a future-dated run would distort the streak and period statistics.

**Divergence (future dates).** The code accepts any parseable date (`parse_date`).

`Run::new` then enforces the run rules: distance must be `> 0` ("Distance must be positive") and `≤ 200` ("Distance seems unrealistic (>200 miles)"). It stamps `created_at = Utc::now()`. An empty Note becomes `None`.

## Submit Flow

```
Enter ─▶ clear messages ─▶ parse Date ─▶ parse Time ─▶ parse Distance ─▶ Run::new
              │ any failure: error_message = reason, fields and edit mode kept
              ▼
     editing_run_id?
       ├─ None ─▶ insert_run ─▶ "Run logged successfully!"
       │                         clear Distance + Note, focus Distance,
       │                         keep Date, Time := now (whole seconds)
       └─ Some(id) ─▶ update_run(id) ─▶ reset form (as Clear),
                                        reload runs, switch to Run List
                                        with run `id` selected
       storage error: error_message = "Database error: <reason>"
```

If the post-edit reload fails, the form stays reset, the switch to Run List still happens, and Run List shows the load error (`RL-012`); the edit itself has already been saved.

**Divergence (after a new run).** The code keeps the Time field unchanged after an insert (`submit`), so a second run logged without retyping Time repeats the first run's date and start time and is rejected by Storage's uniqueness rule (`STORE-005`). The intended behavior refreshes Time to the current clock (`QE-016`).

**Divergence (after an edit).** The code resets the form and switches to Run List, but the reload resets Run List's selection to the top row, so the edited run is not shown selected (`QE-017`, with Run List owning the selection in `RL-011`).

**Divergence (leaving mid-edit).** Nothing clears `editing_run_id` when the user switches screens (`handle_key_event` in `src/input/mod.rs`), so the next Enter on Quick Entry silently overwrites the run that was being edited. The intended behavior cancels the edit on leaving (`QE-023`).

## Display

`src/ui/screens/quick_entry.rs` renders, inside a 2-cell margin:

```
Quick Entry - Log Your Run            ← "Edit Run" while editing
┌Date (YYYY-MM-DD)──────────────┐
│2026-10-12                     │
└───────────────────────────────┘
┌Time (HH:MM:SS)────────────────┐
│07:15:00                       │
└───────────────────────────────┘
┌Distance (miles)───────────────┐   ← focused: yellow border,
│3.5_                           │     bold yellow text, trailing "_"
└───────────────────────────────┘
┌Note (optional)────────────────┐
│Morning run                    │
└───────────────────────────────┘
Run logged successfully!              ← red error takes precedence over green success
```

## Decisions & Alternatives

| Decision | Chosen | Alternatives Considered | Rationale |
|---|---|---|---|
| Default screen and focus | Open on Quick Entry with Distance focused and date/time prefilled | Open on Run List or Analytics; focus Date | Logging today's run should take one number and Enter. |
| Editing UI | Reuse the Quick Entry form in an edit mode | Separate edit dialog; inline editing in Run List | One form keeps one set of parsing and validation rules for new and edited runs. |
| Accepted date/time formats | Several ISO and US formats, including 12-hour AM/PM | Single strict format | Tolerates the formats a user naturally types when backfilling. |
| Future dates | Reject a Date after today | Allow and exclude from analytics | A run records something that already happened; refusing it at entry keeps every downstream measure honest. |
| Distance bounds | `0 < distance ≤ 200` miles | No upper bound; per-run pace check | Catches typos (an extra digit) while allowing any real run, ultramarathons included. |
| Where run rules live | `Run::new` constructor | Only in the parse step | Any code that builds a `Run` gets the same invariants. |
| After a new run is saved | Keep Date, refresh Time to the current clock in whole seconds, clear Distance and Note | Keep Date and Time; clear everything | Keeping Date lets several runs be backfilled for one day without retyping it; a fresh Time gives each run a distinct start time so Storage's uniqueness rule does not reject the next run. |
| After an edit is saved | Reset the form and return to Run List with the edited run selected | Stay on Quick Entry; return to the top of Run List | The user confirms the change by seeing the run's new values in context, not by reading a message. |
| Leaving Quick Entry mid-edit | Cancel the edit and reset the form | Keep the edit pending; prompt before leaving | An edit the user walked away from must never overwrite a run later without their noticing. |
| Error reporting | First failing field only, inline in red | Collect all errors; modal popup | Fields are few and the message sits directly under them. |

## Open Questions & Future Decisions

### Deferred
1. **Date prefill across midnight.** The Date field is captured at startup and on clear, so with the app left open past midnight a new run is logged to the previous day unless the user edits Date. Should Date also refresh after an insert or when the screen is shown?
2. **Non-finite distances.** `"NaN"` passes both `parse_distance` and `Run::new` because NaN comparisons are false; whether SQLite's `NOT NULL` then rejects it is unverified. `"inf"` is rejected only incidentally by the 200-mile check.
3. **Whitespace-only Distance** passes the empty check (which runs before trimming) and reports "Invalid distance…" instead of "Distance is required".
4. **Modifier keys** other than Ctrl+Q/Ctrl+C insert their base character into the field.
5. **Labels and error text undersell accepted formats** — "Time (HH:MM:SS)" and the error messages omit HH:MM, AM/PM and the US date forms.

## References

- Code: `src/app.rs`, `src/input/quick_entry.rs`, `src/logic/validation.rs`, `src/models/run.rs`, `src/ui/screens/quick_entry.rs`, `src/ui/theme.rs` (input/message styles)
- Specs: `docs/intent/quick-entry/quick-entry-specs.md`
- Tests: `src/logic/validation.rs` (inline), `src/input/tests.rs`
- Depends on: Storage (`STORE`) for insert/update and the stored time format; Navigation (`NAV`) for key routing and the clear trigger; Run List (`RL`) for the edit hand-off and post-edit return
- External: `chrono` 0.4, `anyhow` 1, `ratatui` 0.30, `crossterm` 0.29
