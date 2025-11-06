#![allow(dead_code)]
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct WafInfo {
    pub name: String,
    pub confidence: f32,
    pub detected_by: Vec<String>,
}

pub struct WafDetector {
    signatures: HashMap<String, Vec<WafSignature>>,
}

struct WafSignature {
    pattern: Regex,
    header: Option<String>,
    cookie: bool,
}

impl WafDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            signatures: HashMap::new(),
        };
        detector.load_signatures();
        detector
    }
    
    fn load_signatures(&mut self) {
        self.add_waf("Cloudflare", vec![
            ("server", r"cloudflare"),
            ("header", r"cf-ray"),
            ("header", r"cf-cache-status"),
        ]);
        
        self.add_waf("AWS WAF", vec![
            ("cookie", r"awselb"),
            ("header", r"x-amzn-"),
        ]);
        
        self.add_waf("Akamai", vec![
            ("header", r"akamai"),
            ("server", r"AkamaiGHost"),
        ]);
        
        self.add_waf("ModSecurity", vec![
            ("body", r"mod_security"),
            ("body", r"ModSecurity"),
        ]);
        
        self.add_waf("Imperva", vec![
            ("header", r"x-iinfo"),
            ("cookie", r"incap_ses"),
        ]);
        
        self.add_waf("F5 BIG-IP", vec![
            ("server", r"BIG-IP"),
            ("cookie", r"BIGipServer"),
        ]);
        
        self.add_waf("Sucuri", vec![
            ("header", r"x-sucuri"),
            ("server", r"Sucuri"),
        ]);
        
        self.add_waf("Wordfence", vec![
            ("body", r"wordfence"),
            ("cookie", r"wordfence"),
        ]);
    }
    
    fn add_waf(&mut self, name: &str, patterns: Vec<(&str, &str)>) {
        let mut sigs = Vec::new();
        
        for (sig_type, pattern) in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                let signature = WafSignature {
                    pattern: regex,
                    header: if sig_type == "header" || sig_type == "server" {
                        Some(sig_type.to_string())
                    } else {
                        None
                    },
                    cookie: sig_type == "cookie",
                };
                sigs.push(signature);
            }
        }
        
        self.signatures.insert(name.to_string(), sigs);
    }
    
    pub fn detect(&self, headers: &HashMap<String, String>, body: &str) -> Vec<WafInfo> {
        let mut detected = Vec::new();
        
        for (waf_name, signatures) in &self.signatures {
            let mut matches = Vec::new();
            
            for sig in signatures {
                if let Some(header_name) = &sig.header {
                    for (key, value) in headers {
                        if key.to_lowercase().contains(header_name) || 
                           sig.pattern.is_match(value) {
                            matches.push(format!("Header: {}", key));
                        }
                    }
                }
                
                if sig.cookie {
                    if let Some(cookies) = headers.get("set-cookie") {
                        if sig.pattern.is_match(cookies) {
                            matches.push("Cookie".to_string());
                        }
                    }
                }
                
                if sig.pattern.is_match(body) {
                    matches.push("Response body".to_string());
                }
            }
            
            if !matches.is_empty() {
                detected.push(WafInfo {
                    name: waf_name.clone(),
                    confidence: (matches.len() as f32 / signatures.len() as f32),
                    detected_by: matches,
                });
            }
        }
        
        detected.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        detected
    }
}

impl Default for WafDetector {
    fn default() -> Self {
        Self::new()
    }
}
