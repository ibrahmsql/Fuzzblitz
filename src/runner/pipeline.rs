#![allow(dead_code)]
use std::sync::Arc;
use super::executor::ExecutionResult;

/// Execution pipeline with pre and post processing
pub struct ExecutionPipeline {
    preprocessors: Vec<Box<dyn Fn(&str, &str) -> (String, String) + Send + Sync>>,
    postprocessors: Vec<Box<dyn Fn(&ExecutionResult) -> ExecutionResult + Send + Sync>>,
}

impl ExecutionPipeline {
    pub fn new() -> Self {
        Self {
            preprocessors: Vec::new(),
            postprocessors: Vec::new(),
        }
    }
    
    /// Add a preprocessor that transforms URL and payload before execution
    pub fn add_preprocessor<F>(mut self, f: F) -> Self
    where
        F: Fn(&str, &str) -> (String, String) + Send + Sync + 'static,
    {
        self.preprocessors.push(Box::new(f));
        self
    }
    
    /// Add a postprocessor that transforms execution results
    pub fn add_postprocessor<F>(mut self, f: F) -> Self
    where
        F: Fn(&ExecutionResult) -> ExecutionResult + Send + Sync + 'static,
    {
        self.postprocessors.push(Box::new(f));
        self
    }
    
    /// Apply preprocessors to a request
    pub fn preprocess(&self, url: &str, payload: &str) -> (String, String) {
        let mut current_url = url.to_string();
        let mut current_payload = payload.to_string();
        
        for processor in &self.preprocessors {
            let (new_url, new_payload) = processor(&current_url, &current_payload);
            current_url = new_url;
            current_payload = new_payload;
        }
        
        (current_url, current_payload)
    }
    
    /// Apply postprocessors to a result
    pub fn postprocess(&self, result: ExecutionResult) -> ExecutionResult {
        let mut current_result = result;
        
        for processor in &self.postprocessors {
            current_result = processor(&current_result);
        }
        
        current_result
    }
}

impl Default for ExecutionPipeline {
    fn default() -> Self {
        Self::new()
    }
}

// Built-in preprocessors
pub mod preprocessors {
    /// URL encode the payload
    pub fn url_encode(url: &str, payload: &str) -> (String, String) {
        let encoded = urlencoding::encode(payload).to_string();
        (url.to_string(), encoded)
    }
    
    /// Add timestamp to payload
    pub fn add_timestamp(url: &str, payload: &str) -> (String, String) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (url.to_string(), format!("{}_{}", payload, timestamp))
    }
    
    /// Lowercase the payload
    pub fn lowercase(url: &str, payload: &str) -> (String, String) {
        (url.to_string(), payload.to_lowercase())
    }
}

// Built-in postprocessors
pub mod postprocessors {
    use super::super::executor::ExecutionResult;
    
    /// Mark slow requests
    pub fn mark_slow(threshold_ms: u64) -> impl Fn(&ExecutionResult) -> ExecutionResult {
        move |result: &ExecutionResult| {
            let mut new_result = result.clone();
            if result.response_time.as_millis() > threshold_ms as u128 {
                new_result.error = Some(format!("Slow response: {}ms", result.response_time.as_millis()));
            }
            new_result
        }
    }
    
    /// Filter by status code
    pub fn filter_status(allowed_codes: Vec<u16>) -> impl Fn(&ExecutionResult) -> ExecutionResult {
        move |result: &ExecutionResult| {
            let mut new_result = result.clone();
            if let Some(code) = result.status_code {
                if !allowed_codes.contains(&code) {
                    new_result.success = false;
                }
            }
            new_result
        }
    }
}
