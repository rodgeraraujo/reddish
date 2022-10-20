#[cfg(test)]
mod array_tests {
  use reddish;

  #[test]
  fn test_concat() {
    let vec_1 = [1; 15].to_vec();
    let vec_2 = [2; 15].to_vec();
    assert_eq!(reddish::concat(vec_1, vec_2).len(), 30);

    let vec_3 = ["foo"; 2].to_vec();
    let vec_4 = ["bar"; 2].to_vec();
    assert_eq!(reddish::concat(vec_3, vec_4), ["foo", "foo", "bar", "bar"]);
  }

  #[test]
  fn test_difference() {
    let vec = vec![1, 1, 2, 3, 4, 5, 5, 6, 7, 9];
    let values = vec![3, 2, 3, 4, 5, 8, 5, 6, 7];
    assert_eq!(reddish::difference(vec, values), [1, 9]);
  }

  #[test]
  fn test_find_index() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let find_value = |val| val == 3;
    assert_eq!(reddish::find_index(vec, find_value), 2);
  }

  #[test]
  fn test_find_last_index() {
    let vec: Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].to_vec();
    let find_value = |val| val == 7;
    assert_eq!(reddish::find_last_index(vec, find_value), 7);
  }

  #[test]
  fn test_join() {
    let vec_1 = [1; 10].to_vec();
    let vec_2 = ["foo".to_string(), "bar".to_string()].to_vec();

    assert_eq!(reddish::join(vec_1, ","), "1,1,1,1,1,1,1,1,1,1");
    assert_eq!(reddish::join(vec_2, "~"), "foo~bar");
    assert_eq!(
      reddish::join(
        ["f".to_string(), "o".to_string(), "o".to_string()].to_vec(),
        ""
      ),
      "foo"
    );
  }
}
