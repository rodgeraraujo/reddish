#![allow(dead_code)]

use rand::seq::SliceRandom;

/// Selects n random elements from a slice without replacement.
/// Returns a vector with the selected elements.
///
/// ```
/// let vec = vec![1, 2, 3, 4, 5];
/// let result = reddish::sample(&vec, 3);
/// assert_eq!(result.len(), 3);
///
/// // All elements should be from the original vector
/// for item in &result {
///     assert!(vec.contains(item));
/// }
/// ```
///
/// ```
/// let vec = vec![1, 2, 3];
/// let result = reddish::sample(&vec, 5); // More than available
/// assert_eq!(result.len(), 3); // Should return all available elements
/// ```
pub fn sample<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    let sample_size = n.min(slice.len());
    slice
        .choose_multiple(&mut rand::thread_rng(), sample_size)
        .cloned()
        .collect()
}
