#![allow(dead_code)]

use sha2::{Sha256, Digest};

/// Computes the SHA256 hash of the input data and returns it as a hexadecimal string.
///
/// ```
/// let result = reddish::sha256_hash("hello world");
/// assert_eq!(result, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
/// ```
///
/// ```
/// let result = reddish::sha256_hash("");
/// assert_eq!(result, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
/// ```
pub fn sha256_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}
