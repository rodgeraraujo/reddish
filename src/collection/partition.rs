#![allow(dead_code)]

/// Partitions a vector into two vectors based on a predicate function.
/// Returns a tuple where the first vector contains elements that satisfy the predicate,
/// and the second contains elements that don't.
///
/// ```
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// let (evens, odds) = reddish::partition(&numbers, |&n| n % 2 == 0);
/// assert_eq!(evens, vec![2, 4, 6]);
/// assert_eq!(odds, vec![1, 3, 5]);
/// ```
///
/// ```
/// let words = vec!["apple", "banana", "apricot", "cherry"];
/// let (starts_with_a, others) = reddish::partition(&words, |s| s.starts_with('a'));
/// assert_eq!(starts_with_a, vec!["apple", "apricot"]);
/// assert_eq!(others, vec!["banana", "cherry"]);
/// ```
pub fn partition<T, F>(vec: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let mut true_vec = Vec::new();
    let mut false_vec = Vec::new();

    for item in vec {
        if predicate(item) {
            true_vec.push(item.clone());
        } else {
            false_vec.push(item.clone());
        }
    }

    (true_vec, false_vec)
}
