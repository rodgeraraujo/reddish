use std::ops::Fn;

/// Finds the index of the first occurance of the predicate `find`.
///
/// ```
/// let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
/// let find_value = |val| val == 3;
/// assert_eq!(reddish::find_index(vec, find_value), 2);
/// ```
pub fn find_index<T: Clone, F>(vec: Vec<T>, find: F) -> usize
where
  F: Fn(T) -> bool,
{
  vec.into_iter().position(|el| find(el)).unwrap()
}

/// Finds the index of the last occurance of the predicate `find`.
///
/// ```
/// let vec: Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].to_vec();
/// let find_value = |val| val == 7;
/// assert_eq!(reddish::find_last_index(vec, find_value), 7);
/// ```
pub fn find_last_index<T: Clone, F>(vec: Vec<T>, find: F) -> usize
where
  F: Fn(T) -> bool,
{
  let mut result = vec.clone();

  result.reverse();
  result.len() - result.into_iter().position(|el| find(el)).unwrap()
}
