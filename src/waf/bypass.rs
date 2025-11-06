pub struct WafBypass;

impl WafBypass {
    pub fn case_manipulation(payload: &str) -> Vec<String> {
        vec![
            payload.to_uppercase(),
            payload.to_lowercase(),
            Self::random_case(payload),
        ]
    }
    
    pub fn url_encoding(payload: &str) -> Vec<String> {
        vec![
            urlencoding::encode(payload).to_string(),
            Self::double_url_encode(payload),
            Self::unicode_encode(payload),
        ]
    }
    
    pub fn null_byte_injection(payload: &str) -> String {
        format!("%00{}", payload)
    }
    
    pub fn comment_injection(payload: &str) -> Vec<String> {
        vec![
            format!("{}/**/", payload),
            format!("{}--", payload),
            format!("{}#", payload),
        ]
    }
    
    fn random_case(s: &str) -> String {
        s.chars().enumerate().map(|(i, c)| {
            if i % 2 == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        }).collect()
    }
    
    fn double_url_encode(s: &str) -> String {
        let encoded = urlencoding::encode(s);
        urlencoding::encode(&encoded).to_string()
    }
    
    fn unicode_encode(s: &str) -> String {
        s.chars().map(|c| format!("\\u{:04x}", c as u32)).collect()
    }
    
    pub fn generate_bypass_payloads(payload: &str) -> Vec<String> {
        let mut payloads = Vec::new();
        
        payloads.extend(Self::case_manipulation(payload));
        payloads.extend(Self::url_encoding(payload));
        payloads.extend(Self::comment_injection(payload));
        payloads.push(Self::null_byte_injection(payload));
        
        payloads
    }
}
