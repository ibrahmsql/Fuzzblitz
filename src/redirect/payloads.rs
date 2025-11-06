#![allow(dead_code)]
pub struct RedirectPayloads;

impl RedirectPayloads {
    pub fn open_redirect_payloads() -> Vec<String> {
        vec![
            "//evil.com".to_string(),
            "https://evil.com".to_string(),
            "//google.com".to_string(),
            "/\\evil.com".to_string(),
            "///evil.com".to_string(),
            "////evil.com".to_string(),
            "https:evil.com".to_string(),
            "//evil%E3%80%82com".to_string(),
            "\\\\evil.com".to_string(),
            "/\\/\\/evil.com".to_string(),
        ]
    }
    
    pub fn parameter_names() -> Vec<String> {
        vec![
            "url", "redirect", "return", "next", "callback",
            "goto", "redir", "destination", "continue", "target",
            "link", "view", "out", "to", "returnTo",
        ].iter().map(|s| s.to_string()).collect()
    }
}
