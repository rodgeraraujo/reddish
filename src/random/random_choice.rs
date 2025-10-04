#![allow(dead_code)]

use rand::seq::SliceRandom;

/// Selects a random element from a slice. Returns None if the slice is empty.
///
/// ```
/// let vec = vec![1, 2, 3, 4, 5];
/// let result = reddish::random_choice(&vec);
/// assert!(result.is_some());
/// assert!(vec.contains(result.unwrap()));
/// ```
///
/// ```
/// let vec: Vec<i32> = vec![];
/// let result = reddish::random_choice(&vec);
/// assert!(result.is_none());
/// ```
pub fn random_choice<T>(slice: &[T]) -> Option<&T> {
    slice.choose(&mut rand::thread_rng())
}
