extern crate reddish;
use reddish::{chunk, flatten, group_by, unique, partition, zip, count_by};

#[test]
fn test_chunk() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let result = chunk(&vec, 3);
    assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
}

#[test]
fn test_chunk_exact_division() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let result = chunk(&vec, 2);
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
}

#[test]
fn test_chunk_empty() {
    let vec: Vec<i32> = vec![];
    let result = chunk(&vec, 3);
    assert_eq!(result, Vec::<Vec<i32>>::new());
}

#[test]
fn test_chunk_zero_size() {
    let vec = vec![1, 2, 3];
    let result = chunk(&vec, 0);
    assert_eq!(result, Vec::<Vec<i32>>::new());
}

#[test]
fn test_flatten() {
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let result = flatten(&nested);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_flatten_empty() {
    let nested: Vec<Vec<i32>> = vec![];
    let result = flatten(&nested);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_flatten_with_empty_inner() {
    let nested = vec![vec![1, 2], vec![], vec![3, 4]];
    let result = flatten(&nested);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_group_by() {
    let vec = vec!["apple", "banana", "apricot", "blueberry"];
    let result = group_by(&vec, |s| s.chars().next().unwrap());

    assert_eq!(result.get(&'a').unwrap().len(), 2);
    assert_eq!(result.get(&'b').unwrap().len(), 2);
    assert!(result.get(&'a').unwrap().contains(&&"apple"));
    assert!(result.get(&'a').unwrap().contains(&&"apricot"));
    assert!(result.get(&'b').unwrap().contains(&&"banana"));
    assert!(result.get(&'b').unwrap().contains(&&"blueberry"));
}

#[test]
fn test_group_by_numbers() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let result = group_by(&numbers, |&n| n % 2);

    assert_eq!(result.get(&0).unwrap(), &vec![&2, &4, &6]);
    assert_eq!(result.get(&1).unwrap(), &vec![&1, &3, &5]);
}

#[test]
fn test_group_by_empty() {
    let vec: Vec<i32> = vec![];
    let result = group_by(&vec, |&n| n % 2);
    assert!(result.is_empty());
}

#[test]
fn test_unique() {
    let vec = vec![1, 2, 2, 3, 1, 4, 3];
    let result = unique(&vec);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_unique_strings() {
    let vec = vec!["apple", "banana", "apple", "cherry", "banana"];
    let result = unique(&vec);
    assert_eq!(result, vec!["apple", "banana", "cherry"]);
}

#[test]
fn test_unique_empty() {
    let vec: Vec<i32> = vec![];
    let result = unique(&vec);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_unique_no_duplicates() {
    let vec = vec![1, 2, 3, 4];
    let result = unique(&vec);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_partition() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (evens, odds) = partition(&numbers, |&n| n % 2 == 0);
    assert_eq!(evens, vec![2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5]);
}

#[test]
fn test_partition_strings() {
    let words = vec!["apple", "banana", "apricot", "cherry"];
    let (starts_with_a, others) = partition(&words, |s| s.starts_with('a'));
    assert_eq!(starts_with_a, vec!["apple", "apricot"]);
    assert_eq!(others, vec!["banana", "cherry"]);
}

#[test]
fn test_partition_empty() {
    let vec: Vec<i32> = vec![];
    let (true_vec, false_vec) = partition(&vec, |&n| n > 0);
    assert_eq!(true_vec, Vec::<i32>::new());
    assert_eq!(false_vec, Vec::<i32>::new());
}

#[test]
fn test_partition_all_true() {
    let vec = vec![1, 2, 3];
    let (true_vec, false_vec) = partition(&vec, |&n| n > 0);
    assert_eq!(true_vec, vec![1, 2, 3]);
    assert_eq!(false_vec, Vec::<i32>::new());
}

#[test]
fn test_partition_all_false() {
    let vec = vec![1, 2, 3];
    let (true_vec, false_vec) = partition(&vec, |&n| n > 10);
    assert_eq!(true_vec, Vec::<i32>::new());
    assert_eq!(false_vec, vec![1, 2, 3]);
}

#[test]
fn test_zip() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec!["a", "b", "c"];
    let result = zip(&vec1, &vec2);
    assert_eq!(result, vec![(1, "a"), (2, "b"), (3, "c")]);
}

#[test]
fn test_zip_different_lengths() {
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec!["a", "b"];
    let result = zip(&vec1, &vec2);
    assert_eq!(result, vec![(1, "a"), (2, "b")]);
}

#[test]
fn test_zip_empty_first() {
    let vec1: Vec<i32> = vec![];
    let vec2 = vec!["a", "b", "c"];
    let result = zip(&vec1, &vec2);
    assert_eq!(result, Vec::<(i32, &str)>::new());
}

#[test]
fn test_zip_empty_second() {
    let vec1 = vec![1, 2, 3];
    let vec2: Vec<&str> = vec![];
    let result = zip(&vec1, &vec2);
    assert_eq!(result, Vec::<(i32, &str)>::new());
}

#[test]
fn test_count_by() {
    let words = vec!["apple", "banana", "apricot", "blueberry"];
    let result = count_by(&words, |s| s.chars().next().unwrap());

    assert_eq!(result.get(&'a'), Some(&2));
    assert_eq!(result.get(&'b'), Some(&2));
}

#[test]
fn test_count_by_numbers() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let result = count_by(&numbers, |&n| n % 2);

    assert_eq!(result.get(&0), Some(&3)); // even numbers
    assert_eq!(result.get(&1), Some(&3)); // odd numbers
}

#[test]
fn test_count_by_empty() {
    let vec: Vec<i32> = vec![];
    let result = count_by(&vec, |&n| n % 2);
    assert!(result.is_empty());
}

#[test]
fn test_count_by_single_group() {
    let vec = vec![2, 4, 6, 8];
    let result = count_by(&vec, |&n| n % 2);

    assert_eq!(result.get(&0), Some(&4));
    assert_eq!(result.get(&1), None);
}
