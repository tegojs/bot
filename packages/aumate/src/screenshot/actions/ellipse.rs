//! Ellipse action - toggles ellipse drawing mode

use egui::Pos2;

use crate::screenshot::action::{ActionContext, ActionResult, DrawingContext, ScreenAction, ToolCategory};
use crate::screenshot::stroke::{FillMode, ShapeType};

/// Action to toggle ellipse drawing mode
///
/// When active, allows drawing ellipses on the screenshot.
/// Implements drawing lifecycle to handle ellipse shape creation.
pub struct EllipseAction {
    /// Whether ellipse mode is currently active
    active: bool,
    /// Whether currently in a drawing operation
    is_drawing: bool,
}

impl EllipseAction {
    pub fn new() -> Self {
        Self {
            active: false,
            is_drawing: false,
        }
    }
}

impl Default for EllipseAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for EllipseAction {
    fn id(&self) -> &str {
        "ellipse"
    }

    fn name(&self) -> &str {
        "Ellipse"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("ellipse")
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
            ShapeType::Ellipse,
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
