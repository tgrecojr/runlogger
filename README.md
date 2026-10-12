# Run Logger TUI

A terminal user interface (TUI) application for tracking your daily runs and maintaining your streak.

## Goal

Run at least 1 mile every day! This app helps you track your progress toward that goal.

## Features

- **Quick Entry**: Fast run logging with today's date and time pre-filled — type a distance and press Enter
- **Run List**: Browse, edit, and delete your logged runs
- **Analytics**: Current and longest streak, progress toward 365 goal days this year, period stats, a 14-day chart, and monthly mileage
- **Streak Tracking**: Multiple runs on the same day count toward your 1-mile daily goal
- **Escape-based Navigation**: Esc then a key reaches any screen from anywhere, even while typing

## Installation

Build and run the application:

```bash
cargo run --release
```

Or build a release binary:

```bash
cargo build --release
./target/release/runlogger
```

## Usage

### Navigation

- **[Esc] → [1] / [2] / [3]**: Quick Entry / Run List / Analytics — works on every screen
- **[Esc] → [h] or [?]**: Help screen — works on every screen
- **[1-3], [h], [?]**: The same, directly — on every screen except Quick Entry, where keys type into the form
- **[q]**: Quit (except in Quick Entry)
- **[Ctrl+Q] or [Ctrl+C]**: Quit from anywhere

The header shows the key for each screen: on Quick Entry it shows the Esc form (`[Esc 1]`, …), elsewhere the direct key (`[1]`, …).

### Quick Entry (Default Screen)

- **Tab / Shift+Tab**: Next / previous field
- **Enter**: Save the run (or the changes, when editing)
- **Esc → Esc**: Clear the form (this also cancels an edit)
- Date and time are pre-filled; just enter a distance and optionally a note
- After each save, Time resets to the current time, so the next run gets its own start time
- Runs cannot be dated or timed in the future

### Run List

- **Up/Down Arrows**: Select a run (the list scrolls to keep the selection visible)
- **[e]**: Edit the selected run in the Quick Entry form
- **[d]**: Delete the selected run (immediate, no confirmation); the selection stays at the same row
- After saving an edit you return to Run List with the edited run selected
- Leaving an edit for Run List or Analytics cancels it; a quick look at Help keeps it

### Analytics

- **Current Streak**: Consecutive days with 1+ mile. Today counts once its mile is logged; until then the streak counts through yesterday and shows *(run today to extend)*
- **Longest Streak**: Your personal best streak
- **Year Goal**: Days remaining to 365 goal days this year, with completion percentage
- **Totals**: Total runs, distance, and average per run
- **Period Stats**: Runs and average distance for the last 7 days, the last 30 days, and this year
- **Last 14 Days**: Daily mileage, color-coded:
  - **Green**: Goal met (1.00 mi or more)
  - **Yellow**: Partial run (under 1.00 mi)
  - **Red**: No run
- **Mileage by Month**: Shown beside the chart — the 12 most recent months with runs (green at 50+ mi, yellow at 25+ mi)

## Data Storage

Your runs are stored in an SQLite database in your platform's data directory — on macOS:
```
~/Library/Application Support/runlogger/runs.db
```

If stored data can't be read, Run List or Analytics shows the error in red (naming the run and the bad value) and the app keeps running.

## Streak Rules

- Your goal is to run at least 1 mile every day
- Multiple runs on the same day add up toward the 1-mile threshold
- A day's total is judged at the precision shown on screen (hundredths of a mile): a day shown as "1.00 mi" always counts
- The streak counts consecutive days where you've met the goal; an unfinished today never breaks it
- Missing a whole day resets your current streak (your longest streak is preserved)
- Runs dated in the future are ignored by every statistic

## Examples

### Logging a Run
1. Launch the app (Quick Entry is the default screen)
2. Date and time are already filled with the current values
3. Enter distance: `3.5`
4. Optionally add a note: `Morning run in the park`
5. Press Enter to save

### Editing a Run
1. Press Esc → 2 to view Run List
2. Use Up/Down arrows to select a run
3. Press `e` to edit
4. Modify any field (date, time, distance, note)
5. Press Enter to save — you return to Run List with the run selected

### Deleting a Run
1. Navigate to Run List (Esc → 2)
2. Select the run with Up/Down arrows
3. Press `d` to delete immediately

### Checking Your Streak
1. Press Esc → 3 to switch to Analytics
2. See your current streak, and whether today's mile is still to do
3. Check the 14-day chart — green days met the goal

## Keyboard Shortcuts Quick Reference

| Key | Action | Context |
|-----|--------|---------|
| Esc → 1/2/3 | Go to Quick Entry / Run List / Analytics | Any screen |
| Esc → h or ? | Help | Any screen |
| 1/2/3, h or ? | Same, directly | Any screen except Quick Entry |
| Tab / Shift+Tab | Next / previous field | Quick Entry |
| Enter | Save | Quick Entry |
| Esc Esc | Clear form (cancels an edit) | Quick Entry |
| ↑↓ | Select a run | Run List |
| e | Edit selected run | Run List |
| d | Delete selected run | Run List |
| q | Quit | Any screen except Quick Entry |
| Ctrl+Q / Ctrl+C | Quit | Anywhere |

## Tips

- The Quick Entry screen is optimized for speed — just enter a distance and press Enter
- Distance must be more than 0 and at most 200 miles (decimals like 3.5 are fine)
- Log a past run by changing the Date (and Time) fields before saving
- Use Esc → number to switch screens while typing in Quick Entry
- The footer always lists the keys that work on the current screen; Help has the details

## Development

Built with:
- **Rust**: Fast, safe systems programming language
- **Ratatui**: TUI framework
- **Crossterm**: Cross-platform terminal backend
- **SQLite**: Embedded database (via rusqlite)
- **Chrono**: Date and time handling

Project structure:
- `src/db/`: Database layer (schema, queries, connection)
- `src/models/`: Data models (Run, Analytics)
- `src/logic/`: Business logic (streaks and statistics, input validation)
- `src/input/`: Key handling for each screen
- `src/ui/`: User interface (screens, theme)
- `src/app.rs`: Application state
- `src/main.rs`: Entry point and event loop

Design docs live in `docs/` (see `docs/high-level-design.md`); `AGENTS.md` describes the development workflow.

### Building from Source

Requirements:
- Rust (2021 edition)
- Cargo

```bash
# Clone the repository
git clone <repository-url>
cd runlogger

# Build
cargo build --release

# Run tests
cargo test

# Run the application
./target/release/runlogger
```

## License

This is a personal project. Feel free to use and modify as needed.
