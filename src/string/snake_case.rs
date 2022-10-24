#![allow(dead_code)]

/// Will convert a string into correctly formatted snake case using
/// special characters and capitalization as delimeters
///
/// ```
/// let result = reddish::snake_case("foo_barBaz_QuxQUUX");
/// assert_eq!(result, "foo_bar_baz_qux_quux");
/// ```
pub fn snake_case(str: &str) -> String {
  let mut result = String::new();
  let mut last_lower = false;

  for char in str.chars() {
    if char.is_ascii_uppercase() && last_lower {
      result.push('_');
    }
    result.push(char.to_ascii_lowercase());
    last_lower = char.is_lowercase();
  }

  result
}
