#![allow(dead_code)]

use base64::{Engine as _, engine::general_purpose};

/// Encodes the input string to Base64.
///
/// ```
/// let result = reddish::base64_encode("hello world");
/// assert_eq!(result, "aGVsbG8gd29ybGQ=");
/// ```
///
/// ```
/// let result = reddish::base64_encode("The quick brown fox jumps over the lazy dog");
/// assert_eq!(result, "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==");
/// ```
pub fn base64_encode(data: &str) -> String {
    general_purpose::STANDARD.encode(data.as_bytes())
}
