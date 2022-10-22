#![allow(dead_code)]

/// Concatenates two vectors together
///
/// ```
/// let v1 = ["foo"; 2].to_vec();
/// let v2 = ["bar"; 2].to_vec();
/// assert_eq!(reddish::concat(v1, v2), ["foo", "foo", "bar", "bar"]);
/// ```
pub fn concat<T: Clone>(vec: Vec<T>, values: Vec<T>) -> Vec<T> {
  let mut v = vec.clone();
  v.extend(values);
  v
}
