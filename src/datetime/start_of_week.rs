#![allow(dead_code)]

use chrono::{DateTime, Utc, Duration, Weekday, Datelike};

/// Returns the start of the week (Monday) for a given date.
///
/// ```
/// use chrono::{Utc, TimeZone, Weekday, Datelike, Timelike};
/// // Wednesday, December 6, 2023
/// let date = Utc.with_ymd_and_hms(2023, 12, 6, 15, 30, 45).unwrap();
/// let result = reddish::start_of_week(&date);
///
/// assert_eq!(result.weekday(), Weekday::Mon);
/// assert_eq!(result.day(), 4); // Monday, December 4, 2023
/// assert_eq!(result.hour(), 0);
/// assert_eq!(result.minute(), 0);
/// assert_eq!(result.second(), 0);
/// ```
pub fn start_of_week(datetime: &DateTime<Utc>) -> DateTime<Utc> {
    let days_since_monday = match datetime.weekday() {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    let start_of_day = datetime.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let start_of_week = start_of_day - Duration::days(days_since_monday);

    DateTime::from_naive_utc_and_offset(start_of_week, Utc)
}
