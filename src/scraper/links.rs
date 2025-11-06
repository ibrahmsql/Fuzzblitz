use regex::Regex;
use std::collections::HashSet;

/// Extract links from HTML
pub struct LinkExtractor;

impl LinkExtractor {
    /// Extract all href links
    pub fn extract_hrefs(html: &str) -> Vec<String> {
        let re = Regex::new(r#"<a\s[^>]*href="([^"]+)""#).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract all image sources
    pub fn extract_images(html: &str) -> Vec<String> {
        let re = Regex::new(r#"<img\s[^>]*src="([^"]+)""#).unwrap();
        
        re.captures_iter(html)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    /// Extract all internal links (relative URLs)
    pub fn extract_internal_links(html: &str) -> Vec<String> {
        Self::extract_hrefs(html)
            .into_iter()
            .filter(|link| {
                !link.starts_with("http://") 
                && !link.starts_with("https://")
                && !link.starts_with("//")
                && !link.starts_with("mailto:")
                && !link.starts_with("tel:")
                && !link.starts_with("javascript:")
            })
            .collect()
    }
    
    /// Extract all external links
    pub fn extract_external_links(html: &str) -> Vec<String> {
        Self::extract_hrefs(html)
            .into_iter()
            .filter(|link| {
                link.starts_with("http://") 
                || link.starts_with("https://")
                || link.starts_with("//")
            })
            .collect()
    }
    
    /// Extract unique links
    pub fn extract_unique_links(html: &str) -> Vec<String> {
        let links: HashSet<String> = Self::extract_hrefs(html).into_iter().collect();
        links.into_iter().collect()
    }
    
    /// Extract links by domain
    pub fn extract_links_by_domain(html: &str, domain: &str) -> Vec<String> {
        Self::extract_external_links(html)
            .into_iter()
            .filter(|link| link.contains(domain))
            .collect()
    }
}
