/// Pads both ends of a string up to `pad_length`. If `pad_char` is `None` it defaults to space `' '`.
///
/// ```
/// let result = reddish::pad("Foo Bar", 1, None);
/// assert_eq!(result, " Foo Bar ");
/// ```
///
/// ```
/// let result = reddish::pad("Foo Bar", 1, Some('*'));
/// assert_eq!(result, "*Foo Bar*");
/// ```
pub fn pad(s: &str, pad_length: usize, pad_char: Option<char>) -> String {
  let pad_char = pad_char.unwrap_or(' ');
  let mut result = String::new();
  result.reserve(s.len() + (pad_length * 2) as usize);

  for _ in 0..pad_length {
    result.push(pad_char);
  }
  result += s;
  for _ in 0..pad_length {
    result.push(pad_char);
  }
  result
}
