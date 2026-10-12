use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

// @spec QE-008, QE-011
pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    let date_str = date_str.trim();
    if date_str.is_empty() {
        return Ok(Local::now().naive_local().date());
    }

    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(date_str, "%m/%d/%Y"))
        .or_else(|_| NaiveDate::parse_from_str(date_str, "%m-%d-%Y"))
        .map_err(|_| anyhow!("Invalid date. Use YYYY-MM-DD, MM/DD/YYYY or MM-DD-YYYY"))
}

// @spec QE-009, QE-010, QE-011
pub fn parse_time(time_str: &str) -> Result<NaiveTime> {
    let time_str = time_str.trim();
    if time_str.is_empty() {
        return Ok(current_time());
    }

    NaiveTime::parse_from_str(time_str, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%H:%M"))
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%I:%M %p"))
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%I:%M:%S %p"))
        .map_err(|_| anyhow!("Invalid time. Use HH:MM, HH:MM:SS or h:MM AM/PM"))
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
    let distance_str = distance_str.trim();
    if distance_str.is_empty() {
        return Err(anyhow!("Distance is required"));
    }

    // f64 parsing accepts "NaN" and "inf"; neither is a distance.
    distance_str
        .parse::<f64>()
        .ok()
        .filter(|d| d.is_finite())
        .ok_or_else(|| anyhow!("Invalid distance. Enter a number (e.g., 3.5)"))
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

    // @spec QE-012
    #[test]
    fn non_finite_distances_are_invalid() {
        for input in ["NaN", "nan", " NaN ", "inf", "-inf", "infinity"] {
            let err = parse_distance(input).unwrap_err();
            assert_eq!(
                err.to_string(),
                "Invalid distance. Enter a number (e.g., 3.5)",
                "{input:?}"
            );
        }
    }

    // @spec QE-010
    #[test]
    fn empty_time_uses_current_clock_in_whole_seconds() {
        let time = parse_time("").unwrap();
        assert_eq!(time.nanosecond(), 0);
    }

    fn ymd(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn hms(h: u32, m: u32, s: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, s).unwrap()
    }

    // @spec QE-008
    #[test]
    fn date_accepts_iso_slash_and_dash_forms() {
        for input in ["2026-03-07", "03/07/2026", "03-07-2026"] {
            assert_eq!(parse_date(input).unwrap(), ymd(2026, 3, 7), "{input:?}");
        }
    }

    // @spec QE-008
    #[test]
    fn empty_date_uses_today() {
        let before = Local::now().date_naive();
        let parsed = parse_date("").unwrap();
        let after = Local::now().date_naive();
        assert!(parsed == before || parsed == after, "{parsed}");
    }

    // @spec QE-009
    #[test]
    fn time_accepts_24_hour_and_am_pm_forms() {
        let cases = [
            ("07:05:09", hms(7, 5, 9)),
            ("19:05", hms(19, 5, 0)),
            ("7:05 AM", hms(7, 5, 0)),
            ("7:05 PM", hms(19, 5, 0)),
            ("12:30 AM", hms(0, 30, 0)),
            ("7:05:09 AM", hms(7, 5, 9)),
            ("11:05:09 PM", hms(23, 5, 9)),
        ];
        for (input, expected) in cases {
            assert_eq!(parse_time(input).unwrap(), expected, "{input:?}");
        }
    }

    // @spec QE-011
    #[test]
    fn unrecognized_date_is_rejected_with_format_hint() {
        for input in ["2026/03/07", "March 7", "2026-13-01", "abc", "07.03.2026"] {
            let err = parse_date(input).unwrap_err();
            assert_eq!(
                err.to_string(),
                "Invalid date. Use YYYY-MM-DD, MM/DD/YYYY or MM-DD-YYYY",
                "{input:?}"
            );
        }
    }

    // @spec QE-011
    #[test]
    fn unrecognized_time_is_rejected_with_format_hint() {
        for input in ["25:00", "7pm", "abc", "07-05", "13:05 PM"] {
            let err = parse_time(input).unwrap_err();
            assert_eq!(
                err.to_string(),
                "Invalid time. Use HH:MM, HH:MM:SS or h:MM AM/PM",
                "{input:?}"
            );
        }
    }

    // @spec QE-008, QE-009
    #[test]
    fn spaces_around_date_and_time_are_ignored() {
        assert_eq!(
            parse_date("  2026-10-01 ").unwrap(),
            NaiveDate::from_ymd_opt(2026, 10, 1).unwrap()
        );
        assert_eq!(
            parse_time(" 7:15 AM  ").unwrap(),
            NaiveTime::from_hms_opt(7, 15, 0).unwrap()
        );
    }

    // @spec QE-012
    #[test]
    fn distance_of_only_spaces_is_required() {
        assert_eq!(
            parse_distance("   ").unwrap_err().to_string(),
            "Distance is required"
        );
    }
}
