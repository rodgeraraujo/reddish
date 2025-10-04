# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Comprehensive documentation improvements
- Contributing guidelines
- Enhanced README with detailed examples
- Better Cargo.toml metadata for crates.io

## [0.2.0] - 2025-10-04

### Added

- **New DateTime Module** with 11 utility functions:
  - `format_duration()` - Format seconds as human-readable duration
  - `time_ago()` - Relative time formatting (e.g., "2 hours ago")
  - `is_weekend()` - Check if date falls on weekend
  - `days_between()` - Calculate days between two dates
  - `add_days()` - Add/subtract days from a date
  - `start_of_week()` - Get start of week (Monday)
  - `end_of_month()` - Get last day of month
  - `parse_date()` - Parse various date string formats
  - `format_date()` - Custom date formatting
  - `format_date_human()` - Human-readable date format
  - `format_date_iso()` - ISO 8601 date format

- **New Random Module** with 9 utility functions:
  - `random_int()` - Generate random integers in range
  - `random_float()` - Generate random floats in range
  - `random_choice()` - Select random element from slice
  - `shuffle()` - Shuffle vector in place (Fisher-Yates)
  - `sample()` - Random sampling without replacement
  - `random_string()` - Generate random alphanumeric strings
  - `uuid()` - Generate UUID v4
  - `random_bool()` - Generate random boolean
  - `random_bool_with_probability()` - Weighted random boolean

- **New Crypto/Hash Module** with 8 utility functions:
  - `md5_hash()` - Compute MD5 hash
  - `sha256_hash()` - Compute SHA256 hash
  - `base64_encode()` / `base64_decode()` - Base64 encoding/decoding
  - `url_encode()` / `url_decode()` - URL encoding/decoding
  - `hex_encode()` / `hex_decode()` - Hexadecimal encoding/decoding

- **New Collection Module** with 7 utility functions:
  - `chunk()` - Split arrays into chunks
  - `flatten()` - Flatten nested arrays
  - `group_by()` - Group elements by key function
  - `unique()` - Get unique elements preserving order
  - `partition()` - Split array by predicate
  - `zip()` - Combine arrays into tuples
  - `count_by()` - Count elements by key function

- **New Object Module** with 7 utility functions:
  - `keys()` - Get HashMap keys
  - `values()` - Get HashMap values
  - `entries()` - Get key-value pairs
  - `has_key()` - Check if key exists
  - `pick()` - Create HashMap with selected keys
  - `omit()` - Create HashMap without selected keys
  - `merge()` - Merge two HashMaps

### Dependencies

- Added `chrono` for datetime operations
- Added `rand` and `uuid` for random generation
- Added `md5`, `sha2`, `base64`, `percent-encoding`, `hex` for crypto operations

### Features

- All modules are feature-gated for selective compilation
- Default features include all modules
- Comprehensive test coverage (100+ tests)
- Extensive documentation with examples
- Performance optimizations throughout

## [0.1.0] - 2023-XX-XX

### Added

- **String Module** with 8 utility functions:
  - `capitalize()` - Capitalize first character
  - `camel_case()` - Convert to camelCase
  - `snake_case()` - Convert to snake_case
  - `kebab_case()` - Convert to kebab-case
  - `title_case()` - Convert to Title Case
  - `pad()` - Pad string to length
  - `pad_end()` - Pad string at end
  - `truncate()` - Truncate string to length

- **Array Module** with 5 utility functions:
  - `concat()` - Concatenate arrays
  - `difference()` - Array difference
  - `find_index()` - Find first matching index
  - `find_last_index()` - Find last matching index
  - `join()` - Join array elements to string

### Features

- Feature-gated modules (`string`, `array`)
- Comprehensive test suite
- Documentation with examples
- Zero external dependencies for core functionality

### Infrastructure

- MIT License
- Cargo.toml configuration
- Basic README
- Test infrastructure

---

## Release Notes

### Version 0.2.0 Highlights

This major release significantly expands Reddish with **40+ new utility functions** across 5 new modules:

- **🔐 Crypto/Hash**: Secure hashing and encoding utilities
- **🎲 Random**: Random number generation and sampling
- **📅 DateTime**: Comprehensive date/time manipulation
- **📚 Collection**: Advanced array processing functions
- **🗂️ Object**: HashMap utilities for object-like operations

The library now provides **55+ utility functions** total, making it a comprehensive toolkit for Rust developers.

### Breaking Changes

None. All existing APIs remain unchanged and backward compatible.

### Migration Guide

No migration needed. Existing code using v0.1.0 will work unchanged with v0.2.0.

To use new features, simply update your `Cargo.toml`:

```toml
[dependencies]
reddish = "0.2.0"
```

Or enable specific features:

```toml
[dependencies]
reddish = { version = "0.2.0", features = ["string", "datetime", "crypto"] }
```

### Performance Improvements

- Optimized algorithms throughout (e.g., Fisher-Yates shuffle)
- Memory-efficient implementations
- Zero-cost abstractions where possible
- Compile-time optimizations

### Documentation Improvements

- Comprehensive rustdoc for all functions
- 98 doctests ensuring examples work
- Detailed README with usage examples
- Contributing guidelines
- This changelog

---

## Upcoming Features

Planned for future releases:

- **Math Module**: Mathematical utilities and statistics
- **Validation Module**: Data validation functions
- **File Module**: File system utilities
- **Network Module**: HTTP and networking helpers
- **Async Module**: Async/await utilities

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
