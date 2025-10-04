extern crate reddish;
use reddish::{
    format_duration, time_ago, is_weekend, days_between, add_days,
    start_of_week, end_of_month, parse_date, format_date,
    format_date_human, format_date_iso
};
use chrono::{Utc, TimeZone, Duration};

fn main() {
    println!("=== DateTime Methods Examples ===\n");

    // Demonstrate format_duration() function
    println!("Duration formatting:");
    let durations = [45, 90, 3661, 7322, 86400, 90061];
    for &seconds in &durations {
        println!("  {} seconds = {}", seconds, format_duration(seconds));
    }
    println!();

    // Demonstrate time_ago() function
    println!("Relative time formatting:");
    let now = Utc::now();
    let times = [
        now - Duration::seconds(30),
        now - Duration::minutes(5),
        now - Duration::hours(2),
        now - Duration::days(3),
        now - Duration::days(45),
        now - Duration::days(400),
    ];

    for time in &times {
        println!("  {} -> {}", format_date_iso(time), time_ago(time));
    }
    println!();

    // Demonstrate is_weekend() function
    println!("Weekend checking:");
    let dates = [
        Utc.with_ymd_and_hms(2023, 12, 1, 12, 0, 0).unwrap(), // Friday
        Utc.with_ymd_and_hms(2023, 12, 2, 12, 0, 0).unwrap(), // Saturday
        Utc.with_ymd_and_hms(2023, 12, 3, 12, 0, 0).unwrap(), // Sunday
        Utc.with_ymd_and_hms(2023, 12, 4, 12, 0, 0).unwrap(), // Monday
    ];

    for date in &dates {
        println!("  {} ({}) -> Weekend: {}",
                 format_date(date, "%A, %B %d"),
                 format_date(date, "%Y-%m-%d"),
                 is_weekend(date));
    }
    println!();

    // Demonstrate days_between() function
    println!("Days between dates:");
    let date1 = Utc.with_ymd_and_hms(2023, 12, 1, 0, 0, 0).unwrap();
    let date2 = Utc.with_ymd_and_hms(2023, 12, 15, 0, 0, 0).unwrap();
    let date3 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    println!("  {} to {} = {} days",
             format_date(&date1, "%Y-%m-%d"),
             format_date(&date2, "%Y-%m-%d"),
             days_between(&date1, &date2));
    println!("  {} to {} = {} days",
             format_date(&date1, "%Y-%m-%d"),
             format_date(&date3, "%Y-%m-%d"),
             days_between(&date1, &date3));
    println!();

    // Demonstrate add_days() function
    println!("Adding days to dates:");
    let base_date = Utc.with_ymd_and_hms(2023, 12, 15, 10, 30, 0).unwrap();
    println!("  Base date: {}", format_date_human(&base_date));

    for &days in &[7, -3, 30, -10] {
        let new_date = add_days(&base_date, days);
        println!("  {} {} days = {}",
                 if days >= 0 { "Add" } else { "Subtract" },
                 days.abs(),
                 format_date_human(&new_date));
    }
    println!();

    // Demonstrate start_of_week() function
    println!("Start of week:");
    let mid_week = Utc.with_ymd_and_hms(2023, 12, 6, 15, 30, 45).unwrap(); // Wednesday
    let week_start = start_of_week(&mid_week);
    println!("  {} -> {}",
             format_date(&mid_week, "%A, %B %d at %H:%M:%S"),
             format_date(&week_start, "%A, %B %d at %H:%M:%S"));
    println!();

    // Demonstrate end_of_month() function
    println!("End of month:");
    let months = [
        Utc.with_ymd_and_hms(2023, 2, 15, 10, 0, 0).unwrap(),  // February (non-leap)
        Utc.with_ymd_and_hms(2024, 2, 15, 10, 0, 0).unwrap(),  // February (leap year)
        Utc.with_ymd_and_hms(2023, 12, 15, 10, 0, 0).unwrap(), // December
    ];

    for date in &months {
        let month_end = end_of_month(date);
        println!("  {} -> {}",
                 format_date(date, "%B %Y"),
                 format_date(&month_end, "%B %d, %Y at %H:%M:%S"));
    }
    println!();

    // Demonstrate parse_date() function
    println!("Date parsing:");
    let date_strings = [
        "2023-12-25 15:30:00",
        "2023-12-25T15:30:00Z",
        "2023-12-25",
        "25/12/2023",
        "12/25/2023",
        "invalid date",
    ];

    for date_str in &date_strings {
        match parse_date(date_str) {
            Some(date) => println!("  '{}' -> {}", date_str, format_date_human(&date)),
            None => println!("  '{}' -> Failed to parse", date_str),
        }
    }
    println!();

    // Demonstrate format_date() functions
    println!("Date formatting:");
    let sample_date = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();

    println!("  Sample date: {}", format_date_iso(&sample_date));
    println!("  Human format: {}", format_date_human(&sample_date));
    println!("  Custom format: {}", format_date(&sample_date, "%A, %B %d, %Y"));
    println!("  Short format: {}", format_date(&sample_date, "%m/%d/%y"));
    println!("  Time only: {}", format_date(&sample_date, "%H:%M:%S"));
    println!();

    // Practical examples
    println!("=== Practical Use Cases ===");

    // Log entry with timestamp
    let log_time = Utc::now();
    println!("Log entry: [{}] Application started", format_date_iso(&log_time));

    // Event scheduling
    let event_date = Utc.with_ymd_and_hms(2024, 1, 15, 19, 0, 0).unwrap();
    println!("Event: Conference call scheduled for {}", format_date_human(&event_date));
    println!("That's {} from now", time_ago(&event_date));

    // Work week calculation
    let today = Utc::now();
    let week_start = start_of_week(&today);
    let days_worked = days_between(&week_start, &today);
    println!("Work week: Started {}, {} days worked so far",
             format_date(&week_start, "%A %B %d"),
             days_worked);

    // Deadline tracking
    let deadline = add_days(&today, 7);
    println!("Deadline: Project due in 7 days ({})", format_date_human(&deadline));

    // Session duration
    let session_start = Utc::now() - Duration::seconds(1847);
    let session_duration = Utc::now().signed_duration_since(session_start).num_seconds() as u64;
    println!("Session: Active for {}", format_duration(session_duration));
}
