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
