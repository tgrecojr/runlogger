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

`RunListState` holds `runs: Vec<Run>`, `selected_index`, `scroll_offset` and `error_message: Option<String>`.

`load_runs` replaces `runs` with `get_all_runs` (newest first by date, then start time — `STORE-012`) and resets `selected_index` and `scroll_offset` to 0. It runs every time the screen is opened (Navigation calls it on `2` and Esc→`2`), after a delete, and after a Quick Entry edit is saved. There is no cache: the list always reflects the database at open time.

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
| ↑ | Move selection up one row, stopping at the first; lower `scroll_offset` if the selection passes above it. |
| ↓ | Move selection down one row, stopping at the last; raise `scroll_offset` once the selection is 10 or more rows past it. |
| `e` | Clone the selected run, hand it to Quick Entry via `load_run` (`QE-022`), switch to Quick Entry. |
| `d` | Delete the selected run by id immediately, no confirmation (`STORE-011`); reload; then clamp `selected_index` to the last row. If the delete fails, nothing is reloaded and the error is shown (`RL-013`). |

`e` and `d` do nothing when the list is empty.

**Divergence (scrolling).** `scroll_offset` is maintained with a hard-coded 10-row page, but `run_list.rs` never reads it: the `Table` is rendered without a `TableState`, so it always draws from the first row. Once the selection moves below the visible area it is highlighted off-screen and the user cannot see it (`RL-007`).

**Divergence (selection after delete).** The clamp after the delete reload intends to keep the selection near where the deleted run was, but `load_runs` has already reset it to 0, so the selection always jumps to the newest run (`RL-010`).

**Divergence (selection after an edit).** After Quick Entry saves an edit it reloads Run List, which selects the first row rather than the edited run (`RL-011`, the Run List half of `QE-017`).

## Display

`src/ui/screens/run_list.rs` splits the area into an optional 1-row error line (bold red, shown only while `error_message` is set), a table (min 10 rows) and a 3-row "Actions" hint panel:

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

Columns are fixed at 12/10/15 characters with Note taking the rest (min 20). Distance is shown to two decimals. With no runs, the table is replaced by "No runs logged yet. Press Esc → 1 to add a run." and the hint panel still shows.

## Decisions & Alternatives

| Decision | Chosen | Alternatives Considered | Rationale |
|---|---|---|---|
| How edits happen | Hand the run to the Quick Entry form | Inline cell editing; a separate edit dialog | One form keeps one set of parsing and validation rules. |
| Delete confirmation | Delete immediately on `d` | Confirmation prompt; undo | A single-user personal log; deleting is rare and a mistaken delete is quickly re-entered in Quick Entry. |
| Freshness | Reload from Storage every time the list opens or changes | Keep an in-memory list in sync with writes | Always matches the database with no synchronization logic; the history is small. |
| Load and delete failures | A red error line above the table; the app keeps running | Exit the app; silently show an empty list; a shared status line in the footer | A storage problem must not cost the user the rest of the app or be hidden; placing it on the screen it affects keeps the footer's key hints visible. |
| Scope of the list | Full history, newest first, no paging or filtering | Date-range filter; paging | The history stays small enough to scroll; the newest runs are the ones usually edited. |

## Open Questions & Future Decisions

### Deferred
1. **Page size.** The scroll logic assumes 10 visible rows; the real height depends on the terminal. Scrolling should follow the rendered height (`RL-007`).
2. **Hints are split.** The footer (`NAV`) shows `[↑↓] Navigate [1-3] Switch screen [h] Help [q] Quit` while the Actions panel shows `e`/`d`; neither lists everything available on the screen.

## References

- Code: `src/app.rs`, `src/input/run_list.rs`, `src/input/mod.rs` (`load_runs`), `src/ui/screens/run_list.rs`
- Tests: `src/input/tests.rs`, `src/ui/tests.rs`
- Specs: `docs/intent/run-list/run-list-specs.md`
- Depends on: Storage (`STORE`) for the history read and delete; Quick Entry (`QE`) for editing; Navigation (`NAV`) for opening the screen and error handling
- External: `ratatui` 0.30 (`Table`), `crossterm` 0.29
