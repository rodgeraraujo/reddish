# Reddish Architecture 🏗️

This document describes the architecture and design decisions behind the Reddish utility library.

## 🎯 Design Goals

Reddish was designed with the following principles in mind:

1. **Modularity** - Functions are organized into logical modules with feature gates
2. **Performance** - Zero-cost abstractions and efficient algorithms
3. **Safety** - Leverage Rust's type system and memory safety
4. **Usability** - Simple, intuitive APIs inspired by popular JavaScript libraries
5. **Flexibility** - Support for different use cases and selective compilation

## 🏛️ Overall Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Reddish Library                      │
├─────────────────────────────────────────────────────────────┤
│  lib.rs (Main entry point with comprehensive documentation) │
├─────────────────────────────────────────────────────────────┤
│  Feature-Gated Modules:                                    │
│  ┌─────────┬─────────┬─────────┬─────────┬─────────┬───────┐ │
│  │ string  │  array  │ object  │collection│ crypto │random │ │
│  └─────────┴─────────┴─────────┴─────────┴─────────┴───────┘ │
│  ┌─────────┐                                                │
│  │datetime │                                                │
│  └─────────┘                                                │
├─────────────────────────────────────────────────────────────┤
│  External Dependencies (Optional):                          │
│  • chrono (datetime)                                       │
│  • rand, uuid (random)                                     │
│  • md5, sha2, base64, percent-encoding, hex (crypto)       │
└─────────────────────────────────────────────────────────────┘
```

## 📦 Module Structure

Each module follows a consistent structure:

```
src/module_name/
├── mod.rs              # Module declaration and re-exports
├── function1.rs        # Individual function implementation
├── function2.rs        # Individual function implementation
└── ...
```

### Module Organization Principles

1. **Single Responsibility** - Each file contains one function
2. **Clear Naming** - Function names are descriptive and follow Rust conventions
3. **Comprehensive Documentation** - Every function has rustdoc with examples
4. **Consistent API** - Similar functions have similar signatures across modules

## 🔧 Feature System

Reddish uses Cargo's feature system for modular compilation:

```toml
[features]
default = ["array", "string", "object", "collection", "crypto", "random", "datetime"]
array = []
string = []
object = []
collection = []
crypto = []
random = []
datetime = []
```

### Benefits of Feature Gates

1. **Reduced Binary Size** - Include only what you need
2. **Faster Compilation** - Compile only required modules
3. **Dependency Management** - Optional dependencies only when needed
4. **Flexibility** - Different configurations for different use cases

## 🧩 Module Details

### String Module
- **Purpose**: Text manipulation and formatting
- **Dependencies**: None (uses only std)
- **Key Functions**: Case conversions, padding, truncation
- **Design**: Immutable operations returning new strings

### Array Module
- **Purpose**: Basic array operations and searching
- **Dependencies**: None (uses only std)
- **Key Functions**: Finding, concatenation, difference
- **Design**: Generic over types, efficient algorithms

### Object Module
- **Purpose**: HashMap utilities for object-like operations
- **Dependencies**: None (uses std::collections::HashMap)
- **Key Functions**: Key/value extraction, merging, filtering
- **Design**: Generic over key-value types with trait bounds

### Collection Module
- **Purpose**: Advanced collection processing
- **Dependencies**: None (uses only std)
- **Key Functions**: Grouping, chunking, partitioning
- **Design**: Functional programming patterns, iterator-based

### Crypto Module
- **Purpose**: Cryptographic hashing and encoding
- **Dependencies**: md5, sha2, base64, percent-encoding, hex
- **Key Functions**: Hashing, encoding/decoding
- **Design**: Safe wrappers around crypto libraries

### Random Module
- **Purpose**: Random number generation and sampling
- **Dependencies**: rand, uuid
- **Key Functions**: Random generation, sampling, shuffling
- **Design**: Thread-safe random generation, efficient algorithms

### DateTime Module
- **Purpose**: Date and time manipulation
- **Dependencies**: chrono
- **Key Functions**: Formatting, parsing, arithmetic
- **Design**: UTC-based, comprehensive format support

## 🔍 Design Patterns

### Error Handling Strategy

1. **Option for Fallible Operations**: Functions that can fail return `Option<T>`
   ```rust
   pub fn parse_date(date_str: &str) -> Option<DateTime<Utc>>
   ```

2. **Panic for Invalid Input**: Functions panic on clearly invalid input
   ```rust
   pub fn random_int(min: i32, max: i32) -> i32 {
       assert!(min <= max, "min must be <= max");
   }
   ```

3. **Graceful Degradation**: Handle edge cases gracefully
   ```rust
   pub fn capitalize(s: &str) -> String {
       if s.is_empty() { return String::new(); }
       // ... handle empty string case
   }
   ```

### Memory Management

1. **Minimize Allocations**: Use references where possible
2. **Efficient Data Structures**: Choose appropriate containers
3. **Clone When Necessary**: Explicit cloning for owned data
4. **Iterator Patterns**: Lazy evaluation where beneficial

### Generic Programming

```rust
// Generic over types with appropriate trait bounds
pub fn find_index<T, F>(vec: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    // Implementation
}
```

## 🚀 Performance Considerations

### Algorithm Choices

1. **String Operations**: Efficient string building with capacity hints
2. **Array Operations**: Linear time complexity where possible
3. **Shuffling**: Fisher-Yates algorithm for unbiased shuffling
4. **Hashing**: Use of established, fast hashing libraries

### Optimization Strategies

1. **Compile-Time Optimizations**: Leverage Rust's compiler
2. **Zero-Cost Abstractions**: No runtime overhead for abstractions
3. **Memory Layout**: Efficient data structure choices
4. **Algorithmic Complexity**: Document and optimize time/space complexity

## 🧪 Testing Strategy

### Test Organization

```
tests/
├── array_test.rs       # Integration tests for array module
├── string_test.rs      # Integration tests for string module
├── object_test.rs      # Integration tests for object module
├── collection_test.rs  # Integration tests for collection module
├── crypto_test.rs      # Integration tests for crypto module
├── random_test.rs      # Integration tests for random module
└── datetime_test.rs    # Integration tests for datetime module
```

### Test Types

1. **Unit Tests**: Test individual functions in isolation
2. **Integration Tests**: Test module functionality together
3. **Doctests**: Ensure documentation examples work
4. **Property Tests**: Test function properties (where applicable)
5. **Edge Case Tests**: Test boundary conditions

### Test Coverage Goals

- **100% Function Coverage**: Every public function tested
- **Edge Case Coverage**: Empty inputs, boundary values
- **Error Path Coverage**: Test error conditions
- **Documentation Coverage**: All examples in docs tested

## 📚 Documentation Strategy

### Documentation Levels

1. **Crate Level** (`lib.rs`): Overview, quick start, examples
2. **Module Level** (`mod.rs`): Module purpose and organization
3. **Function Level**: Individual function documentation
4. **Example Level**: Practical usage demonstrations

### Documentation Standards

1. **Rustdoc Format**: Standard Rust documentation format
2. **Examples Required**: Every public function has examples
3. **Testable Examples**: All examples are doctests
4. **Clear Descriptions**: Explain what, why, and how

## 🔄 Development Workflow

### Code Organization

1. **Feature Branches**: Each new feature in separate branch
2. **Atomic Commits**: Small, focused commits
3. **Conventional Commits**: Standardized commit messages
4. **Pull Request Reviews**: Code review before merging

### Quality Assurance

1. **Automated Testing**: CI runs all tests
2. **Code Formatting**: `cargo fmt` for consistent style
3. **Linting**: `cargo clippy` for code quality
4. **Documentation**: `cargo doc` for doc generation

## 🔮 Future Architecture Considerations

### Planned Improvements

1. **Async Support**: Async versions of I/O operations
2. **WASM Compatibility**: WebAssembly target support
3. **No-Std Support**: Core functionality without std
4. **Const Functions**: Compile-time evaluation where possible

### Scalability Considerations

1. **Module Growth**: How to handle growing module size
2. **Dependency Management**: Keeping dependencies minimal
3. **API Stability**: Maintaining backward compatibility
4. **Performance Monitoring**: Benchmarking critical functions

## 🤝 Contributing to Architecture

When contributing to Reddish's architecture:

1. **Follow Existing Patterns**: Maintain consistency
2. **Document Decisions**: Explain architectural choices
3. **Consider Performance**: Profile performance-critical changes
4. **Maintain Modularity**: Keep modules focused and independent

## 📖 References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book - Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [Lodash Documentation](https://lodash.com/docs/) (inspiration)
- [Ramda Documentation](https://ramdajs.com/docs/) (inspiration)

---

This architecture document is living documentation that evolves with the project. Please keep it updated as the codebase grows and changes.
