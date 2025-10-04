#![allow(dead_code)]

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

/// URL encodes the input string.
///
/// ```
/// let result = reddish::url_encode("hello world");
/// assert_eq!(result, "hello%20world");
/// ```
///
/// ```
/// let result = reddish::url_encode("hello@example.com");
/// assert_eq!(result, "hello%40example%2Ecom");
/// ```
///
/// ```
/// let result = reddish::url_encode("café");
/// assert_eq!(result, "caf%C3%A9");
/// ```
pub fn url_encode(data: &str) -> String {
    utf8_percent_encode(data, NON_ALPHANUMERIC).to_string()
}
