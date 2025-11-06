use std::collections::HashMap;

pub struct CorsTester;

impl CorsTester {
    pub fn test_origins() -> Vec<String> {
        vec![
            "null".to_string(),
            "http://evil.com".to_string(),
            "https://evil.com".to_string(),
            "http://localhost".to_string(),
            "http://127.0.0.1".to_string(),
        ]
    }
    
    pub fn generate_test_headers(origin: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), origin.to_string());
        headers.insert("Access-Control-Request-Method".to_string(), "GET".to_string());
        headers.insert("Access-Control-Request-Headers".to_string(), "Content-Type".to_string());
        headers
    }
    
    pub fn is_vulnerable(response_headers: &HashMap<String, String>) -> bool {
        if let Some(acao) = response_headers.get("access-control-allow-origin") {
            if acao == "*" || acao.contains("evil.com") || acao == "null" {
                return true;
            }
        }
        
        if let Some(acac) = response_headers.get("access-control-allow-credentials") {
            if acac.to_lowercase() == "true" {
                if let Some(acao) = response_headers.get("access-control-allow-origin") {
                    if acao != "null" && !acao.is_empty() {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}
