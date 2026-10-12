use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

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
        return Ok(current_time());
    }

    NaiveTime::parse_from_str(time_str, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%H:%M"))
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%I:%M %p"))
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%I:%M:%S %p"))
        .map_err(|_| anyhow!("Invalid time format. Use HH:MM or HH:MM:SS"))
}

// @spec QE-024
/// Runs record what has already happened: reject a date and time after `now`.
pub fn check_not_future(date: NaiveDate, time: NaiveTime, now: NaiveDateTime) -> Result<()> {
    if date > now.date() {
        return Err(anyhow!("Date cannot be in the future"));
    }
    if date.and_time(time) > now {
        return Err(anyhow!("Time cannot be in the future"));
    }
    Ok(())
}

pub fn current_time() -> NaiveTime {
    let now = Local::now().naive_local().time();
    now.with_nanosecond(0).unwrap_or(now)
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

    fn at(y: i32, m: u32, d: u32, h: u32, min: u32) -> (NaiveDate, NaiveTime) {
        (
            NaiveDate::from_ymd_opt(y, m, d).unwrap(),
            NaiveTime::from_hms_opt(h, min, 0).unwrap(),
        )
    }

    fn now() -> NaiveDateTime {
        let (d, t) = at(2026, 10, 12, 17, 55);
        d.and_time(t)
    }

    // @spec QE-024
    #[test]
    fn date_after_today_is_rejected() {
        let (d, t) = at(2026, 10, 13, 6, 0);
        let err = check_not_future(d, t, now()).unwrap_err();
        assert_eq!(err.to_string(), "Date cannot be in the future");
    }

    // @spec QE-024
    #[test]
    fn later_time_today_is_rejected() {
        let (d, t) = at(2026, 10, 12, 18, 0);
        let err = check_not_future(d, t, now()).unwrap_err();
        assert_eq!(err.to_string(), "Time cannot be in the future");
    }

    // @spec QE-024
    #[test]
    fn now_and_earlier_are_accepted() {
        let (d, t) = at(2026, 10, 12, 17, 55);
        assert!(check_not_future(d, t, now()).is_ok());
        let (d, t) = at(2026, 10, 11, 23, 59);
        assert!(check_not_future(d, t, now()).is_ok());
    }

    // @spec QE-010
    #[test]
    fn empty_time_uses_current_clock_in_whole_seconds() {
        let time = parse_time("").unwrap();
        assert_eq!(time.nanosecond(), 0);
    }
}
