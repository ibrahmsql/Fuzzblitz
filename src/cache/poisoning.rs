pub struct CachePoisoning;

impl CachePoisoning {
    pub fn test_headers() -> Vec<(String, String)> {
        vec![
            ("X-Forwarded-Host", "evil.com"),
            ("X-Forwarded-For", "127.0.0.1"),
            ("X-Original-URL", "/admin"),
            ("X-Rewrite-URL", "/admin"),
            ("X-Host", "evil.com"),
            ("Host", "evil.com"),
        ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }
    
    pub fn cache_buster_params() -> Vec<String> {
        vec![
            "?cb=",
            "?nocache=",
            "?v=",
            "?t=",
            "?_=",
        ].iter().map(|s| s.to_string()).collect()
    }
}
