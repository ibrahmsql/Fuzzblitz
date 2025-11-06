#![allow(dead_code)]
use base64::{engine::general_purpose, Engine as _};
use urlencoding::encode;

pub fn encode_payload(payload: &str, encoders: &[String]) -> String {
    let mut result = payload.to_string();
    
    for encoder in encoders {
        result = match encoder.to_lowercase().as_str() {
            "urlencode" | "url" => encode(&result).to_string(),
            "b64encode" | "base64" | "b64" => general_purpose::STANDARD.encode(result.as_bytes()),
            "hexencode" | "hex" => hex_encode(&result),
            "double-urlencode" | "double-url" => {
                let once = encode(&result).to_string();
                encode(&once).to_string()
            },
            _ => result, // Unknown encoder, skip
        };
    }
    
    result
}

fn hex_encode(s: &str) -> String {
    s.bytes()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub fn parse_encoder_spec(spec: &str) -> (String, Vec<String>) {
    // Format: "KEYWORD:encoder1,encoder2,encoder3"
    if let Some(pos) = spec.find(':') {
        let keyword = spec[..pos].to_string();
        let encoders: Vec<String> = spec[pos + 1..]
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        (keyword, encoders)
    } else {
        (spec.to_string(), vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        let result = encode_payload("hello world", &["urlencode".to_string()]);
        assert_eq!(result, "hello%20world");
    }

    #[test]
    fn test_base64_encode() {
        let result = encode_payload("hello", &["base64".to_string()]);
        assert_eq!(result, "aGVsbG8=");
    }

    #[test]
    fn test_hex_encode() {
        let result = encode_payload("ABC", &["hex".to_string()]);
        assert_eq!(result, "414243");
    }

    #[test]
    fn test_chained_encoders() {
        let result = encode_payload("hello world", &["urlencode".to_string(), "base64".to_string()]);
        // First URL encode: "hello%20world"
        // Then base64: "aGVsbG8lMjB3b3JsZA=="
        assert_eq!(result, "aGVsbG8lMjB3b3JsZA==");
    }
}
