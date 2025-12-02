//! Sequence action - toggles sequence marker mode (序列号)

use egui::Pos2;

use crate::screenshot::action::{ActionContext, ActionResult, DrawingContext, ScreenAction, ToolCategory};

/// Action to toggle sequence marker mode
///
/// When active, allows placing numbered markers (1, 2, 3...) on the screenshot.
/// Unlike other drawing tools, sequence markers are placed on mouse press (click),
/// not on drag. Each click adds a new marker with an incrementing number.
pub struct SequenceAction {
    /// Whether sequence mode is currently active
    active: bool,
}

impl SequenceAction {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Default for SequenceAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for SequenceAction {
    fn id(&self) -> &str {
        "sequence"
    }

    fn name(&self) -> &str {
        "Sequence"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("sequence")
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

    // ==================== Drawing Lifecycle ====================

    fn is_drawing_tool(&self) -> bool {
        true
    }

    fn on_draw_start(&mut self, pos: Pos2, ctx: &mut DrawingContext) {
        // Sequence markers are placed on click, not drag
        let clamped_pos = ctx.clamp_to_bounds(pos);
        ctx.annotations.add_marker(clamped_pos, ctx.settings.color);
    }

    // on_draw_move and on_draw_end are not needed for sequence markers
    // since they are placed on click, not drag
}
