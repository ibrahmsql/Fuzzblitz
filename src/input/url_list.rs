use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Represents a list of URLs to fuzz
#[derive(Debug, Clone)]
pub struct UrlList {
    pub urls: Vec<String>,
}

impl UrlList {
    /// Load URLs from a file
    /// 
    /// # Arguments
    /// * `path` - Path to the file containing URLs (one per line)
    /// * `auto_fuzz` - If true, automatically append /FUZZ to URLs without FUZZ keyword
    /// 
    /// # Returns
    /// * `Result<Self, String>` - UrlList or error message
    pub fn from_file(path: &str, auto_fuzz: bool) -> Result<Self, String> {
        let file_path = Path::new(path);
        
        if !file_path.exists() {
            return Err(format!("File not found: {}", path));
        }
        
        let file = File::open(file_path)
            .map_err(|e| format!("Failed to open file {}: {}", path, e))?;
        
        let reader = BufReader::new(file);
        let mut urls = Vec::new();
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line
                .map_err(|e| format!("Failed to read line {}: {}", line_num + 1, e))?;
            
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            
            // Validate URL format
            if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
                return Err(format!(
                    "Invalid URL at line {}: '{}' (must start with http:// or https://)",
                    line_num + 1,
                    trimmed
                ));
            }
            
            // Auto-add FUZZ if enabled and not present
            let url = if auto_fuzz && !trimmed.contains("FUZZ") {
                // Add /FUZZ at the end
                if trimmed.ends_with('/') {
                    format!("{}FUZZ", trimmed)
                } else {
                    format!("{}/FUZZ", trimmed)
                }
            } else {
                trimmed.to_string()
            };
            
            urls.push(url);
        }
        
        if urls.is_empty() {
            return Err(format!("No valid URLs found in file: {}", path));
        }
        
        Ok(UrlList { urls })
    }
    
    /// Validate that all URLs contain the FUZZ keyword
    pub fn validate_urls(&self) -> Result<(), String> {
        for (idx, url) in self.urls.iter().enumerate() {
            if !url.contains("FUZZ") {
                return Err(format!(
                    "URL {} does not contain FUZZ keyword: {}\nUse --auto-fuzz to automatically add FUZZ",
                    idx + 1,
                    url
                ));
            }
        }
        Ok(())
    }
    
    /// Get the number of URLs
    pub fn len(&self) -> usize {
        self.urls.len()
    }
    
    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.urls.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auto_fuzz() {
        // This would require creating temporary test files
        // Skipping for now
    }
}
