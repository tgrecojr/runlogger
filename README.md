# Run Logger TUI

A terminal user interface (TUI) application for tracking your daily runs and maintaining your streak.

## Goal

Run at least 1 mile every day! This app helps you track your progress toward that goal.

## Features

- **Quick Entry**: Fast run logging with today's date and time pre-filled
- **Run List**: View, edit, and delete your logged runs
- **Analytics**: Track your current streak, total stats, and visualize progress with color-coded daily charts
- **Streak Tracking**: Multiple runs on the same day count toward your 1-mile daily goal
- **Escape-based Navigation**: Navigate between screens from anywhere, even while typing

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

### Navigation (Works from Any Screen)

- **[Esc] → [1]**: Quick Entry screen
- **[Esc] → [2]**: Run List screen
- **[Esc] → [3]**: Analytics screen
- **[1-3]**: Quick switch (when NOT in Quick Entry)
- **[h] or [?]**: Help screen (when NOT in Quick Entry)
- **[q]**: Quit (except in Quick Entry screen)
- **[Ctrl+Q] or [Ctrl+C]**: Quit from anywhere

### Quick Entry (Default Screen)

- **Tab/Shift+Tab**: Navigate between fields
- **Enter**: Submit run entry (or update if editing)
- **Esc → Esc**: Clear all fields (press Escape twice)
- Date and time are pre-filled with current values
- Just enter distance and optionally a note
- Type freely - all letters and numbers work in fields

### Run List

- **Up/Down Arrows**: Navigate through runs
- **[e]**: Edit the selected run
- **[d]**: Delete the selected run (immediate, no confirmation)
- View date, time, distance, and notes for each run
- Helpful hints displayed at bottom of screen

### Analytics

- **Current Streak**: Consecutive days with 1+ mile (green if active)
- **Longest Streak**: Your personal best streak
- **Totals**: Total runs, distance, and average per run
- **Period Stats**: Runs this week (last 7 days), month (30 days), and year
- **14-Day Chart**: Visual daily mileage with color coding:
  - **Green**: Goal met (≥ 1.0 mile)
  - **Yellow**: Partial run (< 1.0 mile)
  - **Red**: No run (0 miles)

## Data Storage

Your runs are stored in an SQLite database at:
```
~/Library/Application Support/runlogger/runs.db
```

## Streak Rules

- Your goal is to run at least 1 mile every day
- Multiple runs on the same day add up toward the 1-mile threshold
- Streak counts consecutive days where you've met the goal
- Missing a day resets your current streak (but your longest streak is preserved)

## Examples

### Logging a Run
1. Launch the app (Quick Entry is the default screen)
2. Date and time are already filled with today's values
3. Enter distance: `3.5`
4. Optionally add a note: `Morning run in the park`
5. Press Enter to submit

### Editing a Run
1. Press Esc → 2 to view Run List
2. Use Up/Down arrows to select a run
3. Press `e` to edit
4. Modify any field (date, time, distance, note)
5. Press Enter to save changes
6. You'll return to the Run List with updated data

### Deleting a Run
1. Navigate to Run List (Esc → 2)
2. Select the run to delete with Up/Down arrows
3. Press `d` to delete immediately
4. The list refreshes automatically

### Checking Your Streak
1. Press Esc → 3 to switch to Analytics
2. See your current streak displayed prominently
3. View the 14-day chart to see your daily progress
4. Green days show you met your goal!

## Keyboard Shortcuts Quick Reference

| Key | Action | Context |
|-----|--------|---------|
| Esc → 1/2/3 | Navigate to screens | Any screen |
| h or ? | Help screen | Any screen except Quick Entry |
| e | Edit selected run | Run List |
| d | Delete selected run | Run List |
| Enter | Submit/save | Quick Entry |
| Tab | Next field | Quick Entry |
| Shift+Tab | Previous field | Quick Entry |
| Esc Esc | Clear fields | Quick Entry (press twice) |
| ↑↓ | Navigate list | Run List |
| q | Quit | Most screens |
| Ctrl+Q/C | Quit | Anywhere |

## Tips

- The Quick Entry screen is optimized for speed - just enter distance and press Enter
- Distance must be a positive number (decimals like 3.5 are supported)
- You can log runs retroactively by editing the date field
- Use Esc → number shortcuts to navigate while typing in Quick Entry
- View the Help screen (h or Esc → h) anytime for a quick reference
- The analytics chart uses color coding to quickly show goal achievement

## Development

Built with:
- **Rust**: Fast, safe systems programming language
- **Ratatui**: Powerful TUI framework
- **Crossterm**: Cross-platform terminal backend
- **SQLite**: Reliable embedded database (via rusqlite)
- **Chrono**: Date and time handling

Project structure:
- `src/db/`: Database layer (migrations, queries, connection)
- `src/models/`: Data models (Run, Analytics)
- `src/logic/`: Business logic (streak calculation, validation)
- `src/ui/`: User interface (screens, components, themes)
- `src/app.rs`: Application state management
- `src/main.rs`: Entry point and event loop

### Building from Source

Requirements:
- Rust 1.70+ (2021 edition)
- Cargo

```bash
# Clone the repository
git clone <repository-url>
cd runlogger

# Build
cargo build --release

# Run tests (if any)
cargo test

# Run the application
./target/release/runlogger
```

## License

This is a personal project. Feel free to use and modify as needed.
