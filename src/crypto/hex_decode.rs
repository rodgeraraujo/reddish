#![allow(dead_code)]

/// Decodes a hexadecimal string. Returns None if the input is invalid.
///
/// ```
/// let result = reddish::hex_decode("68656c6c6f");
/// assert_eq!(result, Some("hello".to_string()));
/// ```
///
/// ```
/// let result = reddish::hex_decode("48656c6c6f20576f726c6421");
/// assert_eq!(result, Some("Hello World!".to_string()));
/// ```
///
/// ```
/// let result = reddish::hex_decode("invalid hex");
/// assert_eq!(result, None);
/// ```
pub fn hex_decode(data: &str) -> Option<String> {
    match hex::decode(data) {
        Ok(bytes) => String::from_utf8(bytes).ok(),
        Err(_) => None,
    }
}
