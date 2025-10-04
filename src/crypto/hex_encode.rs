#![allow(dead_code)]

/// Encodes the input string to hexadecimal.
///
/// ```
/// let result = reddish::hex_encode("hello");
/// assert_eq!(result, "68656c6c6f");
/// ```
///
/// ```
/// let result = reddish::hex_encode("Hello World!");
/// assert_eq!(result, "48656c6c6f20576f726c6421");
/// ```
pub fn hex_encode(data: &str) -> String {
    hex::encode(data.as_bytes())
}
