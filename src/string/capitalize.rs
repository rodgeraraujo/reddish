#![allow(dead_code)]

pub fn capitalize(str: &str) -> String {
  let mut chars = str.chars();
  match chars.next() {
    None => String::new(),
    Some(char) => char.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
  }
}
