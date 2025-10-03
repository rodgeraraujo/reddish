#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

/// Creates a new HashMap with only the specified keys from the original HashMap.
///
/// ```
/// use std::collections::HashMap;
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
/// map.insert("city", "New York");
///
/// let keys_to_pick = vec!["name", "city"];
/// let result = reddish::pick(&map, &keys_to_pick);
///
/// assert_eq!(result.len(), 2);
/// assert_eq!(result.get("name"), Some(&"John"));
/// assert_eq!(result.get("city"), Some(&"New York"));
/// assert_eq!(result.get("age"), None);
/// ```
pub fn pick<K, V>(map: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    let mut result = HashMap::new();
    for key in keys {
        if let Some(value) = map.get(key) {
            result.insert(key.clone(), value.clone());
        }
    }
    result
}
