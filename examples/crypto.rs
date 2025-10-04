extern crate reddish;
use reddish::{md5_hash, sha256_hash, base64_encode, base64_decode, url_encode, url_decode, hex_encode, hex_decode};

fn main() {
    println!("=== Crypto/Hash Methods Examples ===\n");

    let sample_text = "Hello, World!";
    let _sensitive_data = "user@example.com";
    let special_chars = "café & résumé";

    // Demonstrate MD5 hashing
    let md5_result = md5_hash(sample_text);
    println!("MD5('{}') = {}", sample_text, md5_result);
    // MD5('Hello, World!') = 65a8e27d8879283831b664bd8b7f0ad4

    // Demonstrate SHA256 hashing
    let sha256_result = sha256_hash(sample_text);
    println!("SHA256('{}') = {}", sample_text, sha256_result);
    // SHA256('Hello, World!') = dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f

    println!();

    // Demonstrate Base64 encoding/decoding
    let base64_encoded = base64_encode(sample_text);
    println!("Base64 encode('{}') = {}", sample_text, base64_encoded);
    // Base64 encode('Hello, World!') = SGVsbG8sIFdvcmxkIQ==

    let base64_decoded = base64_decode(&base64_encoded);
    println!("Base64 decode('{}') = {:?}", base64_encoded, base64_decoded);
    // Base64 decode('SGVsbG8sIFdvcmxkIQ==') = Some("Hello, World!")

    println!();

    // Demonstrate URL encoding/decoding
    let url_encoded = url_encode(special_chars);
    println!("URL encode('{}') = {}", special_chars, url_encoded);
    // URL encode('café & résumé') = caf%C3%A9%20%26%20r%C3%A9sum%C3%A9

    let url_decoded = url_decode(&url_encoded);
    println!("URL decode('{}') = {:?}", url_encoded, url_decoded);
    // URL decode('caf%C3%A9%20%26%20r%C3%A9sum%C3%A9') = Some("café & résumé")

    println!();

    // Demonstrate Hex encoding/decoding
    let hex_encoded = hex_encode(sample_text);
    println!("Hex encode('{}') = {}", sample_text, hex_encoded);
    // Hex encode('Hello, World!') = 48656c6c6f2c20576f726c6421

    let hex_decoded = hex_decode(&hex_encoded);
    println!("Hex decode('{}') = {:?}", hex_encoded, hex_decoded);
    // Hex decode('48656c6c6f2c20576f726c6421') = Some("Hello, World!")

    println!();

    // Practical examples
    println!("=== Practical Use Cases ===");

    // Password hashing (for demonstration - use proper password hashing in production)
    let password = "mySecretPassword123";
    let password_hash = sha256_hash(password);
    println!("Password hash: {}", password_hash);

    // API token encoding
    let api_token = "secret-api-key-12345";
    let encoded_token = base64_encode(api_token);
    println!("Encoded API token: {}", encoded_token);

    // URL parameter encoding
    let search_query = "rust programming & web development";
    let encoded_query = url_encode(search_query);
    println!("Encoded search query: {}", encoded_query);

    // Data integrity check
    let important_data = "Critical system configuration";
    let data_checksum = md5_hash(important_data);
    println!("Data checksum (MD5): {}", data_checksum);

    // Binary data representation
    let binary_data = "Binary content with special chars: \x00\x01\x02\x7F";
    let hex_representation = hex_encode(binary_data);
    println!("Hex representation: {}", hex_representation);
}
