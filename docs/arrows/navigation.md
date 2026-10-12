# Arrow: navigation

The application shell: startup/shutdown, terminal ownership, event loop, key routing between screens, header/footer, Help screen and theme.

## Status

**AUDITED** — audited 2026-10-12 (git SHA `88de5b9`); since then the lifecycle moved to `src/lifecycle.rs` with injectable side effects, NAV-019 (ignore key releases) was added, and four specs were reworded. All 19 specs implemented and tested.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/navigation/navigation-design.md

### EARS
- docs/intent/navigation/navigation-specs.md (19 specs)

### Tests
- src/input/tests/edits.rs (NAV-018)
- src/input/tests/errors.rs (NAV-017)
- src/input/tests/key_events.rs (NAV-019)
- src/input/tests/routing.rs (NAV-005, NAV-006, NAV-007, NAV-008, NAV-009, NAV-010, NAV-011, NAV-012)
- src/lifecycle/tests.rs (NAV-001, NAV-002, NAV-003, NAV-004)
- src/ui/tests/chrome.rs (NAV-013, NAV-014, NAV-015, NAV-016)

### Code
- src/input/mod.rs (NAV-005, NAV-006, NAV-007, NAV-008, NAV-009, NAV-010, NAV-011, NAV-012, NAV-017, NAV-018, NAV-019)
- src/lifecycle.rs (NAV-001, NAV-002, NAV-003, NAV-004)
- src/main.rs (NAV-001, NAV-002, NAV-003)
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
| Lifecycle | NAV-001, NAV-002, NAV-003, NAV-004 | 4 | 0 | 0 | 4 |
| Quitting | NAV-005, NAV-006 | 2 | 0 | 0 | 2 |
| Esc Prefix | NAV-007, NAV-008, NAV-009, NAV-010, NAV-018 | 5 | 0 | 0 | 5 |
| Direct Keys | NAV-011, NAV-012, NAV-019 | 3 | 0 | 0 | 3 |
| Shared Chrome | NAV-013, NAV-014, NAV-015, NAV-016 | 4 | 0 | 0 | 4 |
| Failures | NAV-017 | 1 | 0 | 0 | 1 |

**Summary:** 19 of 19 specs marked implemented; 19 of 19 cited by a test. Untested: none.

## Key Findings

1. **Modifiers ignored** — Alt+1 / Ctrl+h switch screens off Quick Entry; Ctrl+letter types the letter on Quick Entry.
2. **No Esc-prefix indicator.**
3. **Help shows the macOS data path** on every platform (NAV-015 names the macOS location).

## Work Required

### Nice to Have
1. Show an Esc-prefix indicator; show the real database path in Help; delete `ui/components` stubs; route inline colors through `theme.rs`.
