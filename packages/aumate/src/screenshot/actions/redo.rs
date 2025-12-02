//! Redo action - redoes the last undone annotation operation

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction, ToolCategory};

/// Action to redo the last undone annotation operation
pub struct RedoAction;

impl RedoAction {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RedoAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for RedoAction {
    fn id(&self) -> &str {
        "redo"
    }

    fn name(&self) -> &str {
        "Redo"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("redo")
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Action
    }

    fn on_click(&mut self, _ctx: &ActionContext) -> ActionResult {
        // The actual redo logic is handled by the screenshot mode
        // This action just signals that redo was requested
        ActionResult::Redo
    }
}
