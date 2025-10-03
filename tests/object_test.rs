extern crate reddish;
use reddish::{keys, values, entries, has_key, pick, omit, merge};
use std::collections::HashMap;

#[test]
fn test_keys() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");

    let result = keys(&map);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&&"name"));
    assert!(result.contains(&&"age"));
}

#[test]
fn test_keys_empty() {
    let map: HashMap<&str, &str> = HashMap::new();
    let result = keys(&map);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_values() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");

    let result = values(&map);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&&"John"));
    assert!(result.contains(&&"30"));
}

#[test]
fn test_values_empty() {
    let map: HashMap<&str, &str> = HashMap::new();
    let result = values(&map);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_entries() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");

    let result = entries(&map);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&(&"name", &"John")));
    assert!(result.contains(&(&"age", &"30")));
}

#[test]
fn test_entries_empty() {
    let map: HashMap<&str, &str> = HashMap::new();
    let result = entries(&map);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_has_key() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");

    assert_eq!(has_key(&map, &"name"), true);
    assert_eq!(has_key(&map, &"age"), true);
    assert_eq!(has_key(&map, &"email"), false);
}

#[test]
fn test_has_key_empty() {
    let map: HashMap<&str, &str> = HashMap::new();
    assert_eq!(has_key(&map, &"name"), false);
}

#[test]
fn test_pick() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");
    map.insert("city", "New York");

    let keys_to_pick = vec!["name", "city"];
    let result = pick(&map, &keys_to_pick);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("name"), Some(&"John"));
    assert_eq!(result.get("city"), Some(&"New York"));
    assert_eq!(result.get("age"), None);
}

#[test]
fn test_pick_nonexistent_keys() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");

    let keys_to_pick = vec!["name", "email", "phone"];
    let result = pick(&map, &keys_to_pick);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("name"), Some(&"John"));
    assert_eq!(result.get("email"), None);
    assert_eq!(result.get("phone"), None);
}

#[test]
fn test_pick_empty() {
    let map: HashMap<&str, &str> = HashMap::new();
    let keys_to_pick = vec!["name"];
    let result = pick(&map, &keys_to_pick);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_omit() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");
    map.insert("city", "New York");

    let keys_to_omit = vec!["age"];
    let result = omit(&map, &keys_to_omit);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("name"), Some(&"John"));
    assert_eq!(result.get("city"), Some(&"New York"));
    assert_eq!(result.get("age"), None);
}

#[test]
fn test_omit_nonexistent_keys() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");

    let keys_to_omit = vec!["email", "phone"];
    let result = omit(&map, &keys_to_omit);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("name"), Some(&"John"));
    assert_eq!(result.get("age"), Some(&"30"));
}

#[test]
fn test_omit_empty() {
    let map: HashMap<&str, &str> = HashMap::new();
    let keys_to_omit = vec!["name"];
    let result = omit(&map, &keys_to_omit);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_merge() {
    let mut map1 = HashMap::new();
    map1.insert("name", "John");
    map1.insert("age", "30");

    let mut map2 = HashMap::new();
    map2.insert("age", "31");
    map2.insert("city", "New York");

    let result = merge(&map1, &map2);

    assert_eq!(result.len(), 3);
    assert_eq!(result.get("name"), Some(&"John"));
    assert_eq!(result.get("age"), Some(&"31")); // overwritten by map2
    assert_eq!(result.get("city"), Some(&"New York"));
}

#[test]
fn test_merge_empty_maps() {
    let map1: HashMap<&str, &str> = HashMap::new();
    let map2: HashMap<&str, &str> = HashMap::new();

    let result = merge(&map1, &map2);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_merge_with_empty_first() {
    let map1: HashMap<&str, &str> = HashMap::new();
    let mut map2 = HashMap::new();
    map2.insert("name", "John");

    let result = merge(&map1, &map2);
    assert_eq!(result.len(), 1);
    assert_eq!(result.get("name"), Some(&"John"));
}

#[test]
fn test_merge_with_empty_second() {
    let mut map1 = HashMap::new();
    map1.insert("name", "John");
    let map2: HashMap<&str, &str> = HashMap::new();

    let result = merge(&map1, &map2);
    assert_eq!(result.len(), 1);
    assert_eq!(result.get("name"), Some(&"John"));
}
