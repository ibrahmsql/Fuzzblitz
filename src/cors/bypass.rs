pub struct CorsBypass;

impl CorsBypass {
    pub fn origin_variations(base_domain: &str) -> Vec<String> {
        vec![
            format!("https://{}", base_domain),
            format!("http://{}", base_domain),
            format!("https://evil.{}", base_domain),
            format!("https://{}.evil.com", base_domain),
            format!("https://{}evil.com", base_domain),
            "null".to_string(),
        ]
    }
    
    pub fn bypass_techniques() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Null origin", "null"),
            ("Arbitrary origin", "https://evil.com"),
            ("Subdomain takeover", "https://subdomain.target.com"),
            ("Pre-domain wildcard", "https://targetcom.evil.com"),
            ("Post-domain wildcard", "https://target.com.evil.com"),
            ("Underscore bypass", "https://target_com"),
            ("Double encode", "https%3A%2F%2Fevil.com"),
        ]
    }
}
