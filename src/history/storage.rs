#![allow(dead_code)]
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use super::session::Session;
use serde_json;

/// Store and retrieve fuzzing history
pub struct HistoryStorage {
    storage_dir: PathBuf,
}

impl HistoryStorage {
    pub fn new(storage_dir: Option<PathBuf>) -> Self {
        let dir = storage_dir.unwrap_or_else(|| {
            let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push(".fuzzblitz");
            path.push("history");
            path
        });
        
        Self {
            storage_dir: dir,
        }
    }
    
    /// Initialize storage directory
    pub fn init(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.storage_dir)?;
        Ok(())
    }
    
    /// Save a session to disk
    pub fn save_session(&self, session: &Session) -> std::io::Result<()> {
        self.init()?;
        
        let filename = format!("{}.json", session.id);
        let filepath = self.storage_dir.join(filename);
        
        let json = serde_json::to_string_pretty(session)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        let mut file = File::create(filepath)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }
    
    /// Load a session from disk
    pub fn load_session(&self, session_id: &str) -> std::io::Result<Session> {
        let filename = format!("{}.json", session_id);
        let filepath = self.storage_dir.join(filename);
        
        let mut file = File::open(filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let session: Session = serde_json::from_str(&contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        Ok(session)
    }
    
    /// List all session IDs
    pub fn list_sessions(&self) -> std::io::Result<Vec<String>> {
        if !self.storage_dir.exists() {
            return Ok(Vec::new());
        }
        
        let entries = fs::read_dir(&self.storage_dir)?;
        let mut sessions = Vec::new();
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    sessions.push(stem.to_string());
                }
            }
        }
        
        sessions.sort();
        Ok(sessions)
    }
    
    /// Delete a session
    pub fn delete_session(&self, session_id: &str) -> std::io::Result<()> {
        let filename = format!("{}.json", session_id);
        let filepath = self.storage_dir.join(filename);
        
        if filepath.exists() {
            fs::remove_file(filepath)?;
        }
        
        Ok(())
    }
    
    /// Clear all sessions
    pub fn clear_all(&self) -> std::io::Result<()> {
        if self.storage_dir.exists() {
            fs::remove_dir_all(&self.storage_dir)?;
            self.init()?;
        }
        Ok(())
    }
}

// Placeholder for dirs crate functionality
mod dirs {
    use std::path::PathBuf;
    
    pub fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
    }
}
