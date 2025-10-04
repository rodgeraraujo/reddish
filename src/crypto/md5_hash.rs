#![allow(dead_code)]

/// Computes the MD5 hash of the input data and returns it as a hexadecimal string.
///
/// ```
/// let result = reddish::md5_hash("hello world");
/// assert_eq!(result, "5eb63bbbe01eeed093cb22bb8f5acdc3");
/// ```
///
/// ```
/// let result = reddish::md5_hash("");
/// assert_eq!(result, "d41d8cd98f00b204e9800998ecf8427e");
/// ```
pub fn md5_hash(data: &str) -> String {
    format!("{:x}", md5::compute(data.as_bytes()))
}
