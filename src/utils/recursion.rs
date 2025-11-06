use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RecursionManager {
    visited: Arc<Mutex<HashSet<String>>>,
    queue: Arc<Mutex<VecDeque<RecursionItem>>>,
    max_depth: usize,
}

#[derive(Debug, Clone)]
pub struct RecursionItem {
    pub url: String,
    pub depth: usize,
    pub parent: Option<String>,
}

impl RecursionManager {
    pub fn new(max_depth: usize) -> Self {
        Self {
            visited: Arc::new(Mutex::new(HashSet::new())),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            max_depth,
        }
    }
    
    pub fn add_url(&self, url: String, depth: usize, parent: Option<String>) -> bool {
        if depth > self.max_depth {
            return false;
        }
        
        let normalized = normalize_url_for_recursion(&url);
        
        let mut visited = self.visited.lock().unwrap();
        if visited.contains(&normalized) {
            return false;
        }
        
        visited.insert(normalized.clone());
        drop(visited);
        
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(RecursionItem {
            url: normalized,
            depth,
            parent,
        });
        
        true
    }
    
    pub fn get_next(&self) -> Option<RecursionItem> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
    
    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }
    
    pub fn queue_size(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }
    
    pub fn visited_count(&self) -> usize {
        let visited = self.visited.lock().unwrap();
        visited.len()
    }
    
    pub fn has_visited(&self, url: &str) -> bool {
        let normalized = normalize_url_for_recursion(url);
        let visited = self.visited.lock().unwrap();
        visited.contains(&normalized)
    }
    
    pub fn add_discovered_paths(&self, base_url: &str, paths: Vec<String>, current_depth: usize) {
        for path in paths {
            let full_url = if path.starts_with("http") {
                path
            } else if path.starts_with('/') {
                format!("{}{}", get_base_url(base_url), path)
            } else {
                format!("{}/{}", base_url.trim_end_matches('/'), path)
            };
            
            self.add_url(full_url, current_depth + 1, Some(base_url.to_string()));
        }
    }
}

fn normalize_url_for_recursion(url: &str) -> String {
    url.trim_end_matches('/').to_lowercase()
}

fn get_base_url(url: &str) -> String {
    if let Some(pos) = url.find("://") {
        let after_scheme = &url[pos + 3..];
        if let Some(slash_pos) = after_scheme.find('/') {
            return url[..pos + 3 + slash_pos].to_string();
        }
    }
    url.to_string()
}

pub fn should_recurse(status_code: u16, content_type: &Option<String>) -> bool {
    if !((200..300).contains(&status_code) || (300..400).contains(&status_code)) {
        return false;
    }
    
    if let Some(ct) = content_type {
        let ct_lower = ct.to_lowercase();
        return ct_lower.contains("html") || ct_lower.contains("json") || ct_lower.contains("xml");
    }
    
    true
}

pub fn extract_recursion_candidates(body: &str, base_url: &str) -> Vec<String> {
    let mut candidates = Vec::new();
    
    let patterns = [
        r#"href=["']([^"']+)["']"#,
        r#"src=["']([^"']+)["']"#,
        r#"window\.location\s*=\s*["']([^"']+)["']"#,
    ];
    
    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for cap in re.captures_iter(body) {
                if let Some(link) = cap.get(1) {
                    let url = link.as_str();
                    if is_valid_recursion_target(url) {
                        candidates.push(url.to_string());
                    }
                }
            }
        }
    }
    
    candidates.sort();
    candidates.dedup();
    candidates
}

fn is_valid_recursion_target(url: &str) -> bool {
    if url.is_empty() || url.starts_with('#') || url.starts_with("javascript:") || url.starts_with("data:") {
        return false;
    }
    
    let excluded_extensions = [
        ".jpg", ".jpeg", ".png", ".gif", ".svg", ".ico",
        ".css", ".js", ".woff", ".woff2", ".ttf", ".eot",
        ".mp4", ".mp3", ".avi", ".pdf", ".zip", ".tar", ".gz",
    ];
    
    let url_lower = url.to_lowercase();
    for ext in &excluded_extensions {
        if url_lower.ends_with(ext) {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursion_manager() {
        let manager = RecursionManager::new(3);
        
        assert!(manager.add_url("http://example.com/path1".to_string(), 1, None));
        assert!(!manager.add_url("http://example.com/path1".to_string(), 1, None));
        
        assert_eq!(manager.visited_count(), 1);
        assert_eq!(manager.queue_size(), 1);
    }

    #[test]
    fn test_is_valid_recursion_target() {
        assert!(is_valid_recursion_target("/admin"));
        assert!(is_valid_recursion_target("http://example.com/test"));
        assert!(!is_valid_recursion_target("#section"));
        assert!(!is_valid_recursion_target("javascript:void(0)"));
        assert!(!is_valid_recursion_target("image.jpg"));
    }
}
