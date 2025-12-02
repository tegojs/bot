//! Rectangle action - toggles rectangle drawing mode

use egui::Pos2;

use crate::screenshot::action::{
    ActionContext, ActionResult, DrawingContext, RenderContext, ScreenAction, ToolCategory,
};
use crate::screenshot::stroke::{FillMode, Shape, ShapeType};

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
        Self { active: false, is_drawing: false }
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

    // ==================== Rendering ====================

    fn render_annotations(&self, ctx: &RenderContext) {
        // Render completed rectangle shapes
        for shape in &ctx.annotations.shapes {
            if shape.shape_type == ShapeType::Rectangle {
                Self::render_shape(ctx.ui, shape);
            }
        }

        // Render current shape being drawn (if rectangle)
        if let Some(ref shape) = ctx.annotations.current_shape {
            if shape.shape_type == ShapeType::Rectangle {
                Self::render_shape(ctx.ui, shape);
            }
        }
    }
}

impl RectangleAction {
    /// Render a single rectangle shape
    fn render_shape(ui: &egui::Ui, shape: &Shape) {
        let stroke = egui::Stroke::new(shape.settings.width, shape.settings.color);

        match shape.fill_mode {
            FillMode::Filled => {
                ui.painter().rect_filled(shape.rect, 0.0, shape.settings.color);
            }
            FillMode::Outline => {
                ui.painter().rect_stroke(shape.rect, 0.0, stroke, egui::StrokeKind::Inside);
            }
        }

        // Draw resize handles if selected
        if shape.selected {
            let handle_size = 6.0;
            for corner in [
                shape.rect.left_top(),
                shape.rect.right_top(),
                shape.rect.left_bottom(),
                shape.rect.right_bottom(),
            ] {
                ui.painter().rect_filled(
                    egui::Rect::from_center_size(corner, egui::vec2(handle_size, handle_size)),
                    1.0,
                    egui::Color32::WHITE,
                );
                ui.painter().rect_stroke(
                    egui::Rect::from_center_size(corner, egui::vec2(handle_size, handle_size)),
                    1.0,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 120, 255)),
                    egui::StrokeKind::Outside,
                );
            }
        }
    }
}
