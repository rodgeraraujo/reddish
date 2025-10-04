#![allow(dead_code)]

/// Combines two vectors into a vector of tuples.
/// The resulting vector will have the length of the shorter input vector.
///
/// ```
/// let vec1 = vec![1, 2, 3];
/// let vec2 = vec!["a", "b", "c"];
/// let result = reddish::zip(&vec1, &vec2);
/// assert_eq!(result, vec![(1, "a"), (2, "b"), (3, "c")]);
/// ```
///
/// ```
/// let vec1 = vec![1, 2, 3, 4];
/// let vec2 = vec!["a", "b"];
/// let result = reddish::zip(&vec1, &vec2);
/// assert_eq!(result, vec![(1, "a"), (2, "b")]);
/// ```
pub fn zip<T, U>(vec1: &[T], vec2: &[U]) -> Vec<(T, U)>
where
    T: Clone,
    U: Clone,
{
    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect()
}
