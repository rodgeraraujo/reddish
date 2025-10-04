#![allow(dead_code)]

use rand::{Rng, distributions::Alphanumeric};

/// Generates a random string of the specified length using alphanumeric characters.
///
/// ```
/// let result = reddish::random_string(10);
/// assert_eq!(result.len(), 10);
/// assert!(result.chars().all(|c| c.is_alphanumeric()));
/// ```
///
/// ```
/// let result = reddish::random_string(0);
/// assert_eq!(result.len(), 0);
/// ```
pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
