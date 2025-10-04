#![allow(dead_code)]

use std::collections::HashSet;
use std::hash::Hash;

/// Returns a vector with unique elements, preserving order of first occurrence.
///
/// ```
/// let vec = vec![1, 2, 2, 3, 1, 4, 3];
/// let result = reddish::unique(&vec);
/// assert_eq!(result, vec![1, 2, 3, 4]);
/// ```
///
/// ```
/// let vec = vec!["apple", "banana", "apple", "cherry", "banana"];
/// let result = reddish::unique(&vec);
/// assert_eq!(result, vec!["apple", "banana", "cherry"]);
/// ```
pub fn unique<T>(vec: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for item in vec {
        if seen.insert(item.clone()) {
            result.push(item.clone());
        }
    }

    result
}
