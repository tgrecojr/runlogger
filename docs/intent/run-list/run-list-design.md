---
parent: high-level-design
prefix: RL
---

# Run List

## Context and Design Philosophy

Run List is the history view: every logged run, newest first, with a movable selection for editing or deleting one. It is deliberately thin. It reads the full history from Storage (`STORE`), shows it as a table, and delegates editing to Quick Entry (`QE`) rather than editing inline, so there is only one place where run fields are parsed and validated.

Run List owns: the list contents and selection (`RunListState`), what happens on load, the arrow-key, `e` and `d` actions, and the table rendering. It does not own the keys that open the screen or the reload-on-open trigger (Navigation, `NAV`), the edit form (`QE`), or the delete itself (`STORE`).

Code: `RunListState` in `src/app.rs`, `handle_run_list_input` in `src/input/run_list.rs`, `load_runs` in `src/input/mod.rs`, `src/ui/screens/run_list.rs`.

## State and Loading

`RunListState` holds `runs: Vec<Run>`, `selected_index`, `table_state` (ratatui's `TableState`, which carries the scroll position between frames) and `error_message: Option<String>`. `select_run(id)` selects the run with that id if it is in the list; Quick Entry uses it after an edit (`RL-011`).

Opening Run List from another screen selects the newest run (`RL-001`). Pressing `2` (or Esc→`2`) while already on Run List reloads but keeps the same run selected, found by id, falling back to the newest if it is gone (`RL-014`), so a refresh never loses the user's place.

`load_runs` replaces `runs` with `get_all_runs` (newest first by date, then start time — `STORE-012`) and resets `selected_index` to 0 and `table_state` to its default. It runs every time the screen is opened (Navigation calls it on `2` and Esc→`2`), after a delete, and after a Quick Entry edit is saved — in which case Quick Entry then calls `select_run` for the edited run. There is no cache: the list always reflects the database at open time.

Storage failures stay on this screen and never end the app:

| Failure | List afterwards | Message |
|---|---|---|
| History read fails on opening Run List, on the return after a Quick Entry edit, or on the reload after a successful delete | Empty, selection reset | `Could not load runs: <reason>` (`RL-012`) |
| Delete fails | Unchanged, selection unchanged | `Could not delete run: <reason>` (`RL-013`) |

The reason includes its full cause chain on one line, so the underlying SQLite or parse error is visible. The message is held in `error_message` and cleared by the next successful load. When the open itself fails, Navigation still switches to Run List so the user sees why the list is empty.

## Actions

Handled by `handle_run_list_input` after Navigation's global keys:

| Key | Effect |
|---|---|
| ↑ | Move selection up one row, stopping at the first. |
| ↓ | Move selection down one row, stopping at the last. |
| `e` | Clone the selected run, hand it to Quick Entry via `load_run` (`QE-022`), switch to Quick Entry. |
| `d` | Delete the selected run by id immediately, no confirmation (`STORE-011`); reload; then clamp `selected_index` to the last row. If the delete fails, nothing is reloaded and the error is shown (`RL-013`). |

`e` and `d` do nothing when the list is empty.

Scrolling is left to ratatui: each frame the renderer sets `table_state`'s selection from `selected_index` and draws the table as a stateful widget, which moves the stored offset just enough to keep the selected row inside the rendered height (`RL-007`). Because `table_state` persists in `RunListState`, the view scrolls one row at a time in both directions rather than jumping. Rendering therefore takes the app state mutably. `load_runs` resets `table_state` along with the selection.

After a successful delete, the handler remembers the deleted row's position, reloads, and restores the selection to that position — now holding the run that followed — or to the last row if the deleted run was last (`RL-010`). If the reload fails, the list is empty and RL-012's error shows instead.

## Display

`src/ui/screens/run_list.rs` splits the area into an optional error area (bold red, shown only while `error_message` is set, as tall as the message needs when wrapped to the screen width), a table (min 10 rows) and a 3-row "Actions" hint panel:

```
┌Run List (42 runs)───────────────────────────────────────┐
│Date        Time      Distance (mi)  Note                │  ← header: bold yellow
│                                                         │
│2026-10-12  07:15:00  3.50           Morning run         │  ← selected: bold yellow
│2026-10-11  18:02:10  1.20                               │
└─────────────────────────────────────────────────────────┘
┌Actions──────────────────────────────────────────────────┐
│[↑↓] Navigate  [e] Edit  [d] Delete  [Esc→1/2/3] Switch screens │
└─────────────────────────────────────────────────────────┘
```

The title counts runs — "Run List (1 run)", "Run List (42 runs)". Columns are fixed at 12/10/15 characters with Note taking the rest (min 20). Distance is shown to two decimals. With no runs, the table is replaced by "No runs logged yet. Press Esc → 1 to add a run." and the hint panel still shows.

## Decisions & Alternatives

| Decision | Chosen | Alternatives Considered | Rationale |
|---|---|---|---|
| Scrolling | ratatui `TableState` persisted in `RunListState`; rendering takes state mutably | A fresh `TableState` each frame; a hand-computed offset with a fixed page size | Keeps the selection in view at any terminal height, and a persisted offset scrolls smoothly in both directions. |
| How edits happen | Hand the run to the Quick Entry form | Inline cell editing; a separate edit dialog | One form keeps one set of parsing and validation rules. |
| Delete confirmation | Delete immediately on `d` | Confirmation prompt; undo | A single-user personal log; deleting is rare and a mistaken delete is quickly re-entered in Quick Entry. |
| Freshness | Reload from Storage every time the list opens or changes | Keep an in-memory list in sync with writes | Always matches the database with no synchronization logic; the history is small. |
| Load and delete failures | A red error line above the table; the app keeps running | Exit the app; silently show an empty list; a shared status line in the footer | A storage problem must not cost the user the rest of the app or be hidden; placing it on the screen it affects keeps the footer's key hints visible. |
| Scope of the list | Full history, newest first, no paging or filtering | Date-range filter; paging | The history stays small enough to scroll; the newest runs are the ones usually edited. |

## Open Questions & Future Decisions

### Deferred
None.

## References

- Code: `src/app.rs`, `src/input/run_list.rs`, `src/input/mod.rs` (`load_runs`), `src/ui/screens/run_list.rs`
- Tests: listed in `docs/arrows/run-list.md`
- Specs: `docs/intent/run-list/run-list-specs.md`
- Depends on: Storage (`STORE`) for the history read and delete; Quick Entry (`QE`) for editing; Navigation (`NAV`) for opening the screen and error handling
- External: `ratatui` 0.30 (`Table`), `crossterm` 0.29
