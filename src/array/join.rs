#![allow(dead_code)]

pub fn join<T: Clone + ToString>(vec: Vec<T>, sep: &str) -> String {
  vec
    .iter()
    .map(|item| item.to_string())
    .collect::<Vec<String>>()
    .join(sep)
}
