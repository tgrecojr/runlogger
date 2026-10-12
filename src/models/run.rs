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
    // @spec QE-013
    pub fn new(
        date: NaiveDate,
        time: NaiveTime,
        distance: f64,
        note: Option<String>,
    ) -> Result<Self> {
        if !distance.is_finite() {
            return Err(anyhow!("Invalid distance. Enter a number (e.g., 3.5)"));
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    // @spec QE-013
    #[test]
    fn non_finite_distance_is_rejected() {
        let date = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
        let time = NaiveTime::from_hms_opt(7, 0, 0).unwrap();
        for distance in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let err = Run::new(date, time, distance, None).unwrap_err();
            assert_eq!(
                err.to_string(),
                "Invalid distance. Enter a number (e.g., 3.5)"
            );
        }
    }
}
