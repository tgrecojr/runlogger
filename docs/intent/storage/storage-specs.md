# Storage Specs

Design: `docs/intent/storage/storage-design.md` · Prefix: `STORE`

Status: `[x]` = implemented; `[ ]` = specified but not yet implemented; `[D]` = deferred. The tests citing each spec are listed in `docs/arrows/storage.md`, regenerated from `@spec` scans.

## Location and Startup

- [x] **STORE-001**: On startup, the system shall open the run database at `runlogger/runs.db` inside the platform's per-user data directory, creating the `runlogger` directory and the database file if they do not exist.
- [x] **STORE-002**: If the per-user data directory cannot be determined, or the `runlogger` directory or database cannot be created or opened, then the system shall exit with an error message before the terminal UI is shown.
- [x] **STORE-003**: On startup, the system shall create the `runs` table and its indexes if they do not exist, and shall leave any existing run data unchanged apart from the start-time repair (STORE-014).
- [x] **STORE-016**: The system shall record the database's schema version (SQLite `user_version`) and, on startup, apply in order every schema migration newer than the recorded version, recording the new version after each; a database already at the current version shall be left unchanged.
- [x] **STORE-017**: If the database's recorded schema version is newer than any this version of the app knows, then the system shall exit at startup, before the terminal UI is shown, with an error saying the database was written by a newer version of Run Logger.
- [x] **STORE-014**: On startup, after creating the schema, the system shall rewrite every stored start time of the form `HH:MM:SS` followed by a dot and fractional-second digits to whole seconds, leaving every other stored value unchanged and leaving unchanged any row whose rewritten time would duplicate another run's date and start time, and shall continue starting up if the rewrite fails.

## Persisted Shape

- [x] **STORE-004**: The system shall persist each run with an automatically assigned integer id, a date, a start time, a distance in miles, an optional note, and a creation timestamp, where every field except the note is required.
- [x] **STORE-005**: If an insert or update would leave two runs with the same date and the same start time, then the system shall reject the write with an error that identifies it as a duplicate of an existing run's date and start time.
- [x] **STORE-006**: The system shall persist a run's date as local-calendar text in `YYYY-MM-DD` form, with no time zone.
- [x] **STORE-007**: The system shall persist a run's start time as local text in `HH:MM:SS` form, with whole seconds and no time zone, regardless of whether the time was typed by the user or taken from the clock.
- [x] **STORE-008**: The system shall persist a run's creation timestamp as RFC 3339 text in UTC, set when the run is first created.

## Operations

- [x] **STORE-009**: When a new run is saved, the system shall insert it and assign it a new id.
- [x] **STORE-010**: When an edited run is saved, the system shall update that run's date, start time, distance and note by id, and shall leave its id and creation timestamp unchanged.
- [x] **STORE-011**: When a run is deleted, the system shall remove the run with that id.
- [x] **STORE-012**: When the full run history is requested, the system shall return every stored run ordered by date descending, then by start time descending.
- [x] **STORE-013**: When stored runs are read, the system shall accept start times stored with fractional seconds and return them truncated to whole seconds.
- [x] **STORE-015**: If any stored run's date, start time, distance or creation timestamp cannot be read when stored runs are read, then the system shall fail the whole read with an error naming that run's id, the field, and the stored value, quoted with control characters escaped.
