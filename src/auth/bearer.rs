#![allow(dead_code)]
pub struct BearerAuth;

impl BearerAuth {
    pub fn create_header(token: &str) -> String {
        format!("Bearer {}", token)
    }
    
    pub fn extract_token(auth_header: &str) -> Option<String> {
        let parts: Vec<&str> = auth_header.split_whitespace().collect();
        if parts.len() != 2 || parts[0] != "Bearer" {
            return None;
        }
        Some(parts[1].to_string())
    }
    
    pub fn is_valid_format(token: &str) -> bool {
        !token.is_empty() && token.len() > 10
    }
}
