#![allow(dead_code)]
use super::registry::{PluginRegistry, PluginMetadata};
use super::hooks::{HookManager, Hook};
use std::collections::HashMap;

/// Main plugin manager
pub struct PluginManager {
    registry: PluginRegistry,
    hook_manager: HookManager,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            hook_manager: HookManager::new(),
        }
    }
    
    /// Load a plugin
    pub fn load_plugin(&mut self, metadata: PluginMetadata) -> Result<(), String> {
        self.registry.register(metadata)?;
        Ok(())
    }
    
    /// Unload a plugin
    pub fn unload_plugin(&mut self, name: &str) -> Result<(), String> {
        self.registry.unregister(name)?;
        Ok(())
    }
    
    /// Register a hook from a plugin
    pub fn register_hook(&mut self, plugin_name: &str, hook: Hook) -> Result<(), String> {
        if !self.registry.is_enabled(plugin_name) {
            return Err(format!("Plugin '{}' is not enabled", plugin_name));
        }
        
        self.hook_manager.register(hook);
        Ok(())
    }
    
    /// Execute hooks
    pub fn execute_hooks(
        &self,
        hook_type: &super::hooks::HookType,
        data: &HashMap<String, String>,
    ) -> super::hooks::HookResult {
        self.hook_manager.execute(hook_type, data)
    }
    
    /// Get registry
    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }
    
    /// List all plugins
    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.registry.list()
    }
    
    /// Enable plugin
    pub fn enable_plugin(&self, name: &str) -> Result<(), String> {
        self.registry.enable(name)
    }
    
    /// Disable plugin
    pub fn disable_plugin(&self, name: &str) -> Result<(), String> {
        self.registry.disable(name)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
