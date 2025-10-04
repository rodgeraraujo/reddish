# Frequently Asked Questions (FAQ) ❓

Common questions and answers about using Reddish.

## 🚀 Getting Started

### Q: How do I install Reddish?

**A:** Add it to your `Cargo.toml`:

```toml
[dependencies]
reddish = "0.2.0"
```

Then use it in your code:
```rust
use reddish::*;
```

### Q: Can I use only specific modules?

**A:** Yes! Use feature flags to include only what you need:

```toml
[dependencies]
reddish = { version = "0.2.0", features = ["string", "array"] }
```

Available features: `string`, `array`, `object`, `collection`, `crypto`, `random`, `datetime`

### Q: What's the minimum Rust version required?

**A:** Reddish requires Rust 1.70 or later.

## 🔧 Usage Questions

### Q: Why do some functions return `Option`?

**A:** Functions that can fail return `Option<T>` instead of panicking:

```rust
// These can fail with invalid input
let result = parse_date("invalid date"); // Returns None
let decoded = base64_decode("invalid base64"); // Returns None

// These panic on invalid input (programming errors)
let random = random_int(10, 5); // Panics: min > max
```

### Q: How do I handle errors from functions that return `Option`?

**A:** Use pattern matching or combinators:

```rust
// Pattern matching
match parse_date("2023-12-25") {
    Some(date) => println!("Parsed: {}", format_date_human(&date)),
    None => println!("Failed to parse date"),
}

// Using combinators
let formatted = parse_date("2023-12-25")
    .map(|date| format_date_human(&date))
    .unwrap_or_else(|| "Invalid date".to_string());
```

### Q: Are the functions thread-safe?

**A:** Yes, all functions are thread-safe:
- Pure functions (string, array, collection) have no shared state
- Random functions use thread-local RNG
- Crypto functions are stateless
- DateTime functions work with immutable data

### Q: Can I use Reddish in `no_std` environments?

**A:** Currently, Reddish requires `std`. `no_std` support is planned for future versions.

## 📦 Module-Specific Questions

### String Module

### Q: Do string functions modify the original string?

**A:** No, all string functions return new strings:

```rust
let original = "hello world";
let capitalized = capitalize(original);
// original is unchanged, capitalized is "Hello world"
```

### Q: How do I chain string operations?

**A:** Since functions return owned strings, you can chain them:

```rust
let result = capitalize(&snake_case("Hello World"));
// "Hello_world"
```

### Array Module

### Q: Do array functions modify the original array?

**A:** No, array functions work with slices and return new data:

```rust
let original = vec![1, 2, 3];
let concatenated = concat(&original, &[4, 5]);
// original is unchanged, concatenated is [1, 2, 3, 4, 5]
```

### Q: Can I use array functions with different types?

**A:** Yes, most functions are generic:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let strings = vec!["a", "b", "c"];

let found_num = find_index(&numbers, |&x| x > 3);
let found_str = find_index(&strings, |&s| s == "b");
```

### Object Module

### Q: What types can I use as HashMap keys?

**A:** Any type that implements `Eq + Hash`:

```rust
use std::collections::HashMap;

// String keys
let mut string_map: HashMap<String, i32> = HashMap::new();

// Integer keys
let mut int_map: HashMap<i32, String> = HashMap::new();

// Custom types (with #[derive(Eq, PartialEq, Hash)])
#[derive(Eq, PartialEq, Hash)]
struct UserId(u64);
let mut user_map: HashMap<UserId, String> = HashMap::new();
```

### Collection Module

### Q: What's the difference between `unique` and using a `HashSet`?

**A:** `unique` preserves order and works with any type that implements `Clone + PartialEq`:

```rust
let numbers = vec![1, 2, 2, 3, 1, 4];
let unique_ordered = unique(&numbers); // [1, 2, 3, 4] - preserves first occurrence order

// HashSet doesn't preserve order and requires Hash + Eq
use std::collections::HashSet;
let set: HashSet<_> = numbers.into_iter().collect(); // Order not guaranteed
```

### Crypto Module

### Q: Are the crypto functions secure for passwords?

**A:** The hashing functions (MD5, SHA256) are **not suitable for password hashing**. Use proper password hashing libraries like `argon2` or `bcrypt` for passwords:

```rust
// DON'T do this for passwords
let bad_password_hash = sha256_hash("my_password");

// DO use proper password hashing (external crate)
// use argon2::{Argon2, PasswordHasher};
```

### Q: When should I use each encoding function?

**A:**
- **Base64**: Binary data in text format (APIs, data URLs)
- **URL encoding**: Query parameters, form data
- **Hex encoding**: Binary data as hexadecimal (debugging, checksums)

### Random Module

### Q: Are the random numbers cryptographically secure?

**A:** No, Reddish uses `rand::thread_rng()` which is suitable for general use but not cryptographic purposes. For cryptographic randomness, use `rand::rngs::OsRng`.

### Q: How do I get reproducible random results?

**A:** The functions use thread-local RNG. For reproducible results, you'd need to use the `rand` crate directly with a seeded RNG.

### DateTime Module

### Q: What timezone do the functions use?

**A:** All functions work with UTC (`DateTime<Utc>`). Convert to local time using chrono's timezone functions:

```rust
use chrono::{Local, TimeZone};

let utc_time = Utc::now();
let local_time = utc_time.with_timezone(&Local);
```

### Q: What date formats does `parse_date` support?

**A:** It supports common formats:
- ISO 8601: `"2023-12-25T15:30:00Z"`
- Simple datetime: `"2023-12-25 15:30:00"`
- Date only: `"2023-12-25"`
- Various separators: `"25/12/2023"`, `"12-25-2023"`, etc.

## 🚀 Performance Questions

### Q: Are Reddish functions fast?

**A:** Yes, they're designed for performance:
- Most operations are O(n) or better
- Zero-cost abstractions where possible
- Efficient algorithms (e.g., Fisher-Yates shuffle)
- See [PERFORMANCE.md](PERFORMANCE.md) for detailed complexity analysis

### Q: Should I worry about memory usage?

**A:** Reddish functions are memory-efficient:
- Minimal allocations
- Functions that can work in-place do so (like `shuffle`)
- Clear memory usage patterns
- No memory leaks

## 🔧 Development Questions

### Q: How do I contribute to Reddish?

**A:** See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed guidelines. In summary:
1. Fork the repository
2. Create a feature branch
3. Add tests for your changes
4. Submit a pull request

### Q: How do I report bugs?

**A:** Create an issue on GitHub with:
- Clear description of the problem
- Minimal code example that reproduces the issue
- Your environment details (Rust version, OS)

### Q: Can I request new functions?

**A:** Yes! Create a feature request issue with:
- Description of the use case
- Examples of how it would be used
- Why existing functions don't solve the problem

## 🔍 Troubleshooting

### Q: I'm getting compilation errors about missing features

**A:** Make sure you've enabled the required features:

```toml
# If using crypto functions
reddish = { version = "0.2.0", features = ["crypto"] }

# If using datetime functions
reddish = { version = "0.2.0", features = ["datetime"] }

# Or enable all features (default)
reddish = "0.2.0"
```

### Q: Functions aren't available even with correct imports

**A:** Check that you're importing from the correct module:

```rust
// Correct - imports all functions
use reddish::*;

// Also correct - specific imports
use reddish::{capitalize, find_index, keys};

// Won't work - modules aren't public
use reddish::string::capitalize; // Error
```

### Q: Random functions give the same results every time

**A:** This shouldn't happen with normal usage. If it does:
1. Make sure you're not accidentally seeding the RNG
2. Check if you're in a special environment (some containers/VMs)
3. Verify you're using the latest version

### Q: DateTime parsing fails for valid dates

**A:** `parse_date` tries common formats but isn't exhaustive. For specific formats, use chrono directly:

```rust
use chrono::{DateTime, Utc, NaiveDateTime};

// For custom formats, use chrono directly
let custom_format = "%d-%b-%Y %H:%M";
let parsed = NaiveDateTime::parse_from_str("25-Dec-2023 15:30", custom_format)
    .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc));
```

## 📚 Learning Resources

### Q: Where can I learn more about functional programming patterns?

**A:** Great resources:
- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust documentation
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [Lodash Documentation](https://lodash.com/docs/) - Similar patterns in JavaScript
- [Functional Programming in Rust](https://github.com/rust-lang/rfcs/blob/master/text/0199-ownership-variants.md)

### Q: How do I learn more about the algorithms used?

**A:** Check out:
- [PERFORMANCE.md](PERFORMANCE.md) - Complexity analysis
- [ARCHITECTURE.md](ARCHITECTURE.md) - Design decisions
- Source code - Well-documented implementations
- Algorithm textbooks for deeper understanding

## 🤝 Community

### Q: Is there a community forum or chat?

**A:** Currently, the best places for discussion are:
- GitHub Issues for bugs and feature requests
- GitHub Discussions for general questions
- Rust community forums for broader Rust questions

### Q: How can I stay updated on new releases?

**A:**
- Watch the GitHub repository
- Check [CHANGELOG.md](../CHANGELOG.md) for version history
- Follow releases on crates.io

---

Don't see your question here? Feel free to ask in GitHub Discussions or create an issue!
