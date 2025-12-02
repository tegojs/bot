//! Eraser action - erases annotations from screenshot

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction, ToolCategory};

/// Action to erase annotations from the screenshot
///
/// When active, clicking on annotations will remove them.
pub struct EraserAction {
    /// Whether eraser mode is currently active
    active: bool,
    /// Eraser size (radius in pixels)
    size: f32,
}

impl EraserAction {
    pub fn new() -> Self {
        Self { active: false, size: 10.0 }
    }

    /// Get current eraser size
    pub fn size(&self) -> f32 {
        self.size
    }

    /// Set eraser size
    pub fn set_size(&mut self, size: f32) {
        self.size = size.clamp(5.0, 50.0);
    }
}

impl Default for EraserAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for EraserAction {
    fn id(&self) -> &str {
        "eraser"
    }

    fn name(&self) -> &str {
        "Eraser"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("eraser")
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Drawing
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn on_click(&mut self, _ctx: &ActionContext) -> ActionResult {
        self.active = !self.active;
        ActionResult::Continue
    }
}
