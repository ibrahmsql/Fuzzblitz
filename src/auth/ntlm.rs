pub struct NtlmAuth;

impl NtlmAuth {
    pub fn create_type1_message() -> String {
        "NTLM TlRMTVNTUAABAAAAB4IIogAAAAAAAAAAAAAAAAAAAAAGAbEdAAAADw==".to_string()
    }
    
    pub fn is_ntlm_challenge(www_authenticate: &str) -> bool {
        www_authenticate.starts_with("NTLM ")
    }
    
    pub fn parse_type2_message(challenge: &str) -> Option<String> {
        if !Self::is_ntlm_challenge(challenge) {
            return None;
        }
        
        let parts: Vec<&str> = challenge.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }
        
        Some(parts[1].to_string())
    }
}
