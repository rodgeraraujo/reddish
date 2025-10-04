# Reddish Examples 🦀

This directory contains comprehensive examples demonstrating how to use each module in the Reddish utility library.

## 🚀 Running Examples

You can run any example using Cargo:

```bash
# Run a specific example
cargo run --example string
cargo run --example array
cargo run --example object
cargo run --example collection
cargo run --example crypto
cargo run --example random
cargo run --example datetime

# Run all examples
for example in string array object collection crypto random datetime; do
    echo "=== Running $example example ==="
    cargo run --example $example
    echo
done
```

## 📚 Available Examples

### 🔤 String Examples (`string.rs`)
Demonstrates string manipulation utilities:
- Text case conversions (camelCase, snake_case, kebab-case, Title Case)
- String padding and truncation
- Capitalization
- Practical use cases for text processing

**Run with:** `cargo run --example string`

### 📊 Array Examples (`array.rs`)
Shows array manipulation and search functions:
- Finding elements and indices
- Array concatenation and differences
- Joining arrays into strings
- Real-world data processing scenarios

**Run with:** `cargo run --example array`

### 🗂️ Object Examples (`object.rs`)
Demonstrates HashMap/object-like operations:
- Extracting keys, values, and entries
- Checking for key existence
- Selecting and omitting keys
- Merging objects
- User data management examples

**Run with:** `cargo run --example object`

### 📚 Collection Examples (`collection.rs`)
Advanced collection processing:
- Chunking arrays into smaller pieces
- Flattening nested structures
- Grouping elements by criteria
- Finding unique elements
- Partitioning data
- Combining arrays with zip
- Counting elements by categories

**Run with:** `cargo run --example collection`

### 🔐 Crypto Examples (`crypto.rs`)
Cryptographic and encoding utilities:
- MD5 and SHA256 hashing
- Base64 encoding/decoding
- URL encoding/decoding
- Hexadecimal encoding/decoding
- Security and data integrity examples

**Run with:** `cargo run --example crypto`

### 🎲 Random Examples (`random.rs`)
Random number generation and sampling:
- Random integers and floats
- Random element selection
- Array shuffling
- Random sampling without replacement
- UUID generation
- Random string generation
- Boolean generation with probabilities
- Gaming and simulation examples

**Run with:** `cargo run --example random`

### 📅 DateTime Examples (`datetime.rs`)
Date and time manipulation:
- Duration formatting
- Relative time display ("2 hours ago")
- Weekend detection
- Date arithmetic (adding/subtracting days)
- Week and month boundaries
- Date parsing from various formats
- Custom date formatting
- Logging and scheduling examples

**Run with:** `cargo run --example datetime`

## 🎯 Example Structure

Each example file follows a consistent structure:

1. **Import statements** - Shows what to import from Reddish
2. **Basic usage** - Simple examples of each function
3. **Practical scenarios** - Real-world use cases
4. **Edge cases** - How functions handle special inputs
5. **Combined usage** - Using multiple functions together

## 💡 Learning Path

If you're new to Reddish, we recommend exploring examples in this order:

1. **String** - Start with familiar text operations
2. **Array** - Learn basic data manipulation
3. **Collection** - Explore advanced data processing
4. **Object** - Understand key-value operations
5. **Random** - Add randomness to your applications
6. **Crypto** - Secure your data with hashing and encoding
7. **DateTime** - Master time-based operations

## 🔧 Customizing Examples

Feel free to modify these examples to experiment with different inputs and scenarios:

```bash
# Copy an example to experiment with
cp examples/string.rs my_string_experiment.rs

# Edit the file with your own test cases
# Then run it with:
cargo run --bin my_string_experiment
```

## 📖 Code Snippets

Here are some quick snippets you can try:

### Quick String Processing
```rust
use reddish::*;

fn main() {
    let text = "hello world";
    println!("Original: {}", text);
    println!("Capitalized: {}", capitalize(text));
    println!("CamelCase: {}", camel_case(text));
    println!("Snake_case: {}", snake_case("Hello World"));
}
```

### Data Processing Pipeline
```rust
use reddish::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Process data through multiple steps
    let chunks = chunk(&numbers, 3);
    let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    let unique_data = unique(&numbers);

    println!("Chunks: {:?}", chunks);
    println!("Evens: {:?}", evens);
    println!("Unique: {:?}", unique_data);
}
```

### Random Data Generation
```rust
use reddish::*;

fn main() {
    println!("Random number: {}", random_int(1, 100));
    println!("Random UUID: {}", uuid());

    let colors = vec!["red", "green", "blue", "yellow"];
    if let Some(color) = random_choice(&colors) {
        println!("Random color: {}", color);
    }
}
```

## 🧪 Testing Examples

All examples are tested to ensure they work correctly:

```bash
# Test that examples compile and run without errors
cargo test --examples

# Run examples with different feature combinations
cargo run --example string --no-default-features --features string
cargo run --example crypto --no-default-features --features crypto
```

## 🤝 Contributing Examples

If you have ideas for improving these examples or want to add new ones:

1. Follow the existing structure and style
2. Include practical, real-world scenarios
3. Add comments explaining the logic
4. Test your examples thoroughly
5. Submit a pull request

See [CONTRIBUTING.md](../CONTRIBUTING.md) for more details.

## 📚 Additional Resources

- [Main README](../README.md) - Library overview and installation
- [API Documentation](https://docs.rs/reddish) - Detailed function documentation
- [Contributing Guide](../CONTRIBUTING.md) - How to contribute to the project
- [Changelog](../CHANGELOG.md) - Version history and changes

---

Happy coding with Reddish! 🦀✨
