# Arrow: analytics

The daily-goal measures (current/longest streak, year goal), volume statistics, and the Analytics screen.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. Read failures now show an error panel. Five gaps: rounding at the goal threshold (AN-001), current streak broken by an unfinished today (AN-002), windows not ending today (AN-004, AN-006), 7-day instead of 14-day chart (AN-011).

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/analytics/analytics-design.md

### EARS
- docs/intent/analytics/analytics-specs.md (14 specs)

### Tests
- src/input/tests.rs (AN-014)
- src/ui/tests.rs (AN-013, AN-014)

### Code
- src/logic/streak.rs
- src/models/analytics.rs
- src/ui/screens/analytics.rs
- src/input/mod.rs (`load_analytics`)

## Architecture

**Purpose:** Show whether the user is meeting the 1-mile-per-day goal and how much they have run.

**Key Components:**
1. `calculate_analytics` — pure function from `&[Run]` and today's date to `Analytics`
2. `models/analytics.rs` — `Analytics`, `DailyData`, `MonthlyData`, `Analytics::empty()`
3. `screens/analytics.rs` — Streaks, Totals/Week/Period, Last 7 Days (intended 14), Mileage by Month panels

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps |
|----------|----------|-------------|----------|------|
| Daily goal | AN-001 to AN-004 | 1 | 0 | 3 |
| Statistics | AN-005 to AN-007 | 2 | 0 | 1 |
| Screen | AN-008 to AN-014 | 6 | 0 | 1 |

**Summary:** 9 of 14 specs implemented; 5 gaps (AN-001, AN-002, AN-004, AN-006, AN-011). Tests cite AN-013 and AN-014.

## Key Findings

1. **Streak reads 0 until today's mile** — `calculate_current_streak` starts at today (`src/logic/streak.rs:111`) (AN-002).
2. **Chart length drift** — README promises 14 days; screen shows 7 (`src/ui/screens/analytics.rs:194-201`); model computes 30 (`src/logic/streak.rs:69`) (AN-011).
3. **Float sums at the threshold** — ten 0.1-mile runs total `0.9999999999999999` and miss the goal (`src/logic/streak.rs:103,115`) (AN-001).
4. **Unbounded windows** — period and year-goal filters have no upper bound (`src/logic/streak.rs:31-33,75`) (AN-004, AN-006).
5. **Duplicated constants** — goal literal `1.0` at `src/ui/screens/analytics.rs:217`; 365 at `src/logic/streak.rs:77-78`, `src/models/analytics.rs:49`, `src/ui/screens/analytics.rs:79`.

## Work Required

### Must Fix
1. Count the current streak from yesterday while today is unfinished (AN-002).
2. Compare daily totals to the goal with a rounding tolerance (AN-001).
3. Show 14 days in the chart and resize its panel (AN-011).
4. End all period windows and the year goal at today (AN-004, AN-006).

### Should Fix
5. Use `DAILY_GOAL_MILES` and a single year-goal constant everywhere.
6. Add tests for AN-001 through AN-007 (`calculate_analytics` is pure; inject today's date to make it testable).

### Nice to Have
7. Clarify rolling-period labels.
