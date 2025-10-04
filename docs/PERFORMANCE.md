# Performance Guide 🚀

This document outlines the performance characteristics of Reddish functions and provides guidance for optimal usage.

## 📊 Performance Overview

Reddish is designed for high performance with Rust's zero-cost abstractions. Most functions have optimal time complexity for their operations.

## ⏱️ Time Complexity Reference

### String Module

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `capitalize` | O(n) | O(n) | Single pass through string |
| `camel_case` | O(n) | O(n) | Processes each character once |
| `snake_case` | O(n) | O(n) | Single pass with case detection |
| `kebab_case` | O(n) | O(n) | Single pass with case detection |
| `title_case` | O(n) | O(n) | Single pass with word boundary detection |
| `pad` | O(n + p) | O(n + p) | n = string length, p = padding |
| `pad_end` | O(n + p) | O(n + p) | n = string length, p = padding |
| `truncate` | O(min(n, len)) | O(min(n, len)) | Early termination possible |

### Array Module

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `concat` | O(n + m) | O(n + m) | n, m = array lengths |
| `difference` | O(n × m) | O(n) | Could be optimized with HashSet |
| `find_index` | O(n) | O(1) | Early termination on match |
| `find_last_index` | O(n) | O(1) | Reverse iteration |
| `join` | O(n × s) | O(n × s) | n = elements, s = avg string length |

### Object Module

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `keys` | O(n) | O(n) | Iterates through all entries |
| `values` | O(n) | O(n) | Iterates through all entries |
| `entries` | O(n) | O(n) | Iterates through all entries |
| `has_key` | O(1) avg | O(1) | HashMap lookup |
| `pick` | O(k) | O(k) | k = number of keys to pick |
| `omit` | O(n) | O(n - k) | n = total keys, k = keys to omit |
| `merge` | O(n + m) | O(n + m) | n, m = map sizes |

### Collection Module

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `chunk` | O(n) | O(n) | Single pass, creates chunks |
| `flatten` | O(n) | O(n) | n = total elements after flattening |
| `group_by` | O(n) | O(n) | HashMap operations |
| `unique` | O(n) | O(n) | Uses HashSet for deduplication |
| `partition` | O(n) | O(n) | Single pass, two output vectors |
| `zip` | O(min(n, m)) | O(min(n, m)) | Limited by shorter array |
| `count_by` | O(n) | O(k) | k = number of unique keys |

### Crypto Module

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `md5_hash` | O(n) | O(1) | n = input length |
| `sha256_hash` | O(n) | O(1) | n = input length |
| `base64_encode` | O(n) | O(4n/3) | Standard Base64 expansion |
| `base64_decode` | O(n) | O(3n/4) | Standard Base64 compression |
| `url_encode` | O(n) | O(n) | Worst case: all chars encoded |
| `url_decode` | O(n) | O(n) | Single pass decoding |
| `hex_encode` | O(n) | O(2n) | Each byte becomes 2 hex chars |
| `hex_decode` | O(n) | O(n/2) | Each 2 hex chars become 1 byte |

### Random Module

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `random_int` | O(1) | O(1) | Single RNG call |
| `random_float` | O(1) | O(1) | Single RNG call |
| `random_choice` | O(1) | O(1) | Direct indexing |
| `shuffle` | O(n) | O(1) | Fisher-Yates algorithm, in-place |
| `sample` | O(n) | O(n) | Creates new vector |
| `random_string` | O(n) | O(n) | n = desired length |
| `uuid` | O(1) | O(1) | Fixed-size generation |
| `random_bool` | O(1) | O(1) | Single RNG call |
| `random_bool_with_probability` | O(1) | O(1) | Single RNG call |

### DateTime Module

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `format_duration` | O(1) | O(1) | Fixed number of operations |
| `time_ago` | O(1) | O(1) | Simple arithmetic |
| `is_weekend` | O(1) | O(1) | Single comparison |
| `days_between` | O(1) | O(1) | Chrono handles complexity |
| `add_days` | O(1) | O(1) | Chrono handles complexity |
| `start_of_week` | O(1) | O(1) | Fixed calculations |
| `end_of_month` | O(1) | O(1) | Chrono handles complexity |
| `parse_date` | O(k) | O(1) | k = number of format attempts |
| `format_date` | O(n) | O(n) | n = format string length |
| `format_date_human` | O(1) | O(1) | Fixed format |
| `format_date_iso` | O(1) | O(1) | Fixed format |

## 🏎️ Performance Tips

### General Guidelines

1. **Use References**: Pass large data structures by reference
   ```rust
   // Good
   let result = find_index(&large_vec, |x| x > 100);

   // Avoid (unnecessary clone)
   let result = find_index(&large_vec.clone(), |x| x > 100);
   ```

2. **Reuse Allocations**: When processing multiple items
   ```rust
   // Good - reuse string buffer
   let mut buffer = String::with_capacity(1000);
   for item in items {
       buffer.clear();
       buffer.push_str(&capitalize(item));
       // use buffer
   }
   ```

3. **Choose Appropriate Functions**: Use the most specific function
   ```rust
   // Good - O(1) lookup
   if has_key(&map, &key) { /* ... */ }

   // Less efficient - O(n) iteration
   if keys(&map).contains(&&key) { /* ... */ }
   ```

### Module-Specific Tips

#### String Operations
- **Pre-allocate**: Use `String::with_capacity()` when building strings
- **Avoid Repeated Conversions**: Cache converted strings when used multiple times
- **Consider `&str` vs `String`**: Use `&str` for temporary operations

#### Array Operations
- **Early Termination**: Use `find_index` instead of filtering then checking
- **Batch Operations**: Process multiple items together when possible
- **Memory Layout**: Consider using `Vec` vs `&[T]` based on usage

#### Collection Operations
- **Chunking Strategy**: Choose chunk size based on cache locality
- **Grouping Keys**: Use efficient key functions for `group_by`
- **Unique Operations**: Consider if you need ordering (use `unique` vs `HashSet`)

#### Object Operations
- **Key Selection**: Use `pick` with small key sets, `omit` with large exclusions
- **Merge Strategy**: Consider which map should be the base for merging
- **Lookup Patterns**: Batch key existence checks when possible

#### Crypto Operations
- **Batch Hashing**: Hash larger chunks rather than many small pieces
- **Encoding Choice**: Choose appropriate encoding for your use case
- **Caching**: Cache hash results for frequently accessed data

#### Random Operations
- **Seeding**: Use consistent seeds for reproducible results
- **Batch Generation**: Generate multiple random values in one call when possible
- **Shuffle vs Sample**: Use `shuffle` for in-place, `sample` for new collection

#### DateTime Operations
- **Format Caching**: Cache format strings for repeated use
- **Timezone Awareness**: Use UTC for calculations, local time for display
- **Parsing Strategy**: Try most common formats first in `parse_date`

## 📈 Benchmarking

### Running Benchmarks

```bash
# Install cargo-criterion for better benchmarks
cargo install cargo-criterion

# Run benchmarks (if implemented)
cargo criterion

# Profile specific functions
cargo build --release
perf record --call-graph=dwarf ./target/release/examples/string
perf report
```

### Creating Benchmarks

When contributing performance improvements, include benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reddish::*;

fn benchmark_capitalize(c: &mut Criterion) {
    let input = "hello world".repeat(1000);
    c.bench_function("capitalize", |b| {
        b.iter(|| capitalize(black_box(&input)))
    });
}

criterion_group!(benches, benchmark_capitalize);
criterion_main!(benches);
```

## 🔍 Performance Monitoring

### Key Metrics to Watch

1. **Memory Allocations**: Minimize unnecessary allocations
2. **CPU Usage**: Profile hot paths
3. **Cache Efficiency**: Consider data locality
4. **Branch Prediction**: Minimize unpredictable branches

### Profiling Tools

- **`cargo flamegraph`**: Visual profiling
- **`perf`**: Linux performance analysis
- **`Instruments`**: macOS profiling
- **`cargo-profdata`**: LLVM-based profiling

## ⚡ Optimization Examples

### Before and After Optimizations

#### String Padding Optimization
```rust
// Before: Multiple allocations
pub fn pad_slow(s: &str, length: usize, pad_char: char) -> String {
    let mut result = String::new();
    let pad_needed = length.saturating_sub(s.len());
    let left_pad = pad_needed / 2;
    let right_pad = pad_needed - left_pad;

    for _ in 0..left_pad {
        result.push(pad_char);
    }
    result.push_str(s);
    for _ in 0..right_pad {
        result.push(pad_char);
    }
    result
}

// After: Single allocation with capacity
pub fn pad(s: &str, length: usize, pad_char: char) -> String {
    if s.len() >= length {
        return s.to_string();
    }

    let mut result = String::with_capacity(length);
    let pad_needed = length - s.len();
    let left_pad = pad_needed / 2;
    let right_pad = pad_needed - left_pad;

    result.extend(std::iter::repeat(pad_char).take(left_pad));
    result.push_str(s);
    result.extend(std::iter::repeat(pad_char).take(right_pad));
    result
}
```

#### Array Difference Optimization
```rust
// Before: O(n × m) nested loops
pub fn difference_slow<T: PartialEq + Clone>(vec1: &[T], vec2: &[T]) -> Vec<T> {
    vec1.iter()
        .filter(|item| !vec2.contains(item))
        .cloned()
        .collect()
}

// After: O(n + m) with HashSet (for hashable types)
use std::collections::HashSet;
use std::hash::Hash;

pub fn difference<T: Eq + Hash + Clone>(vec1: &[T], vec2: &[T]) -> Vec<T> {
    let set2: HashSet<_> = vec2.iter().collect();
    vec1.iter()
        .filter(|item| !set2.contains(item))
        .cloned()
        .collect()
}
```

## 🎯 Performance Goals

### Target Performance Characteristics

1. **String Operations**: < 1ms for strings up to 10KB
2. **Array Operations**: < 1ms for arrays up to 10K elements
3. **Object Operations**: < 1ms for maps up to 1K entries
4. **Collection Operations**: < 10ms for collections up to 100K elements
5. **Crypto Operations**: Competitive with underlying libraries
6. **Random Operations**: < 1μs for single value generation
7. **DateTime Operations**: < 1ms for formatting/parsing

### Memory Usage Goals

1. **Minimal Overhead**: < 10% memory overhead for operations
2. **Predictable Allocation**: Clear memory usage patterns
3. **No Memory Leaks**: All allocations properly cleaned up
4. **Efficient Data Structures**: Use most appropriate containers

## 🔧 Contributing Performance Improvements

When contributing performance improvements:

1. **Measure First**: Profile before optimizing
2. **Benchmark Changes**: Include before/after benchmarks
3. **Document Trade-offs**: Explain any complexity increases
4. **Test Thoroughly**: Ensure correctness is maintained
5. **Consider All Cases**: Test with various input sizes

---

Performance is an ongoing concern. This document will be updated as the library evolves and new optimizations are discovered.
