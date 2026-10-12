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

Date **follows the clock until edited** (`QE-025`): `QuickEntryState` tracks `date_edited`, set when the user types or deletes in the Date field or when a run is loaded for editing, and cleared by `new()` and `clear()`. While it is false, `follow_clock(today)` — called by the event loop before every frame — rewrites Date to today, so a form left open past midnight logs to the new day. Time is a snapshot taken at startup, on clear, and after each new run is saved (`QE-016`); it does not tick, because a run's start time is often typed after the run.

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
| Distance (`parse_distance`) | a finite number after trimming (`NaN` and `inf` are rejected) | error | "Distance is required" / "Invalid distance. Enter a number (e.g., 3.5)" |

Once Date and Time have parsed, and before Distance is checked, the submission's date and time together must not be after the current local date and time (`QE-024`): a date after today is rejected with "Date cannot be in the future", and today's date with a later time with "Time cannot be in the future". Runs record what has happened; a future run would distort the streak and period statistics. The check is a pure function of the submission and the current date-time (`check_not_future` in `validation.rs`), so it can be tested at any fixed "now".

`Run::new` then enforces the run rules: distance must be finite ("Invalid distance. Enter a number (e.g., 3.5)"), `> 0` ("Distance must be positive") and `≤ 200` ("Distance seems unrealistic (>200 miles)"). It stamps `created_at = Utc::now()`. An empty Note becomes `None`.

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
       storage error: error_message = what happened + what to do (below)
```

A failed save keeps every field and the edit mode, and the message says what happened and what to do next (`QE-020`):

| Storage failure | Message |
|---|---|
| Duplicate date and start time (`StorageError::DuplicateStart`, `STORE-005`) | `Another run already starts at <date> <time> — change the Time and press Enter to save.` |
| Anything else | `Could not save run: <reason including its underlying cause>. Your entry is still here — press Enter to try again.` |

If the post-edit reload fails, the form stays reset, the switch to Run List still happens, and Run List shows the load error (`RL-012`); the edit itself has already been saved.

After the post-edit reload, Quick Entry asks Run List to select the run it just saved, by id (`RunListState::select_run`); Run List owns the selection and keeps it in view (`RL-011`, `RL-007`).

While in edit mode, switching to Run List or Analytics cancels the edit: the form resets as when cleared (`QE-023`). Help is a quick look — switching to Help keeps the edit, so the user can check a key and return with Esc→`1` — but a later switch from Help to Run List or Analytics still cancels it. The returning user sees the plain "Quick Entry - Log Your Run" title instead of "Edit Run", which is the feedback that the edit was dropped. The cancel happens in Navigation's single screen-switch function, so every route is covered. Leaving while *not* editing keeps whatever was typed, so a half-entered new run survives a look at Analytics.

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
| Future runs | Reject a date and time after now | Check the date only; allow and exclude from analytics | A run records something that already happened; refusing it at entry keeps every downstream measure honest, and checking the time too stops a run being logged before it is finished. |
| Date while the form sits open | Follows the clock until the user edits it | Snapshot at startup; correct the date at submit | A form left open past midnight must log to the new day, and the screen must show the date that will be saved. |
| Time after a new run is saved | Always refresh to the current clock, even after a hand-typed time | Refresh only when Time still holds the prefill | Each entry starts at "now"; a kept hand-typed time would duplicate the run just saved and be rejected by Storage. |
| Leaving mid-edit | Cancel on any switch to Run List or Analytics, in Navigation's single screen-switch function; Help keeps the edit | Cancel on any departure from Quick Entry; cancel in each navigation key branch | Checking Help must not cost the user their edit, while visiting another screen is a clear sign the edit was abandoned; one function covers every route. |
| Feedback on a cancelled edit | The title returns to "Quick Entry - Log Your Run" | An "Edit cancelled" message | The title change already shows the form is no longer editing; a message would be noise. |
| Distance bounds | `0 < distance ≤ 200` miles | No upper bound; per-run pace check | Catches typos (an extra digit) while allowing any real run, ultramarathons included. |
| Where run rules live | `Run::new` constructor | Only in the parse step | Any code that builds a `Run` gets the same invariants. |
| After a new run is saved | Keep Date, refresh Time to the current clock in whole seconds, clear Distance and Note | Keep Date and Time; clear everything | Keeping Date lets several runs be backfilled for one day without retyping it; a fresh Time gives each run a distinct start time so Storage's uniqueness rule does not reject the next run. |
| After an edit is saved | Reset the form and return to Run List with the edited run selected | Stay on Quick Entry; return to the top of Run List | The user confirms the change by seeing the run's new values in context, not by reading a message. |
| Leaving Quick Entry mid-edit | Cancel the edit and reset the form | Keep the edit pending; prompt before leaving | An edit the user walked away from must never overwrite a run later without their noticing. |
| Save-failure messages | Name the problem and the next step; a duplicate start time gets its own message | Generic "Database error" text | The user should know what went wrong and how to fix it without reading SQLite errors. |
| Error reporting | First failing field only, inline in red | Collect all errors; modal popup | Fields are few and the message sits directly under them. |

## Open Questions & Future Decisions

### Deferred
1. **Whitespace-only Distance** passes the empty check (which runs before trimming) and reports "Invalid distance…" instead of "Distance is required".
2. **Modifier keys** other than Ctrl+Q/Ctrl+C insert their base character into the field.
3. **Labels and error text undersell accepted formats** — "Time (HH:MM:SS)" and the error messages omit HH:MM, AM/PM and the US date forms.

## References

- Code: `src/app.rs`, `src/input/quick_entry.rs`, `src/logic/validation.rs`, `src/models/run.rs`, `src/ui/screens/quick_entry.rs`, `src/ui/theme.rs` (input/message styles)
- Specs: `docs/intent/quick-entry/quick-entry-specs.md`
- Tests: listed in `docs/arrows/quick-entry.md`
- Depends on: Storage (`STORE`) for insert/update and the stored time format; Navigation (`NAV`) for key routing and the clear trigger; Run List (`RL`) for the edit hand-off and post-edit return
- External: `chrono` 0.4, `anyhow` 1, `ratatui` 0.30, `crossterm` 0.29
