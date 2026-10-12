# Analytics Specs

Design: `docs/intent/analytics/analytics-design.md` · Prefix: `AN`

Status: `[x]` = implemented; `[ ]` = specified but not yet implemented; `[D]` = deferred. Tests: `src/input/tests.rs` (AN-014), `src/ui/tests.rs` (AN-013, AN-014).

## Daily Goal

- [ ] **AN-001**: The system shall total the distance of all runs on each local calendar date, and shall treat a date as goal-met when that total is at least 1.0 mile, allowing for floating-point rounding so that distances summing to exactly 1.0 in decimal (such as ten 0.1-mile runs) meet the goal.
- [ ] **AN-002**: The system shall compute the current streak as the number of consecutive goal-met dates ending today when today is goal-met, or ending yesterday when today is not yet goal-met, so that an unfinished today never breaks the streak.
- [x] **AN-003**: The system shall compute the longest streak as the longest sequence of consecutive goal-met dates anywhere in the run history.
- [ ] **AN-004**: The system shall compute year-goal progress as the number of goal-met dates from January 1 of the current year through today, reporting days remaining as 365 minus that number and completion as that number divided by 365, as a percentage.

## Statistics

- [x] **AN-005**: The system shall compute, over the full run history, the total number of runs, the total distance, and the average distance per run.
- [ ] **AN-006**: The system shall compute the run count and average distance per run for each of three windows ending today — the last 7 days including today, the last 30 days including today, and the current calendar year from January 1 through today — excluding any run dated after today, and reporting an average of 0 for a window with no runs.
- [x] **AN-007**: The system shall compute a monthly breakdown of total distance, run count and average distance per run for the 12 most recent calendar months that contain runs, ordered newest first.

## Screen

- [x] **AN-008**: When the user opens the Analytics screen with `3` or Esc → `3`, the system shall recompute all analytics from the full run history as of today's local date.
- [x] **AN-009**: The Analytics screen shall show a Streaks panel with the current streak in days (bold green when above 0, red otherwise), the longest streak in days, and the year goal as "N days (P%)" remaining — in yellow when fewer than 100 days remain, and as "Goal Complete! 🎉 (100.0%)" in bold green when none remain.
- [x] **AN-010**: The Analytics screen shall show a Totals panel (total runs, total distance and average per run), a Week panel (runs and average for the last 7 days), and a Period panel (runs and average for the last 30 days and for the current year), with distances to two decimal places.
- [ ] **AN-011**: The Analytics screen shall show a Last 14 Days panel with one line per date for the fourteen dates ending today, giving the date as `MM/DD`, the day's total distance to two decimals, and a bar of 10 `█` characters per mile capped at 50, colored bold green when the day is goal-met, yellow when it has a distance below the goal, and red when it has no distance.
- [x] **AN-012**: The Analytics screen shall show a Mileage by Month panel with one line per month of the monthly breakdown giving month and year, total distance to one decimal, run count and average per run, with the total colored bold green at 50 miles or more and yellow at 25 miles or more.
- [x] **AN-013**: While the run history has loaded successfully and is empty, the Analytics screen shall show zero streaks and statistics, 365 days remaining for the year goal, and "No data to display" in the Last 14 Days and Mileage by Month panels.
- [x] **AN-014**: If reading the run history fails when the Analytics screen is opened, then the system shall show a single bold red panel reading "Could not load analytics: " followed by the reason including its underlying cause in place of all analytics panels, until the next successful load, and shall keep the app running.
