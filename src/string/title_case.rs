#![allow(dead_code)]

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
