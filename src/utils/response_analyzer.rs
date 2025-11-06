#![allow(dead_code)]
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ResponseAnalysis {
    pub status_code: u16,
    pub body_length: usize,
    pub lines: usize,
    pub words: usize,
    pub headers_count: usize,
    pub response_time_ms: i64,
    pub content_type: Option<String>,
    pub server: Option<String>,
    pub redirects: Vec<String>,
    pub links: Vec<String>,
    pub forms: usize,
    pub scripts: usize,
    pub cookies: Vec<String>,
}

impl ResponseAnalysis {
    pub fn from_response(
        status_code: u16,
        body: &str,
        headers: &HashMap<String, String>,
        response_time_ms: i64,
    ) -> Self {
        let lines = body.lines().count();
        let words = body.split_whitespace().count();
        let body_length = body.len();
        
        let content_type = headers.get("content-type").cloned();
        let server = headers.get("server").cloned();
        let cookies = extract_cookies(headers);
        
        let links = extract_links(body);
        let redirects = extract_redirects(body);
        let forms = count_forms(body);
        let scripts = count_scripts(body);
        
        Self {
            status_code,
            body_length,
            lines,
            words,
            headers_count: headers.len(),
            response_time_ms,
            content_type,
            server,
            redirects,
            links,
            forms,
            scripts,
            cookies,
        }
    }
    
    pub fn has_keyword(&self, body: &str, keyword: &str) -> bool {
        body.contains(keyword)
    }
    
    pub fn matches_regex(&self, body: &str, pattern: &str) -> bool {
        if let Ok(re) = Regex::new(pattern) {
            re.is_match(body)
        } else {
            false
        }
    }
    
    pub fn extract_data(&self, body: &str, pattern: &str) -> Vec<String> {
        let mut results = Vec::new();
        if let Ok(re) = Regex::new(pattern) {
            for cap in re.captures_iter(body) {
                if let Some(m) = cap.get(0) {
                    results.push(m.as_str().to_string());
                }
            }
        }
        results
    }
    
    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.status_code)
    }
    
    pub fn is_error(&self) -> bool {
        self.status_code >= 400
    }
    
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status_code)
    }
}

fn extract_cookies(headers: &HashMap<String, String>) -> Vec<String> {
    headers
        .get("set-cookie")
        .map(|s| vec![s.clone()])
        .unwrap_or_default()
}

fn extract_links(body: &str) -> Vec<String> {
    let mut links = Vec::new();
    
    let patterns = [
        r#"href=["']([^"']+)["']"#,
        r#"src=["']([^"']+)["']"#,
        r#"action=["']([^"']+)["']"#,
    ];
    
    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            for cap in re.captures_iter(body) {
                if let Some(link) = cap.get(1) {
                    links.push(link.as_str().to_string());
                }
            }
        }
    }
    
    links.sort();
    links.dedup();
    links
}

fn extract_redirects(body: &str) -> Vec<String> {
    let mut redirects = Vec::new();
    
    if let Ok(re) = Regex::new(r#"location:\s*([^\s<>"']+)"#) {
        for cap in re.captures_iter(body) {
            if let Some(location) = cap.get(1) {
                redirects.push(location.as_str().to_string());
            }
        }
    }
    
    redirects
}

fn count_forms(body: &str) -> usize {
    body.matches("<form").count()
}

fn count_scripts(body: &str) -> usize {
    body.matches("<script").count()
}

pub fn detect_technologies(body: &str, headers: &HashMap<String, String>) -> Vec<String> {
    let mut techs = Vec::new();
    
    if body.contains("wp-content") || body.contains("wordpress") {
        techs.push("WordPress".to_string());
    }
    
    if body.contains("joomla") {
        techs.push("Joomla".to_string());
    }
    
    if body.contains("drupal") {
        techs.push("Drupal".to_string());
    }
    
    if body.contains("react") || body.contains("__REACT") {
        techs.push("React".to_string());
    }
    
    if body.contains("vue") || body.contains("__VUE__") {
        techs.push("Vue.js".to_string());
    }
    
    if body.contains("angular") || body.contains("ng-") {
        techs.push("Angular".to_string());
    }
    
    if let Some(server) = headers.get("server") {
        if server.contains("nginx") {
            techs.push("Nginx".to_string());
        } else if server.contains("apache") {
            techs.push("Apache".to_string());
        } else if server.contains("microsoft") || server.contains("iis") {
            techs.push("IIS".to_string());
        }
    }
    
    if let Some(powered_by) = headers.get("x-powered-by") {
        techs.push(format!("X-Powered-By: {}", powered_by));
    }
    
    techs
}

pub fn extract_endpoints(body: &str) -> Vec<String> {
    let mut endpoints = Vec::new();
    
    let patterns = [
        r#"/api/[a-zA-Z0-9_\-/]+"#,
        r#"/v[0-9]/[a-zA-Z0-9_\-/]+"#,
        r#"/(admin|panel|dashboard|api)/[a-zA-Z0-9_\-/]+"#,
    ];
    
    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            for cap in re.captures_iter(body) {
                if let Some(endpoint) = cap.get(0) {
                    endpoints.push(endpoint.as_str().to_string());
                }
            }
        }
    }
    
    endpoints.sort();
    endpoints.dedup();
    endpoints
}
