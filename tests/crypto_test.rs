extern crate reddish;
use reddish::{md5_hash, sha256_hash, base64_encode, base64_decode, url_encode, url_decode, hex_encode, hex_decode};

#[test]
fn test_md5_hash() {
    assert_eq!(md5_hash("hello world"), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    assert_eq!(md5_hash(""), "d41d8cd98f00b204e9800998ecf8427e");
    assert_eq!(md5_hash("The quick brown fox jumps over the lazy dog"), "9e107d9d372bb6826bd81d3542a419d6");
}

#[test]
fn test_md5_hash_unicode() {
    assert_eq!(md5_hash("café"), "07117fe4a1ebd544965dc19573183da2");
    assert_eq!(md5_hash("🦀"), "cce906c42b6c90f0cd516a0b3bae0e3e");
}

#[test]
fn test_sha256_hash() {
    assert_eq!(sha256_hash("hello world"), "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    assert_eq!(sha256_hash(""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(sha256_hash("abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

#[test]
fn test_sha256_hash_unicode() {
    // Test with Unicode characters
    let result = sha256_hash("café");
    assert_eq!(result.len(), 64); // SHA256 always produces 64 hex characters
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_base64_encode() {
    assert_eq!(base64_encode("hello world"), "aGVsbG8gd29ybGQ=");
    assert_eq!(base64_encode(""), "");
    assert_eq!(base64_encode("f"), "Zg==");
    assert_eq!(base64_encode("fo"), "Zm8=");
    assert_eq!(base64_encode("foo"), "Zm9v");
    assert_eq!(base64_encode("The quick brown fox jumps over the lazy dog"), "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==");
}

#[test]
fn test_base64_encode_unicode() {
    assert_eq!(base64_encode("café"), "Y2Fmw6k=");
    assert_eq!(base64_encode("🦀"), "8J+mgA==");
}

#[test]
fn test_base64_decode() {
    assert_eq!(base64_decode("aGVsbG8gd29ybGQ="), Some("hello world".to_string()));
    assert_eq!(base64_decode(""), Some("".to_string()));
    assert_eq!(base64_decode("Zg=="), Some("f".to_string()));
    assert_eq!(base64_decode("Zm8="), Some("fo".to_string()));
    assert_eq!(base64_decode("Zm9v"), Some("foo".to_string()));
    assert_eq!(base64_decode("VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw=="), Some("The quick brown fox jumps over the lazy dog".to_string()));
}

#[test]
fn test_base64_decode_invalid() {
    assert_eq!(base64_decode("invalid base64!"), None);
    assert_eq!(base64_decode("aGVsbG8gd29ybGQ"), None); // Missing padding
}

#[test]
fn test_base64_roundtrip() {
    let original = "Hello, World! 🦀 café";
    let encoded = base64_encode(original);
    let decoded = base64_decode(&encoded);
    assert_eq!(decoded, Some(original.to_string()));
}

#[test]
fn test_url_encode() {
    assert_eq!(url_encode("hello world"), "hello%20world");
    assert_eq!(url_encode("hello@example.com"), "hello%40example%2Ecom");
    assert_eq!(url_encode(""), "");
    assert_eq!(url_encode("abc123"), "abc123");
    assert_eq!(url_encode("hello & goodbye"), "hello%20%26%20goodbye");
}

#[test]
fn test_url_encode_special_chars() {
    assert_eq!(url_encode("café"), "caf%C3%A9");
    assert_eq!(url_encode("100%"), "100%25");
    assert_eq!(url_encode("a+b=c"), "a%2Bb%3Dc");
}

#[test]
fn test_url_decode() {
    assert_eq!(url_decode("hello%20world"), Some("hello world".to_string()));
    assert_eq!(url_decode("hello%40example%2Ecom"), Some("hello@example.com".to_string()));
    assert_eq!(url_decode(""), Some("".to_string()));
    assert_eq!(url_decode("abc123"), Some("abc123".to_string()));
    assert_eq!(url_decode("hello%20%26%20goodbye"), Some("hello & goodbye".to_string()));
}

#[test]
fn test_url_decode_unicode() {
    assert_eq!(url_decode("caf%C3%A9"), Some("café".to_string()));
    assert_eq!(url_decode("100%25"), Some("100%".to_string()));
    assert_eq!(url_decode("a%2Bb%3Dc"), Some("a+b=c".to_string()));
}

#[test]
fn test_url_roundtrip() {
    let original = "hello world & café 100%";
    let encoded = url_encode(original);
    let decoded = url_decode(&encoded);
    assert_eq!(decoded, Some(original.to_string()));
}

#[test]
fn test_hex_encode() {
    assert_eq!(hex_encode("hello"), "68656c6c6f");
    assert_eq!(hex_encode(""), "");
    assert_eq!(hex_encode("A"), "41");
    assert_eq!(hex_encode("Hello World!"), "48656c6c6f20576f726c6421");
}

#[test]
fn test_hex_encode_unicode() {
    assert_eq!(hex_encode("café"), "636166c3a9");
    assert_eq!(hex_encode("🦀"), "f09fa680");
}

#[test]
fn test_hex_decode() {
    assert_eq!(hex_decode("68656c6c6f"), Some("hello".to_string()));
    assert_eq!(hex_decode(""), Some("".to_string()));
    assert_eq!(hex_decode("41"), Some("A".to_string()));
    assert_eq!(hex_decode("48656c6c6f20576f726c6421"), Some("Hello World!".to_string()));
}

#[test]
fn test_hex_decode_invalid() {
    assert_eq!(hex_decode("invalid hex"), None);
    assert_eq!(hex_decode("68656c6c6"), None); // Odd number of characters
    assert_eq!(hex_decode("68656c6c6g"), None); // Invalid hex character
}

#[test]
fn test_hex_roundtrip() {
    let original = "Hello, World! 🦀 café";
    let encoded = hex_encode(original);
    let decoded = hex_decode(&encoded);
    assert_eq!(decoded, Some(original.to_string()));
}

#[test]
fn test_hex_case_insensitive() {
    assert_eq!(hex_decode("48656C6C6F"), Some("Hello".to_string()));
    assert_eq!(hex_decode("48656c6c6f"), Some("Hello".to_string()));
}

// Integration tests combining multiple functions
#[test]
fn test_crypto_chain() {
    let original = "sensitive data";

    // Hash -> Encode -> Decode chain
    let hash = sha256_hash(original);
    let encoded = base64_encode(&hash);
    let decoded = base64_decode(&encoded).unwrap();

    assert_eq!(decoded, hash);
    assert_eq!(decoded.len(), 64); // SHA256 hex length
}

#[test]
fn test_encoding_chain() {
    let original = "test data with special chars: café & 100%";

    // URL encode -> Base64 encode -> Hex encode -> reverse
    let url_encoded = url_encode(original);
    let b64_encoded = base64_encode(&url_encoded);
    let hex_encoded = hex_encode(&b64_encoded);

    let hex_decoded = hex_decode(&hex_encoded).unwrap();
    let b64_decoded = base64_decode(&hex_decoded).unwrap();
    let url_decoded = url_decode(&b64_decoded).unwrap();

    assert_eq!(url_decoded, original);
}
