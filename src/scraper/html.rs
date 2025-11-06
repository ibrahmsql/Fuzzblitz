#![allow(dead_code)]
use regex::Regex;
use std::collections::HashSet;

/// HTML scraping utilities
pub struct HtmlScraper;

impl HtmlScraper {
    /// Extract all tags of a specific type
    pub fn extract_tags(html: &str, tag: &str) -> Vec<String> {
        let pattern = format!(r"<{}\s[^>]*>|<{}>", tag, tag);
        let re = Regex::new(&pattern).unwrap();
        
        re.find_iter(html)
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract tag attributes
    pub fn extract_attribute(html: &str, tag: &str, attr: &str) -> Vec<String> {
        let pattern = format!(r#"<{}\s[^>]*{}="([^"]+)""#, tag, attr);
        let re = Regex::new(&pattern).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract all comments
    pub fn extract_comments(html: &str) -> Vec<String> {
        let re = Regex::new(r"<!--(.*?)-->").unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
            .collect()
    }
    
    /// Extract script sources
    pub fn extract_scripts(html: &str) -> Vec<String> {
        Self::extract_attribute(html, "script", "src")
    }
    
    /// Extract stylesheet links
    pub fn extract_stylesheets(html: &str) -> Vec<String> {
        let pattern = r#"<link[^>]*rel="stylesheet"[^>]*href="([^"]+)""#;
        let re = Regex::new(pattern).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract meta tags
    pub fn extract_meta(html: &str) -> Vec<(String, String)> {
        let pattern = r#"<meta\s+name="([^"]+)"\s+content="([^"]+)""#;
        let re = Regex::new(pattern).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| {
                let name = cap.get(1)?.as_str().to_string();
                let content = cap.get(2)?.as_str().to_string();
                Some((name, content))
            })
            .collect()
    }
    
    /// Extract title
    pub fn extract_title(html: &str) -> Option<String> {
        let re = Regex::new(r"<title[^>]*>(.*?)</title>").unwrap();
        re.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
    }
    
    /// Check if HTML contains specific text
    pub fn contains_text(html: &str, text: &str) -> bool {
        html.contains(text)
    }
    
    /// Extract all text content (strip HTML tags)
    pub fn extract_text(html: &str) -> String {
        let re = Regex::new(r"<[^>]+>").unwrap();
        re.replace_all(html, " ").to_string()
    }
}
