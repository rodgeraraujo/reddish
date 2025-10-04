#![allow(dead_code)]

use chrono::{DateTime, Utc};

/// Formats a datetime using a specified format string.
/// Uses strftime format specifiers.
///
/// ```
/// use chrono::{Utc, TimeZone};
/// let date = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();
/// let result = reddish::format_date(&date, "%Y-%m-%d %H:%M:%S");
/// assert_eq!(result, "2023-12-25 15:30:45");
/// ```
///
/// ```
/// use chrono::{Utc, TimeZone};
/// let date = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();
/// let result = reddish::format_date(&date, "%B %d, %Y");
/// assert_eq!(result, "December 25, 2023");
/// ```
pub fn format_date(datetime: &DateTime<Utc>, format: &str) -> String {
    datetime.format(format).to_string()
}

/// Formats a datetime in a common human-readable format.
///
/// ```
/// use chrono::{Utc, TimeZone};
/// let date = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();
/// let result = reddish::format_date_human(&date);
/// assert!(result.contains("December 25, 2023"));
/// assert!(result.contains("3:30 PM"));
/// ```
pub fn format_date_human(datetime: &DateTime<Utc>) -> String {
    datetime.format("%B %d, %Y at %l:%M %p").to_string()
}

/// Formats a datetime in ISO 8601 format.
///
/// ```
/// use chrono::{Utc, TimeZone};
/// let date = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();
/// let result = reddish::format_date_iso(&date);
/// assert_eq!(result, "2023-12-25T15:30:45Z");
/// ```
pub fn format_date_iso(datetime: &DateTime<Utc>) -> String {
    datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
