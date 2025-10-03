#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Creates a new HashMap excluding the specified keys from the original HashMap.
///
/// ```
/// use std::collections::HashMap;
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
/// map.insert("city", "New York");
///
/// let keys_to_omit = vec!["age"];
/// let result = reddish::omit(&map, &keys_to_omit);
///
/// assert_eq!(result.len(), 2);
/// assert_eq!(result.get("name"), Some(&"John"));
/// assert_eq!(result.get("city"), Some(&"New York"));
/// assert_eq!(result.get("age"), None);
/// ```
pub fn omit<K, V>(map: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    let mut result = map.clone();
    for key in keys {
        result.remove(key);
    }
    result
}
