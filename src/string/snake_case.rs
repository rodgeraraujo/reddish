#![allow(dead_code)]

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
