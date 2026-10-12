use chrono::NaiveDate;

/// The year goal: this many goal-met days per calendar year.
pub const YEAR_GOAL_DAYS: i32 = 365;

#[derive(Debug, Clone)]
pub struct Analytics {
    pub current_streak: u32,
    pub longest_streak: u32,
    pub total_runs: u32,
    pub total_distance: f64,
    pub average_distance: f64,
    pub runs_this_week: u32,
    pub runs_this_month: u32,
    pub runs_this_year: u32,
    pub recent_trend: Vec<DailyData>,
    pub days_remaining_to_year_goal: i32,
    pub year_goal_completion_percentage: f64,
    pub average_distance_this_week: f64,
    pub average_distance_this_month: f64,
    pub average_distance_this_year: f64,
    pub monthly_breakdown: Vec<MonthlyData>,
    /// Whether today's runs already meet the daily goal.
    pub today_goal_met: bool,
}

#[derive(Debug, Clone)]
pub struct DailyData {
    pub date: NaiveDate,
    pub distance: f64,
}

#[derive(Debug, Clone)]
pub struct MonthlyData {
    pub year: i32,
    pub month: u32,
    pub total_distance: f64,
    pub run_count: u32,
    pub average_distance: f64,
}

impl Analytics {
    // @spec AN-013
    pub fn empty() -> Self {
        Self {
            current_streak: 0,
            longest_streak: 0,
            total_runs: 0,
            total_distance: 0.0,
            average_distance: 0.0,
            runs_this_week: 0,
            runs_this_month: 0,
            runs_this_year: 0,
            recent_trend: Vec::new(),
            days_remaining_to_year_goal: YEAR_GOAL_DAYS,
            year_goal_completion_percentage: 0.0,
            average_distance_this_week: 0.0,
            average_distance_this_month: 0.0,
            average_distance_this_year: 0.0,
            monthly_breakdown: Vec::new(),
            today_goal_met: false,
        }
    }
}
