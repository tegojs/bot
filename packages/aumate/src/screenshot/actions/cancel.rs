//! Cancel action - exits screenshot mode without saving

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction};

/// Action to cancel screenshot and exit
pub struct CancelAction;

impl CancelAction {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CancelAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for CancelAction {
    fn id(&self) -> &str {
        "cancel"
    }

    fn name(&self) -> &str {
        "Cancel"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("cancel")
    }

    fn on_click(&mut self, _ctx: &ActionContext) -> ActionResult {
        ActionResult::Exit
    }
}
