#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Returns a vector containing all the values of a HashMap.
///
/// ```
/// use std::collections::HashMap;
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
///
/// let result = reddish::values(&map);
/// assert_eq!(result.len(), 2);
/// assert!(result.contains(&&"John"));
/// assert!(result.contains(&&"30"));
/// ```
pub fn values<K, V>(map: &HashMap<K, V>) -> Vec<&V>
where
    K: Eq + Hash,
{
    map.values().collect()
}
