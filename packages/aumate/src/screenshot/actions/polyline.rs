//! Polyline action - toggles polyline drawing mode (折线绘制)

use egui::Pos2;

use crate::screenshot::action::{
    ActionContext, ActionResult, DrawingContext, RenderContext, ScreenAction, ToolCategory,
};
use crate::screenshot::stroke::Polyline;

/// Action to toggle polyline drawing mode
///
/// When active, allows drawing connected line segments on the screenshot.
/// Implements drawing lifecycle to handle polyline creation.
/// Note: Traditional polyline uses click-to-add-vertex behavior.
/// This implementation uses drag-to-draw for consistency with other tools.
pub struct PolylineAction {
    /// Whether polyline mode is currently active
    active: bool,
    /// Whether currently in a drawing operation
    is_drawing: bool,
}

impl PolylineAction {
    pub fn new() -> Self {
        Self { active: false, is_drawing: false }
    }
}

impl Default for PolylineAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for PolylineAction {
    fn id(&self) -> &str {
        "polyline"
    }

    fn name(&self) -> &str {
        "Polyline"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("polyline")
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
        ctx.annotations.start_polyline(clamped_pos, ctx.settings);
        self.is_drawing = true;
    }

    fn on_draw_move(&mut self, pos: Pos2, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        let clamped_pos = ctx.clamp_to_bounds(pos);
        ctx.annotations.update_polyline_preview(clamped_pos);
    }

    fn on_draw_end(&mut self, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        ctx.annotations.finish_polyline();
        self.is_drawing = false;
    }

    // ==================== Rendering ====================

    fn render_annotations(&self, ctx: &RenderContext) {
        // Render completed polylines
        for polyline in &ctx.annotations.polylines {
            Self::render_polyline(ctx.ui, polyline);
        }

        // Render current polyline being drawn
        if let Some(ref polyline) = ctx.annotations.current_polyline {
            Self::render_polyline(ctx.ui, polyline);
        }
    }
}

impl PolylineAction {
    /// Render a single polyline
    fn render_polyline(ui: &egui::Ui, polyline: &Polyline) {
        if polyline.points.len() < 2 {
            return;
        }

        let stroke = egui::Stroke::new(polyline.settings.width, polyline.settings.color);

        // Draw line segments between points
        for window in polyline.points.windows(2) {
            ui.painter().line_segment([window[0], window[1]], stroke);
        }

        // If closed, connect last to first
        if polyline.closed && polyline.points.len() >= 3 {
            ui.painter()
                .line_segment([*polyline.points.last().unwrap(), polyline.points[0]], stroke);
        }

        // Draw vertex handles if selected
        if polyline.selected {
            let handle_size = 6.0;
            for point in &polyline.points {
                ui.painter().circle_filled(*point, handle_size / 2.0, egui::Color32::WHITE);
                ui.painter().circle_stroke(
                    *point,
                    handle_size / 2.0,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 120, 255)),
                );
            }
        }
    }
}
