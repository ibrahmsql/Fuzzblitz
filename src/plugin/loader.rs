use super::registry::PluginMetadata;
use std::path::Path;

/// Plugin loader (for future dynamic loading)
pub struct PluginLoader {
    plugin_dir: String,
}

impl PluginLoader {
    pub fn new(plugin_dir: String) -> Self {
        Self { plugin_dir }
    }
    
    /// Discover plugins in directory
    pub fn discover(&self) -> Vec<String> {
        let path = Path::new(&self.plugin_dir);
        
        if !path.exists() {
            return Vec::new();
        }
        
        let mut plugins = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "so" || ext == "dylib" || ext == "dll" {
                            if let Some(name) = path.file_stem() {
                                plugins.push(name.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
        
        plugins
    }
    
    /// Load plugin metadata from file
    pub fn load_metadata(&self, plugin_name: &str) -> Result<PluginMetadata, String> {
        let meta_path = format!("{}/{}.meta", self.plugin_dir, plugin_name);
        let path = Path::new(&meta_path);
        
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(meta) = serde_json::from_str::<PluginMetadata>(&content) {
                    return Ok(meta);
                }
            }
        }
        
        Ok(PluginMetadata::new(
            plugin_name.to_string(),
            "0.1.0".to_string(),
        ))
    }
    
    /// Load all plugins from directory
    pub fn load_all(&self) -> Result<Vec<PluginMetadata>, String> {
        let plugins = self.discover();
        let mut metadata_list = Vec::new();
        
        for plugin_name in plugins {
            match self.load_metadata(&plugin_name) {
                Ok(metadata) => metadata_list.push(metadata),
                Err(e) => eprintln!("Failed to load plugin {}: {}", plugin_name, e),
            }
        }
        
        Ok(metadata_list)
    }
}

// Built-in plugins
pub mod builtin {
    use super::super::registry::PluginMetadata;
    
    /// Logger plugin metadata
    pub fn logger_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("logger".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "Logs all requests and responses".to_string();
        meta
    }
    
    /// Rate limiter plugin metadata
    pub fn rate_limiter_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("rate_limiter".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "Advanced rate limiting".to_string();
        meta
    }
    
    /// Retry plugin metadata
    pub fn retry_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("retry".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "Automatic retry on failures".to_string();
        meta
    }
    
    /// WAF bypass plugin
    pub fn waf_bypass_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("waf_bypass".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "WAF detection and bypass strategies".to_string();
        meta
    }
    
    /// Auth tester plugin
    pub fn auth_tester_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("auth_tester".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "Authentication testing (Basic, Bearer, JWT, etc.)".to_string();
        meta
    }
    
    /// API fuzzer plugin
    pub fn api_fuzzer_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("api_fuzzer".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "REST and GraphQL API fuzzing".to_string();
        meta
    }
    
    /// Cache poisoning plugin
    pub fn cache_poison_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("cache_poison".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "Cache poisoning detection".to_string();
        meta
    }
    
    /// Parameter miner plugin
    pub fn param_miner_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("param_miner".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "Hidden parameter discovery".to_string();
        meta
    }
    
    /// Vulnerability scanner plugin
    pub fn vuln_scanner_plugin() -> PluginMetadata {
        let mut meta = PluginMetadata::new("vuln_scanner".to_string(), "1.0.0".to_string());
        meta.author = "FuzzBlitz Team".to_string();
        meta.description = "Automated vulnerability detection".to_string();
        meta
    }
    
    /// Get all built-in plugins
    pub fn all_plugins() -> Vec<PluginMetadata> {
        vec![
            logger_plugin(),
            rate_limiter_plugin(),
            retry_plugin(),
            waf_bypass_plugin(),
            auth_tester_plugin(),
            api_fuzzer_plugin(),
            cache_poison_plugin(),
            param_miner_plugin(),
            vuln_scanner_plugin(),
        ]
    }
}
