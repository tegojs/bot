//! Ellipse action - toggles ellipse drawing mode

use egui::Pos2;

use crate::screenshot::action::{
    ActionContext, ActionResult, DrawingContext, RenderContext, ScreenAction, ToolCategory,
};
use crate::screenshot::stroke::{FillMode, Shape, ShapeType};

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
        Self { active: false, is_drawing: false }
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

    // ==================== Rendering ====================

    fn render_annotations(&self, ctx: &RenderContext) {
        // Render completed ellipse shapes
        for shape in &ctx.annotations.shapes {
            if shape.shape_type == ShapeType::Ellipse {
                Self::render_shape(ctx.ui, shape);
            }
        }

        // Render current shape being drawn (if ellipse)
        if let Some(ref shape) = ctx.annotations.current_shape {
            if shape.shape_type == ShapeType::Ellipse {
                Self::render_shape(ctx.ui, shape);
            }
        }
    }
}

impl EllipseAction {
    /// Render a single ellipse shape
    fn render_shape(ui: &egui::Ui, shape: &Shape) {
        let stroke = egui::Stroke::new(shape.settings.width, shape.settings.color);
        let center = shape.rect.center();
        let radius = shape.rect.size() / 2.0;
        // egui doesn't have native ellipse, approximate with circle for now
        let avg_radius = (radius.x + radius.y) / 2.0;

        match shape.fill_mode {
            FillMode::Filled => {
                ui.painter().circle_filled(center, avg_radius, shape.settings.color);
            }
            FillMode::Outline => {
                ui.painter().circle_stroke(center, avg_radius, stroke);
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
