use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub data: SessionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub target_url: String,
    pub wordlist_paths: Vec<String>,
    pub total_requests: usize,
    pub matched_count: usize,
    pub fuzzing_mode: String,
    pub filters: HashMap<String, String>,
    pub matchers: HashMap<String, String>,
    pub results: Vec<SessionResult>,
    pub duration_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub url: String,
    pub status_code: u16,
    pub size: usize,
    pub words: usize,
    pub lines: usize,
    pub response_time_ms: i64,
    pub payload: String,
}

impl Session {
    pub fn new(id: String) -> Self {
        Self {
            id,
            created_at: Utc::now(),
            data: SessionData {
                target_url: String::new(),
                wordlist_paths: Vec::new(),
                total_requests: 0,
                matched_count: 0,
                fuzzing_mode: String::from("clusterbomb"),
                filters: HashMap::new(),
                matchers: HashMap::new(),
                results: Vec::new(),
                duration_secs: 0,
            },
        }
    }
    
    pub fn add_result(&mut self, result: SessionResult) {
        self.data.results.push(result);
        self.data.matched_count = self.data.results.len();
    }
    
    pub fn set_target(&mut self, url: String) {
        self.data.target_url = url;
    }
    
    pub fn add_wordlist(&mut self, path: String) {
        self.data.wordlist_paths.push(path);
    }
    
    pub fn set_duration(&mut self, secs: u64) {
        self.data.duration_secs = secs;
    }
}

impl SessionResult {
    pub fn new(
        url: String,
        status_code: u16,
        size: usize,
        words: usize,
        lines: usize,
        response_time_ms: i64,
        payload: String,
    ) -> Self {
        Self {
            url,
            status_code,
            size,
            words,
            lines,
            response_time_ms,
            payload,
        }
    }
}
