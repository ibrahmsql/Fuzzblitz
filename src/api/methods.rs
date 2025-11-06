#![allow(dead_code)]
pub struct HttpMethods;

impl HttpMethods {
    pub fn all_methods() -> Vec<String> {
        vec![
            "GET", "POST", "PUT", "PATCH", "DELETE",
            "HEAD", "OPTIONS", "TRACE", "CONNECT",
        ].iter().map(|s| s.to_string()).collect()
    }
    
    pub fn dangerous_methods() -> Vec<String> {
        vec!["PUT", "DELETE", "TRACE", "CONNECT"]
            .iter().map(|s| s.to_string()).collect()
    }
    
    pub fn test_method_override_headers() -> Vec<(String, String)> {
        vec![
            ("X-HTTP-Method-Override", "PUT"),
            ("X-HTTP-Method-Override", "DELETE"),
            ("X-HTTP-Method", "PUT"),
            ("X-HTTP-Method", "DELETE"),
            ("X-Method-Override", "PUT"),
            ("X-Method-Override", "DELETE"),
        ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }
}
