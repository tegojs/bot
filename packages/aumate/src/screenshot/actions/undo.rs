//! Undo action - undoes the last annotation operation

use crate::screenshot::action::{ActionContext, ActionResult, ScreenAction, ToolCategory};

/// Action to undo the last annotation operation
pub struct UndoAction;

impl UndoAction {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UndoAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for UndoAction {
    fn id(&self) -> &str {
        "undo"
    }

    fn name(&self) -> &str {
        "Undo"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("undo")
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Action
    }

    fn on_click(&mut self, _ctx: &ActionContext) -> ActionResult {
        // The actual undo logic is handled by the screenshot mode
        // This action just signals that undo was requested
        ActionResult::Undo
    }
}
