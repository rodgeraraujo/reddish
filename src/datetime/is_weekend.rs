#![allow(dead_code)]

use chrono::{DateTime, Utc, Weekday, Datelike};

/// Checks if a given date falls on a weekend (Saturday or Sunday).
///
/// ```
/// use chrono::{Utc, TimeZone};
/// // Saturday
/// let saturday = Utc.with_ymd_and_hms(2023, 12, 2, 12, 0, 0).unwrap();
/// assert_eq!(reddish::is_weekend(&saturday), true);
/// ```
///
/// ```
/// use chrono::{Utc, TimeZone};
/// // Monday
/// let monday = Utc.with_ymd_and_hms(2023, 12, 4, 12, 0, 0).unwrap();
/// assert_eq!(reddish::is_weekend(&monday), false);
/// ```
pub fn is_weekend(datetime: &DateTime<Utc>) -> bool {
    matches!(datetime.weekday(), Weekday::Sat | Weekday::Sun)
}
