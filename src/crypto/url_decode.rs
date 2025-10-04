#![allow(dead_code)]

use percent_encoding::percent_decode_str;

/// URL decodes the input string. Returns None if the input is invalid.
///
/// ```
/// let result = reddish::url_decode("hello%20world");
/// assert_eq!(result, Some("hello world".to_string()));
/// ```
///
/// ```
/// let result = reddish::url_decode("hello%40example%2Ecom");
/// assert_eq!(result, Some("hello@example.com".to_string()));
/// ```
///
/// ```
/// let result = reddish::url_decode("caf%C3%A9");
/// assert_eq!(result, Some("café".to_string()));
/// ```
pub fn url_decode(data: &str) -> Option<String> {
    percent_decode_str(data).decode_utf8().ok().map(|s| s.to_string())
}
