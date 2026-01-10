use chrono::NaiveDate;

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
}

#[derive(Debug, Clone)]
pub struct DailyData {
    pub date: NaiveDate,
    pub distance: f64,
}

impl Analytics {
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
        }
    }
}
