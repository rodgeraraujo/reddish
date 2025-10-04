#![allow(dead_code)]

/// Formats a duration in seconds into a human-readable string.
///
/// ```
/// let result = reddish::format_duration(3661);
/// assert_eq!(result, "1h 1m 1s");
/// ```
///
/// ```
/// let result = reddish::format_duration(90);
/// assert_eq!(result, "1m 30s");
/// ```
///
/// ```
/// let result = reddish::format_duration(45);
/// assert_eq!(result, "45s");
/// ```
pub fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return "0s".to_string();
    }

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    let mut parts = Vec::new();

    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if secs > 0 {
        parts.push(format!("{}s", secs));
    }

    parts.join(" ")
}
