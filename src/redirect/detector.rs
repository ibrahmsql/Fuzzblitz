#![allow(dead_code)]
pub struct RedirectDetector;

impl RedirectDetector {
    pub fn is_redirect(status_code: u16) -> bool {
        matches!(status_code, 301 | 302 | 303 | 307 | 308)
    }
    
    pub fn extract_location(headers: &std::collections::HashMap<String, String>) -> Option<String> {
        headers.get("location").cloned()
    }
    
    pub fn is_open_redirect(location: &str, payload: &str) -> bool {
        location.contains(payload) || location.contains(&urlencoding::encode(payload).to_string())
    }
}
