# Arrow: navigation

The application shell: startup/shutdown, terminal ownership, event loop, key routing between screens, header/footer, Help screen and theme.

## Status

**AUDITED** — audited 2026-10-12 (git SHA `88de5b9`). All 18 specs verified against code; every header, footer and Help hint is true. Four specs are worded more broadly than the intended behavior.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/navigation/navigation-design.md

### EARS
- docs/intent/navigation/navigation-specs.md (18 specs)

### Tests
- src/input/tests.rs (NAV-017, NAV-018)
- src/ui/tests.rs (NAV-013, NAV-014, NAV-016)

### Code
- src/input/mod.rs (NAV-005, NAV-006, NAV-007, NAV-008, NAV-009, NAV-010, NAV-011, NAV-012, NAV-017, NAV-018)
- src/main.rs (NAV-001, NAV-002, NAV-003, NAV-004)
- src/ui/mod.rs (NAV-013, NAV-014, NAV-016)
- src/ui/screens/help.rs (NAV-015, NAV-016)

## Architecture

**Purpose:** Run the app, own the terminal, and route keys between screens.

**Key Components:**
1. `main` / `setup_terminal` / `restore_terminal` / `setup_panic_hook` — lifecycle and terminal safety
2. `run_app` — draw / poll / dispatch loop
3. `handle_key_event` (`src/input/mod.rs`) — quit keys, Esc prefix (`handle_esc_command`), direct keys (`handle_direct_keys`), hand-off to screen handlers; never fails on storage errors
4. `ui/mod.rs` — header tabs (Esc form on Quick Entry) and per-screen footer that fits 80 columns
5. `screens/help.rs` — static Help screen
6. `theme.rs` — shared styles

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps | Tested |
|----------|----------|-------------|----------|------|--------|
| Lifecycle | NAV-001, NAV-002, NAV-003, NAV-004 | 4 | 0 | 0 | 0 |
| Quitting | NAV-005, NAV-006 | 2 | 0 | 0 | 0 |
| Esc Prefix | NAV-007, NAV-008, NAV-009, NAV-010, NAV-018 | 5 | 0 | 0 | 1 |
| Direct Keys | NAV-011, NAV-012 | 2 | 0 | 0 | 0 |
| Shared Chrome | NAV-013, NAV-014, NAV-015, NAV-016 | 4 | 0 | 0 | 3 |
| Failures | NAV-017 | 1 | 0 | 0 | 1 |

**Summary:** 18 of 18 specs marked implemented; 5 of 18 cited by a test. Untested: NAV-001, NAV-002, NAV-003, NAV-004, NAV-005, NAV-006, NAV-007, NAV-008, NAV-009, NAV-010, NAV-011, NAV-012, NAV-015.

## Key Findings

1. **Spec wording** — NAV-004 promises a redraw "at least every 100 ms" but the loop waits up to 100 ms after each draw; NAV-010 ("ignore any other key after Esc") and NAV-012 ("every key goes to the form") don't except Ctrl+Q/Ctrl+C (NAV-005) and the key after Esc (NAV-007); NAV-015 promises "usage tips" and "the run database location", but tips are folded into other sections and the path shown is macOS-only.
2. **Key presses only** — key events are not filtered to presses; on Windows, crossterm also reports releases, so each key could act twice.
3. **Modifiers ignored** — Alt+1 / Ctrl+h switch screens off Quick Entry; Ctrl+letter types the letter on Quick Entry.
4. **No Esc-prefix indicator.**

## Work Required

### Should Fix
1. Reword NAV-004, NAV-010, NAV-012 and NAV-015 to match the intended behavior.
2. Add tests for key routing (NAV-005 to NAV-012) and lifecycle (NAV-001 to NAV-004).

### Nice to Have
3. Filter key events to presses; show an Esc-prefix indicator; show the real database path in Help; delete `ui/components` stubs; route inline colors through `theme.rs`.
