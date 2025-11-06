#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Plugin metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub enabled: bool,
}

impl PluginMetadata {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            author: String::new(),
            description: String::new(),
            enabled: true,
        }
    }
}

/// Plugin registry
pub struct PluginRegistry {
    plugins: Arc<Mutex<HashMap<String, PluginMetadata>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Register a plugin
    pub fn register(&self, metadata: PluginMetadata) -> Result<(), String> {
        let mut plugins = self.plugins.lock().unwrap();
        
        if plugins.contains_key(&metadata.name) {
            return Err(format!("Plugin '{}' already registered", metadata.name));
        }
        
        plugins.insert(metadata.name.clone(), metadata);
        Ok(())
    }
    
    /// Unregister a plugin
    pub fn unregister(&self, name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.lock().unwrap();
        
        if plugins.remove(name).is_none() {
            return Err(format!("Plugin '{}' not found", name));
        }
        
        Ok(())
    }
    
    /// Get plugin metadata
    pub fn get(&self, name: &str) -> Option<PluginMetadata> {
        let plugins = self.plugins.lock().unwrap();
        plugins.get(name).cloned()
    }
    
    /// List all registered plugins
    pub fn list(&self) -> Vec<PluginMetadata> {
        let plugins = self.plugins.lock().unwrap();
        plugins.values().cloned().collect()
    }
    
    /// Enable a plugin
    pub fn enable(&self, name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.lock().unwrap();
        
        if let Some(plugin) = plugins.get_mut(name) {
            plugin.enabled = true;
            Ok(())
        } else {
            Err(format!("Plugin '{}' not found", name))
        }
    }
    
    /// Disable a plugin
    pub fn disable(&self, name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.lock().unwrap();
        
        if let Some(plugin) = plugins.get_mut(name) {
            plugin.enabled = false;
            Ok(())
        } else {
            Err(format!("Plugin '{}' not found", name))
        }
    }
    
    /// Check if plugin is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        let plugins = self.plugins.lock().unwrap();
        plugins.get(name).map(|p| p.enabled).unwrap_or(false)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for PluginRegistry {
    fn clone(&self) -> Self {
        Self {
            plugins: Arc::clone(&self.plugins),
        }
    }
}
