#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(default)]
    pub http: HttpConfig,
    
    #[serde(default)]
    pub fuzzing: FuzzingConfig,
    
    #[serde(default)]
    pub filtering: FilteringConfig,
    
    #[serde(default)]
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpConfig {
    pub method: Option<String>,
    pub headers: Option<Vec<String>>,
    pub cookies: Option<String>,
    pub data: Option<String>,
    pub proxy: Option<String>,
    pub timeout: Option<u64>,
    pub follow_redirects: Option<bool>,
    pub http2: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FuzzingConfig {
    pub wordlists: Option<Vec<String>>,
    pub mode: Option<String>,
    pub threads: Option<usize>,
    pub rate: Option<u64>,
    pub delay: Option<String>,
    pub extensions: Option<Vec<String>>,
    pub encoders: Option<Vec<String>>,
    pub recursion: Option<bool>,
    pub recursion_depth: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilteringConfig {
    pub match_codes: Option<String>,
    pub match_size: Option<Vec<String>>,
    pub match_words: Option<Vec<String>>,
    pub match_lines: Option<Vec<String>>,
    pub match_regexp: Option<Vec<String>>,
    pub filter_codes: Option<Vec<String>>,
    pub filter_size: Option<Vec<String>>,
    pub filter_words: Option<Vec<String>>,
    pub filter_lines: Option<Vec<String>>,
    pub filter_regexp: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputConfig {
    pub output_file: Option<String>,
    pub output_format: Option<String>,
    pub output_dir: Option<String>,
    pub colorize: Option<bool>,
    pub verbose: Option<bool>,
    pub silent: Option<bool>,
}

pub struct ConfigParser;

impl ConfigParser {
    /// Parse YAML config file
    pub fn from_yaml(path: &Path) -> Result<ConfigFile, String> {
        let mut file = File::open(path)
            .map_err(|e| format!("Failed to open config file: {}", e))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        serde_yaml::from_str(&contents)
            .map_err(|e| format!("Failed to parse YAML: {}", e))
    }
    
    /// Parse JSON config file
    pub fn from_json(path: &Path) -> Result<ConfigFile, String> {
        let mut file = File::open(path)
            .map_err(|e| format!("Failed to open config file: {}", e))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse JSON: {}", e))
    }
    
    /// Auto-detect format and parse
    pub fn from_file(path: &Path) -> Result<ConfigFile, String> {
        let extension = path.extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "No file extension found".to_string())?;
        
        match extension {
            "yml" | "yaml" => Self::from_yaml(path),
            "json" => Self::from_json(path),
            _ => Err(format!("Unsupported config format: {}", extension)),
        }
    }
}
