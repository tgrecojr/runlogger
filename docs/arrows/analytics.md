# Arrow: analytics

The daily-goal measures (current/longest streak, year goal), volume statistics, and the Analytics screen.

## Status

**MAPPED** — mapped 2026-10-12; not yet audited. All 16 specs implemented.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/analytics/analytics-design.md

### EARS
- docs/intent/analytics/analytics-specs.md (16 specs)

### Tests
- src/logic/streak/tests.rs (AN-001 to AN-006, AN-011, AN-013, AN-015)
- src/input/tests.rs (AN-014)
- src/ui/tests.rs (AN-011 to AN-014, AN-016)

### Code
- src/logic/streak.rs (`calculate_analytics`, `goal_met`, `hundredths`)
- src/models/analytics.rs (`Analytics`, `YEAR_GOAL_DAYS`)
- src/ui/screens/analytics/ (`mod.rs`, `streaks.rs`, `stats.rs`, `history.rs`)
- src/input/mod.rs (`load_analytics`)

## Architecture

**Purpose:** Show whether the user is meeting the 1-mile-per-day goal and how much they have run.

**Key Components:**
1. `calculate_analytics(runs, today)` — pure function: drops runs after today, then computes every figure
2. `goal_met` — the daily-goal rule at displayed (hundredths) precision, used by every streak, year-goal and chart-color decision
3. `models/analytics.rs` — `Analytics` (including `today_goal_met`), `DailyData`, `MonthlyData`, `Analytics::empty()`
4. `screens/analytics/` — layout and error panel (`mod.rs`), Streaks with the run-today hint, statistics, and the 14-day chart beside the 46-column monthly panel (`history.rs`)

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps |
|----------|----------|-------------|----------|------|
| Daily goal | AN-001 to AN-004 | 4 | 0 | 0 |
| Statistics | AN-005 to AN-007, AN-015 | 4 | 0 | 0 |
| Screen | AN-008 to AN-014, AN-016 | 8 | 0 | 0 |

**Summary:** 16 of 16 specs implemented. Tests cite every spec except AN-007 to AN-010.

## Key Findings

1. **Stale "today"** — analytics are computed when the screen opens; leaving it open past midnight shows yesterday's view until reopened.
2. **Labels** — "This Week"/"This Month" are rolling 7/30-day windows; the Week panel says "Avg Distance" where Period says "Avg".

## Work Required

### Should Fix
1. Add tests for AN-007 to AN-010.

### Nice to Have
2. Clarify rolling-period labels.
