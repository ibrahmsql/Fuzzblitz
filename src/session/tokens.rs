#![allow(dead_code)]
pub struct TokenManager;

impl TokenManager {
    pub fn extract_from_headers(headers: &std::collections::HashMap<String, String>) -> Vec<String> {
        let mut tokens = Vec::new();
        
        for (key, value) in headers {
            let key_lower = key.to_lowercase();
            if key_lower.contains("token") || key_lower.contains("authorization") {
                tokens.push(value.clone());
            }
        }
        
        tokens
    }
    
    pub fn extract_from_body(body: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        
        let patterns = [
            r#""token":\s*"([^"]+)""#,
            r#""access_token":\s*"([^"]+)""#,
            r#""refresh_token":\s*"([^"]+)""#,
        ];
        
        for pattern in &patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for cap in re.captures_iter(body) {
                    if let Some(token) = cap.get(1) {
                        tokens.push(token.as_str().to_string());
                    }
                }
            }
        }
        
        tokens
    }
}
