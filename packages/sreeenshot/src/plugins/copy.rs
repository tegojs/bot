use super::{Plugin, PluginContext, PluginResult};
use crate::capture::capture_and_save_to_clipboard;

pub struct CopyPlugin;

impl CopyPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for CopyPlugin {
    fn id(&self) -> &str {
        "copy"
    }
    
    fn name(&self) -> &str {
        "Copy"
    }
    
    fn icon(&self) -> Option<&[u8]> {
        Some(include_bytes!("../../icons/copy.png"))
    }
    
    fn on_click(&mut self, context: &PluginContext) -> PluginResult {
        let coords = match context.selection_coords {
            Some(c) => c,
            None => return PluginResult::Failure("No selection".to_string()),
        };
        
        let monitor = match &context.monitor {
            Some(m) => m,
            None => return PluginResult::Failure("No monitor available".to_string()),
        };
        
        match capture_and_save_to_clipboard(monitor, coords) {
            Ok(_) => PluginResult::Exit, // 复制成功后退出
            Err(e) => PluginResult::Failure(format!("Failed to copy to clipboard: {}", e)),
        }
    }
}

