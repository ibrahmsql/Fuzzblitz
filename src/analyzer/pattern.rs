#![allow(dead_code)]
use regex::Regex;
use std::collections::HashMap;

/// Analyze patterns in responses
pub struct PatternAnalyzer {
    patterns: HashMap<String, Regex>,
}

impl PatternAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            patterns: HashMap::new(),
        };
        
        // Add default patterns
        analyzer.add_default_patterns();
        analyzer
    }
    
    fn add_default_patterns(&mut self) {
        // Error patterns
        self.add_pattern("sql_error", r"(?i)(SQL|MySQL|PostgreSQL|Oracle).*error");
        self.add_pattern("php_error", r"(?i)(<b>)?Fatal error(</b>)?:");
        self.add_pattern("asp_error", r"(?i)Microsoft.*ODBC.*Driver");
        self.add_pattern("python_error", r"Traceback \(most recent call last\)");
        
        // Sensitive information
        self.add_pattern("email", r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b");
        self.add_pattern("ip_address", r"\b(?:\d{1,3}\.){3}\d{1,3}\b");
        self.add_pattern("api_key", r#"(?i)(api[_-]?key|apikey)[\s]*[:=][\s]*['"]?([a-zA-Z0-9_\-]+)['"]?"#);
        self.add_pattern("jwt_token", r"eyJ[a-zA-Z0-9_-]+\.eyJ[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+");
        
        // Directory listing
        self.add_pattern("dir_listing", r"(?i)(<title>)?Index of ");
        
        // Version disclosure
        self.add_pattern("version", r"(?i)(version|v)[\s]*[:=][\s]*(\d+\.\d+\.\d+)");
    }
    
    pub fn add_pattern(&mut self, name: &str, pattern: &str) {
        if let Ok(regex) = Regex::new(pattern) {
            self.patterns.insert(name.to_string(), regex);
        }
    }
    
    /// Find all matching patterns in text
    pub fn analyze(&self, text: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        
        for (name, regex) in &self.patterns {
            if regex.is_match(text) {
                matches.push(PatternMatch {
                    pattern_name: name.clone(),
                    matched_text: regex.find(text).map(|m| m.as_str().to_string()),
                });
            }
        }
        
        matches
    }
    
    /// Check if specific pattern exists
    pub fn has_pattern(&self, text: &str, pattern_name: &str) -> bool {
        self.patterns.get(pattern_name)
            .map(|regex| regex.is_match(text))
            .unwrap_or(false)
    }
    
    /// Extract all matches for a pattern
    pub fn extract_pattern(&self, text: &str, pattern_name: &str) -> Vec<String> {
        self.patterns.get(pattern_name)
            .map(|regex| {
                regex.find_iter(text)
                    .map(|m| m.as_str().to_string())
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_name: String,
    pub matched_text: Option<String>,
}

impl Default for PatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
