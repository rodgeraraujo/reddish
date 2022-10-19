#![allow(dead_code)]

pub fn kebab_case(str: &str) -> String {
  let mut result = String::new();
  let mut is_last_special_char = false;
  let mut is_last_small_char = false;

  for (_index, char) in str.chars().enumerate() {
    if char.is_alphanumeric() {
      if is_last_special_char {
        if result.len() > 0 {
          result.push('-');
        }

        is_last_special_char = false;
      } else if is_last_small_char && char.is_uppercase() && result.len() > 0 {
        result.push('-');
      }

      result.push(char.to_ascii_lowercase());

      if char.is_lowercase() {
        is_last_small_char = true;
      } else {
        is_last_small_char = false;
      }
    } else {
      is_last_special_char = true;
    }
  }

  return result;
}
