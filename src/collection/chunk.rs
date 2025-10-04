#![allow(dead_code)]

/// Splits a vector into chunks of the specified size.
///
/// ```
/// let vec = vec![1, 2, 3, 4, 5, 6, 7];
/// let result = reddish::chunk(&vec, 3);
/// assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
/// ```
///
/// ```
/// let vec = vec!["a", "b", "c", "d"];
/// let result = reddish::chunk(&vec, 2);
/// assert_eq!(result, vec![vec!["a", "b"], vec!["c", "d"]]);
/// ```
pub fn chunk<T: Clone>(vec: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return vec![];
    }

    vec.chunks(size)
        .map(|chunk| chunk.to_vec())
        .collect()
}
