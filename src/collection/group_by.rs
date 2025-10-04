#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Groups elements of a vector by a key function.
///
/// ```
/// let vec = vec!["apple", "banana", "apricot", "blueberry"];
/// let result = reddish::group_by(&vec, |s| s.chars().next().unwrap());
///
/// assert_eq!(result.get(&'a').unwrap().len(), 2);
/// assert_eq!(result.get(&'b').unwrap().len(), 2);
/// assert!(result.get(&'a').unwrap().contains(&&"apple"));
/// assert!(result.get(&'a').unwrap().contains(&&"apricot"));
/// ```
///
/// ```
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// let result = reddish::group_by(&numbers, |&n| n % 2);
///
/// assert_eq!(result.get(&0).unwrap(), &vec![&2, &4, &6]);
/// assert_eq!(result.get(&1).unwrap(), &vec![&1, &3, &5]);
/// ```
pub fn group_by<T, K, F>(vec: &[T], key_fn: F) -> HashMap<K, Vec<&T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut groups = HashMap::new();

    for item in vec {
        let key = key_fn(item);
        groups.entry(key).or_insert_with(Vec::new).push(item);
    }

    groups
}
