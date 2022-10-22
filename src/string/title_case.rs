#![allow(dead_code)]

/// Will look to capitalize the first char of a string while converting the rest to lower case
///
/// ```
/// let result = reddish::title_case("foo-bar_baz-QUAX");
/// assert_eq!(result, "Foo-bar_baz-quax");
/// ```
///
/// ```
/// let result = reddish::title_case("FOO BAR");
/// assert_eq!(result, "Foo bar");
/// ```
pub fn title_case(s: &str) -> String {
  let mut chars = s.chars();
  match chars.next() {
    None => String::new(),
    Some(f) => f
      .to_uppercase()
      .chain(chars.flat_map(|t| t.to_lowercase()))
      .collect(),
  }
}
