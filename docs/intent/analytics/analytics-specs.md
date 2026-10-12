# Analytics Specs

Design: `docs/intent/analytics/analytics-design.md` · Prefix: `AN`

Status: `[x]` = implemented; `[ ]` = specified but not yet implemented; `[D]` = deferred. The tests citing each spec are listed in `docs/arrows/analytics.md`, regenerated from `@spec` scans.

## Daily Goal

- [x] **AN-001**: The system shall total the distance of all counted runs (runs dated on or before today, AN-015) on each local calendar date, and shall treat a date as goal-met when that total, rounded to the nearest hundredth of a mile, is at least 1.00 mile.
- [x] **AN-002**: The system shall compute the current streak as the number of consecutive goal-met dates ending today when today is goal-met, or ending yesterday when today is not yet goal-met, so that an unfinished today never breaks the streak.
- [x] **AN-003**: The system shall compute the longest streak as the longest sequence of consecutive goal-met dates among the counted runs (AN-015).
- [x] **AN-004**: The system shall compute year-goal progress as the number of goal-met dates from January 1 of the current year through today, reporting days remaining as 365 minus that number and completion as that number divided by 365, as a percentage.

## Statistics

- [x] **AN-005**: The system shall compute, over all counted runs (AN-015), the total number of runs, the total distance, and the average distance per run.
- [x] **AN-006**: The system shall compute the run count and average distance per run for each of three windows ending today — the last 7 days including today, the last 30 days including today, and the current calendar year from January 1 through today — reporting an average of 0 for a window with no runs.
- [x] **AN-007**: The system shall compute a monthly breakdown of total distance, run count and average distance per run for the 12 most recent calendar months that contain runs, ordered newest first.
- [x] **AN-015**: The system shall compute every analytics figure — streaks, year goal, totals, period statistics, the 14-day trend and the monthly breakdown — only from runs dated on or before today, ignoring any run dated after today.

## Screen

- [x] **AN-008**: When the user opens the Analytics screen with `3` or Esc → `3`, the system shall recompute all analytics from the full run history as of today's local date, and shall recompute them again whenever the local date changes while the Analytics screen is shown.
- [x] **AN-009**: The Analytics screen shall show a Streaks panel with the current streak in days (bold green when above 0, red otherwise), the longest streak in days, and the year goal as "N days (P%)" remaining — in bold yellow when fewer than 100 days remain, and as "Goal Complete! 🎉 (100.0%)" in bold green when none remain.
- [x] **AN-016**: While the current streak is above 0 and today is not yet goal-met, the Streaks panel shall show "(run today to extend)" in yellow after the current streak.
- [x] **AN-010**: The Analytics screen shall show a Totals panel (total runs, total distance and average per run), a Last 7 Days panel (runs and average for the last 7 days), and a Last 30 Days / Year panel (runs and average for the last 30 days and for the current calendar year), with distances to two decimal places.
- [x] **AN-011**: The Analytics screen shall show a Last 14 Days panel, beside the Mileage by Month panel, with one line per date for the fourteen dates ending today, giving the date as `MM/DD`, the day's total distance to two decimals rounded exactly as the goal rule rounds it (AN-001) so that the shown distance and the goal status always agree, and a bar of 10 `█` characters per mile capped at 50, colored bold green when the day is goal-met, yellow when its total rounded to hundredths is above 0.00 but not goal-met, and red otherwise; if the panel is too short for all fourteen day lines, the system shall omit the oldest dates so that today is always shown.
- [x] **AN-012**: The Analytics screen shall show a Mileage by Month panel, 46 columns wide beside the Last 14 Days panel, with one line per month of the monthly breakdown giving month and year, total distance to one decimal, run count and average per run, with the total colored bold green when its displayed (one-decimal) value is 50.0 or more and yellow when it is 25.0 or more.
- [x] **AN-013**: While the run history has loaded successfully and has no counted runs (AN-015), the Analytics screen shall show zero streaks and statistics, 365 days remaining for the year goal, and "No data to display" in the Last 14 Days and Mileage by Month panels.
- [x] **AN-014**: If reading the run history fails when the Analytics screen is opened, then the system shall show a single bold red panel reading "Could not load analytics: " followed by the reason including its underlying cause in place of all analytics panels, until the next successful load, and shall keep the app running.
