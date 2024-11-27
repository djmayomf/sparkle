use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::error::Result;

pub struct Base64Utils;

impl Base64Utils {
    /// Encode a string to base64
    pub fn encode(input: &str) -> String {
        STANDARD.encode(input.as_bytes())
    }

    /// Encode bytes to base64
    pub fn encode_bytes(input: &[u8]) -> String {
        STANDARD.encode(input)
    }

    /// Decode base64 to string
    pub fn decode_to_string(input: &str) -> Result<String> {
        let bytes = STANDARD.decode(input)?;
        String::from_utf8(bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64 to string: {}", e))
    }

    /// Decode base64 to bytes
    pub fn decode_to_bytes(input: &str) -> Result<Vec<u8>> {
        STANDARD.decode(input)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_string() {
        let original = "Hello, World!";
        let encoded = Base64Utils::encode(original);
        let decoded = Base64Utils::decode_to_string(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_encode_decode_bytes() {
        let original = vec![1, 2, 3, 4, 5];
        let encoded = Base64Utils::encode_bytes(&original);
        let decoded = Base64Utils::decode_to_bytes(&encoded).unwrap();
        assert_eq!(original, decoded);
    }
} 