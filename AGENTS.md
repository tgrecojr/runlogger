# Run Logger

## Overview
A single-user terminal app for logging daily runs and tracking a streak toward the goal of running at least one mile every day. See `docs/high-level-design.md` for problem, goals, non-goals and tenets.

## Tech Stack
- Language: Rust (2021 edition)
- Framework: Ratatui (TUI) with the Crossterm backend
- Database: SQLite via `rusqlite` (bundled), one file at `<platform data dir>/runlogger/runs.db`
- Other: `chrono` (dates/times), `anyhow` / `thiserror` (errors), `dirs` (data directory)

## Commands
- `cargo run --release` — Build and run the app
- `cargo build --release` — Build the release binary (`target/release/runlogger`)
- `cargo test` — Run tests
- `cargo clippy -- -D warnings` — Lint
- `cargo fmt --check` — Check formatting

## Architecture
Five components, each with a design doc and EARS specs under `docs/intent/<component>/`:

- **storage** (`STORE`) — `src/db/`: database location, schema, stored formats, insert/read/update/delete
- **quick-entry** (`QE`) — the logging form: `QuickEntryState` in `src/app.rs`, `src/input/quick_entry.rs`, `src/logic/validation.rs`, `Run::new` in `src/models/run.rs`, `src/ui/screens/quick_entry.rs`
- **run-list** (`RL`) — history view: `RunListState`, `src/input/run_list.rs`, `load_runs` in `src/input/mod.rs`, `src/ui/screens/run_list.rs`
- **analytics** (`AN`) — daily-goal rule, streaks, statistics: `src/logic/streak.rs`, `src/models/analytics.rs`, `src/ui/screens/analytics/`, `load_analytics` in `src/input/mod.rs`
- **navigation** (`NAV`) — lifecycle, event loop, key routing, header/footer, Help, theme: `src/main.rs`, `handle_key_event` in `src/input/mod.rs`, `App`/`Screen` in `src/app.rs`, `src/ui/mod.rs`, `src/ui/screens/help.rs`, `src/ui/theme.rs`

Tests are `#[cfg(test)]` modules beside the code they test (a `tests.rs` or `tests/` folder next to each module), sharing helpers from `src/test_support.rs` (in-memory database, raw rows, key events, full-UI rendering). Each test that exercises a spec carries a `// @spec` comment; `docs/arrows/<segment>.md` lists them.

## Environment Variables
None. The app reads no environment variables.

## LID
- Mode: Full
- Version: 1.3.0

## Linked-Intent Development (MANDATORY)

**Consult the `linked-intent-dev` skill for ALL code changes.** All changes flow through the arrow of intent in one direction:

```
HLD → LLDs → EARS → Tests → Code
```

- **New features and refactors**: full six-phase workflow (HLD check → LLD check/draft → EARS → intent-narrowing edge audit → tests-first → code).
- **Bug fixes**: walk the arrow like any other change — find where behavior diverged from intent and cascade from there. No short-circuit.
- **If unsure**: use the full workflow.

Stop after each phase for user review. **Docs carry current intent, written to be read cold** — write each doc as if authored fresh today, from current intent alone: no narration of how it changed, no meaning that needs the conversation that produced it, no rebuttals to questions only a past discussion raised. Rationale, considered alternatives, and constraints a fresh author would independently write stay; record rejected alternatives and why in the LLD's Decisions & Alternatives table, not as asides in body prose.

**Memory vs. intent.** Before saving durable project knowledge to agent or tool memory, test whether it is project *intent* — would a fresh agent, in any tool, next session, need it to build this system correctly? If yes, record it in the arrow (HLD / LLD / EARS / decision doc), which travels and cascades — not in private, per-tool memory, where intent escapes the arrow. Knowledge about the user or how they like to work stays in memory.

### Navigation

| What you need | Where to look |
|---|---|
| High-level design | `docs/high-level-design.md` |
| Design tree (sub-HLDs, LLDs, their specs) | `docs/intent/` — one folder per node |
| EARS specs | beside each design doc as `{node}-specs.md` in the node's folder under `docs/intent/` |
| Decision docs | `docs/decisions/` (project-level) and `docs/intent/<segment>/decisions/` |
| Arrow of intent overlay | `docs/arrows/index.yaml` and per-segment docs in `docs/arrows/` |

### Terminology

- **HLD**: High-Level Design — single project-level doc at `docs/high-level-design.md`.
- **LLD**: Low-Level Design — detailed component design doc in `docs/intent/`. The design layer is a recursive tree: the root is the HLD, leaf LLDs own EARS, and a component deep enough to outgrow one doc becomes a sub-HLD (HLD-shaped, owns no EARS) with children beneath it. "HLD" and "LLD" are roles by position; depth-2 (one HLD over flat leaf LLDs) is the default.
- **EARS**: Easy Approach to Requirements Syntax — structured one-line requirements beside each design doc as `{node}-specs.md` in the node's folder under `docs/intent/`. IDs are path-concatenated — the root-to-leaf path of the owning segment plus a number — so a prefix grep gathers a subtree. Markers: `[x]` implemented, `[ ]` active gap, `[D]` deferred.
- **Arrow**: the unidirectional chain from vision to code (HLD → LLDs → EARS → Tests → Code). Strictly a DAG of intent.
- **Arrow segment**: the territory owned by one leaf LLD — the LLD itself plus the specs, tests, and code that cite its EARS IDs. The boundary is the leaf prefix. Within-segment cascade is free; across-segment cascade pauses.
- **Cascade**: propagating a change downstream through the arrow so adjacent levels stay coherent.

### Code annotations

Annotate code and tests with `@spec` comments citing EARS IDs:

```
// @spec AUTH-UI-001, AUTH-UI-002
```

Place the annotation at the *entry point of the behavior's implementation graph* — the topmost function or module owning the specified behavior, not every helper. When a behavior spans multiple subsystems (UI + API + database, for example), annotate at the entry point in each subsystem. Tests follow the same rule: annotate the test that directly exercises the spec, not every inner assertion.
