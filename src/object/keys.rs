#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Returns a vector containing all the keys of a HashMap.
///
/// ```
/// use std::collections::HashMap;
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
///
/// let result = reddish::keys(&map);
/// assert_eq!(result.len(), 2);
/// assert!(result.contains(&&"name"));
/// assert!(result.contains(&&"age"));
/// ```
pub fn keys<K, V>(map: &HashMap<K, V>) -> Vec<&K>
where
    K: Eq + Hash,
{
    map.keys().collect()
}
