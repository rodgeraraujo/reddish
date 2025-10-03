#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Merges two HashMaps, with values from the second HashMap overwriting values from the first.
///
/// ```
/// use std::collections::HashMap;
/// let mut map1 = HashMap::new();
/// map1.insert("name", "John");
/// map1.insert("age", "30");
///
/// let mut map2 = HashMap::new();
/// map2.insert("age", "31");
/// map2.insert("city", "New York");
///
/// let result = reddish::merge(&map1, &map2);
///
/// assert_eq!(result.len(), 3);
/// assert_eq!(result.get("name"), Some(&"John"));
/// assert_eq!(result.get("age"), Some(&"31")); // overwritten by map2
/// assert_eq!(result.get("city"), Some(&"New York"));
/// ```
pub fn merge<K, V>(map1: &HashMap<K, V>, map2: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    let mut result = map1.clone();
    for (key, value) in map2 {
        result.insert(key.clone(), value.clone());
    }
    result
}
