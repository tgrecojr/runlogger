# Quick Entry Specs

Design: `docs/intent/quick-entry/quick-entry-design.md` · Prefix: `QE`

Status: `[x]` = implemented; `[ ]` = specified but not yet implemented; `[D]` = deferred. Tests: `src/logic/validation.rs` (QE-010, QE-012, QE-024), `src/models/run.rs` (QE-013), `src/input/tests.rs` (QE-014, QE-016, QE-017, QE-018, QE-023, QE-024).

## Form

- [x] **QE-001**: When the app starts, the system shall prefill the Quick Entry Date field with today's local date as `YYYY-MM-DD` and the Time field with the current local time as `HH:MM:SS`, leave Distance and Note empty, and focus the Distance field.
- [x] **QE-002**: While the Quick Entry screen is shown, when the user presses Tab, the system shall move focus to the next field in the cycle Date → Time → Distance → Note → Date, and when the user presses Shift+Tab, the system shall move focus to the previous field in that cycle.
- [x] **QE-003**: While the Quick Entry screen is shown, when the user types a character, the system shall append it to the focused field, including digits and letters that act as commands on other screens.
- [x] **QE-004**: While the Quick Entry screen is shown, when the user presses Backspace, the system shall remove the last character of the focused field.
- [x] **QE-005**: The Quick Entry screen shall display the Date, Time, Distance and Note fields labeled "Date (YYYY-MM-DD)", "Time (HH:MM:SS)", "Distance (miles)" and "Note (optional)", and shall show the focused field with a yellow border, bold yellow text and a trailing `_` cursor.
- [x] **QE-006**: While Quick Entry is in edit mode, the system shall title the screen "Edit Run"; otherwise it shall title the screen "Quick Entry - Log Your Run".
- [x] **QE-007**: When the Quick Entry form is cleared (Esc Esc on Quick Entry), the system shall reset Date and Time to the current local date and time, empty Distance and Note, focus Distance, remove any message, and leave edit mode.

## Parsing and Validation

- [x] **QE-008**: When a Quick Entry submission is parsed, the system shall accept the Date in `YYYY-MM-DD`, `MM/DD/YYYY` or `MM-DD-YYYY` form, and shall use today's local date if the Date field is empty.
- [x] **QE-009**: When a Quick Entry submission is parsed, the system shall accept the Time in `HH:MM:SS`, `HH:MM`, `h:MM AM/PM` or `h:MM:SS AM/PM` form.
- [x] **QE-010**: If the Time field is empty when a Quick Entry submission is parsed, then the system shall use the current local time truncated to whole seconds.
- [x] **QE-011**: If the Date field of a Quick Entry submission matches no accepted form, then the system shall reject it with "Invalid date format. Use YYYY-MM-DD"; if the Time field matches no accepted form, then the system shall reject it with "Invalid time format. Use HH:MM or HH:MM:SS".
- [x] **QE-012**: If the Distance field of a Quick Entry submission is empty, then the system shall reject it with "Distance is required"; if it is not a finite number after trimming surrounding whitespace (including `NaN` and `inf`), then the system shall reject it with "Invalid distance. Enter a number (e.g., 3.5)".
- [x] **QE-013**: If a run's distance is not a finite number, then the system shall reject it with "Invalid distance. Enter a number (e.g., 3.5)"; if it is zero or negative, then the system shall reject it with "Distance must be positive"; if it exceeds 200 miles, then the system shall reject it with "Distance seems unrealistic (>200 miles)".
- [x] **QE-024**: If the date and time of a Quick Entry submission are after the current local date and time, then the system shall reject it with "Date cannot be in the future" when the date is after today, or with "Time cannot be in the future" when the date is today and the time is later than now.
- [x] **QE-014**: When a Quick Entry submission has more than one invalid field, the system shall report only the first failure in the order Date format, Time format, not in the future (QE-024), Distance, distance bounds.

## Submit

- [x] **QE-015**: When the user presses Enter on the Quick Entry screen, the system shall remove any previous success or error message before validating the submission.
- [x] **QE-016**: When the user presses Enter on the Quick Entry screen while not in edit mode and the submission is valid, the system shall save it as a new run, show "Run logged successfully!", clear Distance and Note, keep Date, set Time to the current local time in whole seconds, and focus Distance.
- [x] **QE-017**: When the user presses Enter on the Quick Entry screen while in edit mode and the submission is valid, the system shall save it as an update to the run being edited, reset the Quick Entry form as when cleared (QE-007), reload the run history, and switch to the Run List screen with the edited run selected so its new values are visible.
- [x] **QE-018**: If reloading the run history after a Quick Entry edit is saved fails, then the system shall keep the Quick Entry form reset and still switch to the Run List screen, which shows the load error (RL-012).
- [x] **QE-019**: If a Quick Entry submission is invalid, then the system shall show the validation message and keep every field value and the edit mode unchanged.
- [x] **QE-020**: If saving a valid Quick Entry submission fails in storage, then the system shall show "Database error: " followed by the reason and keep every field value and the edit mode unchanged.
- [x] **QE-021**: The Quick Entry screen shall show an error message in bold red, or otherwise a success message in bold green, below the fields.

## Edit Mode

- [x] **QE-022**: When a run is handed to Quick Entry for editing (`e` on Run List), the system shall fill Date as `YYYY-MM-DD`, Time as `HH:MM:SS`, Distance as a plain number and Note (empty if none) from that run, focus Distance, remove any message, and enter edit mode for that run.
- [x] **QE-023**: While Quick Entry is in edit mode, when the user switches to the Run List or Analytics screen (directly or by way of Help), the system shall cancel the edit by resetting the Quick Entry form as when cleared (QE-007), so the abandoned edit cannot overwrite the run later; switching to Help alone shall keep the edit.
