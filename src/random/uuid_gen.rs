#![allow(dead_code)]

use uuid::Uuid;

/// Generates a random UUID v4.
///
/// ```
/// let result = reddish::uuid();
/// assert_eq!(result.len(), 36); // Standard UUID format length
/// assert!(result.contains('-')); // UUIDs contain hyphens
/// ```
///
/// ```
/// let uuid1 = reddish::uuid();
/// let uuid2 = reddish::uuid();
/// assert_ne!(uuid1, uuid2); // UUIDs should be unique
/// ```
pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}
