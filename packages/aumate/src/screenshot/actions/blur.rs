//! Blur action - toggles blur privacy mode

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction, ToolCategory};

/// Action to toggle blur privacy mode
///
/// When active, allows drawing blur regions to obscure sensitive information.
pub struct BlurAction {
    /// Whether blur mode is currently active
    active: bool,
}

impl BlurAction {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Default for BlurAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for BlurAction {
    fn id(&self) -> &str {
        "blur"
    }

    fn name(&self) -> &str {
        "Blur"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("blur")
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Privacy
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
