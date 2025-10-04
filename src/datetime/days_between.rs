#![allow(dead_code)]

use chrono::{DateTime, Utc};

/// Calculates the number of days between two dates.
/// Returns a positive number if the second date is after the first, negative otherwise.
///
/// ```
/// use chrono::{Utc, TimeZone};
/// let date1 = Utc.with_ymd_and_hms(2023, 12, 1, 0, 0, 0).unwrap();
/// let date2 = Utc.with_ymd_and_hms(2023, 12, 5, 0, 0, 0).unwrap();
/// assert_eq!(reddish::days_between(&date1, &date2), 4);
/// ```
///
/// ```
/// use chrono::{Utc, TimeZone};
/// let date1 = Utc.with_ymd_and_hms(2023, 12, 5, 0, 0, 0).unwrap();
/// let date2 = Utc.with_ymd_and_hms(2023, 12, 1, 0, 0, 0).unwrap();
/// assert_eq!(reddish::days_between(&date1, &date2), -4);
/// ```
pub fn days_between(date1: &DateTime<Utc>, date2: &DateTime<Utc>) -> i64 {
    let duration = date2.signed_duration_since(*date1);
    duration.num_days()
}
