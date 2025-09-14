use regex::Regex;

/// Zero-Width Non-Joiner (U+200C), used to represent a '0' bit.
const I_0: &str = "\u{200C}";
/// Zero-Width Joiner (U+200D), used to represent a '1' bit.
const I_1: &str = "\u{200D}";
/// The expected byte size of the above Unicode characters in UTF-8.
const EXP_SIZE: usize = 3;

/// Encodes a single byte into a sequence of zero-width characters.
/// Each bit of the byte is converted into either I_0 (for 0) or I_1 (for 1).
///
/// # Arguments
///
/// * `byte` - The u8 byte to encode.
///
/// # Returns
///
/// A `String` containing 8 zero-width characters representing the byte.
fn encode_byte(byte: u8) -> String {
    let mut result = String::with_capacity(8 * EXP_SIZE);
    for i in 0..8 {
        // Check the i-th bit of the byte
        if (byte >> i) & 1 == 1 {
            result.push_str(I_1);
        } else {
            result.push_str(I_0);
        }
    }
    result
}

/// Decodes a sequence of 8 zero-width characters back into a single byte.
///
/// # Arguments
///
/// * `data` - A string slice expected to contain 8 zero-width characters.
///
/// # Returns
///
/// An `Option<u8>` containing the decoded byte if successful, or `None` if the
/// input string has an incorrect length.
fn decode_byte(data: &str) -> Option<u8> {
    if data.len() != 8 * EXP_SIZE {
        return None;
    }

    let mut result: u8 = 0;
    let bytes = data.as_bytes();

    // Iterate over the string in chunks of `EXP_SIZE` bytes.
    for (i, chunk) in bytes.chunks_exact(EXP_SIZE).enumerate() {
        if chunk == I_1.as_bytes() {
            // Set the i-th bit of the result byte
            result |= 1 << i;
        }
    }
    Some(result)
}

/// Encodes a string slice into a sequence of zero-width characters.
///
/// # Arguments
///
/// * `data` - The string slice to encode.
///
/// # Returns
///
/// A `String` containing the full encoded message.
fn encode(data: &str) -> String {
    data.bytes().map(encode_byte).collect()
}

/// Decodes a string of zero-width characters back into the original string.
///
/// # Arguments
///
/// * `data` - The encoded string of zero-width characters.
///
/// # Returns
///
/// An `Option<String>` containing the decoded string if successful, or `None` if the
/// input is malformed (e.g., wrong length, invalid UTF-8).
fn decode(data: &str) -> Option<String> {
    if data.len() % (8 * EXP_SIZE) != 0 {
        return None;
    }

    let bytes: Option<Vec<u8>> = data
        .as_bytes()
        .chunks_exact(8 * EXP_SIZE)
        .map(|chunk| {
            // The chunk must be valid UTF-8 to be decoded as a str
            let s = std::str::from_utf8(chunk).ok()?;
            decode_byte(s)
        })
        .collect();

    // from_utf8 converts the vector of bytes back into a String.
    // This can also fail if the resulting bytes are not valid UTF-8.
    bytes.and_then(|b| String::from_utf8(b).ok())
}

/// Filters a string, returning only the zero-width characters used for encoding.
///
/// # Arguments
///
/// * `data` - The string containing mixed regular text and zero-width characters.
///
/// # Returns
///
/// A `String` containing only the `I_0` and `I_1` characters.
fn remove_unnecessary_symbols(data: &str) -> String {
    // Using a regex is more robust than byte-wise iteration from the C++ version.
    // It correctly handles all Unicode characters, not just ASCII.
    let re = Regex::new(&format!("[{}|{}]", I_0, I_1)).unwrap();
    re.find_iter(data).map(|mat| mat.as_str()).collect()
}

/// High-level function to encode a secret message. Alias for `encode`.
pub fn create_secret(normal_str: &str, secret: &str) -> String {
    let mut mid = normal_str.len() / 2;
    while !normal_str.is_char_boundary(mid) {
        mid -= 1;
    }

    let hidden_content = encode(secret);
    format!("{}{}{}", &normal_str[..mid], hidden_content, &normal_str[mid..])
}

/// High-level function to find and decode a secret message from a larger string.
pub fn extract_secret(message: &str) -> Option<String> {
    let filtered = remove_unnecessary_symbols(message);
    decode(&filtered)
}


// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_correct_size() {
        assert_eq!(I_0.len(), EXP_SIZE);
        assert_eq!(I_1.len(), EXP_SIZE);
    }

    #[test]
    fn test_encode_decode_byte() {
        println!("Testing encode_decode_byte");
        let c = 'a';
        let result = encode_byte(c as u8);

        assert_eq!(result.len(), 8 * EXP_SIZE);
        
        let decoded_char = decode_byte(&result).expect("Decoding failed");
        assert_eq!(decoded_char, c as u8);
    }

    #[test]
    fn test_encode_decode_ascii() {
        println!("Testing encode_decode_ascii");
        let data = "Hello, World!";
        let encoded = encode(data);
        let decoded = decode(&encoded).expect("Decoding failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_encode_decode_other() {
        println!("Testing encode_decode_other");
        // Cyrillic characters to test multi-byte UTF-8 handling
        let data = "ДАРОВА БРАТВА!";
        let encoded = encode(data);
        let decoded = decode(&encoded).expect("Decoding failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_full_pipeline() {
        println!("Testing full pipeline");
        let secret = "суперsecret";
        let mixed_message = create_secret("Hello world", secret);
        
        let extracted_secret = extract_secret(&mixed_message).expect("Extraction failed");
        assert_eq!(secret, extracted_secret);
    }
    
    #[test]
    fn test_extraction_from_complex_text() {
        println!("Testing extraction from complex text");
        let secret = "TopSecret123";
        
        let message = create_secret("Это тест, а вот и продолжение спрятанного сообщения", secret);

        let extracted = extract_secret(&message).expect("Extraction failed");
        assert_eq!(secret, extracted);
    }
}
