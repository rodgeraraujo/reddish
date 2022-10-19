# reddish

[![Crates.io](https://img.shields.io/crates/v/reddish.svg)](https://crates.io/crates/reddish)
[![Documentation](https://docs.rs/reddish/badge.svg)](https://docs.rs/reddish/)
[![License](https://img.shields.io/crates/l/octavo.svg)](LICENSE)

![reddish library logo](./docs/assets/reddish.png)

A Rust utility library.

---

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
reddish = "0.1.0"
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

- [ ] “Array” Methods
  - TODO:

- [ ] “Object” Methods
  - TODO:

- [ ] “Collection” Methods
  - TODO:
