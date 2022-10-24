#![allow(dead_code)]

/// Will attempt to transform the first char in a string to uppercase and convert
/// the rest of the string to lower case.
///
/// ```
/// let result = reddish::capitalize("foo bar");
/// assert_eq!(result, "Foo bar");
/// ```
///
/// ```
/// let result = reddish::capitalize("__FOO_BAR__");
/// assert_eq!(result, "__foo_bar__");
/// ```
pub fn capitalize(str: &str) -> String {
  let mut chars = str.chars();
  match chars.next() {
    None => String::new(),
    Some(char) => char.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
  }
}
