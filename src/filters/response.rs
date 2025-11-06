#![allow(dead_code)]
/// Represents a fuzzing response with analyzed metrics

#[derive(Debug, Clone)]
pub struct FuzzResponse {
    pub status_code: u16,
    pub body: String,
    pub body_length: usize,
    pub lines: usize,
    pub words: usize,
    pub response_time_ms: i64,
}

impl FuzzResponse {
    pub fn new(status_code: u16, body: String, response_time_ms: i64) -> Self {
        let lines = body.lines().count();
        let words = body.split_whitespace().count();
        let body_length = body.len();
        
        Self {
            status_code,
            body,
            body_length,
            lines,
            words,
            response_time_ms,
        }
    }
    
    pub fn empty(status_code: u16) -> Self {
        Self {
            status_code,
            body: String::new(),
            body_length: 0,
            lines: 0,
            words: 0,
            response_time_ms: 0,
        }
    }
}
