//! Text action - toggles text input mode for adding text to screenshot

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction};

/// Action to toggle text input mode
///
/// When active, allows adding text annotations to the screenshot.
pub struct TextAction {
    /// Whether text mode is currently active
    active: bool,
}

impl TextAction {
    pub fn new() -> Self {
        Self { active: false }
    }

    /// Check if text mode is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Default for TextAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for TextAction {
    fn id(&self) -> &str {
        "text"
    }

    fn name(&self) -> &str {
        if self.active { "Text ✓" } else { "Text" }
    }

    fn icon(&self) -> Option<&[u8]> {
        // TODO: Add icon bytes
        None
    }

    fn on_click(&mut self, _ctx: &ActionContext) -> ActionResult {
        self.active = !self.active;
        ActionResult::Continue
    }
}
