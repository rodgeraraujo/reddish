#![allow(dead_code)]

use rand::Rng;

/// Generates a random floating-point number between min and max (exclusive of max).
///
/// ```
/// let result = reddish::random_float(0.0, 1.0);
/// assert!(result >= 0.0 && result < 1.0);
/// ```
///
/// ```
/// let result = reddish::random_float(-10.0, 10.0);
/// assert!(result >= -10.0 && result < 10.0);
/// ```
pub fn random_float(min: f64, max: f64) -> f64 {
    if min >= max {
        panic!("min must be less than max");
    }
    rand::thread_rng().gen_range(min..max)
}
