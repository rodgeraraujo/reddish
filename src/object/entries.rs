#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Returns a vector containing all the key-value pairs of a HashMap as tuples.
///
/// ```
/// use std::collections::HashMap;
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
///
/// let result = reddish::entries(&map);
/// assert_eq!(result.len(), 2);
/// assert!(result.contains(&(&"name", &"John")) || result.contains(&(&"age", &"30")));
/// ```
pub fn entries<K, V>(map: &HashMap<K, V>) -> Vec<(&K, &V)>
where
    K: Eq + Hash,
{
    map.iter().collect()
}
