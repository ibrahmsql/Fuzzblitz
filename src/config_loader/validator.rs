#![allow(dead_code)]
use super::parser::ConfigFile;

pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate a configuration file
    pub fn validate(config: &ConfigFile) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validate HTTP config
        if let Some(ref method) = config.http.method {
            if !Self::is_valid_method(method) {
                errors.push(format!("Invalid HTTP method: {}", method));
            }
        }
        
        // Validate timeout
        if let Some(timeout) = config.http.timeout {
            if timeout == 0 {
                errors.push("Timeout cannot be 0".to_string());
            }
        }
        
        // Validate threads
        if let Some(threads) = config.fuzzing.threads {
            if threads == 0 {
                errors.push("Thread count cannot be 0".to_string());
            }
            if threads > 1000 {
                errors.push("Thread count too high (max: 1000)".to_string());
            }
        }
        
        // Validate mode
        if let Some(ref mode) = config.fuzzing.mode {
            if !Self::is_valid_mode(mode) {
                errors.push(format!("Invalid fuzzing mode: {}", mode));
            }
        }
        
        // Validate output format
        if let Some(ref format) = config.output.output_format {
            if !Self::is_valid_output_format(format) {
                errors.push(format!("Invalid output format: {}", format));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn is_valid_method(method: &str) -> bool {
        matches!(
            method.to_uppercase().as_str(),
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS"
        )
    }
    
    fn is_valid_mode(mode: &str) -> bool {
        matches!(
            mode.to_lowercase().as_str(),
            "clusterbomb" | "pitchfork" | "sniper"
        )
    }
    
    fn is_valid_output_format(format: &str) -> bool {
        matches!(
            format.to_lowercase().as_str(),
            "json" | "csv" | "html" | "md" | "markdown" | "all"
        )
    }
}
