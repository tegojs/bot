//! Arrow action - toggles arrow drawing mode

use egui::Pos2;

use crate::screenshot::action::{
    ActionContext, ActionResult, DrawingContext, RenderContext, ScreenAction, ToolCategory,
};
use crate::screenshot::stroke::Arrow;

/// Action to toggle arrow drawing mode
///
/// When active, allows drawing arrows on the screenshot.
/// Implements drawing lifecycle to handle arrow creation with arrowhead at end point.
pub struct ArrowAction {
    /// Whether arrow mode is currently active
    active: bool,
    /// Whether currently in a drawing operation
    is_drawing: bool,
}

impl ArrowAction {
    pub fn new() -> Self {
        Self { active: false, is_drawing: false }
    }
}

impl Default for ArrowAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for ArrowAction {
    fn id(&self) -> &str {
        "arrow"
    }

    fn name(&self) -> &str {
        "Arrow"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("arrow")
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
        ctx.annotations.start_arrow(clamped_pos, ctx.settings);
        self.is_drawing = true;
    }

    fn on_draw_move(&mut self, pos: Pos2, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        let clamped_pos = ctx.clamp_to_bounds(pos);
        // snap = false for smooth dragging, could add shift-key detection later
        ctx.annotations.update_arrow(clamped_pos, false);
    }

    fn on_draw_end(&mut self, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        ctx.annotations.finish_arrow();
        self.is_drawing = false;
    }

    // ==================== Rendering ====================

    fn render_annotations(&self, ctx: &RenderContext) {
        // Render completed arrows
        for arrow in &ctx.annotations.arrows {
            Self::render_arrow(ctx.ui, arrow);
        }

        // Render current arrow being drawn
        if let Some(ref arrow) = ctx.annotations.current_arrow {
            Self::render_arrow(ctx.ui, arrow);
        }
    }
}

impl ArrowAction {
    /// Render a single arrow
    fn render_arrow(ui: &egui::Ui, arrow: &Arrow) {
        let stroke = egui::Stroke::new(arrow.settings.width, arrow.settings.color);

        // Draw the line
        ui.painter().line_segment([arrow.start, arrow.end], stroke);

        // Draw arrowhead
        if arrow.length() > 5.0 {
            let dir = arrow.direction();
            let perp = egui::vec2(-dir.y, dir.x);
            let head_size = arrow.settings.width * 3.0;

            let tip = arrow.end;
            let left = tip - dir * head_size + perp * head_size * 0.5;
            let right = tip - dir * head_size - perp * head_size * 0.5;

            // Draw filled arrowhead triangle
            let triangle = egui::epaint::PathShape::convex_polygon(
                vec![tip, left, right],
                arrow.settings.color,
                egui::Stroke::NONE,
            );
            ui.painter().add(egui::Shape::Path(triangle));
        }

        // Draw control points if selected
        if arrow.selected {
            let handle_size = 8.0;
            ui.painter().circle_filled(arrow.start, handle_size / 2.0, egui::Color32::WHITE);
            ui.painter().circle_stroke(
                arrow.start,
                handle_size / 2.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 120, 255)),
            );
            ui.painter().circle_filled(arrow.end, handle_size / 2.0, egui::Color32::WHITE);
            ui.painter().circle_stroke(
                arrow.end,
                handle_size / 2.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 120, 255)),
            );
        }
    }
}
