/// Truncates a string to the desired `truncate_len`
///
/// ```
/// let result = reddish::truncate("Foo", 1);
/// assert_eq!(result, "Fo");
/// ```
///
/// ```
/// let result = reddish::truncate("Foo", 4);
/// assert_eq!(result, "");
/// ```
pub fn truncate(s: &str, truncate_len: usize) -> String {
  if truncate_len >= s.len() {
    return String::from("");
  }

  let mut result = String::new();
  result.reserve(truncate_len);

  let target = s.len() - truncate_len;
  for (i, c) in s.char_indices() {
    if i == target {
      break;
    }
    result.push(c);
  }
  result
}
