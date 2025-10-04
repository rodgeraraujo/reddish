# reddish

[![Crates.io](https://img.shields.io/crates/v/reddish.svg)](https://crates.io/crates/reddish)
[![Documentation](https://docs.rs/reddish/badge.svg)](https://docs.rs/reddish/)
[![License](https://img.shields.io/crates/l/octavo.svg)](LICENSE)

![reddish library logo](./docs/assets/reddish.png)

A Rust utility library, making easier by taking the hassle out of working.

---

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
reddish = "0.2.0"
```

## Example

```rust
extern crate reddish;
use reddish::*;

fn main() {
  println!("{}", snake_case("fooBarBaz"));
  // foo_bar_baz
}
```

## Features

- [x] “String” Methods
  - `.camel_case([string=''])`: Converts `string` to camel case.
  - `.capitalize([string=''])`: Capitalize first char of `string`.
  - `.kebab_case([string=''])`: Converts `string` to kebab case.
  - `.snake_case([string=''])`: Converts `string` to snake case.
  - `.title_case([string=''])`: Converts `string` to title case.
  - `.pad([string=''], pad_length, pad_char=' ')`: Adds padding of length `pad_length` to both ends of `string` using optional `pad_char`, if no `pad_char` is supplied the `pad_char` defaults to ' '
  - `.pad_end([string=''], pad_length, pad_char=' ')`: Adds padding of length `pad_length` to the end of `string` using optional `pad_char`, if no `pad_char` is supplied the `pad_char` defaults to ' '
  - `.truncate([string=''], truncate_length)`: Truncates `string` to `truncate_length`.

- [x] “Array” Methods
  - `.concat(vec: Vec<T>, values: Vec<T>)`: Creates a new array concatenating an array with any additional array values.
  - `.difference(vec: Vec<T>, values: Vec<T>)`: Creates an array of values not included in the other given arrays using the same for equality comparisons.
  - `.find_index(vec: Vec<T>, find: F)`: Returns the index of the first found element.
  - `.find_last_index(vec: Vec<T>, find: F)`: Iterates over elements of collection from right to left, and returns the index of the found element.
  - `.join(vec: Vec<T>, sep: &str)`: Converts all elements in array into a string separated by separator.

- [x] "Object" Methods
  - `.keys(map: &HashMap<K, V>)`: Returns a vector containing all the keys of a HashMap.
  - `.values(map: &HashMap<K, V>)`: Returns a vector containing all the values of a HashMap.
  - `.entries(map: &HashMap<K, V>)`: Returns a vector containing all the key-value pairs of a HashMap as tuples.
  - `.has_key(map: &HashMap<K, V>, key: &K)`: Checks if a HashMap contains a specific key.
  - `.pick(map: &HashMap<K, V>, keys: &[K])`: Creates a new HashMap with only the specified keys from the original HashMap.
  - `.omit(map: &HashMap<K, V>, keys: &[K])`: Creates a new HashMap excluding the specified keys from the original HashMap.
  - `.merge(map1: &HashMap<K, V>, map2: &HashMap<K, V>)`: Merges two HashMaps, with values from the second HashMap overwriting values from the first.

- [x] "Collection" Methods
  - `.chunk(vec: &[T], size: usize)`: Splits a vector into chunks of the specified size.
  - `.flatten(nested: &[Vec<T>])`: Flattens a nested vector into a single-level vector.
  - `.group_by(vec: &[T], key_fn: F)`: Groups elements of a vector by a key function.
  - `.unique(vec: &[T])`: Returns a vector with unique elements, preserving order of first occurrence.
  - `.partition(vec: &[T], predicate: F)`: Partitions a vector into two vectors based on a predicate function.
  - `.zip(vec1: &[T], vec2: &[U])`: Combines two vectors into a vector of tuples.
  - `.count_by(vec: &[T], key_fn: F)`: Counts elements in a vector by a key function.

- [x] "Crypto/Hash" Methods
  - `.md5_hash(data: &str)`: Computes the MD5 hash of the input data and returns it as a hexadecimal string.
  - `.sha256_hash(data: &str)`: Computes the SHA256 hash of the input data and returns it as a hexadecimal string.
  - `.base64_encode(data: &str)`: Encodes the input string to Base64.
  - `.base64_decode(data: &str)`: Decodes a Base64 encoded string. Returns None if the input is invalid.
  - `.url_encode(data: &str)`: URL encodes the input string.
  - `.url_decode(data: &str)`: URL decodes the input string. Returns None if the input is invalid.
  - `.hex_encode(data: &str)`: Encodes the input string to hexadecimal.
  - `.hex_decode(data: &str)`: Decodes a hexadecimal string. Returns None if the input is invalid.

- [x] "Random" Methods
  - `.random_int(min: i32, max: i32)`: Generates a random integer between min and max (inclusive).
  - `.random_float(min: f64, max: f64)`: Generates a random floating-point number between min and max (exclusive of max).
  - `.random_choice(slice: &[T])`: Selects a random element from a slice. Returns None if the slice is empty.
  - `.shuffle(vec: &mut Vec<T>)`: Shuffles a vector in place using the Fisher-Yates algorithm.
  - `.sample(slice: &[T], n: usize)`: Selects n random elements from a slice without replacement.
  - `.random_string(length: usize)`: Generates a random string of the specified length using alphanumeric characters.
  - `.uuid()`: Generates a random UUID v4.
  - `.random_bool()`: Generates a random boolean value.
  - `.random_bool_with_probability(probability: f64)`: Generates a random boolean with a specified probability of being true.

- [x] "DateTime" Methods
  - `.format_duration(seconds: u64)`: Formats a duration in seconds into a human-readable string.
  - `.time_ago(datetime: &DateTime<Utc>)`: Formats a datetime as a relative time string (e.g., "2 hours ago").
  - `.is_weekend(datetime: &DateTime<Utc>)`: Checks if a given date falls on a weekend (Saturday or Sunday).
  - `.days_between(date1: &DateTime<Utc>, date2: &DateTime<Utc>)`: Calculates the number of days between two dates.
  - `.add_days(datetime: &DateTime<Utc>, days: i64)`: Adds a specified number of days to a date.
  - `.start_of_week(datetime: &DateTime<Utc>)`: Returns the start of the week (Monday) for a given date.
  - `.end_of_month(datetime: &DateTime<Utc>)`: Returns the last day of the month for a given date.
  - `.parse_date(date_str: &str)`: Parses a date string in various common formats.
  - `.format_date(datetime: &DateTime<Utc>, format: &str)`: Formats a datetime using a specified format string.
  - `.format_date_human(datetime: &DateTime<Utc>)`: Formats a datetime in a common human-readable format.
  - `.format_date_iso(datetime: &DateTime<Utc>)`: Formats a datetime in ISO 8601 format.
