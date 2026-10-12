---
parent: high-level-design
prefix: STORE
---

# Storage

## Context and Design Philosophy

Storage keeps the user's run history on local disk and hands it back to the rest of the app. Run Logger is a single-user, offline terminal app, so storage is one embedded SQLite file in the platform's per-user data directory — no server, no sync, no accounts.

Storage owns the *persisted shape* of a run: which columns exist, how dates and times are written as text, and what the database itself refuses (duplicates, missing required fields). It does not own what makes a run *valid* to log — distance bounds and input formats belong to Quick Entry (`QE`), which builds `Run` values via `Run::new` before handing them to storage.

Every consumer reads the whole history: Run List and Analytics both call `get_all_runs` and work in memory. At one user logging roughly one run a day, the table stays in the low thousands of rows for years, so full reads are cheap and keep consumers simple.

Code: `src/db/connection.rs`, `src/db/migrations.rs`, `src/db/queries.rs`, and the `Run` struct's field layout in `src/models/run.rs:4-13`.

## Database Location and Startup

At startup `main` calls `get_db_path` then `init_db` before the terminal enters raw mode:

1. `get_db_path` resolves `dirs::data_dir()` (on macOS: `~/Library/Application Support`), appends `runlogger/`, creates that directory if missing, and returns `runlogger/runs.db`.
2. `init_db` opens the SQLite file (creating it if absent) and runs `init_database`.
3. Any failure here propagates out of `main` as an `anyhow` error with context ("Failed to get data directory", "Failed to create application directory", "Failed to open database connection", "Failed to initialize database"), so the process exits before the TUI is drawn and the message is printed to a normal terminal.

The single `rusqlite::Connection` lives for the whole process and is passed by reference into every key handler.

## Schema

`init_database` (`src/db/migrations.rs`) runs idempotent DDL on every launch:

```sql
CREATE TABLE IF NOT EXISTS runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    time_started TEXT NOT NULL,
    distance_miles REAL NOT NULL,
    note TEXT,
    created_at TEXT NOT NULL,
    UNIQUE(date, time_started)
);
CREATE INDEX IF NOT EXISTS idx_runs_date ON runs(date DESC);
CREATE INDEX IF NOT EXISTS idx_runs_created_at ON runs(created_at DESC);
```

**Schema versioning (`STORE-016`, `STORE-017`).** The schema version lives in SQLite's `user_version`. `MIGRATIONS` is an ordered list of SQL batches; migration *n* brings a database to version *n*. Migration 1 is the DDL above — written with `IF NOT EXISTS`, so databases created before versioning (version 0, tables already present) pass through it unchanged. On startup `init_database` reads `user_version`, runs each newer migration inside a transaction that also sets `user_version` to that migration's number, and leaves a current database untouched. A recorded version higher than `MIGRATIONS.len()` means a newer app wrote the file; startup fails with `StorageError::NewerSchema` ("This database was written by a newer version of Run Logger (schema N; this version supports M)") rather than risk misreading it. The next schema change is a new entry appended to `MIGRATIONS`.

After the DDL, `repair_start_times` repairs start times written with fractional seconds by earlier versions:

```sql
UPDATE OR IGNORE runs SET time_started = substr(time_started, 1, 8)
 WHERE time_started GLOB '[0-2][0-9]:[0-5][0-9]:[0-5][0-9].[0-9]*'
   AND substr(time_started, 10) NOT GLOB '*[^0-9]*';
```

The pattern matches only the shape older versions wrote — `HH:MM:SS`, a dot, and one or more digits — so any other malformed value is left untouched for the read to report with its id and value (`STORE-015`) rather than being silently truncated. No two runs may share a date and a whole-second start time (`STORE-005`). A collision during repair would require two runs submitted within the same second, which Quick Entry rules out by clearing Distance after every save; `OR IGNORE` is a safety net that leaves such a row unchanged and readable through the tolerant read below rather than aborting startup. If the repair statement fails for any reason, startup continues — the tolerant read keeps unrepaired rows usable. The statement matches no rows once a database is clean, so it is safe to run on every launch.

## Stored Formats

| Column | Rust type | Written as | Read back with |
|---|---|---|---|
| `date` | `NaiveDate` | `NaiveDate::to_string()` → `YYYY-MM-DD` | `parse_from_str("%Y-%m-%d")` |
| `time_started` | `NaiveTime` | `format("%H:%M:%S")` — whole seconds, whatever the value's precision | `parse_from_str("%H:%M:%S%.f")`, then truncated to whole seconds |
| `distance_miles` | `f64` | REAL, miles | REAL |
| `note` | `Option<String>` | TEXT or NULL | TEXT or NULL |
| `created_at` | `DateTime<Utc>` | `to_rfc3339()` | `parse_from_rfc3339` → UTC |

Date and start time are *naive local* values — no time zone — because the daily goal is judged by the user's local calendar day. `created_at` is a UTC audit timestamp set by `Run::new` and never displayed.

Storage writes start times in whole seconds itself rather than trusting callers to supply whole-second values, and reads tolerate a fractional part so that rows written before this rule remain readable. A run read back always carries a whole-second start time.

## Operations

All in `src/db/queries.rs`; every function takes `&Connection` and returns `anyhow::Result`.

| Function | Effect | Caller |
|---|---|---|
| `insert_run(run) -> i64` | INSERT of date, time, distance, note, created_at; returns new rowid | Quick Entry submit (`src/input/quick_entry.rs`) |
| `get_all_runs() -> Vec<Run>` | SELECT all, `ORDER BY date DESC, time_started DESC` | `load_runs`, `load_analytics` (`src/input/mod.rs`) |
| `get_runs_by_date_range(start, end)` | Same conversion, filtered by inclusive date range | none — `#[allow(dead_code)]` |
| `update_run(run)` | UPDATE date, time, distance, note by id; errors if `run.id` is `None`; `created_at` untouched | Quick Entry submit while editing |
| `delete_run(id)` | DELETE by id | Run List `d` (`src/input/run_list.rs`) |

A history read is all-or-nothing: if any row's date, start time or `created_at` cannot be parsed, the whole read fails with an error naming the run's id, the field and the stored value — for example `Run 12 has an unreadable date: "2026-13-45"`. The value is shown quoted with Rust debug escaping, so newlines or control characters in a hand-edited database cannot disturb the screen. No partial history is returned, so no screen ever shows figures computed from an incomplete history; the screen that asked shows the error instead and the app keeps running (`NAV-017`).

Distance is read as a raw SQLite value: a real or integer is accepted; anything else (text, blob, NULL) fails the read the same way as an unreadable date, as `Run N has an unreadable distance: <value>`.

Row decoding is shared by every read: `query_runs` fetches rows as raw column values (`StoredRun`) and `to_run` converts each to a `Run`, outside the SQLite row callback, so conversion errors carry ordinary `anyhow` context.

A uniqueness violation on insert or update is reported as a typed `StorageError::DuplicateStart { date, time }` (a `thiserror` enum inside the `anyhow` error), recognized from SQLite's extended result code `SQLITE_CONSTRAINT_UNIQUE`, so callers can tell the user exactly which run conflicts (`STORE-005`). Other SQLite errors surface as `anyhow` errors with context ("Failed to insert run", "Failed to update run", "Failed to delete run").

## Decisions & Alternatives

| Decision | Chosen | Alternatives Considered | Rationale |
|---|---|---|---|
| Storage engine | Embedded SQLite via `rusqlite` with the `bundled` feature | JSON/CSV flat file; system SQLite | Single-user local app wants a durable file with real constraints and ordering; `bundled` avoids depending on the OS's libsqlite version. |
| Database location | `dirs::data_dir()/runlogger/runs.db` | Working directory; config flag; XDG override | Platform-conventional per-user data location; app works from any directory with no setup. |
| Date/time representation | Separate naive-local `date` and `time_started` TEXT columns | Single UTC timestamp; Unix epoch integer | The local calendar day is the unit of the daily goal, so the date is stored directly rather than derived from a zone-dependent instant; ISO text sorts correctly. |
| Duplicate protection | `UNIQUE(date, time_started)`, reported as a typed `DuplicateStart` error | No constraint; dedupe in app; callers match SQLite's message text | Guards against double-submitting the same entry; a typed error lets the UI explain the conflict without depending on SQLite's wording. |
| Schema evolution | Ordered migrations recorded in `user_version`; refuse a newer schema | `IF NOT EXISTS` DDL only; a migration crate | Gives the next schema change a safe path for existing databases without a dependency; refusing a newer schema protects data written by a newer app. |
| Read strategy | Load the entire table on each screen switch | Date-range queries; SQL aggregation for analytics | Data volume is tiny; in-memory computation keeps analytics logic in Rust and testable without SQL. |
| Connection lifetime | One connection for the process, passed by reference | Open per operation; pool | Single-threaded event loop, single user. |
| Start times written before whole-second storage | Tolerant read plus a startup repair that rewrites `HH:MM:SS.<digits>` values to whole seconds | Tolerant read only; repair only; repair anything longer than 8 characters | The repair brings stored data to the one canonical format; the tolerant read covers rows the repair must skip to avoid duplicates and any database the repair has not yet run on. |
| Unreadable rows | Fail the whole read with an error naming run id, field and value | Skip the row and warn | Streaks and totals computed from a partial history would be wrong without looking wrong; a precise error tells the user exactly which row to fix. |
| Distance unit | Miles only, `REAL` | Store meters and convert for display | The goal and all UI are expressed in miles. |

## Open Questions & Future Decisions

### Deferred
1. **Unused surface.** `get_runs_by_date_range` is dead code; `idx_runs_created_at` serves no query; `open_connection` is public but only called by `init_db`. Keep, remove, or put to use?

## References

- Code: `src/db/connection.rs`, `src/db/migrations.rs`, `src/db/queries.rs`, the `Run` struct in `src/models/run.rs`
- Tests: listed in `docs/arrows/storage.md`
- Specs: `docs/intent/storage/storage-specs.md`
- Consumers: Quick Entry (`QE`) for insert/update; Run List (`RL`) for read/delete; Analytics (`AN`) for read; Navigation (`NAV`) for startup and error propagation
- External: `rusqlite` 0.40 (bundled SQLite), `chrono` 0.4, `dirs` 7, `anyhow` 1
