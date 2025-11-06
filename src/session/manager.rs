use std::collections::HashMap;

pub struct SessionManager {
    sessions: HashMap<String, SessionData>,
}

pub struct SessionData {
    pub cookies: Vec<String>,
    pub tokens: Vec<String>,
    pub headers: HashMap<String, String>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
    
    pub fn create_session(&mut self, id: String) -> &mut SessionData {
        self.sessions.entry(id).or_insert(SessionData {
            cookies: Vec::new(),
            tokens: Vec::new(),
            headers: HashMap::new(),
        })
    }
    
    pub fn get_session(&self, id: &str) -> Option<&SessionData> {
        self.sessions.get(id)
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
