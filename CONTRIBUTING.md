# Contributing to Reddish 🦀

Thank you for your interest in contributing to Reddish! We welcome contributions from everyone, whether you're fixing a bug, adding a new feature, improving documentation, or just asking questions.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)

### Development Setup

1. **Fork and clone the repository**

   ```bash
   git clone https://github.com/your-username/reddish.git
   cd reddish
   ```

2. **Install dependencies and build**

   ```bash
   cargo build --all-features
   ```

3. **Run tests to ensure everything works**

   ```bash
   cargo test --all-features
   ```

4. **Run examples to see the library in action**

   ```bash
   cargo run --example string
   cargo run --example array
   # ... etc
   ```

## 🛠️ Development Workflow

### Project Structure

```shell
reddish/
├── src/
│   ├── lib.rs              # Main library file with documentation
│   ├── string/             # String utility functions
│   ├── array/              # Array utility functions
│   ├── object/             # HashMap/object utility functions
│   ├── collection/         # Collection utility functions
│   ├── crypto/             # Cryptographic utility functions
│   ├── random/             # Random generation functions
│   └── datetime/           # Date/time utility functions
├── tests/                  # Integration tests
├── examples/               # Usage examples
├── docs/                   # Additional documentation
└── scripts/                # Development scripts
```

### Adding a New Function

When adding a new utility function, follow these steps:

1. **Choose the appropriate module** (string, array, object, collection, crypto, random, datetime)

2. **Create the function file** in the module directory:

   ```rust
   // src/module_name/function_name.rs
   #![allow(dead_code)]

   /// Brief description of what the function does.
   ///
   /// # Arguments
   ///
   /// * `param1` - Description of parameter 1
   /// * `param2` - Description of parameter 2
   ///
   /// # Returns
   ///
   /// Description of what the function returns
   ///
   /// # Examples
   ///
   /// ```
   /// use reddish::*;
   ///
   /// let result = your_function("input");
   /// assert_eq!(result, "expected_output");
   /// ```
   ///
   /// ```
   /// // Add more examples showing edge cases
   /// let result = your_function("");
   /// assert_eq!(result, "");
   /// ```
   pub fn your_function(param1: &str, param2: usize) -> String {
       // Implementation here
       todo!()
   }
   ```

3. **Add the function to the module's mod.rs**:

   ```rust
   mod your_function;
   pub use your_function::*;
   ```

4. **Write comprehensive tests** in `tests/module_name_test.rs`:

   ```rust
   #[test]
   fn test_your_function() {
       assert_eq!(your_function("input", 5), "expected");
   }

   #[test]
   fn test_your_function_edge_cases() {
       assert_eq!(your_function("", 0), "");
       // Test other edge cases
   }
   ```

5. **Add examples** to `examples/module_name.rs`

6. **Update documentation** if needed

### Code Style Guidelines

- **Follow Rust conventions**: Use `cargo fmt` to format your code
- **Use meaningful names**: Functions and variables should be descriptive
- **Write documentation**: Every public function must have rustdoc comments with examples
- **Include doctests**: Examples in documentation should be testable
- **Handle edge cases**: Consider empty inputs, boundary conditions, etc.
- **Use appropriate error handling**: Return `Option` or `Result` when operations can fail

### Testing Requirements

All contributions must include tests:

1. **Unit tests**: Test individual functions with various inputs
2. **Integration tests**: Test functions work together correctly
3. **Doctests**: Examples in documentation must be valid and testable
4. **Edge case tests**: Test boundary conditions and error cases

Run tests with:

```bash
# Run all tests
cargo test --all-features

# Run tests for specific module
cargo test --test string_test

# Run doctests
cargo test --doc

# Run with coverage (if you have cargo-tarpaulin installed)
cargo tarpaulin --all-features
```

### Performance Considerations

- **Avoid unnecessary allocations**: Use references where possible
- **Consider algorithmic complexity**: Document time/space complexity for non-trivial functions
- **Benchmark when relevant**: For performance-critical functions, include benchmarks
- **Use appropriate data structures**: Choose the most efficient data structure for the task

## 📝 Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: add new string utility function capitalize_words`
- `fix: handle empty string in camel_case function`
- `docs: update README with new examples`
- `test: add edge case tests for array functions`
- `refactor: improve performance of group_by function`
- `chore: update dependencies`

## 🔄 Pull Request Process

1. **Create a feature branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the guidelines above

3. **Test thoroughly**

   ```bash
   cargo test --all-features
   cargo clippy --all-features
   cargo fmt --check
   ```

4. **Update documentation** if needed

5. **Commit your changes** with descriptive commit messages

6. **Push to your fork**

   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request** with:
   - Clear title and description
   - Reference any related issues
   - Include examples of the new functionality
   - Describe any breaking changes

### Pull Request Checklist

- [ ] Code follows the project's style guidelines
- [ ] Self-review of the code has been performed
- [ ] Code is commented, particularly in hard-to-understand areas
- [ ] Corresponding changes to documentation have been made
- [ ] Changes generate no new warnings
- [ ] Tests have been added that prove the fix is effective or feature works
- [ ] New and existing unit tests pass locally
- [ ] Any dependent changes have been merged and published

## 🐛 Reporting Bugs

When reporting bugs, please include:

1. **Clear bug description**: What happened vs. what you expected
2. **Reproduction steps**: Minimal code example that reproduces the issue
3. **Environment details**: Rust version, OS, etc.
4. **Error messages**: Full error output if applicable

Use the bug report template in GitHub Issues.

## 💡 Suggesting Features

For feature requests, please:

1. **Check existing issues** to avoid duplicates
2. **Describe the use case**: Why would this feature be useful?
3. **Provide examples**: Show how the feature would be used
4. **Consider alternatives**: Are there existing ways to achieve this?

## 📚 Documentation

Documentation improvements are always welcome:

- Fix typos or unclear explanations
- Add more examples
- Improve rustdoc comments
- Update README or other markdown files

## 🏷️ Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality in a backwards compatible manner
- **PATCH**: Backwards compatible bug fixes

## 📄 License

By contributing to Reddish, you agree that your contributions will be licensed under the MIT License.

## 🤝 Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code:

- **Be respectful**: Treat everyone with respect and kindness
- **Be inclusive**: Welcome newcomers and help them learn
- **Be constructive**: Provide helpful feedback and suggestions
- **Be patient**: Remember that everyone has different experience levels

## 🆘 Getting Help

If you need help:

1. **Check the documentation**: README, rustdoc, examples
2. **Search existing issues**: Your question might already be answered
3. **Ask in discussions**: Use GitHub Discussions for questions
4. **Create an issue**: For bugs or feature requests

## 🙏 Recognition

Contributors will be recognized in:

- The project's README
- Release notes for significant contributions
- The project's contributors page

Thank you for contributing to Reddish! 🦀✨
