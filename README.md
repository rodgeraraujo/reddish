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

- [ ] “Object” Methods
  - TODO:

- [ ] “Collection” Methods
  - TODO:
