use std::collections::HashMap;

/// Types of hooks that plugins can register
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HookType {
    /// Before request is sent
    PreRequest,
    /// After response is received
    PostResponse,
    /// Before filtering
    PreFilter,
    /// After filtering
    PostFilter,
    /// On match found
    OnMatch,
    /// On scan start
    OnStart,
    /// On scan complete
    OnComplete,
    /// On error
    OnError,
}

/// Result from a hook execution
#[derive(Debug, Clone)]
pub enum HookResult {
    /// Continue normal execution
    Continue,
    /// Skip this item
    Skip,
    /// Stop execution
    Stop,
    /// Modified data
    Modified(HashMap<String, String>),
}

/// A hook function
pub type HookFn = Box<dyn Fn(&HashMap<String, String>) -> HookResult + Send + Sync>;

/// Hook registration
pub struct Hook {
    pub name: String,
    pub hook_type: HookType,
    pub priority: i32,
    pub function: HookFn,
}

impl Hook {
    pub fn new(
        name: String,
        hook_type: HookType,
        function: HookFn,
    ) -> Self {
        Self {
            name,
            hook_type,
            priority: 0,
            function,
        }
    }
    
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn execute(&self, data: &HashMap<String, String>) -> HookResult {
        (self.function)(data)
    }
}

/// Hook manager
pub struct HookManager {
    hooks: HashMap<HookType, Vec<Hook>>,
}

impl HookManager {
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }
    
    /// Register a new hook
    pub fn register(&mut self, hook: Hook) {
        let hooks = self.hooks.entry(hook.hook_type.clone()).or_insert_with(Vec::new);
        hooks.push(hook);
        hooks.sort_by(|a, b| b.priority.cmp(&a.priority)); // Higher priority first
    }
    
    /// Execute all hooks of a specific type
    pub fn execute(&self, hook_type: &HookType, data: &HashMap<String, String>) -> HookResult {
        if let Some(hooks) = self.hooks.get(hook_type) {
            for hook in hooks {
                match hook.execute(data) {
                    HookResult::Continue => continue,
                    result => return result,
                }
            }
        }
        HookResult::Continue
    }
    
    /// Get number of registered hooks for a type
    pub fn count(&self, hook_type: &HookType) -> usize {
        self.hooks.get(hook_type).map(|h| h.len()).unwrap_or(0)
    }
    
    /// Clear all hooks of a specific type
    pub fn clear(&mut self, hook_type: &HookType) {
        self.hooks.remove(hook_type);
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}
