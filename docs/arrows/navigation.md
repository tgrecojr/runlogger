# Arrow: navigation

The application shell: startup/shutdown, terminal ownership, event loop, key routing between screens, header/footer, Help screen and theme.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. Storage errors no longer end the app. Two gaps: key hints that misdescribe Quick Entry behavior (NAV-016) and no Esc→Help (NAV-018).

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/navigation/navigation-design.md

### EARS
- docs/intent/navigation/navigation-specs.md (18 specs)

### Tests
- src/input/tests.rs (NAV-017)
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
3. `handle_key_event` (`src/input/mod.rs`) — quit keys, Esc prefix, direct screen keys, hand-off to screen handlers; never fails on storage errors
4. `ui/mod.rs` — header tabs and per-screen footer
5. `screens/help.rs` — static Help screen
6. `theme.rs` — shared styles

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps |
|----------|----------|-------------|----------|------|
| Lifecycle | NAV-001 to NAV-004 | 4 | 0 | 0 |
| Quitting | NAV-005 to NAV-006 | 2 | 0 | 0 |
| Esc prefix | NAV-007 to NAV-010, NAV-018 | 4 | 0 | 1 |
| Direct keys | NAV-011 to NAV-012 | 2 | 0 | 0 |
| Shared chrome | NAV-013 to NAV-016 | 3 | 0 | 1 |
| Failures | NAV-017 | 1 | 0 | 0 |

**Summary:** 16 of 18 specs implemented; 2 gaps (NAV-016, NAV-018). Tests cite NAV-017 only.

## Key Findings

1. **Hint drift** — Quick Entry footer advertises `[h] Help` and `[Esc] Clear` (`render_footer` in `src/ui/mod.rs`); Help claims `h` works everywhere (`src/ui/screens/help.rs`); README promises Esc→`h`, which is missing (NAV-016, NAV-018).
2. **Function size** — `handle_key_event` is ~82 lines (limit 50).

## Work Required

### Must Fix
1. Add Esc→`h`/`?` to open Help (NAV-018).
2. Make footer and Help hints accurate (NAV-016); align README.

### Should Fix
3. Split `handle_key_event` under the 50-line limit.
4. Add tests for key routing (NAV-005 to NAV-012) against `App` state without a terminal.

### Nice to Have
5. Show an indicator while the Esc prefix is armed; delete `ui/components` stubs; route inline colors through `theme.rs`; show the real database path in Help.
