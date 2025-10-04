# 🦀 Reddish

[![Crates.io](https://img.shields.io/crates/v/reddish.svg)](https://crates.io/crates/reddish)
[![Documentation](https://docs.rs/reddish/badge.svg)](https://docs.rs/reddish)
[![License](https://img.shields.io/crates/l/reddish.svg)](LICENSE)
[![Build Status](https://github.com/rodgeraraujo/reddish/workflows/CI/badge.svg)](https://github.com/rodgeraraujo/reddish/actions)

**Reddish** is a comprehensive Rust utility library inspired by popular JavaScript libraries like Lodash and Ramda. It provides a rich collection of utility functions for common programming tasks including string manipulation, array operations, object handling, cryptographic functions, random number generation, and date/time operations.

## ✨ Features

- 🔧 **55+ Utility Functions** across 7 modules
- 🎯 **Zero Dependencies** for core functionality (optional dependencies for specific modules)
- 🚀 **High Performance** with Rust's memory safety guarantees
- 📦 **Modular Design** with feature flags for selective compilation
- 🧪 **100% Test Coverage** with comprehensive unit tests and doctests
- 📚 **Extensive Documentation** with examples for every function
- 🔒 **Type Safe** leveraging Rust's powerful type system

## 🚀 Quick Start

Add Reddish to your `Cargo.toml`:

```toml
[dependencies]
reddish = "0.2.0"
```

Or install with specific features only:

```toml
[dependencies]
reddish = { version = "0.2.0", features = ["string", "array"] }
```

### Basic Usage

```rust
use reddish::*;

fn main() {
    // String utilities
    let title = capitalize("hello world");
    println!("{}", title); // "Hello world"

    // Array operations
    let numbers = vec![1, 2, 3, 4, 5];
    let evens = find(&numbers, |&x| x % 2 == 0);
    println!("{:?}", evens); // Some(2)

    // Object manipulation
    use std::collections::HashMap;
    let mut user = HashMap::new();
    user.insert("name", "John");
    user.insert("age", "30");

    let keys = keys(&user);
    println!("{:?}", keys); // ["name", "age"]

    // Date/time operations
    use chrono::Utc;
    let now = Utc::now();
    let duration = format_duration(3661);
    println!("{}", duration); // "1h 1m 1s"

    // Random utilities
    let random_num = random_int(1, 100);
    let uuid = uuid();
    println!("Random: {}, UUID: {}", random_num, uuid);

    // Crypto functions
    let hash = md5_hash("hello world");
    let encoded = base64_encode("hello world");
    println!("MD5: {}, Base64: {}", hash, encoded);
}
```

## 📦 Modules & Features

Reddish is organized into feature-gated modules, allowing you to include only what you need:

### 🔤 String Methods

*Feature: `string`*

Transform and manipulate strings with ease.

- **`capitalize(s: &str)`** - Capitalizes the first character
- **`camel_case(s: &str)`** - Converts to camelCase
- **`snake_case(s: &str)`** - Converts to snake_case
- **`kebab_case(s: &str)`** - Converts to kebab-case
- **`title_case(s: &str)`** - Converts to Title Case
- **`pad(s: &str, length: usize, pad_char: char)`** - Pads string to length
- **`pad_end(s: &str, length: usize, pad_char: char)`** - Pads string at end
- **`truncate(s: &str, length: usize)`** - Truncates string to length

```rust
use reddish::*;

let text = "hello world";
assert_eq!(capitalize(text), "Hello world");
assert_eq!(camel_case(text), "helloWorld");
assert_eq!(snake_case("Hello World"), "hello_world");
```

### 📊 Array Methods

*Feature: `array`*

Powerful array manipulation and search functions.

- **`concat(vec1: &[T], vec2: &[T])`** - Concatenates two arrays
- **`difference(vec1: &[T], vec2: &[T])`** - Returns elements in first but not second
- **`find_index(vec: &[T], predicate: F)`** - Finds first matching index
- **`find_last_index(vec: &[T], predicate: F)`** - Finds last matching index
- **`join(vec: &[T], separator: &str)`** - Joins elements into string

```rust
use reddish::*;

let numbers = vec![1, 2, 3, 4, 5];
let index = find_index(&numbers, |&x| x > 3);
assert_eq!(index, Some(3)); // Index of element 4

let words = vec!["hello", "world"];
assert_eq!(join(&words, " "), "hello world");
```

### 🗂️ Object Methods

*Feature: `object`*

HashMap utilities for object-like operations.

- **`keys(map: &HashMap<K, V>)`** - Returns all keys
- **`values(map: &HashMap<K, V>)`** - Returns all values
- **`entries(map: &HashMap<K, V>)`** - Returns key-value pairs
- **`has_key(map: &HashMap<K, V>, key: &K)`** - Checks if key exists
- **`pick(map: &HashMap<K, V>, keys: &[&K])`** - Creates new map with selected keys
- **`omit(map: &HashMap<K, V>, keys: &[&K])`** - Creates new map without selected keys
- **`merge(map1: &HashMap<K, V>, map2: &HashMap<K, V>)`** - Merges two maps

```rust
use reddish::*;
use std::collections::HashMap;

let mut user = HashMap::new();
user.insert("name", "Alice");
user.insert("age", "25");
user.insert("city", "NYC");

let personal = pick(&user, &["name", "age"]);
// personal contains only "name" and "age"
```

### 📚 Collection Methods

*Feature: `collection`*

Advanced collection manipulation functions.

- **`chunk(vec: &[T], size: usize)`** - Splits array into chunks
- **`flatten(vec: &[Vec<T>])`** - Flattens nested arrays
- **`group_by(vec: &[T], key_fn: F)`** - Groups elements by key function
- **`unique(vec: &[T])`** - Returns unique elements
- **`partition(vec: &[T], predicate: F)`** - Splits array by predicate
- **`zip(vec1: &[T], vec2: &[U])`** - Combines two arrays into tuples
- **`count_by(vec: &[T], key_fn: F)`** - Counts elements by key function

```rust
use reddish::*;

let numbers = vec![1, 2, 3, 4, 5, 6];
let chunks = chunk(&numbers, 2);
// chunks: [[1, 2], [3, 4], [5, 6]]

let words = vec!["apple", "banana", "apricot"];
let grouped = group_by(&words, |s| s.chars().next().unwrap());
// Groups by first letter: {'a': ["apple", "apricot"], 'b': ["banana"]}
```

### 🔐 Crypto/Hash Methods

*Feature: `crypto`*

Cryptographic and encoding utilities.

- **`md5_hash(data: &str)`** - Computes MD5 hash
- **`sha256_hash(data: &str)`** - Computes SHA256 hash
- **`base64_encode(data: &str)`** - Encodes to Base64
- **`base64_decode(data: &str)`** - Decodes from Base64
- **`url_encode(data: &str)`** - URL encodes string
- **`url_decode(data: &str)`** - URL decodes string
- **`hex_encode(data: &str)`** - Encodes to hexadecimal
- **`hex_decode(data: &str)`** - Decodes from hexadecimal

```rust
use reddish::*;

let text = "hello world";
let hash = sha256_hash(text);
let encoded = base64_encode(text);
let url_safe = url_encode("hello world!");

println!("SHA256: {}", hash);
println!("Base64: {}", encoded);
println!("URL: {}", url_safe);
```

### 🎲 Random Methods

*Feature: `random`*

Random number generation and sampling utilities.

- **`random_int(min: i32, max: i32)`** - Random integer in range
- **`random_float(min: f64, max: f64)`** - Random float in range
- **`random_choice(slice: &[T])`** - Random element from slice
- **`shuffle(vec: &mut Vec<T>)`** - Shuffles vector in place
- **`sample(slice: &[T], n: usize)`** - Random sample without replacement
- **`random_string(length: usize)`** - Random alphanumeric string
- **`uuid()`** - Generates UUID v4
- **`random_bool()`** - Random boolean
- **`random_bool_with_probability(probability: f64)`** - Weighted random boolean

```rust
use reddish::*;

let dice_roll = random_int(1, 6);
let coin_flip = random_bool();
let user_id = uuid();

let colors = vec!["red", "green", "blue"];
let chosen_color = random_choice(&colors);

let mut deck = vec![1, 2, 3, 4, 5];
shuffle(&mut deck); // Randomizes order
```

### 📅 DateTime Methods

*Feature: `datetime`*

Comprehensive date and time manipulation.

- **`format_duration(seconds: u64)`** - Formats duration as human readable
- **`time_ago(datetime: &DateTime<Utc>)`** - Relative time formatting
- **`is_weekend(datetime: &DateTime<Utc>)`** - Checks if date is weekend
- **`days_between(date1, date2)`** - Calculates days between dates
- **`add_days(datetime, days)`** - Adds/subtracts days
- **`start_of_week(datetime)`** - Gets start of week (Monday)
- **`end_of_month(datetime)`** - Gets end of month
- **`parse_date(date_str: &str)`** - Parses various date formats
- **`format_date(datetime, format)`** - Custom date formatting
- **`format_date_human(datetime)`** - Human-readable date format
- **`format_date_iso(datetime)`** - ISO 8601 format

```rust
use reddish::*;
use chrono::{Utc, TimeZone};

let now = Utc::now();
let yesterday = add_days(&now, -1);

println!("Yesterday was: {}", time_ago(&yesterday));
println!("Duration: {}", format_duration(3661)); // "1h 1m 1s"

let date = parse_date("2023-12-25").unwrap();
println!("Christmas: {}", format_date_human(&date));
```

## 🔧 Feature Flags

Reddish uses Cargo features to allow selective compilation:

```toml
[dependencies]
# Include all features (default)
reddish = "0.2.0"

# Include only specific features
reddish = { version = "0.2.0", features = ["string", "array", "crypto"] }

# Minimal installation (no optional dependencies)
reddish = { version = "0.2.0", default-features = false, features = ["string"] }
```

### Available Features

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `string` | String manipulation utilities | None |
| `array` | Array operations | None |
| `object` | HashMap utilities | None |
| `collection` | Advanced collection functions | None |
| `crypto` | Cryptographic functions | `md5`, `sha2`, `base64`, `percent-encoding`, `hex` |
| `random` | Random number generation | `rand`, `uuid` |
| `datetime` | Date/time operations | `chrono` |

## 📖 Examples

The `examples/` directory contains comprehensive examples for each module:

```bash
# Run examples
cargo run --example string
cargo run --example array
cargo run --example object
cargo run --example collection
cargo run --example crypto
cargo run --example random
cargo run --example datetime
```

Each example demonstrates practical use cases and best practices.

## 🧪 Testing

Reddish has extensive test coverage with over 100 tests:

```bash
# Run all tests
cargo test

# Run tests for specific feature
cargo test --features string

# Run with all features
cargo test --all-features

# Run doctests
cargo test --doc
```

## 📊 Performance

Reddish is designed for performance:

- **Zero-cost abstractions** - No runtime overhead
- **Memory efficient** - Minimal allocations where possible
- **Optimized algorithms** - Uses efficient algorithms (e.g., Fisher-Yates shuffle)
- **Compile-time optimizations** - Leverages Rust's compiler optimizations

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
git clone https://github.com/rodgeraraujo/reddish.git
cd reddish
cargo build --all-features
cargo test --all-features
```

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and changes.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [Lodash](https://lodash.com/) and [Ramda](https://ramdajs.com/)
- Built with ❤️ in Rust
- Thanks to all contributors and the Rust community

## 🔗 Links

- [Crates.io](https://crates.io/crates/reddish)
- [Documentation](https://docs.rs/reddish)
- [Repository](https://github.com/rodgeraraujo/reddish)
- [Issues](https://github.com/rodgeraraujo/reddish/issues)

---

<div align="center">
  <strong>Made with 🦀 Rust</strong>
</div>
