#![allow(dead_code)]
use std::collections::HashMap;
use crate::filters::FuzzResponse;

#[derive(Debug, Clone)]
pub struct CalibrationResult {
    pub baseline_sizes: Vec<usize>,
    pub baseline_lines: Vec<usize>,
    pub baseline_words: Vec<usize>,
    pub baseline_status_codes: Vec<u16>,
    pub should_filter_size: Option<usize>,
    pub should_filter_lines: Option<usize>,
    pub should_filter_words: Option<usize>,
}

impl CalibrationResult {
    pub fn new() -> Self {
        Self {
            baseline_sizes: Vec::new(),
            baseline_lines: Vec::new(),
            baseline_words: Vec::new(),
            baseline_status_codes: Vec::new(),
            should_filter_size: None,
            should_filter_lines: None,
            should_filter_words: None,
        }
    }
    
    pub fn add_sample(&mut self, response: &FuzzResponse) {
        self.baseline_sizes.push(response.body_length);
        self.baseline_lines.push(response.lines);
        self.baseline_words.push(response.words);
        self.baseline_status_codes.push(response.status_code);
    }
    
    pub fn calculate(&mut self) {
        if self.baseline_sizes.len() < 3 {
            return;
        }
        
        self.should_filter_size = find_common_value(&self.baseline_sizes);
        self.should_filter_lines = find_common_value(&self.baseline_lines);
        self.should_filter_words = find_common_value(&self.baseline_words);
    }
    
    pub fn is_likely_404(&self, response: &FuzzResponse) -> bool {
        if let Some(size) = self.should_filter_size {
            if response.body_length == size {
                return true;
            }
        }
        
        if let Some(lines) = self.should_filter_lines {
            if response.lines == lines {
                return true;
            }
        }
        
        if let Some(words) = self.should_filter_words {
            if response.words == words {
                return true;
            }
        }
        
        false
    }
}

fn find_common_value(values: &[usize]) -> Option<usize> {
    if values.is_empty() {
        return None;
    }
    
    let mut freq_map: HashMap<usize, usize> = HashMap::new();
    for &value in values {
        *freq_map.entry(value).or_insert(0) += 1;
    }
    
    let max_freq = freq_map.values().max()?;
    let threshold = (values.len() as f64 * 0.6) as usize;
    
    if *max_freq >= threshold {
        freq_map.iter()
            .find(|(_, &count)| count == *max_freq)
            .map(|(&value, _)| value)
    } else {
        None
    }
}

pub async fn auto_calibrate(
    client: &crate::network::FuzzClient,
    url_template: &str,
    test_payloads: Vec<String>,
) -> CalibrationResult {
    let mut result = CalibrationResult::new();
    
    for payload in test_payloads.iter().take(5) {
        let test_url = url_template.replace("FUZZ", payload);
        
        if let Ok((status_code, body)) = client.send_request(&test_url, false).await {
            let response = FuzzResponse::new(status_code, body, 0);
            result.add_sample(&response);
        }
    }
    
    result.calculate();
    result
}

pub fn generate_calibration_payloads() -> Vec<String> {
    vec![
        format!("__rustfuzz_404_{}", rand::random::<u32>()),
        format!("__notfound_{}", rand::random::<u32>()),
        format!("__test_{}", rand::random::<u32>()),
        format!("__calibration_{}", rand::random::<u32>()),
        format!("__invalid_{}", rand::random::<u32>()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_common_value() {
        let values = vec![100, 100, 100, 200, 300];
        assert_eq!(find_common_value(&values), Some(100));
        
        let values = vec![100, 200, 300, 400, 500];
        assert_eq!(find_common_value(&values), None);
    }

    #[test]
    fn test_calibration_result() {
        let mut result = CalibrationResult::new();
        
        let response1 = FuzzResponse::new(404, "Not found".to_string(), 10);
        let response2 = FuzzResponse::new(404, "Not found".to_string(), 10);
        let response3 = FuzzResponse::new(404, "Not found".to_string(), 10);
        
        result.add_sample(&response1);
        result.add_sample(&response2);
        result.add_sample(&response3);
        
        result.calculate();
        
        assert!(result.should_filter_size.is_some());
    }
}
