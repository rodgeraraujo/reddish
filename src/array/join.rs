#![allow(dead_code)]

/// Joins all elements of `vec` into a `String` separated by `sep`.
///
/// ```
/// let v = [1; 10].to_vec();
///
/// assert_eq!(reddish::join(v, ","), "1,1,1,1,1,1,1,1,1,1");
/// ```
///
/// ```
/// assert_eq!(
///   reddish::join(
///     ["f".to_string(), "o".to_string(), "o".to_string()].to_vec(),
///     ""
///   ),
///   "foo"
/// );
/// ```
pub fn join<T: Clone + ToString>(vec: Vec<T>, sep: &str) -> String {
  vec
    .iter()
    .map(|item| item.to_string())
    .collect::<Vec<String>>()
    .join(sep)
}
