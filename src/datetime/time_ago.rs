#![allow(dead_code)]

use chrono::{DateTime, Utc, Duration};

/// Formats a datetime as a relative time string (e.g., "2 hours ago").
///
/// ```
/// use chrono::{Utc, Duration};
/// let now = Utc::now();
/// let two_hours_ago = now - Duration::hours(2);
/// let result = reddish::time_ago(&two_hours_ago);
/// assert!(result.contains("hours ago"));
/// ```
///
/// ```
/// use chrono::{Utc, Duration};
/// let now = Utc::now();
/// let thirty_seconds_ago = now - Duration::seconds(30);
/// let result = reddish::time_ago(&thirty_seconds_ago);
/// assert_eq!(result, "just now");
/// ```
pub fn time_ago(datetime: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*datetime);

    if duration < Duration::zero() {
        return "in the future".to_string();
    }

    let seconds = duration.num_seconds();
    let minutes = duration.num_minutes();
    let hours = duration.num_hours();
    let days = duration.num_days();

    if seconds < 60 {
        "just now".to_string()
    } else if minutes < 60 {
        if minutes == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{} minutes ago", minutes)
        }
    } else if hours < 24 {
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else if days < 30 {
        if days == 1 {
            "1 day ago".to_string()
        } else {
            format!("{} days ago", days)
        }
    } else if days < 365 {
        let months = days / 30;
        if months == 1 {
            "1 month ago".to_string()
        } else {
            format!("{} months ago", months)
        }
    } else {
        let years = days / 365;
        if years == 1 {
            "1 year ago".to_string()
        } else {
            format!("{} years ago", years)
        }
    }
}
