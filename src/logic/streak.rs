use chrono::{Datelike, Local, NaiveDate};
use std::collections::BTreeMap;
use crate::models::run::Run;
use crate::models::analytics::{Analytics, DailyData};

const DAILY_GOAL_MILES: f64 = 1.0;

pub fn calculate_analytics(runs: &[Run]) -> Analytics {
    if runs.is_empty() {
        return Analytics::empty();
    }

    let daily_totals = group_by_date(runs);
    let current_streak = calculate_current_streak(&daily_totals);
    let longest_streak = calculate_longest_streak(&daily_totals);

    let total_runs = runs.len() as u32;
    let total_distance: f64 = runs.iter().map(|r| r.distance_miles).sum();
    let average_distance = if total_runs > 0 {
        total_distance / total_runs as f64
    } else {
        0.0
    };

    let today = Local::now().naive_local().date();
    let week_start = today - chrono::Duration::days(6); // Last 7 days including today
    let month_start = today - chrono::Duration::days(29); // Last 30 days including today
    let year_start = NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap();

    let runs_this_week = runs.iter().filter(|r| r.date >= week_start).count() as u32;
    let runs_this_month = runs.iter().filter(|r| r.date >= month_start).count() as u32;
    let runs_this_year = runs.iter().filter(|r| r.date >= year_start).count() as u32;

    let recent_trend = calculate_recent_trend(&daily_totals, 30);

    Analytics {
        current_streak,
        longest_streak,
        total_runs,
        total_distance,
        average_distance,
        runs_this_week,
        runs_this_month,
        runs_this_year,
        recent_trend,
    }
}

fn group_by_date(runs: &[Run]) -> BTreeMap<NaiveDate, f64> {
    let mut daily_totals: BTreeMap<NaiveDate, f64> = BTreeMap::new();

    for run in runs {
        *daily_totals.entry(run.date).or_insert(0.0) += run.distance_miles;
    }

    daily_totals
}

fn calculate_current_streak(daily_totals: &BTreeMap<NaiveDate, f64>) -> u32 {
    let mut streak = 0;
    let mut current_date = Local::now().naive_local().date();

    loop {
        if let Some(&distance) = daily_totals.get(&current_date) {
            if distance >= DAILY_GOAL_MILES {
                streak += 1;
                if let Some(prev_date) = current_date.pred_opt() {
                    current_date = prev_date;
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    streak
}

fn calculate_longest_streak(daily_totals: &BTreeMap<NaiveDate, f64>) -> u32 {
    if daily_totals.is_empty() {
        return 0;
    }

    let mut longest = 0;
    let mut current = 0;
    let mut prev_date: Option<NaiveDate> = None;

    // Iterate in reverse (newest to oldest)
    for (&date, &distance) in daily_totals.iter().rev() {
        if distance >= DAILY_GOAL_MILES {
            if let Some(prev) = prev_date {
                // Check if current date is exactly one day before the previous date
                if date + chrono::Duration::days(1) == prev {
                    current += 1;
                } else {
                    // Streak broken, save the current streak and start a new one
                    longest = longest.max(current);
                    current = 1;
                }
            } else {
                current = 1;
            }
            prev_date = Some(date);
        }
        // Note: We don't reset on days with < goal, we just skip them
    }

    longest.max(current)
}

fn calculate_recent_trend(daily_totals: &BTreeMap<NaiveDate, f64>, days: i64) -> Vec<DailyData> {
    let today = Local::now().naive_local().date();
    let start_date = today - chrono::Duration::days(days - 1);

    let mut trend = Vec::new();
    let mut current_date = start_date;

    while current_date <= today {
        let distance = daily_totals.get(&current_date).copied().unwrap_or(0.0);

        trend.push(DailyData {
            date: current_date,
            distance,
        });

        current_date = current_date + chrono::Duration::days(1);
    }

    trend
}
