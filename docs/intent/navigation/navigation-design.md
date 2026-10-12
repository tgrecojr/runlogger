---
parent: high-level-design
prefix: NAV
---

# Navigation

## Context and Design Philosophy

Navigation is the application shell: it starts and stops the app, owns the terminal, runs the event loop, and decides which screen a key belongs to. It also draws the parts of the UI common to every screen — the header tabs, the footer key hints and the Help screen — and holds the shared color theme.

The central constraint is that Quick Entry must accept *any* typed character, including digits and letters, because notes are free text. So single-key commands (`1`–`3`, `h`, `q`) cannot work on Quick Entry. Navigation resolves this with an **Esc prefix**: Esc arms a one-key navigation command that works from every screen, while on other screens the single keys work directly.

Navigation owns: startup and shutdown order, terminal setup/restore and the panic hook, the event loop, global key routing (quit keys, the Esc prefix, direct screen keys), the `App`/`Screen` state, the header, the footer, the Help screen, and `theme.rs`. It triggers, but does not own, each screen's data load (`RL-001`, `AN-008`), Quick Entry's clear (`QE-007`) and edit cancel (`QE-023`).

Code: `src/main.rs` (`main`), `src/lifecycle.rs` (`start`, `with_restore`, `install_panic_hook`, `run_app`, `EventSource`), `handle_key_event` in `src/input/mod.rs`, `Screen` and `App` in `src/app.rs`, `src/ui/mod.rs`, `src/ui/screens/help.rs`, `src/ui/theme.rs`.

## Lifecycle

```
main
 ├─ install_panic_hook(restore)   restore terminal before the original panic output
 ├─ start(open_db, enter_terminal)  database first (STORE), then raw mode + alternate screen
 ├─ App::new                      screen = QuickEntry
 └─ with_restore(run_app, restore)  restore runs whether run_app returns Ok or Err
      run_app: loop { app.tick(today); draw; next_event(≤100 ms); handle key; break on should_quit }
```

The lifecycle lives in `src/lifecycle.rs`, with every side effect passed in so tests can check order and guarantees without a real terminal:

| Function | Contract |
|---|---|
| `start(open_db, enter_terminal)` | Calls `open_db` first; calls `enter_terminal` only if it succeeded, so a startup failure prints to a normal terminal (`NAV-001`). |
| `with_restore(body, restore)` | Runs `body`, then always runs `restore` — leave raw mode, leave the alternate screen, show the cursor — returning `body`'s error if any (`NAV-002`). |
| `install_panic_hook(restore)` | Wraps the current panic hook so `restore` runs before the original hook prints the panic (`NAV-003`). |
| `run_app(terminal, app, conn, events)` | Generic over the ratatui `Backend` and an `EventSource` (`next_event(timeout)`); each iteration ticks the app clock, draws, then waits at most 100 ms for an event, so the screen redraws after every key and about every 100 ms while idle (`NAV-004`). Production uses `CrosstermEvents` (crossterm `poll` + `read`); tests use `TestBackend` and scripted events. |

Key-release events are dropped before routing (`NAV-019`): crossterm reports releases on Windows, and acting on them would apply every key twice. Presses and repeats (a held key) are handled.

## Key Routing

`handle_key_event` applies these rules in order. It and the screen handlers it dispatches to live in `src/input.rs`; `src/main.rs` keeps only startup, the terminal and the event loop.

| # | Condition | Effect |
|---|---|---|
| 1 | Ctrl+Q or Ctrl+C | Quit (any screen). |
| 2 | Esc prefix armed (`handle_esc_command`) | Disarm, then: `1` → Quick Entry; `2` → load runs, Run List; `3` → load analytics, Analytics; `h` or `?` → Help; Esc → clear Quick Entry if it is shown; anything else → ignored. |
| 3 | Esc | Arm the Esc prefix (no visible indicator). |
| 4 | `q`, not on Quick Entry | Quit. |
| 5 | On Quick Entry | Hand the key to Quick Entry (`QE-003`). |
| 6 | `1` / `2` / `3` / `h` or `?` (`handle_direct_keys`) | Switch to Quick Entry / Run List (with load) / Analytics (with load) / Help. |
| 7 | On Run List | Hand the key to Run List (`RL-006`, `RL-008`, `RL-009`). |

Esc→`h` / Esc→`?` makes Help reachable from every screen, including Quick Entry (`NAV-018`). `handle_key_event` delegates each rule group to a small function — quit keys, the Esc command, direct screen keys — to stay within the 50-line function limit.

Every screen change goes through `App::switch_to_screen`. When the target is Run List or Analytics and Quick Entry is in edit mode, it resets the Quick Entry form, cancelling the edit (`QE-023`) — whether the switch starts on Quick Entry or on Help. Switching to Help or to Quick Entry (Run List's `e`), and any switch while not editing, leave the form alone.

Key handling never ends the app because of storage: `handle_key_event` returns nothing, and every storage failure it triggers is recorded on the screen that owns it (`RL-012`, `RL-013`, `AN-014`, `QE-020`). Switching to Run List or Analytics happens even when that screen's load fails, so the error is visible. Only terminal input/output errors in `run_app` (draw, poll, read) end the loop (`NAV-017`).

## Shared Chrome

`ui::render` (`src/ui/mod.rs`) splits the frame into a 3-row header, the active screen, and a 3-row footer.

- **Header** — bordered "Run Logger" block listing the four screens with the key that opens each; the active tab is bold yellow, others gray. On Quick Entry, where single keys type text, the tabs show the Esc form — `[Esc 1] Quick Entry  [Esc 2] Run List  [Esc 3] Analytics  [Esc h] Help` — and on every other screen the direct form `[1] Quick Entry  [2] Run List  [3] Analytics  [h] Help` (`NAV-013`).
- **Footer** — one dark-gray hint line per screen (`render_footer`). Every hint must be true on the screen it is shown for (`NAV-016`), and every line fits the 78 columns inside the footer border on an 80-column terminal (`NAV-014`); the Help screen carries the longer explanations.

| Screen | Footer |
|---|---|
| Quick Entry | `[Tab] Field  [Enter] Save  [Esc Esc] Clear  [Esc 1-3/h] Screen/Help  [^Q] Quit` |
| Run List | `[↑↓] Navigate  [e] Edit  [d] Delete  [1-3] Screen  [h] Help  [q] Quit` |
| Analytics | `[1-3] Screen  [h] Help  [q] Quit` |
| Help | `[1-3] Screen  [q] Quit` |
| *any screen, after Esc* | `Esc → [1] Quick Entry  [2] Run List  [3] Analytics  [h] Help` in bold cyan, plus `  [Esc] Clear` on Quick Entry (`NAV-020`) |

While the key after Esc is awaited, the footer turns into that menu, so the user can see that the next key is a command and what it can do; the next key restores the normal footer.

- **Help screen** (`src/ui/screens/help.rs`) — static reference in sections: Navigation (Esc commands work everywhere; direct `1`–`3`, `h`, `?`, `q` everywhere except Quick Entry, where keys type text), Quick Entry (fields, save, clear, Time refreshing after a save, no future runs, leaving mid-edit cancels the edit), Run List (select, edit, delete without confirmation), Your Goal (a mile a day, runs on one day add up, today counts once its mile is logged), and Data Location — the full path of the database actually in use, which `main` stores on `App` as `db_location` after opening it (`NAV-015`).
- **Theme** (`src/ui/theme.rs`) — every style the UI uses, named by role rather than color: chrome (`active_tab`, `inactive_tab`, `footer_hint`, `esc_menu`, `screen_title`, `help_title`, `heading`, `panel_heading`, `muted`, `error_style`, `success_style`), the Quick Entry form (`focused_input_style`, `unfocused_input_style`, `focused_border`, `unfocused_border`), Run List (`table_header`, `selected_row`, `navigate_key`, `edit_key`, `delete_key`, `switch_key`), and Analytics figures and goal status (`stat_style`, `goal_met_style`, `goal_partial_style`, `goal_not_met_style`, `goal_close_style`, `reminder_style`, `volume_high`, `volume_mid`, `volume_low`). Screens never name a color, so a palette change is made once, in `theme.rs`.

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
| Testable lifecycle | Side effects passed into `start`, `with_restore`, `install_panic_hook` and `run_app` | Leave lifecycle in `main` untested; integration tests in a pseudo-terminal | Startup order, terminal restore and redraw cadence can be unit-tested without a real terminal, at the cost of a little indirection in `main`. |
| Key-release events | Ignored | Act on every key event | Windows reports both press and release; acting on releases would double every key. |
| Styling | Role-named functions in `theme.rs`; screens never name a color | Colors inline in each screen | One place to change the palette, and a style's purpose is readable at the call site. |
| Failure policy | Screen-level errors are shown on that screen; only quit commands and terminal failures end the app | Exit on any error | A storage problem must not cost the user the rest of the app. |
| Header keys on Quick Entry | Show the Esc form (`[Esc 1]` …) on Quick Entry, the direct form elsewhere | Same labels everywhere; treat the header as a screen map only | The header is read as key hints; on Quick Entry the direct keys type text, so only the Esc form is true there. |
| Footer length | Compact hints that fit an 80-column terminal; the Help screen explains in full | Descriptive hints that clip below ~105 columns | A clipped hint is a hidden hint; the footer is a reminder, Help is the reference. |
| Where key handling lives | `src/input.rs`, separate from lifecycle in `src/main.rs` | Keep everything in `main.rs`; a library crate with `tests/` | Key routing and screen actions can be unit-tested against an in-memory database without a terminal, and each file stays within the 300-line limit. |

## Open Questions & Future Decisions

### Deferred
None.

## References

- Code: `src/main.rs`, `src/lifecycle.rs`, `src/input/mod.rs`, `src/app.rs`, `src/ui/mod.rs`, `src/ui/screens/help.rs`, `src/ui/theme.rs`
- Specs: `docs/intent/navigation/navigation-specs.md`
- Tests: listed in `docs/arrows/navigation.md`
- Triggers: Storage startup (`STORE-001`), Run List load (`RL-001`), Analytics load (`AN-008`), Quick Entry clear (`QE-007`) and edit cancel (`QE-023`)
- External: `crossterm` 0.29, `ratatui` 0.30, `anyhow` 1
