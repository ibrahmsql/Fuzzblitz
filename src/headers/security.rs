#![allow(dead_code)]
use std::collections::HashMap;

pub struct SecurityHeaders;

impl SecurityHeaders {
    pub fn check_headers(headers: &HashMap<String, String>) -> Vec<String> {
        let mut missing = Vec::new();
        
        let security_headers = vec![
            "x-frame-options",
            "x-content-type-options",
            "content-security-policy",
            "strict-transport-security",
            "x-xss-protection",
            "referrer-policy",
            "permissions-policy",
        ];
        
        for header in security_headers {
            if !headers.contains_key(header) {
                missing.push(header.to_string());
            }
        }
        
        missing
    }
    
    pub fn recommended_headers() -> Vec<(&'static str, &'static str)> {
        vec![
            ("X-Frame-Options", "DENY"),
            ("X-Content-Type-Options", "nosniff"),
            ("Content-Security-Policy", "default-src 'self'"),
            ("Strict-Transport-Security", "max-age=31536000; includeSubDomains"),
            ("X-XSS-Protection", "1; mode=block"),
            ("Referrer-Policy", "no-referrer"),
            ("Permissions-Policy", "geolocation=(), microphone=(), camera=()"),
        ]
    }
}
