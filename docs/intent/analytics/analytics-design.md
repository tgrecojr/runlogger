---
parent: high-level-design
prefix: AN
---

# Analytics

## Context and Design Philosophy

Analytics answers the question the app exists for: *am I running at least one mile every day?* It turns the run history into the daily-goal measures — current streak, longest streak, progress toward 365 goal-met days this year — plus supporting volume statistics (totals, rolling-period averages, monthly mileage) and a color-coded view of the last fourteen days.

All computation is a pure function of the run list and today's date: `calculate_analytics(runs, today) -> Analytics` in `src/logic/streak.rs`. The caller (`load_analytics`) passes today's local date, so tests can fix "today" to any date. Nothing is cached or stored; the screen recomputes from the full history each time it opens. The renderer only formats and colors the precomputed `Analytics` value.

Analytics owns: the daily-goal rule, the streak and year-goal arithmetic, the statistics, the `Analytics` data shape, and the Analytics screen. It does not own when the screen opens (Navigation, `NAV`) or how runs are read (Storage, `STORE`).

Code: `src/logic/streak.rs`, `src/models/analytics.rs`, `src/ui/screens/analytics/`, `load_analytics` in `src/input/mod.rs`.

## Which Runs Count

Every analytics figure is computed from the runs dated on or before today; runs dated after today are dropped once, at the start of `calculate_analytics` (`AN-015`). Quick Entry refuses future runs (`QE-024`), but databases written before that rule may still hold some, and a single filter keeps streaks, totals, windows, the chart and the monthly breakdown describing the same history. A history whose only runs are after today is treated as empty: zeros, 365 days remaining, and "No data to display" (`AN-013`). Run List still lists such runs, so the user can find and correct or delete them; saving a correction must move the run to the past (`QE-024`).

## The Daily Goal

Runs are summed per local calendar date, so several short runs on one day combine. A date is **goal-met** when its total, **rounded to the nearest hundredth of a mile**, is at least 1.00 (`AN-001`). Hundredths are the precision the screen displays, so a day shown as "1.00 mi" always counts and a day shown as "0.99 mi" never does; the rounding also absorbs floating-point summation error (ten 0.1-mile runs sum to `0.9999999999999999` in `f64`). One function, `goal_met`, applies the rule, and every streak, year-goal and chart-color decision calls it. The goal (`DAILY_GOAL_MILES`) and the year target (`YEAR_GOAL_DAYS`) are each defined once.

| Measure | Definition |
|---|---|
| Current streak | Count consecutive goal-met dates backward from today when today is goal-met, or from yesterday when today is not yet goal-met. Today is still in progress, so it can extend the streak but never break it (`AN-002`). `Analytics::today_goal_met` records which case applies. |
| Longest streak | Longest sequence of consecutive goal-met dates in the history (`AN-003`). |
| Year goal | Goal-met dates from Jan 1 of the current year through today; days remaining = 365 − that count; completion = count ÷ 365 × 100 (`AN-004`). |

## Statistics

| Measure | Window | Definition |
|---|---|---|
| Totals | All counted runs | Run count, total miles, average miles per run |
| Last 7 Days | Today and the 6 days before | Run count, average miles per run |
| Last 30 Days | Today and the 29 days before | Run count, average miles per run |
| This Year | Jan 1 of the current year through today | Run count, average miles per run |
| Recent trend | The 14 dates ending today | Daily total per date, 0 for dates with no run |
| Monthly breakdown | 12 most recent calendar months *that have runs* | Total miles, run count, average per run, newest first |

Averages are per run, not per day, and are 0 when a window has no runs. With no runs at all, `Analytics::empty()` supplies zeros, 365 days remaining, and empty trend and monthly lists.

## Screen

`load_analytics` reads the full history and stores the result in `AnalyticsState` — with the date it was computed for — when Navigation opens the screen (`3`, Esc→`3`). While the screen stays open, the event loop compares that date with today before each frame and recomputes when the day changes, so streaks and "today" stay current past midnight (`AN-008`). The statistics row's panels are titled **Totals**, **Last 7 Days** and **Last 30 Days / Year**, each row reading `Runs: N` / `Avg: X.XX mi` (with "Last 30 Days" / "This Year" sub-labels in the third panel), so the rolling windows are not mistaken for calendar periods (`AN-010`). If the read fails, the Analytics screen shows a single bordered panel in bold red — `Could not load analytics: <reason>` — in place of all panels, and the app keeps running (`AN-014`). The reason includes its full cause chain on one line. `AnalyticsState` holds the message in `error_message`; the next successful load clears it. No figures are shown from a failed read, since they would describe a history that could not be read.

The screen (`src/ui/screens/analytics/`, 1-cell margin) stacks three rows: Streaks (6 rows), the three statistics panels side by side (8 rows), and a bottom row taking the remaining height in which **Last 14 Days** and **Mileage by Month** sit side by side: Mileage by Month at a fixed 46 columns (wide enough for its header line, "Monthly Totals (Green ≥50 mi, Yellow ≥25 mi)", plus borders) and the chart taking the remaining width. Placing the chart beside the monthly panel keeps both visible on short terminals; on narrow terminals the longest bars are clipped at the chart panel's edge.

```
┌Streaks──────────────────────────────────────────────────────────────────┐
│Current Streak: 12 days (run today to extend)  ← streak bold green if > 0,│
│                                  else red; hint yellow, AN-016          │
│Longest Streak: 40 days                                                  │
│Year Goal (365 days @ 1mi): 80 days (78.1%)  ← bold yellow < 100 left;        │
└─────────────────────────────────────────────    "Goal Complete! 🎉 (100.0%)" when none left
┌Totals─────────────┐┌Week──────────────┐┌Period──────────────┐
│Total Runs: 300    ││This Week: 7 runs ││This Month: 30 runs │
│Total Distance     ││Avg Distance      ││Avg / This Year / Avg│
└───────────────────┘└──────────────────┘└────────────────────┘
┌Last 14 Days───────────────────────────────┐┌Mileage by Month──────────────┐
│Daily Mileage (Green = Goal Met ≥1.0 mi)   ││Monthly Totals (Green ≥50 mi, │
│                                           ││                              │
│09/29   3.10 mi  ███████████████████████   ││Oct 2026    42.3 mi  (14 runs │
│09/30   0.00 mi                            ││Sep 2026    51.0 mi  (30 runs │
│ … 14 day lines                            ││ …                            │
└───────────────────────────────────────────┘└──────────────────────────────┘
```

Day lines: date `MM/DD`, the day's total to two decimals — formatted from the same whole-hundredths integer that `goal_met` compares, so the shown value and the goal status cannot disagree — and a bar of 10 `█` per mile (max 50), computed from the rounded total so a day shown as "1.00 mi" gets exactly 10. When the panel is too short for all fourteen day lines, the oldest days are dropped so today is always on screen. Color follows the rounded total: goal-met bold green, above 0.00 yellow, otherwise red (`AN-011`). Month lines: the total is formatted from whole tenths of a mile and colored on that same value — bold green at 50.0 or more, yellow at 25.0 or more — so a month shown as "50.0 mi" is always green (`AN-012`). With no runs, both bottom panels show "No data to display" (`AN-013`).

The module is split by panel so each file stays within the project's 300-line limit: `mod.rs` (layout and error panel), `streaks.rs`, `stats.rs`, and `history.rs` (the 14-day chart and monthly breakdown).

## Decisions & Alternatives

| Decision | Chosen | Alternatives Considered | Rationale |
|---|---|---|---|
| Unit of the goal | Sum of all runs on a local calendar date | Single run must reach 1 mile; rolling 24 hours | Several short runs on one day count, and "a day" means the user's own calendar day. |
| Today in the current streak | An unfinished today extends the streak once goal-met but never breaks it | Always count back from today; always count only through yesterday | The streak should not read 0 every morning; it only breaks once a whole day has passed without a mile. |
| Goal comparison precision | Round each daily total to hundredths, then compare with 1.00 | Exact `f64` `>=`; a tiny epsilon; store distances as integer hundredths | Matches the two-decimal display, so what the user sees always agrees with goal status; absorbs floating-point error; leans generous per the HLD tenet. |
| Displayed distance on the chart | Format from the integer hundredths that `goal_met` uses | Rust's `{:.2}` | `{:.2}` rounds the exact binary value, so a 0.995 total displays "0.99" while ×100-and-round gives 100 and counts as goal-met; formatting from one integer keeps screen and goal status in step. |
| Runs after today | Dropped once before any figure is computed | Exclude only from period windows | Every figure describes the same history; legacy future-dated rows cannot inflate streaks or totals. |
| "Today" as a parameter | `calculate_analytics(runs, today)` | Read the clock inside | Date-dependent rules (streak through yesterday, windows ending today) are testable at a fixed date. |
| Year goal target | 365 goal-met days per calendar year | Days elapsed so far this year; 366 in leap years | A fixed, memorable target: a mile every day of the year. |
| Rolling vs calendar periods | Week = last 7 days, Month = last 30 days, Year = calendar year to date, all ending today | Calendar week/month | Rolling windows give comparable numbers on any day instead of resetting on the 1st or on Monday. |
| Chart length | Last 14 days | 7 days; 30 days | Two full weeks show the pattern of the daily habit. |
| Chart placement | Beside Mileage by Month in the bottom row | Stacked, with an 18-row chart panel | Keeps the monthly breakdown visible on short terminals; clipped bars on narrow terminals are the lesser cost. |
| Morning streak | Show the streak through yesterday, with a yellow "(run today to extend)" while today is not yet goal-met | Show the generous count with no hint; show 0 until today is met | Keeps the count generous while being honest that today's mile is still owed. |
| Read failures | A red error panel replaces all analytics panels; the app keeps running | Exit the app; show zeros with an error line; a shared status line in the footer | A storage problem must not cost the user the rest of the app; zeros would read as a broken streak. |
| Average basis | Miles per run | Miles per day in the window | Describes a typical run's length, independent of rest days. |
| Where analytics are computed | Pure Rust over the in-memory history, recomputed on each open | SQL aggregation; cached results | Keeps the goal rules in one testable function; the history is small. |
| Monthly view | 12 most recent months that have runs | Last 12 calendar months including empty ones | Shows real activity without rows of zeros. |
| Color thresholds | Day: goal-met green / above 0.00 yellow / otherwise red; month: ≥50 green / ≥25 yellow | Gradient; no color | Goal status is readable at a glance. |

## Open Questions & Future Decisions

### Deferred
None.

## References

- Code: `src/logic/streak.rs`, `src/models/analytics.rs`, `src/ui/screens/analytics/`, `src/input/mod.rs` (`load_analytics`), `src/ui/theme.rs` (goal styles)
- Specs: `docs/intent/analytics/analytics-specs.md`
- Tests: listed in `docs/arrows/analytics.md`
- Depends on: Storage (`STORE`) for the history read; Navigation (`NAV`) for opening the screen and error handling
- External: `chrono` 0.4, `ratatui` 0.30
