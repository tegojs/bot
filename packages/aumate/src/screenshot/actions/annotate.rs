//! Annotate action - toggles annotation mode for drawing on screenshot

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction};

/// Action to toggle annotation mode
///
/// When active, allows drawing on the screenshot before saving/copying.
pub struct AnnotateAction {
    /// Whether annotation mode is currently active
    active: bool,
}

impl AnnotateAction {
    pub fn new() -> Self {
        Self { active: false }
    }

    /// Check if annotation mode is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Default for AnnotateAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for AnnotateAction {
    fn id(&self) -> &str {
        "annotate"
    }

    fn name(&self) -> &str {
        if self.active { "Annotate ✓" } else { "Annotate" }
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
