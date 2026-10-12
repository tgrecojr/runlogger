use crate::models::analytics::{Analytics, DailyData, MonthlyData, YEAR_GOAL_DAYS};
use crate::models::run::Run;
use chrono::{Datelike, NaiveDate};
use std::collections::BTreeMap;

pub const DAILY_GOAL_MILES: f64 = 1.0;
const MONTHS_TO_SHOW: usize = 12;
const TREND_DAYS: i64 = 14;

/// Computes every analytics figure as of `today`. Runs dated after today are
/// ignored, so all figures describe the same history.
// @spec AN-002, AN-003, AN-004, AN-005, AN-006, AN-007, AN-015
pub fn calculate_analytics(runs: &[Run], today: NaiveDate) -> Analytics {
    let runs: Vec<&Run> = runs.iter().filter(|r| r.date <= today).collect();
    if runs.is_empty() {
        return Analytics::empty();
    }

    let daily_totals = group_by_date(&runs);
    let today_goal_met = daily_totals.get(&today).is_some_and(|&d| goal_met(d));
    let (total_runs, average_distance) = window_stats(&runs, NaiveDate::MIN);
    let total_distance = runs.iter().map(|r| r.distance_miles).sum();

    let year_start = NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap();
    let (runs_this_week, average_distance_this_week) = window_stats(&runs, days_back(today, 6));
    let (runs_this_month, average_distance_this_month) = window_stats(&runs, days_back(today, 29));
    let (runs_this_year, average_distance_this_year) = window_stats(&runs, year_start);

    let goal_days_this_year = daily_totals
        .range(year_start..)
        .filter(|(_, &miles)| goal_met(miles))
        .count() as i32;

    Analytics {
        current_streak: calculate_current_streak(&daily_totals, today, today_goal_met),
        longest_streak: calculate_longest_streak(&daily_totals),
        total_runs,
        total_distance,
        average_distance,
        runs_this_week,
        runs_this_month,
        runs_this_year,
        recent_trend: calculate_recent_trend(&daily_totals, today),
        days_remaining_to_year_goal: YEAR_GOAL_DAYS - goal_days_this_year,
        year_goal_completion_percentage: goal_days_this_year as f64 / YEAR_GOAL_DAYS as f64 * 100.0,
        average_distance_this_week,
        average_distance_this_month,
        average_distance_this_year,
        monthly_breakdown: calculate_monthly_breakdown(&runs, MONTHS_TO_SHOW),
        today_goal_met,
    }
}

/// A distance in whole hundredths of a mile — the precision the screen shows.
pub fn hundredths(miles: f64) -> i64 {
    (miles * 100.0).round() as i64
}

/// A distance as shown on screen: two decimals from the same hundredths
/// `goal_met` uses, so the display and the goal status always agree.
pub fn format_miles(miles: f64) -> String {
    let h = hundredths(miles);
    format!("{}.{:02} mi", h / 100, h % 100)
}

/// Whether a day's total meets the daily goal, judged at displayed precision.
// @spec AN-001
pub fn goal_met(total_miles: f64) -> bool {
    hundredths(total_miles) >= hundredths(DAILY_GOAL_MILES)
}

fn days_back(today: NaiveDate, days: i64) -> NaiveDate {
    today - chrono::Duration::days(days)
}

/// Run count and average distance per run for runs dated on or after `start`.
fn window_stats(runs: &[&Run], start: NaiveDate) -> (u32, f64) {
    let in_window: Vec<f64> = runs
        .iter()
        .filter(|r| r.date >= start)
        .map(|r| r.distance_miles)
        .collect();
    let count = in_window.len() as u32;
    let average = if count > 0 {
        in_window.iter().sum::<f64>() / count as f64
    } else {
        0.0
    };
    (count, average)
}

fn group_by_date(runs: &[&Run]) -> BTreeMap<NaiveDate, f64> {
    let mut daily_totals: BTreeMap<NaiveDate, f64> = BTreeMap::new();

    for run in runs {
        *daily_totals.entry(run.date).or_insert(0.0) += run.distance_miles;
    }

    daily_totals
}

/// Counts back from today when today is goal-met, otherwise from yesterday:
/// an unfinished today can extend the streak but never breaks it.
fn calculate_current_streak(
    daily_totals: &BTreeMap<NaiveDate, f64>,
    today: NaiveDate,
    today_goal_met: bool,
) -> u32 {
    let mut streak = 0;
    let mut current_date = if today_goal_met {
        Some(today)
    } else {
        today.pred_opt()
    };

    while let Some(date) = current_date {
        match daily_totals.get(&date) {
            Some(&miles) if goal_met(miles) => streak += 1,
            _ => break,
        }
        current_date = date.pred_opt();
    }

    streak
}

fn calculate_longest_streak(daily_totals: &BTreeMap<NaiveDate, f64>) -> u32 {
    let mut longest = 0;
    let mut current = 0;
    let mut prev_date: Option<NaiveDate> = None;

    // Oldest to newest over goal-met days only; a gap between them resets the run.
    for (&date, _) in daily_totals.iter().filter(|(_, &miles)| goal_met(miles)) {
        let continues = prev_date.is_some_and(|prev| prev + chrono::Duration::days(1) == date);
        current = if continues { current + 1 } else { 1 };
        longest = longest.max(current);
        prev_date = Some(date);
    }

    longest
}

fn calculate_recent_trend(
    daily_totals: &BTreeMap<NaiveDate, f64>,
    today: NaiveDate,
) -> Vec<DailyData> {
    (0..TREND_DAYS)
        .rev()
        .map(|back| {
            let date = days_back(today, back);
            DailyData {
                date,
                distance: daily_totals.get(&date).copied().unwrap_or(0.0),
            }
        })
        .collect()
}

fn calculate_monthly_breakdown(runs: &[&Run], months: usize) -> Vec<MonthlyData> {
    // Group runs by (year, month)
    let mut monthly_totals: BTreeMap<(i32, u32), (f64, u32)> = BTreeMap::new();

    for run in runs {
        let key = (run.date.year(), run.date.month());
        let entry = monthly_totals.entry(key).or_insert((0.0, 0));
        entry.0 += run.distance_miles;
        entry.1 += 1;
    }

    // Newest month first, limited to `months`
    monthly_totals
        .into_iter()
        .rev()
        .take(months)
        .map(|((year, month), (total_distance, run_count))| MonthlyData {
            year,
            month,
            total_distance,
            run_count,
            average_distance: total_distance / run_count as f64,
        })
        .collect()
}

#[cfg(test)]
mod tests;
