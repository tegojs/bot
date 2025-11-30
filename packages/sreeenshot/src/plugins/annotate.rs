use super::{Plugin, PluginContext, PluginResult};

pub struct AnnotatePlugin {
    active: bool,
}

impl AnnotatePlugin {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Plugin for AnnotatePlugin {
    fn id(&self) -> &str {
        "annotate"
    }
    
    fn name(&self) -> &str {
        "Annotate"
    }
    
    fn icon(&self) -> Option<&[u8]> {
        Some(include_bytes!("../../icons/annotate.png"))
    }
    
    fn on_click(&mut self, _context: &PluginContext) -> PluginResult {
        // Toggle annotation mode
        self.active = !self.active;
        // TODO: Implement annotation functionality
        PluginResult::Continue
    }
}

