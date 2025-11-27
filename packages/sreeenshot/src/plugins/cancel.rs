use super::{Plugin, PluginContext, PluginResult};

pub struct CancelPlugin;

impl CancelPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for CancelPlugin {
    fn id(&self) -> &str {
        "cancel"
    }
    
    fn name(&self) -> &str {
        "Cancel"
    }
    
    fn icon(&self) -> Option<&[u8]> {
        Some(include_bytes!("../../icons/cancel.png"))
    }
    
    fn on_click(&mut self, _context: &PluginContext) -> PluginResult {
        // Cancel operation - exit without saving
        PluginResult::Exit
    }
}

