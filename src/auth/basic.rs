use base64::{Engine as _, engine::general_purpose};

pub struct BasicAuth;

impl BasicAuth {
    pub fn encode(username: &str, password: &str) -> String {
        let credentials = format!("{}:{}", username, password);
        let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
        format!("Basic {}", encoded)
    }
    
    pub fn decode(auth_header: &str) -> Option<(String, String)> {
        let parts: Vec<&str> = auth_header.split_whitespace().collect();
        if parts.len() != 2 || parts[0] != "Basic" {
            return None;
        }
        
        let decoded = general_purpose::STANDARD.decode(parts[1]).ok()?;
        let credentials = String::from_utf8(decoded).ok()?;
        
        let cred_parts: Vec<&str> = credentials.splitn(2, ':').collect();
        if cred_parts.len() != 2 {
            return None;
        }
        
        Some((cred_parts[0].to_string(), cred_parts[1].to_string()))
    }
    
    pub fn generate_wordlist(usernames: &[String], passwords: &[String]) -> Vec<(String, String)> {
        let mut combos = Vec::new();
        for username in usernames {
            for password in passwords {
                combos.push((username.clone(), password.clone()));
            }
        }
        combos
    }
}
