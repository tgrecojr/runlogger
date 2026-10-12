---
parent: high-level-design
prefix: AN
---

# Analytics

## Context and Design Philosophy

Analytics answers the question the app exists for: *am I running at least one mile every day?* It turns the run history into the daily-goal measures — current streak, longest streak, progress toward 365 goal-met days this year — plus supporting volume statistics (totals, rolling-period averages, monthly mileage) and a color-coded view of the last fourteen days.

All computation is a pure function of the run list and today's local date: `calculate_analytics(&[Run]) -> Analytics` in `src/logic/streak.rs`. Nothing is cached or stored; the screen recomputes from the full history each time it opens. The renderer only formats and colors the precomputed `Analytics` value.

Analytics owns: the daily-goal rule, the streak and year-goal arithmetic, the statistics, the `Analytics` data shape, and the Analytics screen. It does not own when the screen opens (Navigation, `NAV`) or how runs are read (Storage, `STORE`).

Code: `src/logic/streak.rs`, `src/models/analytics.rs`, `src/ui/screens/analytics.rs`, `load_analytics` in `src/input/mod.rs`.

## The Daily Goal

`DAILY_GOAL_MILES = 1.0` (`streak.rs:6`). Runs are summed per local calendar date (`group_by_date`, `streak.rs:99-107`), so several short runs on one day combine. A date is **goal-met** when its total is ≥ 1.0 mile, compared with a small tolerance so that floating-point summation does not cost a day its goal: ten 0.1-mile runs sum to `0.9999999999999999` in `f64` and must still count. Every streak, year-goal and chart-color figure uses this one rule.

**Divergence (rounding).** The code compares the raw `f64` sum with `>=` (`streak.rs:75,115,144`; `analytics.rs:217`), so such days currently miss the goal (`AN-001`).

| Measure | Definition (code) |
|---|---|
| Current streak | Count consecutive goal-met dates backward from today when today is goal-met, or from yesterday when today is not yet goal-met. Today is still in progress, so it can extend the streak but never break it. |
| Longest streak | Longest sequence of consecutive goal-met dates anywhere in the history (`streak.rs:133-163`). |
| Year goal | Goal-met dates from Jan 1 of the current year through today; days remaining = 365 − that count; completion = count ÷ 365 × 100 (`streak.rs:72-78`). |

**Divergence (current streak).** The code always starts from today (`streak.rs:111`), so the streak reads 0 each day until that day's mile is logged (`AN-002`).

## Statistics

| Measure | Window | Definition |
|---|---|---|
| Totals | All history | Run count, total miles, average miles per run (`streak.rs:18-24`) |
| Week | Today and the 6 days before, ending today | Run count, average miles per run (`streak.rs:27,31,36-45`) |
| Month | Today and the 29 days before, ending today | Run count, average miles per run (`streak.rs:28,32,47-56`) |
| Year | Jan 1 of the current year through today | Run count, average miles per run (`streak.rs:29,33,58-67`) |
| Recent trend | The 30 dates ending today | Daily total per date, 0 for dates with no run (`streak.rs:165-184`) |
| Monthly breakdown | 12 most recent calendar months *that have runs* | Total miles, run count, average per run, newest first (`streak.rs:186-223`) |

Averages are per run, not per day, and are 0 when a window has no runs. Every window ends today; runs dated after today are excluded. Quick Entry refuses future dates (`QE-024`), but databases written before that rule may still hold some.

**Divergence (window end).** The code filters with only a lower bound (`r.date >= start`, `streak.rs:31-33,38,49,60,75`), so future-dated runs are counted in the Week/Month/Year stats and the year goal (`AN-004`, `AN-006`).

With no runs at all, `Analytics::empty()` (`src/models/analytics.rs:37-56`) supplies zeros, 365 days remaining, and empty trend and monthly lists.

## Screen

`load_analytics` reads the full history and stores the result in `AnalyticsState` when Navigation opens the screen (`3`, Esc→`3`). If the read fails, the Analytics screen shows a single bordered panel in bold red — `Could not load analytics: <reason>` — in place of all four panels, and the app keeps running (`AN-014`). The reason includes its full cause chain on one line. `AnalyticsState` holds the message in `error_message`; the next successful load clears it. No figures are shown from a failed read, since they would describe a history that could not be read.

`src/ui/screens/analytics.rs` lays out four stacked panels (1-cell margin): Streaks, the three statistics panels side by side, Last 14 Days (two header lines plus fourteen day lines), and Mileage by Month taking the remaining height.

```
┌Streaks──────────────────────────────────────────────────────┐
│Current Streak: 12 days          ← green if > 0, else red    │
│Longest Streak: 40 days                                      │
│Year Goal (365 days @ 1mi): 80 days (78.1%)                  │
└─────────────────────────────────────────────────────────────┘   white; yellow < 100 left;
┌Totals─────────┐┌Week──────────┐┌Period─────────┐                 "Goal Complete! 🎉 (100.0%)"
│Total Runs: 300││This Week: 7  ││This Month: 30 │                 in green when ≤ 0 left
│Total Distance ││Avg Distance  ││Avg / This Year│
│Average        ││              ││Avg            │
└───────────────┘└──────────────┘└───────────────┘
┌Last 14 Days─────────────────────────────────────────────────┐
│Daily Mileage (Green = Goal Met ≥1.0 mi)                     │
│09/29   3.10 mi  ███████████████████████████████             │  green ≥ 1.0, yellow > 0,
│10/07   0.00 mi                                              │  red = 0; 10 █ per mile,
└─────────────────────────────────────────────────────────────┘  max 50
┌Mileage by Month─────────────────────────────────────────────┐
│Monthly Totals (Green ≥50 mi, Yellow ≥25 mi)                 │
│Oct 2026    42.3 mi  (14 runs, 3.0 avg)                      │
└─────────────────────────────────────────────────────────────┘
```

The Last 14 Days panel takes the final 14 entries of the 30-date recent trend. With no runs, the chart and monthly panels show "No data to display".

**Divergence (chart length).** The code titles the panel "Last 7 Days", shows 7 entries, and sizes it at 12 rows (`analytics.rs:18,194-201`) (`AN-011`).

## Decisions & Alternatives

| Decision | Chosen | Alternatives Considered | Rationale |
|---|---|---|---|
| Unit of the goal | Sum of all runs on a local calendar date | Single run must reach 1 mile; rolling 24 hours | Several short runs on one day count, and "a day" means the user's own calendar day. |
| Today in the current streak | An unfinished today extends the streak once goal-met but never breaks it | Always count back from today; always count only through yesterday | The streak should not read 0 every morning; it only breaks once a whole day has passed without a mile. |
| Goal comparison precision | Compare daily totals with a small tolerance | Exact `f64` `>=`; store distances as integer hundredths | Floating-point sums of decimal distances must not miss a goal that the user's own arithmetic meets. |
| Year goal target | 365 goal-met days per calendar year | Days elapsed so far this year; 366 in leap years | A fixed, memorable target: a mile every day of the year. |
| Rolling vs calendar periods | Week = last 7 days, Month = last 30 days, Year = calendar year to date, all ending today | Calendar week/month | Rolling windows give comparable numbers on any day instead of resetting on the 1st or on Monday. |
| Chart length | Last 14 days | 7 days; 30 days | Two full weeks show the pattern of the daily habit while still fitting on one screen. |
| Read failures | A red error panel replaces all analytics panels; the app keeps running | Exit the app; show zeros with an error line; a shared status line in the footer | A storage problem must not cost the user the rest of the app; zeros would read as a broken streak. |
| Average basis | Miles per run | Miles per day in the window | Describes a typical run's length, independent of rest days. |
| Where analytics are computed | Pure Rust over the in-memory history, recomputed on each open | SQL aggregation; cached results | Keeps the goal rules in one testable function; the history is small. |
| Monthly view | 12 most recent months that have runs | Last 12 calendar months including empty ones | Shows real activity without rows of zeros. |
| Color thresholds | Day: ≥1.0 green / >0 yellow / 0 red; month: ≥50 green / ≥25 yellow | Gradient; no color | Goal status is readable at a glance. |

## Open Questions & Future Decisions

### Deferred
1. **Duplicated thresholds.** The chart re-types the 1.0-mile goal (`analytics.rs:217`) instead of using `DAILY_GOAL_MILES`; 365 appears in `streak.rs:77-78`, `models/analytics.rs:49` and the panel label (`analytics.rs:79`).
2. **Stale "today".** Analytics are computed when the screen opens; leaving it open past midnight shows yesterday's view until reopened.
3. **Labels.** "This Week"/"This Month" read as calendar periods but are rolling 7/30 days; the Week panel's average is labeled "Avg Distance" while the Period panel uses "Avg".
4. **Unused trend length.** The model computes 30 days of daily totals but the screen uses 14.
5. **Small terminals.** A 14-line chart pushes Mileage by Month further down; on short terminals it may be cut off.

## References

- Code: `src/logic/streak.rs`, `src/models/analytics.rs`, `src/ui/screens/analytics.rs`, `src/input/mod.rs` (`load_analytics`), `src/ui/theme.rs` (goal styles)
- Specs: `docs/intent/analytics/analytics-specs.md`
- Tests: `src/input/tests.rs`, `src/ui/tests.rs`
- Depends on: Storage (`STORE`) for the history read; Navigation (`NAV`) for opening the screen and error handling
- External: `chrono` 0.4, `ratatui` 0.30
