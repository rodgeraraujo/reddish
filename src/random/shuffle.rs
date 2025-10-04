#![allow(dead_code)]

use rand::seq::SliceRandom;

/// Shuffles a vector in place using the Fisher-Yates algorithm.
///
/// ```
/// let mut vec = vec![1, 2, 3, 4, 5];
/// let original = vec.clone();
/// reddish::shuffle(&mut vec);
///
/// // The vector should contain the same elements
/// assert_eq!(vec.len(), original.len());
/// for item in &original {
///     assert!(vec.contains(item));
/// }
/// ```
pub fn shuffle<T>(vec: &mut Vec<T>) {
    vec.shuffle(&mut rand::thread_rng());
}
