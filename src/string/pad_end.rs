/// Pads characters onto the end of the string up to `pad_length`. If the `pad_char`is `None`
/// the `pad_char` defaults to space `' '`.
///
/// ```
/// let result = reddish::pad_end("Foo Bar", 1, None);
/// assert_eq!(result, "Foo Bar ");
/// ```
///
/// ```
/// let result = reddish::pad_end("Foo Bar", 1, Some('*'));
/// assert_eq!(result, "Foo Bar*");
/// ```
pub fn pad_end(s: &str, pad_length: usize, pad_char: Option<char>) -> String {
  let pad_char = pad_char.unwrap_or(' ');
  let mut result = String::new();
  result.reserve(s.len() + pad_length as usize);

  result += s;
  for _ in 0..pad_length {
    result.push(pad_char);
  }
  result
}
