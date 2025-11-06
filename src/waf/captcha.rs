use regex::Regex;

pub struct CaptchaDetector;

impl CaptchaDetector {
    pub fn detect_captcha(body: &str, headers: &std::collections::HashMap<String, String>) -> bool {
        Self::detect_recaptcha(body) ||
        Self::detect_hcaptcha(body) ||
        Self::detect_cloudflare_challenge(body) ||
        Self::detect_generic_captcha(body) ||
        Self::detect_rate_limit(headers)
    }
    
    pub fn detect_recaptcha(body: &str) -> bool {
        body.contains("recaptcha") ||
        body.contains("g-recaptcha") ||
        body.contains("grecaptcha") ||
        body.contains("google.com/recaptcha")
    }
    
    pub fn detect_hcaptcha(body: &str) -> bool {
        body.contains("hcaptcha") ||
        body.contains("h-captcha") ||
        body.contains("hcaptcha.com")
    }
    
    pub fn detect_cloudflare_challenge(body: &str) -> bool {
        body.contains("cf-challenge") ||
        body.contains("Cloudflare") && body.contains("challenge") ||
        body.contains("cf-chl-bypass") ||
        body.contains("Just a moment...")
    }
    
    pub fn detect_generic_captcha(body: &str) -> bool {
        let patterns = vec![
            r"captcha",
            r"verification",
            r"bot[ -]?(check|detection)",
            r"human[ -]?verification",
            r"prove you('re| are) (not )?((a )?human|robot)",
        ];
        
        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&body.to_lowercase()) {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn detect_rate_limit(headers: &std::collections::HashMap<String, String>) -> bool {
        for (key, value) in headers {
            let key_lower = key.to_lowercase();
            if key_lower.contains("rate") || 
               key_lower.contains("limit") ||
               key_lower.contains("retry-after") {
                return true;
            }
            
            if key_lower == "status" && value.contains("429") {
                return true;
            }
        }
        
        false
    }
}

pub struct CaptchaBypass;

impl CaptchaBypass {
    pub fn suggest_bypasses() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Slow down requests", "Use --rate-limit or -p (delay) option"),
            ("Rotate User-Agents", "Use different User-Agent headers"),
            ("Rotate IPs", "Use proxy rotation or Tor"),
            ("Add delays", "Use random delays between requests"),
            ("Session handling", "Maintain cookies and session tokens"),
            ("Header manipulation", "Add legitimate browser headers"),
        ]
    }
    
    pub fn recommended_options() -> Vec<String> {
        vec![
            "--rate 10 (max 10 req/sec)".to_string(),
            "-p 500-1000 (500-1000ms delay)".to_string(),
            "-H 'User-Agent: ...' (custom UA)".to_string(),
            "-x http://proxy:port (use proxy)".to_string(),
            "-b 'cookies' (use cookies)".to_string(),
        ]
    }
}
