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

- [ ] “Collection” Methods
  - TODO:
