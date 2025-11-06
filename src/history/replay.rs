#![allow(dead_code)]
use super::session::{Session, SessionResult};
use std::collections::HashMap;

/// Replay fuzzing sessions
pub struct ReplayManager {
    sessions: HashMap<String, Session>,
}

impl ReplayManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
    
    /// Add a session to replay manager
    pub fn add_session(&mut self, session: Session) {
        self.sessions.insert(session.id.clone(), session);
    }
    
    /// Get a session by ID
    pub fn get_session(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }
    
    /// Replay specific results from a session
    pub fn replay_results(&self, session_id: &str, filter: impl Fn(&SessionResult) -> bool) -> Vec<SessionResult> {
        if let Some(session) = self.get_session(session_id) {
            session.data.results.iter()
                .filter(|r| filter(r))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Replay all results with specific status code
    pub fn replay_by_status(&self, session_id: &str, status_code: u16) -> Vec<SessionResult> {
        self.replay_results(session_id, |r| r.status_code == status_code)
    }
    
    /// Replay all results with size in range
    pub fn replay_by_size_range(&self, session_id: &str, min: usize, max: usize) -> Vec<SessionResult> {
        self.replay_results(session_id, |r| r.size >= min && r.size <= max)
    }
    
    /// Get all URLs from a session
    pub fn get_urls(&self, session_id: &str) -> Vec<String> {
        if let Some(session) = self.get_session(session_id) {
            session.data.results.iter()
                .map(|r| r.url.clone())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get all payloads from a session
    pub fn get_payloads(&self, session_id: &str) -> Vec<String> {
        if let Some(session) = self.get_session(session_id) {
            session.data.results.iter()
                .map(|r| r.payload.clone())
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for ReplayManager {
    fn default() -> Self {
        Self::new()
    }
}
