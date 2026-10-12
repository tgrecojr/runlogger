# Run List Specs

Design: `docs/intent/run-list/run-list-design.md` · Prefix: `RL`

Status: `[x]` = implemented; `[ ]` = specified but not yet implemented; `[D]` = deferred. The tests citing each spec are listed in `docs/arrows/run-list.md`, regenerated from `@spec` scans.

## Loading

- [x] **RL-001**: When the user opens the Run List screen with `2` or Esc → `2`, the system shall load the full run history from storage, newest first by date and then start time, and select the first (newest) run.

## Display

- [x] **RL-002**: While the run history is not empty, the Run List screen shall show a table titled "Run List (N runs)", where N is the number of runs ("Run List (1 run)" when there is exactly one), with columns Date (`YYYY-MM-DD`), Time (`HH:MM:SS`), Distance (mi) to two decimal places, and Note (blank when the run has none).
- [x] **RL-003**: The Run List screen shall show the selected run's row in bold yellow.
- [x] **RL-004**: While the run history has loaded successfully and is empty, the Run List screen shall show "No runs logged yet. Press Esc → 1 to add a run." in place of the table.
- [x] **RL-005**: The Run List screen shall show an Actions panel below the table listing `[↑↓] Navigate`, `[e] Edit`, `[d] Delete` and `[Esc→1/2/3] Switch screens`, whether or not the history is empty.

## Selection

- [x] **RL-006**: While the Run List screen is shown, when the user presses ↑ the system shall move the selection up one run, and when the user presses ↓ the system shall move it down one run, stopping at the first and last runs.
- [x] **RL-007**: While the Run List screen is shown, the system shall scroll the table so that the selected run is always visible within the rendered height.

## Edit and Delete

- [x] **RL-008**: While the Run List screen shows at least one run, when the user presses `e`, the system shall hand the selected run to Quick Entry for editing and switch to the Quick Entry screen.
- [x] **RL-009**: While the Run List screen shows at least one run, when the user presses `d`, the system shall delete the selected run from storage immediately, without confirmation, and reload the run history.
- [x] **RL-010**: When the Run List reloads after a delete, the system shall keep the selection at the same row position (now holding the run that followed the deleted one), or on the last run if the deleted run was the last row.
- [x] **RL-011**: When the Run List screen is shown after Quick Entry saves an edit, the system shall select the edited run.

## Errors

- [x] **RL-012**: If loading the run history for the Run List screen fails — when the screen is opened, after a Quick Entry edit is saved, or after a delete — then the system shall show the Run List screen with no runs and, above the table in bold red, "Could not load runs: " followed by the reason including its underlying cause, wrapped across as many lines as it needs, until the next successful load, and shall keep the app running.
- [x] **RL-013**: If deleting a run from the Run List screen fails, then the system shall leave the list and selection unchanged and show, above the table in bold red, "Could not delete run: " followed by the reason including its underlying cause, wrapped across as many lines as it needs, until the next successful load, and shall keep the app running.
