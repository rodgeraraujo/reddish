#![allow(dead_code)]

use rand::Rng;

/// Generates a random boolean value.
///
/// ```
/// let result = reddish::random_bool();
/// assert!(result == true || result == false);
/// ```
pub fn random_bool() -> bool {
    rand::thread_rng().gen_bool(0.5)
}

/// Generates a random boolean value with a specified probability of being true.
/// The probability should be between 0.0 and 1.0.
///
/// ```
/// let result = reddish::random_bool_with_probability(0.0);
/// assert_eq!(result, false);
/// ```
///
/// ```
/// let result = reddish::random_bool_with_probability(1.0);
/// assert_eq!(result, true);
/// ```
pub fn random_bool_with_probability(probability: f64) -> bool {
    if probability < 0.0 || probability > 1.0 {
        panic!("probability must be between 0.0 and 1.0");
    }
    rand::thread_rng().gen_bool(probability)
}
