#![allow(dead_code)]
use url::{Url, ParseError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UrlBuilder {
    template: String,
    base_url: Option<Url>,
}

impl UrlBuilder {
    pub fn new(template: &str) -> Result<Self, ParseError> {
        let base_url = if template.contains("FUZZ") || template.contains("://") {
            Url::parse(&template.replace("FUZZ", "placeholder")).ok()
        } else {
            None
        };
        
        Ok(Self {
            template: template.to_string(),
            base_url,
        })
    }
    
    pub fn build(&self, replacements: &HashMap<String, String>) -> String {
        let mut result = self.template.clone();
        for (keyword, value) in replacements {
            result = result.replace(keyword, value);
        }
        result
    }
    
    pub fn build_with_extension(&self, replacements: &HashMap<String, String>, extension: &str) -> String {
        let base = self.build(replacements);
        add_extension(&base, extension)
    }
    
    pub fn has_keyword(&self, keyword: &str) -> bool {
        self.template.contains(keyword)
    }
    
    pub fn extract_domain(&self) -> Option<String> {
        self.base_url.as_ref().and_then(|u| u.host_str().map(|s| s.to_string()))
    }
    
    pub fn extract_path(&self) -> Option<String> {
        self.base_url.as_ref().map(|u| u.path().to_string())
    }
    
    pub fn extract_scheme(&self) -> Option<String> {
        self.base_url.as_ref().map(|u| u.scheme().to_string())
    }
}

pub fn add_extension(url: &str, extension: &str) -> String {
    let ext = if extension.starts_with('.') {
        extension.to_string()
    } else {
        format!(".{}", extension)
    };
    
    if let Some(query_pos) = url.find('?') {
        let (path, query) = url.split_at(query_pos);
        format!("{}{}{}", path, ext, query)
    } else if let Some(fragment_pos) = url.find('#') {
        let (path, fragment) = url.split_at(fragment_pos);
        format!("{}{}{}", path, ext, fragment)
    } else {
        format!("{}{}", url, ext)
    }
}

pub fn normalize_url(url: &str) -> Result<String, ParseError> {
    let parsed = Url::parse(url)?;
    Ok(parsed.to_string())
}

pub fn extract_urls_from_response(body: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let patterns = [
        r#"https?://[^\s<>"'{}|\\^`\[\]]+"#,
        r#"//[^\s<>"'{}|\\^`\[\]]+"#,
        r#"/[a-zA-Z0-9_\-./]+"#,
    ];
    
    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for cap in re.captures_iter(body) {
                if let Some(url) = cap.get(0) {
                    urls.push(url.as_str().to_string());
                }
            }
        }
    }
    
    urls.sort();
    urls.dedup();
    urls
}

pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

pub fn join_url(base: &str, path: &str) -> Result<String, ParseError> {
    let base_url = Url::parse(base)?;
    let joined = base_url.join(path)?;
    Ok(joined.to_string())
}

pub fn url_to_filename(url: &str) -> String {
    url.replace("://", "_")
        .replace('/', "_")
        .replace('?', "_")
        .replace('&', "_")
        .replace('=', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_extension() {
        assert_eq!(add_extension("http://example.com/test", ".php"), "http://example.com/test.php");
        assert_eq!(add_extension("http://example.com/test?id=1", ".php"), "http://example.com/test.php?id=1");
    }

    #[test]
    fn test_url_to_filename() {
        let filename = url_to_filename("https://example.com/path/file");
        assert!(!filename.contains('/'));
        assert!(!filename.contains(':'));
    }
}
