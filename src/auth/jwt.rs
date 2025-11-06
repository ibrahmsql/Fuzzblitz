#![allow(dead_code)]
use base64::{Engine as _, engine::general_purpose};

pub struct JwtHelper;

impl JwtHelper {
    pub fn decode_parts(token: &str) -> Option<(String, String, String)> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        
        let header = Self::decode_base64(parts[0])?;
        let payload = Self::decode_base64(parts[1])?;
        let signature = parts[2].to_string();
        
        Some((header, payload, signature))
    }
    
    fn decode_base64(input: &str) -> Option<String> {
        let padded = match input.len() % 4 {
            2 => format!("{}==", input),
            3 => format!("{}=", input),
            _ => input.to_string(),
        };
        
        let decoded = general_purpose::STANDARD.decode(padded).ok()?;
        String::from_utf8(decoded).ok()
    }
    
    pub fn is_jwt(token: &str) -> bool {
        token.split('.').count() == 3
    }
    
    pub fn extract_claims(token: &str) -> Option<String> {
        let (_, payload, _) = Self::decode_parts(token)?;
        Some(payload)
    }
    
    pub fn generate_none_algorithm_token(payload: &str) -> String {
        let header = r#"{"alg":"none","typ":"JWT"}"#;
        let header_b64 = general_purpose::URL_SAFE_NO_PAD.encode(header);
        let payload_b64 = general_purpose::URL_SAFE_NO_PAD.encode(payload);
        format!("{}.{}.", header_b64, payload_b64)
    }
}
