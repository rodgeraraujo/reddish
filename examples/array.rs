extern crate reddish;
use reddish::{concat, difference, find_index, join};

fn main() {
  println!("{:?}", concat([1; 5].to_vec(), [2; 5].to_vec()));
  // [1, 1, 1, 1, 1, 2, 2, 2, 2, 2]
  println!("{:?}", difference(vec![1, 1, 2, 3], vec![2, 3, 4]));
  // [1]
  let find_value = |val| val == 3;
  println!("{}", find_index([1, 1, 2, 3].to_vec(), find_value));
  // 3
  println!("{}", join(["f", "o", "o", "b", "a", "r"].to_vec(), ""));
  // foobar
}
