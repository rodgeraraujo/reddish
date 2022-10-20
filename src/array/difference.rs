#![allow(dead_code)]

pub fn difference<T: Clone + Eq>(vec: Vec<T>, values: Vec<T>) -> Vec<T> {
  // deduplicate vec and values array
  let mut base_values: Vec<T> = vec.clone();
  let mut exclude_values: Vec<T> = values.clone();

  base_values.dedup();
  exclude_values.dedup();

  // collect difference from right to left
  base_values.retain(|item| !exclude_values.contains(item));

  base_values
}
