---
parent: high-level-design
prefix: NAV
---

# Navigation

## Context and Design Philosophy

Navigation is the application shell: it starts and stops the app, owns the terminal, runs the event loop, and decides which screen a key belongs to. It also draws the parts of the UI common to every screen — the header tabs, the footer key hints and the Help screen — and holds the shared color theme.

The central constraint is that Quick Entry must accept *any* typed character, including digits and letters, because notes are free text. So single-key commands (`1`–`3`, `h`, `q`) cannot work on Quick Entry. Navigation resolves this with an **Esc prefix**: Esc arms a one-key navigation command that works from every screen, while on other screens the single keys work directly.

Navigation owns: startup and shutdown order, terminal setup/restore and the panic hook, the event loop, global key routing (quit keys, the Esc prefix, direct screen keys), the `App`/`Screen` state, the header, the footer, the Help screen, and `theme.rs`. It triggers, but does not own, each screen's data load (`RL-001`, `AN-008`), Quick Entry's clear (`QE-007`) and edit cancel (`QE-023`).

Code: `src/main.rs` (`main`, terminal helpers, `run_app`), `handle_key_event` in `src/input/mod.rs`, `Screen` and `App` in `src/app.rs`, `src/ui/mod.rs`, `src/ui/screens/help.rs`, `src/ui/theme.rs`, `src/ui/components/*`.

## Lifecycle

```
main
 ├─ setup_panic_hook        restore terminal before the default panic output
 ├─ get_db_path, init_db    (STORE) — fails here print to a normal terminal
 ├─ setup_terminal          raw mode + alternate screen
 ├─ App::new                screen = QuickEntry
 ├─ run_app                 loop { draw; poll 100 ms; handle key; break on should_quit }
 └─ restore_terminal        leave raw mode + alternate screen, show cursor
```

The database is opened before the terminal enters raw mode, so startup failures are readable. The panic hook disables raw mode and leaves the alternate screen, then chains to the original hook. `run_app` redraws every iteration and polls for input with a 100 ms timeout; a terminal input/output error breaks the loop and exits after restoring the terminal.

## Key Routing

`handle_key_event` applies these rules in order. It and the screen handlers it dispatches to live in `src/input.rs`; `src/main.rs` keeps only startup, the terminal and the event loop.

| # | Condition | Effect |
|---|---|---|
| 1 | Ctrl+Q or Ctrl+C | Quit (any screen). |
| 2 | Esc prefix armed | Disarm, then: `1` → Quick Entry; `2` → load runs, Run List; `3` → load analytics, Analytics; `h` or `?` → Help; Esc → clear Quick Entry if it is shown; anything else → ignored. |
| 3 | Esc | Arm the Esc prefix (no visible indicator). |
| 4 | `q`, not on Quick Entry | Quit. |
| 5 | On Quick Entry | Hand the key to Quick Entry (`QE-003`). |
| 6 | `1` / `2` / `3` / `h` or `?` | Switch to Quick Entry / Run List (with load) / Analytics (with load) / Help. |
| 7 | On Run List | Hand the key to Run List (`RL-006`, `RL-008`, `RL-009`). |

Esc→`h` / Esc→`?` makes Help reachable from every screen, including Quick Entry (`NAV-018`).

**Divergence (Esc→Help).** The code has no `h`/`?` case after Esc (`handle_key_event`), so Help cannot be reached from Quick Entry without first switching elsewhere.

**Divergence (leaving mid-edit).** Switching away from Quick Entry does not touch its edit mode; Quick Entry specifies that leaving cancels the edit (`QE-023`), which Navigation must trigger on every screen switch away from Quick Entry.

Key handling never ends the app because of storage: `handle_key_event` returns nothing, and every storage failure it triggers is recorded on the screen that owns it (`RL-012`, `RL-013`, `AN-014`, `QE-020`). Switching to Run List or Analytics happens even when that screen's load fails, so the error is visible. Only terminal input/output errors in `run_app` (draw, poll, read) end the loop (`NAV-017`).

## Shared Chrome

`ui::render` (`src/ui/mod.rs`) splits the frame into a 3-row header, the active screen, and a 3-row footer.

- **Header** — bordered "Run Logger" block with `[1] Quick Entry  [2] Run List  [3] Analytics  [h] Help`; the active tab is bold yellow, others gray.
- **Footer** — one dark-gray hint line per screen (`src/ui/mod.rs:72-87`):

| Screen | Footer text today | Problem |
|---|---|---|
| Quick Entry | `[Tab] Next [Shift+Tab] Prev [Enter] Submit [Esc] Clear [h] Help [Ctrl+Q] Quit` | `h` types a letter; Esc alone only arms the prefix (clear is Esc Esc) |
| Run List | `[↑↓] Navigate [1-3] Switch screen [h] Help [q] Quit` | omits `e`/`d` (shown in Run List's own Actions panel) |
| Analytics | `[1-3] Switch screen [h] Help [q] Quit` | — |
| Help | `[1-3] Switch screen [q] Quit` | — |

- **Help screen** (`src/ui/screens/help.rs`) — static text: Navigation, Quick Entry Screen, Run List Screen, Your Goal, Tips, Data Location. It states that `h`/`?` work "from any screen" (not true on Quick Entry) and hard-codes the macOS database path.
- **Theme** (`src/ui/theme.rs`) — focused/unfocused input, error, success, stat, goal-met and goal-not-met styles. Several screens also set colors inline rather than through the theme.

**Divergence (hints).** Key hints in the footer and Help screen do not match key behavior on Quick Entry (`NAV-016`). The README repeats these.

## Decisions & Alternatives

| Decision | Chosen | Alternatives Considered | Rationale |
|---|---|---|---|
| Navigating out of a text form | Esc prefix, then a single key | Function keys; Ctrl/Alt shortcuts; leaving fields with Tab first | Works on every terminal and keyboard, from every screen, without stealing any printable character from Quick Entry. |
| Reaching Help | Esc→`h`/`?` from every screen, plus direct `h`/`?` off Quick Entry | Direct keys only | Esc is already the way to issue commands from Quick Entry; Help should not be the one screen it cannot reach. |
| Single-key commands | Direct `1`–`3`, `h`/`?`, `q` on every screen except Quick Entry | Esc prefix everywhere | Fast on read-only screens; Quick Entry needs every character for typing. |
| Quitting | Ctrl+Q/Ctrl+C anywhere, `q` outside Quick Entry | Confirmation prompt | Data is saved on every submit, so quitting never loses work. |
| Data freshness on screen switch | Reload Run List / Analytics on every switch to them | Push updates on write; cache | Always current with no synchronization logic; the history is small. |
| Startup order | Open the database before entering raw mode | Terminal first | Startup errors print to a normal terminal instead of a garbled alternate screen. |
| Panic safety | Panic hook restores the terminal | Rely on normal exit | A crash never leaves the user's shell in raw mode. |
| Event loop | Single-threaded, 100 ms poll, redraw every iteration | Async runtime; redraw only on events | Simple and responsive enough for a keyboard-driven form. |
| Failure policy | Screen-level errors are shown on that screen; only quit commands and terminal failures end the app | Exit on any error | A storage problem must not cost the user the rest of the app. |
| Where key handling lives | `src/input.rs`, separate from lifecycle in `src/main.rs` | Keep everything in `main.rs`; a library crate with `tests/` | Key routing and screen actions can be unit-tested against an in-memory database without a terminal, and each file stays within the 300-line limit. |

## Open Questions & Future Decisions

### Deferred
1. **No Esc-prefix indicator.** After Esc nothing on screen shows that the next key is a command.
2. **Hard-coded data path in Help.** Shows the macOS path even where `dirs::data_dir()` differs.
3. **Function size.** `handle_key_event` (~82 lines) exceeds the 50-line limit.
4. **Unused stubs.** `src/ui/components/{chart,input,stats}.rs` contain only comments.
5. **Inline colors.** Header, Run List and Analytics set colors directly instead of using `theme.rs`.

## References

- Code: `src/main.rs`, `src/input/mod.rs`, `src/app.rs`, `src/ui/mod.rs`, `src/ui/screens/help.rs`, `src/ui/theme.rs`, `src/ui/components/*`
- Specs: `docs/intent/navigation/navigation-specs.md`
- Tests: `src/input/tests.rs`
- Test support: `src/test_support.rs` (in-memory database, raw rows, write-blocking triggers, key constructors)
- Triggers: Storage startup (`STORE-001`), Run List load (`RL-001`), Analytics load (`AN-008`), Quick Entry clear (`QE-007`) and edit cancel (`QE-023`)
- External: `crossterm` 0.29, `ratatui` 0.30, `anyhow` 1
