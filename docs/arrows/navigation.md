# Arrow: navigation

The application shell: startup/shutdown, terminal ownership, event loop, key routing between screens, header/footer, Help screen and theme.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. All 18 specs implemented.

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
- src/test_support.rs (shared test helpers)

### Code
- src/main.rs (`main`, terminal helpers, `run_app`)
- src/input/mod.rs (`handle_key_event`)
- src/app.rs (`Screen`, `App`, `App::switch_to_screen`)
- src/ui/mod.rs
- src/ui/screens/help.rs
- src/ui/theme.rs
- src/ui/components/ (comment-only stubs)

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

| Category | Spec IDs | Implemented | Deferred | Gaps |
|----------|----------|-------------|----------|------|
| Lifecycle | NAV-001 to NAV-004 | 4 | 0 | 0 |
| Quitting | NAV-005 to NAV-006 | 2 | 0 | 0 |
| Esc prefix | NAV-007 to NAV-010, NAV-018 | 5 | 0 | 0 |
| Direct keys | NAV-011 to NAV-012 | 2 | 0 | 0 |
| Shared chrome | NAV-013 to NAV-016 | 4 | 0 | 0 |
| Failures | NAV-017 | 1 | 0 | 0 |

**Summary:** 18 of 18 specs implemented. Tests cite NAV-013, NAV-014, NAV-016, NAV-017 and NAV-018.

## Key Findings

1. **Hard-coded data path in Help** — shows the macOS path even where `dirs::data_dir()` differs.
2. **No Esc-prefix indicator** — after Esc nothing on screen shows that the next key is a command.

## Work Required

### Should Fix
1. Add tests for key routing (NAV-005 to NAV-012) and lifecycle (NAV-001 to NAV-004).

### Nice to Have
2. Show an indicator while the Esc prefix is armed; show the real database path in Help; delete `ui/components` stubs; route inline colors through `theme.rs`.
