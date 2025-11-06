pub struct DigestAuth;

impl DigestAuth {
    pub fn parse_challenge(www_authenticate: &str) -> Option<DigestChallenge> {
        if !www_authenticate.starts_with("Digest ") {
            return None;
        }
        
        Some(DigestChallenge {
            realm: Self::extract_value(www_authenticate, "realm")?,
            nonce: Self::extract_value(www_authenticate, "nonce")?,
            qop: Self::extract_value(www_authenticate, "qop"),
            opaque: Self::extract_value(www_authenticate, "opaque"),
        })
    }
    
    fn extract_value(header: &str, key: &str) -> Option<String> {
        let pattern = format!("{}=\"([^\"]+)\"", key);
        let re = regex::Regex::new(&pattern).ok()?;
        let caps = re.captures(header)?;
        caps.get(1).map(|m| m.as_str().to_string())
    }
    
    pub fn generate_response(
        challenge: &DigestChallenge,
        username: &str,
        password: &str,
        method: &str,
        uri: &str,
    ) -> String {
        let ha1 = md5::compute(format!("{}:{}:{}", username, challenge.realm, password));
        let ha2 = md5::compute(format!("{}:{}", method, uri));
        let response = md5::compute(format!("{:x}:{}:{:x}", ha1, challenge.nonce, ha2));
        
        format!(
            "Digest username=\"{}\", realm=\"{}\", nonce=\"{}\", uri=\"{}\", response=\"{:x}\"",
            username, challenge.realm, challenge.nonce, uri, response
        )
    }
}

pub struct DigestChallenge {
    pub realm: String,
    pub nonce: String,
    pub qop: Option<String>,
    pub opaque: Option<String>,
}
