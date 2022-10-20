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
