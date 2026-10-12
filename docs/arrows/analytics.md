# Arrow: analytics

The daily-goal measures (current/longest streak, year goal), volume statistics, and the Analytics screen.

## Status

**AUDITED** — audited 2026-10-12 (git SHA `88de5b9`); since then AN-011 (display rounding) was changed and AN-009 reworded. All 16 specs implemented and tested.

## References

### HLD
- docs/high-level-design.md

### LLD
- docs/intent/analytics/analytics-design.md

### EARS
- docs/intent/analytics/analytics-specs.md (16 specs)

### Tests
- src/input/tests/errors.rs (AN-014)
- src/logic/streak/tests.rs (AN-001, AN-002, AN-003, AN-004, AN-005, AN-006, AN-007, AN-011, AN-013, AN-015)
- src/ui/screens/analytics/tests.rs (AN-008, AN-009, AN-010)
- src/ui/tests/analytics.rs (AN-001, AN-011, AN-012, AN-013, AN-014, AN-016)

### Code
- src/input/mod.rs (AN-008, AN-014)
- src/logic/streak.rs (AN-001, AN-002, AN-003, AN-004, AN-005, AN-006, AN-007, AN-015)
- src/models/analytics.rs (AN-013)
- src/ui/screens/analytics/history.rs (AN-011, AN-012, AN-013)
- src/ui/screens/analytics/mod.rs (AN-011, AN-012, AN-014)
- src/ui/screens/analytics/stats.rs (AN-010)
- src/ui/screens/analytics/streaks.rs (AN-009, AN-016)

## Architecture

**Purpose:** Show whether the user is meeting the 1-mile-per-day goal and how much they have run.

**Key Components:**
1. `calculate_analytics(runs, today)` — pure function: drops runs after today, then computes every figure
2. `goal_met` — the daily-goal rule at displayed (hundredths) precision, used by every streak, year-goal and chart-color decision
3. `models/analytics.rs` — `Analytics` (including `today_goal_met`), `DailyData`, `MonthlyData`, `Analytics::empty()`
4. `screens/analytics/` — layout and error panel (`mod.rs`), Streaks with the run-today hint, statistics, and the 14-day chart beside the 46-column monthly panel (`history.rs`)

## Spec Coverage

| Category | Spec IDs | Implemented | Deferred | Gaps | Tested |
|----------|----------|-------------|----------|------|--------|
| Daily Goal | AN-001, AN-002, AN-003, AN-004 | 4 | 0 | 0 | 4 |
| Statistics | AN-005, AN-006, AN-007, AN-015 | 4 | 0 | 0 | 4 |
| Screen | AN-008, AN-009, AN-016, AN-010, AN-011, AN-012, AN-013, AN-014 | 8 | 0 | 0 | 8 |

**Summary:** 16 of 16 specs marked implemented; 16 of 16 cited by a test. Untested: none.

## Key Findings

1. **Tiny panels** — when the chart panel is under 5 rows tall, no day lines show, not even today.
2. **Monthly thresholds use the raw total** — 49.96 mi displays as "50.0 mi" in yellow.
3. **Stale "today"** when the screen stays open past midnight; rolling-period labels read as calendar periods.

## Work Required

### Nice to Have
1. Judge monthly colors on the displayed (one-decimal) total; clarify rolling-period labels.
