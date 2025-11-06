#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzResult {
    pub fuzz_word: String,
    pub url: String,
    pub status_code: u16,
    pub body_length: usize,
    pub lines: usize,
    pub words: usize,
    pub response_time_ms: i64,
    pub timestamp: String,
}

impl FuzzResult {
    pub fn new(
        fuzz_word: String,
        url: String,
        status_code: u16,
        body_length: usize,
        lines: usize,
        words: usize,
        response_time_ms: i64,
    ) -> Self {
        Self {
            fuzz_word,
            url,
            status_code,
            body_length,
            lines,
            words,
            response_time_ms,
            timestamp: Local::now().to_rfc3339(),
        }
    }
}
