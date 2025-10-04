#![allow(dead_code)]

use base64::{Engine as _, engine::general_purpose};

/// Decodes a Base64 encoded string. Returns None if the input is invalid.
///
/// ```
/// let result = reddish::base64_decode("aGVsbG8gd29ybGQ=");
/// assert_eq!(result, Some("hello world".to_string()));
/// ```
///
/// ```
/// let result = reddish::base64_decode("VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==");
/// assert_eq!(result, Some("The quick brown fox jumps over the lazy dog".to_string()));
/// ```
///
/// ```
/// let result = reddish::base64_decode("invalid base64!");
/// assert_eq!(result, None);
/// ```
pub fn base64_decode(data: &str) -> Option<String> {
    match general_purpose::STANDARD.decode(data) {
        Ok(bytes) => String::from_utf8(bytes).ok(),
        Err(_) => None,
    }
}
