#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Counts elements in a vector by a key function.
///
/// ```
/// let words = vec!["apple", "banana", "apricot", "blueberry"];
/// let result = reddish::count_by(&words, |s| s.chars().next().unwrap());
///
/// assert_eq!(result.get(&'a'), Some(&2));
/// assert_eq!(result.get(&'b'), Some(&2));
/// ```
///
/// ```
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// let result = reddish::count_by(&numbers, |&n| n % 2);
///
/// assert_eq!(result.get(&0), Some(&3)); // even numbers
/// assert_eq!(result.get(&1), Some(&3)); // odd numbers
/// ```
pub fn count_by<T, K, F>(vec: &[T], key_fn: F) -> HashMap<K, usize>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut counts = HashMap::new();

    for item in vec {
        let key = key_fn(item);
        *counts.entry(key).or_insert(0) += 1;
    }

    counts
}
