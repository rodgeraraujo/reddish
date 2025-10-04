#![allow(dead_code)]

use rand::Rng;

/// Generates a random integer between min and max (inclusive).
///
/// ```
/// let result = reddish::random_int(1, 10);
/// assert!(result >= 1 && result <= 10);
/// ```
///
/// ```
/// let result = reddish::random_int(-5, 5);
/// assert!(result >= -5 && result <= 5);
/// ```
pub fn random_int(min: i32, max: i32) -> i32 {
    if min > max {
        panic!("min cannot be greater than max");
    }
    rand::thread_rng().gen_range(min..=max)
}
