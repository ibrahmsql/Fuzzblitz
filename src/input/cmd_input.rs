#![allow(dead_code)]
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub struct CommandInput {
    command: String,
    shell: String,
    max_lines: usize,
}

impl CommandInput {
    pub fn new(command: String) -> Self {
        Self {
            command,
            shell: String::from("/bin/sh"),
            max_lines: 1000,
        }
    }
    
    pub fn with_shell(mut self, shell: String) -> Self {
        self.shell = shell;
        self
    }
    
    pub fn with_max_lines(mut self, max: usize) -> Self {
        self.max_lines = max;
        self
    }
    
    pub fn execute(&self) -> Result<Vec<String>, String> {
        let output = Command::new(&self.shell)
            .arg("-c")
            .arg(&self.command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
        
        let stdout = output.stdout.ok_or("Failed to capture stdout")?;
        let reader = BufReader::new(stdout);
        
        let mut lines = Vec::new();
        for (idx, line) in reader.lines().enumerate() {
            if idx >= self.max_lines {
                break;
            }
            
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    lines.push(line.trim().to_string());
                }
            }
        }
        
        Ok(lines)
    }
}

pub struct DirsearchMode {
    base_extensions: Vec<String>,
}

impl DirsearchMode {
    pub fn new() -> Self {
        Self {
            base_extensions: vec![
                ".php".to_string(),
                ".asp".to_string(),
                ".aspx".to_string(),
                ".jsp".to_string(),
                ".html".to_string(),
                ".js".to_string(),
            ],
        }
    }
    
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.base_extensions = extensions;
        self
    }
    
    pub fn generate_urls(&self, base_url: &str, word: &str) -> Vec<String> {
        let mut urls = Vec::new();
        
        urls.push(format!("{}/{}", base_url.trim_end_matches('/'), word));
        
        for ext in &self.base_extensions {
            urls.push(format!("{}/{}{}", base_url.trim_end_matches('/'), word, ext));
        }
        
        urls
    }
}

impl Default for DirsearchMode {
    fn default() -> Self {
        Self::new()
    }
}
