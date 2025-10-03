#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Checks if a HashMap contains a specific key.
///
/// ```
/// use std::collections::HashMap;
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
///
/// assert_eq!(reddish::has_key(&map, &"name"), true);
/// assert_eq!(reddish::has_key(&map, &"email"), false);
/// ```
pub fn has_key<K, V>(map: &HashMap<K, V>, key: &K) -> bool
where
    K: Eq + Hash,
{
    map.contains_key(key)
}
