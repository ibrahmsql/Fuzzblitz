pub struct CacheKeys;

impl CacheKeys {
    pub fn generate_variations(base_url: &str) -> Vec<String> {
        vec![
            format!("{}", base_url),
            format!("{}?v=1", base_url),
            format!("{}?nocache=1", base_url),
            format!("{}#fragment", base_url),
        ]
    }
    
    pub fn cache_control_headers() -> Vec<(String, String)> {
        vec![
            ("Cache-Control", "no-cache"),
            ("Cache-Control", "no-store"),
            ("Pragma", "no-cache"),
            ("Expires", "0"),
        ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }
}
