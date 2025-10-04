//! # 🦀 Reddish
//!
//! **Reddish** is a comprehensive Rust utility library inspired by popular JavaScript libraries
//! like Lodash and Ramda. It provides a rich collection of utility functions for common
//! programming tasks including string manipulation, array operations, object handling,
//! cryptographic functions, random number generation, and date/time operations.
//!
//! ## ✨ Features
//!
//! - 🔧 **55+ Utility Functions** across 7 modules
//! - 🎯 **Zero Dependencies** for core functionality (optional dependencies for specific modules)
//! - 🚀 **High Performance** with Rust's memory safety guarantees
//! - 📦 **Modular Design** with feature flags for selective compilation
//! - 🧪 **100% Test Coverage** with comprehensive unit tests and doctests
//! - 📚 **Extensive Documentation** with examples for every function
//! - 🔒 **Type Safe** leveraging Rust's powerful type system
//!
//! ## 🚀 Quick Start
//!
//! Add Reddish to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! reddish = "0.2.0"
//! ```
//!
//! ### Basic Usage
//!
//! ```rust
//! use reddish::*;
//!
//! // String utilities
//! let title = capitalize("hello world");
//! assert_eq!(title, "Hello world");
//!
//! // Collection operations
//! let numbers = vec![1, 2, 3, 4, 5, 6];
//! let chunks = chunk(&numbers, 2);
//! assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
//! ```
//!
//! ## 📦 Modules
//!
//! Reddish is organized into feature-gated modules:
//!
//! - **`string`** - String manipulation utilities (capitalize, camelCase, etc.)
//! - **`array`** - Array operations (find, concat, difference, etc.)
//! - **`object`** - HashMap utilities (keys, values, merge, etc.)
//! - **`collection`** - Advanced collection functions (chunk, group_by, etc.)
//! - **`crypto`** - Cryptographic functions (hash, encode/decode, etc.)
//! - **`random`** - Random number generation and sampling
//! - **`datetime`** - Date/time manipulation and formatting
//!
//! ## 🔧 Feature Flags
//!
//! Use feature flags to include only what you need:
//!
//! ```toml
//! [dependencies]
//! # Include specific features only
//! reddish = { version = "0.2.0", features = ["string", "array"] }
//!
//! # Minimal installation
//! reddish = { version = "0.2.0", default-features = false, features = ["string"] }
//! ```
//!
//! ## Examples
//!
//! ### String Operations
//!
//! See individual function documentation for usage examples.
//!
//! ### Array Operations
//!
//! See individual function documentation for usage examples.
//!
//! ### Collection Operations
//!
//! ```rust
//! use reddish::*;
//!
//! let numbers = vec![1, 2, 3, 4, 5, 6];
//! let chunks = chunk(&numbers, 2);
//! assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
//!
//! let words = vec!["apple", "banana", "apricot"];
//! let grouped = group_by(&words, |s| s.chars().next().unwrap());
//! assert_eq!(grouped.get(&'a').unwrap().len(), 2); // "apple", "apricot"
//! ```
//!
//! ### Object Operations
//!
//! ```rust
//! use reddish::*;
//! use std::collections::HashMap;
//!
//! let mut user = HashMap::new();
//! user.insert("name", "Alice");
//! user.insert("age", "25");
//! user.insert("city", "NYC");
//!
//! let user_keys = keys(&user);
//! assert_eq!(user_keys.len(), 3);
//!
//! let personal = pick(&user, &["name", "age"]);
//! assert_eq!(personal.len(), 2);
//! ```
//!
//! For more examples, see the `examples/` directory in the repository.

#[cfg(feature = "string")]
mod string;
#[cfg(feature = "string")]
pub use string::*;

#[cfg(feature = "array")]
mod array;
#[cfg(feature = "array")]
pub use array::*;

#[cfg(feature = "object")]
mod object;
#[cfg(feature = "object")]
pub use object::*;

#[cfg(feature = "collection")]
mod collection;
#[cfg(feature = "collection")]
pub use collection::*;

#[cfg(feature = "crypto")]
mod crypto;
#[cfg(feature = "crypto")]
pub use crypto::*;

#[cfg(feature = "random")]
mod random;
#[cfg(feature = "random")]
pub use random::*;

#[cfg(feature = "datetime")]
mod datetime;
#[cfg(feature = "datetime")]
pub use datetime::*;
