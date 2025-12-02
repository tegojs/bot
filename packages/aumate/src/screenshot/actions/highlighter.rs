//! Highlighter action - toggles highlighter drawing mode (荧光笔)

use egui::{Color32, Pos2};

use crate::screenshot::action::{ActionContext, ActionResult, DrawingContext, ScreenAction, ToolCategory};

/// Action to toggle highlighter drawing mode
///
/// When active, allows drawing semi-transparent highlight rectangles
/// to emphasize areas of interest.
pub struct HighlighterAction {
    /// Whether highlighter mode is currently active
    active: bool,
    /// Whether currently in a drawing operation
    is_drawing: bool,
}

impl HighlighterAction {
    pub fn new() -> Self {
        Self {
            active: false,
            is_drawing: false,
        }
    }

    /// Get the default highlight color (yellow with 40% opacity)
    fn highlight_color(&self) -> Color32 {
        Color32::from_rgba_unmultiplied(255, 255, 0, 100)
    }
}

impl Default for HighlighterAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for HighlighterAction {
    fn id(&self) -> &str {
        "highlighter"
    }

    fn name(&self) -> &str {
        "Highlighter"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("highlighter")
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Drawing
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
        if !active {
            self.is_drawing = false;
        }
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
        let clamped_pos = ctx.clamp_to_bounds(pos);
        ctx.annotations.start_highlighter(clamped_pos, self.highlight_color());
        self.is_drawing = true;
    }

    fn on_draw_move(&mut self, pos: Pos2, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        let clamped_pos = ctx.clamp_to_bounds(pos);
        ctx.annotations.update_highlighter(clamped_pos);
    }

    fn on_draw_end(&mut self, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        ctx.annotations.finish_highlighter();
        self.is_drawing = false;
    }
}
