#![allow(dead_code)]

use chrono::{DateTime, Utc, Datelike, TimeZone};

/// Returns the last day of the month for a given date.
///
/// ```
/// use chrono::{Utc, TimeZone, Datelike, Timelike};
/// // December 15, 2023
/// let date = Utc.with_ymd_and_hms(2023, 12, 15, 10, 30, 0).unwrap();
/// let result = reddish::end_of_month(&date);
///
/// assert_eq!(result.day(), 31); // December has 31 days
/// assert_eq!(result.month(), 12);
/// assert_eq!(result.year(), 2023);
/// assert_eq!(result.hour(), 23);
/// assert_eq!(result.minute(), 59);
/// assert_eq!(result.second(), 59);
/// ```
///
/// ```
/// use chrono::{Utc, TimeZone, Datelike};
/// // February 15, 2024 (leap year)
/// let date = Utc.with_ymd_and_hms(2024, 2, 15, 10, 30, 0).unwrap();
/// let result = reddish::end_of_month(&date);
///
/// assert_eq!(result.day(), 29); // February 2024 has 29 days (leap year)
/// assert_eq!(result.month(), 2);
/// ```
pub fn end_of_month(datetime: &DateTime<Utc>) -> DateTime<Utc> {
    let year = datetime.year();
    let month = datetime.month();

    // Get the first day of next month, then subtract one day
    let next_month = if month == 12 {
        Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap()
    } else {
        Utc.with_ymd_and_hms(year, month + 1, 1, 0, 0, 0).unwrap()
    };

    next_month - chrono::Duration::seconds(1)
}
