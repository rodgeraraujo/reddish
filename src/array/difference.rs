#![allow(dead_code)]

/// Finds the elements contained in `vec` but not in `values`.
///
/// ```
/// let vec = vec![1, 1, 2, 3, 4, 5, 5, 6, 7, 9];
/// let values = vec![3, 2, 3, 4, 5, 8, 5, 6, 7];
/// assert_eq!(reddish::difference(vec, values), [1, 9]);
/// ```
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
