//! Rectangle action - toggles rectangle drawing mode

use egui::Pos2;

use crate::screenshot::action::{ActionContext, ActionResult, DrawingContext, ScreenAction, ToolCategory};
use crate::screenshot::stroke::{FillMode, ShapeType};

/// Action to toggle rectangle drawing mode
///
/// When active, allows drawing rectangles on the screenshot.
/// Implements drawing lifecycle to handle rectangle shape creation.
pub struct RectangleAction {
    /// Whether rectangle mode is currently active
    active: bool,
    /// Whether currently in a drawing operation
    is_drawing: bool,
}

impl RectangleAction {
    pub fn new() -> Self {
        Self {
            active: false,
            is_drawing: false,
        }
    }
}

impl Default for RectangleAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for RectangleAction {
    fn id(&self) -> &str {
        "rectangle"
    }

    fn name(&self) -> &str {
        "Rectangle"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("rectangle")
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
        ctx.annotations.start_shape(
            clamped_pos,
            ShapeType::Rectangle,
            FillMode::Outline,
            ctx.settings,
        );
        self.is_drawing = true;
    }

    fn on_draw_move(&mut self, pos: Pos2, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        let clamped_pos = ctx.clamp_to_bounds(pos);
        ctx.annotations.update_shape(clamped_pos);
    }

    fn on_draw_end(&mut self, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        ctx.annotations.finish_shape();
        self.is_drawing = false;
    }
}
