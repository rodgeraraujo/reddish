#![allow(dead_code)]

use chrono::{DateTime, Utc, NaiveDateTime};

/// Parses a date string in various common formats.
/// Returns None if the string cannot be parsed.
///
/// ```
/// use chrono::Datelike;
/// let result = reddish::parse_date("2023-12-25 15:30:00");
/// assert!(result.is_some());
/// let date = result.unwrap();
/// assert_eq!(date.year(), 2023);
/// assert_eq!(date.month(), 12);
/// assert_eq!(date.day(), 25);
/// ```
///
/// ```
/// let result = reddish::parse_date("2023-12-25T15:30:00Z");
/// assert!(result.is_some());
/// ```
///
/// ```
/// let result = reddish::parse_date("invalid date");
/// assert!(result.is_none());
/// ```
pub fn parse_date(date_str: &str) -> Option<DateTime<Utc>> {
    // Try different common formats
    let formats = [
        "%Y-%m-%d %H:%M:%S",           // 2023-12-25 15:30:00
        "%Y-%m-%dT%H:%M:%SZ",          // 2023-12-25T15:30:00Z
        "%Y-%m-%dT%H:%M:%S%.fZ",       // 2023-12-25T15:30:00.123Z
        "%Y-%m-%d",                    // 2023-12-25
        "%d/%m/%Y",                    // 25/12/2023
        "%m/%d/%Y",                    // 12/25/2023
        "%d-%m-%Y",                    // 25-12-2023
        "%Y/%m/%d",                    // 2023/12/25
    ];

    // First try parsing as RFC3339 (ISO 8601)
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try each format
    for format in &formats {
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, format) {
            return Some(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
        }

        // For date-only formats, try parsing as date and add midnight time
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str, format) {
            if let Some(naive_dt) = naive_date.and_hms_opt(0, 0, 0) {
                return Some(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
            }
        }
    }

    None
}
