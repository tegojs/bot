use super::{Plugin, PluginContext, PluginResult};
use std::collections::HashMap;

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    enabled_ids: Vec<String>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            enabled_ids: Vec::new(),
        }
    }
    
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        let id = plugin.id().to_string();
        self.plugins.insert(id.clone(), plugin);
    }
    
    pub fn enable(&mut self, id: &str) {
        if self.plugins.contains_key(id) && !self.enabled_ids.contains(&id.to_string()) {
            self.enabled_ids.push(id.to_string());
        }
    }
    
    pub fn get_enabled_plugin_info(&self) -> Vec<super::PluginInfo> {
        self.enabled_ids
            .iter()
            .filter_map(|id| {
                self.plugins.get(id).map(|p| {
                    super::PluginInfo {
                        id: p.id().to_string(),
                        name: p.name().to_string(),
                        icon: p.icon().map(|data| data.to_vec()),
                    }
                })
            })
            .collect()
    }
    
    pub fn execute_plugin(&mut self, id: &str, context: &PluginContext) -> PluginResult {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.on_click(context)
        } else {
            PluginResult::Failure(format!("Plugin '{}' not found", id))
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

