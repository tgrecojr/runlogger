use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate, NaiveTime, Timelike};

// @spec QE-008, QE-011
pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    if date_str.is_empty() {
        return Ok(Local::now().naive_local().date());
    }

    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(date_str, "%m/%d/%Y"))
        .or_else(|_| NaiveDate::parse_from_str(date_str, "%m-%d-%Y"))
        .map_err(|_| anyhow!("Invalid date format. Use YYYY-MM-DD"))
}

// @spec QE-009, QE-010, QE-011
pub fn parse_time(time_str: &str) -> Result<NaiveTime> {
    if time_str.is_empty() {
        let now = Local::now().naive_local().time();
        return Ok(now.with_nanosecond(0).unwrap_or(now));
    }

    NaiveTime::parse_from_str(time_str, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%H:%M"))
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%I:%M %p"))
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%I:%M:%S %p"))
        .map_err(|_| anyhow!("Invalid time format. Use HH:MM or HH:MM:SS"))
}

// @spec QE-012
pub fn parse_distance(distance_str: &str) -> Result<f64> {
    if distance_str.is_empty() {
        return Err(anyhow!("Distance is required"));
    }

    distance_str
        .trim()
        .parse::<f64>()
        .map_err(|_| anyhow!("Invalid distance. Enter a number (e.g., 3.5)"))
}

pub fn format_date(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn format_time(time: &NaiveTime) -> String {
    time.format("%H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // @spec QE-010
    #[test]
    fn empty_time_uses_current_clock_in_whole_seconds() {
        let time = parse_time("").unwrap();
        assert_eq!(time.nanosecond(), 0);
    }
}
