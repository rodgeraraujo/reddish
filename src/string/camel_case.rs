///Converts a string from snake to camel case.
///
/// Treats `_` as indicator to capitilze the next word.
/// ```
/// let result = reddish::camel_case("foo_barBaz_QuxQUUX");
/// assert_eq!(result, "FooBarBazQuxQUUX");
/// ```
pub fn camel_case(s: &str) -> String {
  let mut capitalize_next = true;
  let mut result = String::new();
  result.reserve(s.len());

  for c in s.chars() {
    if c == '_' {
      capitalize_next = true;
    } else if capitalize_next {
      result.push(c.to_ascii_uppercase());
      capitalize_next = false;
    } else {
      result.push(c);
    }
  }

  result
}
