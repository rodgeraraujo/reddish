use std::ops::Fn;

pub fn find_index<T: Clone, F>(vec: Vec<T>, find: F) -> usize
where
  F: Fn(T) -> bool,
{
  vec.into_iter().position(|el| find(el)).unwrap()
}

pub fn find_last_index<T: Clone, F>(vec: Vec<T>, find: F) -> usize
where
  F: Fn(T) -> bool,
{
  let mut result = vec.clone();

  result.reverse();
  result.len() - result.into_iter().position(|el| find(el)).unwrap()
}
