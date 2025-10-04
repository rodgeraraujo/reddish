#![allow(dead_code)]

use chrono::{DateTime, Utc, Duration};

/// Adds a specified number of days to a date.
/// Can add negative days to subtract.
///
/// ```
/// use chrono::{Utc, TimeZone, Datelike};
/// let date = Utc.with_ymd_and_hms(2023, 12, 1, 12, 0, 0).unwrap();
/// let result = reddish::add_days(&date, 5);
/// assert_eq!(result.day(), 6);
/// assert_eq!(result.month(), 12);
/// ```
///
/// ```
/// use chrono::{Utc, TimeZone, Datelike};
/// let date = Utc.with_ymd_and_hms(2023, 12, 5, 12, 0, 0).unwrap();
/// let result = reddish::add_days(&date, -3);
/// assert_eq!(result.day(), 2);
/// assert_eq!(result.month(), 12);
/// ```
pub fn add_days(datetime: &DateTime<Utc>, days: i64) -> DateTime<Utc> {
    *datetime + Duration::days(days)
}
