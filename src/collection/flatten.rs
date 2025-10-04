#![allow(dead_code)]

/// Flattens a nested vector into a single-level vector.
///
/// ```
/// let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
/// let result = reddish::flatten(&nested);
/// assert_eq!(result, vec![1, 2, 3, 4, 5]);
/// ```
///
/// ```
/// let nested = vec![vec!["a", "b"], vec!["c"], vec!["d", "e"]];
/// let result = reddish::flatten(&nested);
/// assert_eq!(result, vec!["a", "b", "c", "d", "e"]);
/// ```
pub fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter()
        .flat_map(|inner| inner.iter())
        .cloned()
        .collect()
}
