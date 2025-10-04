extern crate reddish;
use reddish::{
    format_duration, time_ago, is_weekend, days_between, add_days,
    start_of_week, end_of_month, parse_date, format_date,
    format_date_human, format_date_iso
};
use chrono::{Utc, TimeZone, Duration, Weekday, Datelike, Timelike};

#[test]
fn test_format_duration() {
    assert_eq!(format_duration(0), "0s");
    assert_eq!(format_duration(45), "45s");
    assert_eq!(format_duration(60), "1m");
    assert_eq!(format_duration(90), "1m 30s");
    assert_eq!(format_duration(3600), "1h");
    assert_eq!(format_duration(3661), "1h 1m 1s");
    assert_eq!(format_duration(7322), "2h 2m 2s");
    assert_eq!(format_duration(86400), "24h");
}

#[test]
fn test_time_ago() {
    let now = Utc::now();

    // Test "just now"
    let thirty_seconds_ago = now - Duration::seconds(30);
    assert_eq!(time_ago(&thirty_seconds_ago), "just now");

    // Test minutes
    let five_minutes_ago = now - Duration::minutes(5);
    assert_eq!(time_ago(&five_minutes_ago), "5 minutes ago");

    let one_minute_ago = now - Duration::minutes(1);
    assert_eq!(time_ago(&one_minute_ago), "1 minute ago");

    // Test hours
    let two_hours_ago = now - Duration::hours(2);
    assert_eq!(time_ago(&two_hours_ago), "2 hours ago");

    let one_hour_ago = now - Duration::hours(1);
    assert_eq!(time_ago(&one_hour_ago), "1 hour ago");

    // Test days
    let three_days_ago = now - Duration::days(3);
    assert_eq!(time_ago(&three_days_ago), "3 days ago");

    let one_day_ago = now - Duration::days(1);
    assert_eq!(time_ago(&one_day_ago), "1 day ago");

    // Test future
    let future = now + Duration::hours(1);
    assert_eq!(time_ago(&future), "in the future");
}

#[test]
fn test_is_weekend() {
    // Saturday
    let saturday = Utc.with_ymd_and_hms(2023, 12, 2, 12, 0, 0).unwrap();
    assert_eq!(is_weekend(&saturday), true);

    // Sunday
    let sunday = Utc.with_ymd_and_hms(2023, 12, 3, 12, 0, 0).unwrap();
    assert_eq!(is_weekend(&sunday), true);

    // Monday
    let monday = Utc.with_ymd_and_hms(2023, 12, 4, 12, 0, 0).unwrap();
    assert_eq!(is_weekend(&monday), false);

    // Friday
    let friday = Utc.with_ymd_and_hms(2023, 12, 1, 12, 0, 0).unwrap();
    assert_eq!(is_weekend(&friday), false);
}

#[test]
fn test_days_between() {
    let date1 = Utc.with_ymd_and_hms(2023, 12, 1, 0, 0, 0).unwrap();
    let date2 = Utc.with_ymd_and_hms(2023, 12, 5, 0, 0, 0).unwrap();

    assert_eq!(days_between(&date1, &date2), 4);
    assert_eq!(days_between(&date2, &date1), -4);
    assert_eq!(days_between(&date1, &date1), 0);
}

#[test]
fn test_days_between_different_months() {
    let date1 = Utc.with_ymd_and_hms(2023, 11, 30, 0, 0, 0).unwrap();
    let date2 = Utc.with_ymd_and_hms(2023, 12, 2, 0, 0, 0).unwrap();

    assert_eq!(days_between(&date1, &date2), 2);
}

#[test]
fn test_add_days() {
    let date = Utc.with_ymd_and_hms(2023, 12, 1, 12, 0, 0).unwrap();

    let result = add_days(&date, 5);
    assert_eq!(result.day(), 6);
    assert_eq!(result.month(), 12);
    assert_eq!(result.year(), 2023);

    let result = add_days(&date, -3);
    assert_eq!(result.day(), 28);
    assert_eq!(result.month(), 11);
    assert_eq!(result.year(), 2023);
}

#[test]
fn test_add_days_cross_year() {
    let date = Utc.with_ymd_and_hms(2023, 12, 30, 12, 0, 0).unwrap();

    let result = add_days(&date, 5);
    assert_eq!(result.day(), 4);
    assert_eq!(result.month(), 1);
    assert_eq!(result.year(), 2024);
}

#[test]
fn test_start_of_week() {
    // Wednesday, December 6, 2023
    let wednesday = Utc.with_ymd_and_hms(2023, 12, 6, 15, 30, 45).unwrap();
    let result = start_of_week(&wednesday);

    assert_eq!(result.weekday(), Weekday::Mon);
    assert_eq!(result.day(), 4); // Monday, December 4, 2023
    assert_eq!(result.month(), 12);
    assert_eq!(result.year(), 2023);
    assert_eq!(result.hour(), 0);
    assert_eq!(result.minute(), 0);
    assert_eq!(result.second(), 0);
}

#[test]
fn test_start_of_week_monday() {
    // Monday, December 4, 2023
    let monday = Utc.with_ymd_and_hms(2023, 12, 4, 15, 30, 45).unwrap();
    let result = start_of_week(&monday);

    assert_eq!(result.weekday(), Weekday::Mon);
    assert_eq!(result.day(), 4);
    assert_eq!(result.hour(), 0);
    assert_eq!(result.minute(), 0);
    assert_eq!(result.second(), 0);
}

#[test]
fn test_start_of_week_sunday() {
    // Sunday, December 3, 2023
    let sunday = Utc.with_ymd_and_hms(2023, 12, 3, 15, 30, 45).unwrap();
    let result = start_of_week(&sunday);

    assert_eq!(result.weekday(), Weekday::Mon);
    assert_eq!(result.day(), 27); // Monday, November 27, 2023
    assert_eq!(result.month(), 11);
}

#[test]
fn test_end_of_month() {
    // December 2023 (31 days)
    let december = Utc.with_ymd_and_hms(2023, 12, 15, 10, 30, 0).unwrap();
    let result = end_of_month(&december);

    assert_eq!(result.day(), 31);
    assert_eq!(result.month(), 12);
    assert_eq!(result.year(), 2023);
    assert_eq!(result.hour(), 23);
    assert_eq!(result.minute(), 59);
    assert_eq!(result.second(), 59);
}

#[test]
fn test_end_of_month_february_leap() {
    // February 2024 (leap year, 29 days)
    let february_leap = Utc.with_ymd_and_hms(2024, 2, 15, 10, 30, 0).unwrap();
    let result = end_of_month(&february_leap);

    assert_eq!(result.day(), 29);
    assert_eq!(result.month(), 2);
    assert_eq!(result.year(), 2024);
}

#[test]
fn test_end_of_month_february_non_leap() {
    // February 2023 (non-leap year, 28 days)
    let february_non_leap = Utc.with_ymd_and_hms(2023, 2, 15, 10, 30, 0).unwrap();
    let result = end_of_month(&february_non_leap);

    assert_eq!(result.day(), 28);
    assert_eq!(result.month(), 2);
    assert_eq!(result.year(), 2023);
}

#[test]
fn test_parse_date() {
    // ISO format
    let result = parse_date("2023-12-25T15:30:00Z");
    assert!(result.is_some());
    let date = result.unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), 12);
    assert_eq!(date.day(), 25);
    assert_eq!(date.hour(), 15);
    assert_eq!(date.minute(), 30);

    // Simple datetime
    let result = parse_date("2023-12-25 15:30:00");
    assert!(result.is_some());
    let date = result.unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), 12);
    assert_eq!(date.day(), 25);

    // Date only
    let result = parse_date("2023-12-25");
    assert!(result.is_some());
    let date = result.unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), 12);
    assert_eq!(date.day(), 25);
    assert_eq!(date.hour(), 0);
    assert_eq!(date.minute(), 0);
}

#[test]
fn test_parse_date_various_formats() {
    let formats = [
        "25/12/2023",
        "12/25/2023",
        "25-12-2023",
        "2023/12/25",
    ];

    for format in &formats {
        let result = parse_date(format);
        assert!(result.is_some(), "Failed to parse: {}", format);
        let date = result.unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 25);
    }
}

#[test]
fn test_parse_date_invalid() {
    assert!(parse_date("invalid date").is_none());
    assert!(parse_date("").is_none());
    assert!(parse_date("2023-13-01").is_none()); // Invalid month
    assert!(parse_date("2023-12-32").is_none()); // Invalid day
}

#[test]
fn test_format_date() {
    let date = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();

    assert_eq!(format_date(&date, "%Y-%m-%d"), "2023-12-25");
    assert_eq!(format_date(&date, "%H:%M:%S"), "15:30:45");
    assert_eq!(format_date(&date, "%Y-%m-%d %H:%M:%S"), "2023-12-25 15:30:45");
}

#[test]
fn test_format_date_human() {
    let date = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();
    let result = format_date_human(&date);
    assert!(result.contains("December 25, 2023"));
    assert!(result.contains("3:30 PM"));
}

#[test]
fn test_format_date_iso() {
    let date = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();
    assert_eq!(format_date_iso(&date), "2023-12-25T15:30:45Z");
}

// Integration tests
#[test]
fn test_datetime_workflow() {
    // Parse a date string
    let date_str = "2023-12-01 10:00:00";
    let parsed_date = parse_date(date_str).unwrap();

    // Add some days
    let future_date = add_days(&parsed_date, 14);

    // Check if it's a weekend
    let _is_weekend_day = is_weekend(&future_date);

    // Calculate days between
    let days_diff = days_between(&parsed_date, &future_date);
    assert_eq!(days_diff, 14);

    // Format the result
    let formatted = format_date_human(&future_date);
    assert!(formatted.contains("December 15, 2023"));

    // Get start of week
    let week_start = start_of_week(&future_date);
    assert_eq!(week_start.weekday(), Weekday::Mon);

    // Get end of month
    let month_end = end_of_month(&future_date);
    assert_eq!(month_end.day(), 31);
}

#[test]
fn test_time_calculations() {
    let base_date = Utc.with_ymd_and_hms(2023, 6, 15, 12, 0, 0).unwrap(); // Mid-year

    // Test adding various day amounts
    let next_week = add_days(&base_date, 7);
    assert_eq!(days_between(&base_date, &next_week), 7);

    let last_month = add_days(&base_date, -30);
    assert_eq!(days_between(&last_month, &base_date), 30);

    // Test week boundaries
    let week_start = start_of_week(&base_date);
    let days_from_monday = days_between(&week_start, &base_date);
    assert!(days_from_monday >= 0 && days_from_monday <= 6);

    // Test month boundaries
    let month_end = end_of_month(&base_date);
    assert_eq!(month_end.month(), base_date.month());
    assert!(month_end.day() >= 28 && month_end.day() <= 31);
}

#[test]
fn test_duration_edge_cases() {
    assert_eq!(format_duration(0), "0s");
    assert_eq!(format_duration(1), "1s");
    assert_eq!(format_duration(59), "59s");
    assert_eq!(format_duration(60), "1m");
    assert_eq!(format_duration(61), "1m 1s");
    assert_eq!(format_duration(3599), "59m 59s");
    assert_eq!(format_duration(3600), "1h");
    assert_eq!(format_duration(3661), "1h 1m 1s");
}
