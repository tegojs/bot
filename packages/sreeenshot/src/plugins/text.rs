use super::{Plugin, PluginContext, PluginResult};

pub struct TextPlugin {
    active: bool,
}

impl TextPlugin {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Plugin for TextPlugin {
    fn id(&self) -> &str {
        "text"
    }
    
    fn name(&self) -> &str {
        "Text"
    }
    
    fn icon(&self) -> Option<&[u8]> {
        // 暂时没有图标，返回None
        None
    }
    
    fn on_click(&mut self, _context: &PluginContext) -> PluginResult {
        // 切换文本模式
        self.active = !self.active;
        PluginResult::Continue
    }
}

