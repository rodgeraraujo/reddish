extern crate reddish;
use reddish::{keys, values, entries, has_key, pick, omit, merge};
use std::collections::HashMap;

fn main() {
    // Create a sample HashMap
    let mut person = HashMap::new();
    person.insert("name", "John Doe");
    person.insert("age", "30");
    person.insert("city", "New York");
    person.insert("country", "USA");

    // Demonstrate keys() function
    let person_keys = keys(&person);
    println!("Keys: {:?}", person_keys);
    // Keys: ["name", "age", "city", "country"] (order may vary)

    // Demonstrate values() function
    let person_values = values(&person);
    println!("Values: {:?}", person_values);
    // Values: ["John Doe", "30", "New York", "USA"] (order may vary)

    // Demonstrate entries() function
    let person_entries = entries(&person);
    println!("Entries: {:?}", person_entries);
    // Entries: [("name", "John Doe"), ("age", "30"), ("city", "New York"), ("country", "USA")] (order may vary)

    // Demonstrate has_key() function
    println!("Has 'name' key: {}", has_key(&person, &"name"));
    // Has 'name' key: true
    println!("Has 'email' key: {}", has_key(&person, &"email"));
    // Has 'email' key: false

    // Demonstrate pick() function
    let basic_info = pick(&person, &["name", "age"]);
    println!("Basic info: {:?}", basic_info);
    // Basic info: {"name": "John Doe", "age": "30"}

    // Demonstrate omit() function
    let without_location = omit(&person, &["city", "country"]);
    println!("Without location: {:?}", without_location);
    // Without location: {"name": "John Doe", "age": "30"}

    // Demonstrate merge() function
    let mut additional_info = HashMap::new();
    additional_info.insert("email", "john@example.com");
    additional_info.insert("age", "31"); // This will overwrite the existing age

    let merged_person = merge(&person, &additional_info);
    println!("Merged person: {:?}", merged_person);
    // Merged person: {"name": "John Doe", "age": "31", "city": "New York", "country": "USA", "email": "john@example.com"}
}
