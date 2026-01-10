use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};

#[derive(Debug, Clone)]
pub struct Run {
    #[allow(dead_code)]
    pub id: Option<i64>,
    pub date: NaiveDate,
    pub time_started: NaiveTime,
    pub distance_miles: f64,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Run {
    pub fn new(
        date: NaiveDate,
        time: NaiveTime,
        distance: f64,
        note: Option<String>,
    ) -> Result<Self> {
        if distance <= 0.0 {
            return Err(anyhow!("Distance must be positive"));
        }

        if distance > 200.0 {
            return Err(anyhow!("Distance seems unrealistic (>200 miles)"));
        }

        Ok(Self {
            id: None,
            date,
            time_started: time,
            distance_miles: distance,
            note,
            created_at: Utc::now(),
        })
    }
}
