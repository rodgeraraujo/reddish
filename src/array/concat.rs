#![allow(dead_code)]

pub fn concat<T: Clone>(vec: Vec<T>, values: Vec<T>) -> Vec<T> {
  let mut v = vec.clone();
  v.extend(values);
  v
}
