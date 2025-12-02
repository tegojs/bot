//! Mosaic action - toggles mosaic privacy mode

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction, ToolCategory};

/// Action to toggle mosaic privacy mode
///
/// When active, allows drawing mosaic regions to obscure sensitive information.
pub struct MosaicAction {
    /// Whether mosaic mode is currently active
    active: bool,
}

impl MosaicAction {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Default for MosaicAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for MosaicAction {
    fn id(&self) -> &str {
        "mosaic"
    }

    fn name(&self) -> &str {
        "Mosaic"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("mosaic")
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
