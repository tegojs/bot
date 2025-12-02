//! Annotate action - toggles freehand annotation mode for drawing on screenshot

use egui::Pos2;

use crate::screenshot::action::{
    ActionContext, ActionResult, DrawingContext, RenderContext, ScreenAction, ToolCategory,
};
use crate::screenshot::stroke::{Stroke, StrokeStyle};

/// Action to toggle freehand annotation mode
///
/// When active, allows freehand drawing on the screenshot before saving/copying.
/// Implements the drawing lifecycle methods to handle stroke creation.
pub struct AnnotateAction {
    /// Whether annotation mode is currently active
    active: bool,
    /// Whether currently in a drawing operation
    is_drawing: bool,
}

impl AnnotateAction {
    pub fn new() -> Self {
        Self { active: false, is_drawing: false }
    }
}

impl Default for AnnotateAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenAction for AnnotateAction {
    fn id(&self) -> &str {
        "annotate"
    }

    fn name(&self) -> &str {
        "Pencil"
    }

    fn icon_id(&self) -> Option<&str> {
        Some("annotate")
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
        // Clamp position to selection bounds
        let clamped_pos = ctx.clamp_to_bounds(pos);

        // Start a new stroke
        ctx.annotations.start_stroke(clamped_pos, ctx.settings);
        self.is_drawing = true;
    }

    fn on_draw_move(&mut self, pos: Pos2, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        // Clamp position to selection bounds
        let clamped_pos = ctx.clamp_to_bounds(pos);

        // Add point to current stroke
        ctx.annotations.add_point(clamped_pos);
    }

    fn on_draw_end(&mut self, ctx: &mut DrawingContext) {
        if !self.is_drawing {
            return;
        }

        // Finish the stroke
        ctx.annotations.finish_stroke();
        self.is_drawing = false;
    }

    // ==================== Rendering ====================

    fn render_annotations(&self, ctx: &RenderContext) {
        // Render completed strokes
        for stroke in &ctx.annotations.strokes {
            Self::render_stroke(ctx.ui, stroke);
        }

        // Render current stroke being drawn
        if let Some(ref stroke) = ctx.annotations.current_stroke {
            Self::render_stroke(ctx.ui, stroke);
        }
    }
}

impl AnnotateAction {
    /// Render a single stroke
    fn render_stroke(ui: &egui::Ui, stroke: &Stroke) {
        if stroke.points.len() < 2 {
            return;
        }

        let egui_stroke = egui::Stroke::new(stroke.settings.width, stroke.settings.color);

        match stroke.settings.style {
            StrokeStyle::Solid => {
                // Draw continuous line segments
                for window in stroke.points.windows(2) {
                    ui.painter().line_segment([window[0], window[1]], egui_stroke);
                }
            }
            StrokeStyle::Dashed => {
                // Draw dashed line
                let dash_length = stroke.settings.width * 3.0;
                let gap_length = stroke.settings.width * 2.0;
                Self::render_dashed_line(ui, &stroke.points, egui_stroke, dash_length, gap_length);
            }
            StrokeStyle::Dotted => {
                // Draw dotted line
                let dot_spacing = stroke.settings.width * 2.5;
                Self::render_dotted_line(
                    ui,
                    &stroke.points,
                    stroke.settings.color,
                    stroke.settings.width / 2.0,
                    dot_spacing,
                );
            }
        }
    }

    /// Render a dashed line along points
    fn render_dashed_line(
        ui: &egui::Ui,
        points: &[Pos2],
        stroke: egui::Stroke,
        dash_length: f32,
        gap_length: f32,
    ) {
        if points.len() < 2 {
            return;
        }

        let mut accumulated = 0.0;
        let mut drawing = true;
        let mut current_start = points[0];

        for window in points.windows(2) {
            let start = window[0];
            let end = window[1];
            let segment_vec = end - start;
            let segment_length = segment_vec.length();

            if segment_length < 0.001 {
                continue;
            }

            let direction = segment_vec / segment_length;
            let mut pos_along = 0.0;

            while pos_along < segment_length {
                let remaining_in_state =
                    if drawing { dash_length } else { gap_length } - accumulated;
                let remaining_in_segment = segment_length - pos_along;
                let step = remaining_in_state.min(remaining_in_segment);

                if drawing {
                    let line_end = start + direction * (pos_along + step);
                    ui.painter().line_segment([current_start, line_end], stroke);
                    current_start = line_end;
                } else {
                    current_start = start + direction * (pos_along + step);
                }

                pos_along += step;
                accumulated += step;

                if accumulated >= (if drawing { dash_length } else { gap_length }) {
                    drawing = !drawing;
                    accumulated = 0.0;
                }
            }
        }
    }

    /// Render a dotted line along points
    fn render_dotted_line(
        ui: &egui::Ui,
        points: &[Pos2],
        color: egui::Color32,
        radius: f32,
        spacing: f32,
    ) {
        if points.is_empty() {
            return;
        }

        let mut accumulated = 0.0;

        // Draw first dot
        ui.painter().circle_filled(points[0], radius, color);

        for window in points.windows(2) {
            let start = window[0];
            let end = window[1];
            let segment_vec = end - start;
            let segment_length = segment_vec.length();

            if segment_length < 0.001 {
                continue;
            }

            let direction = segment_vec / segment_length;
            let mut pos_along = 0.0;

            while pos_along < segment_length {
                let remaining_to_next_dot = spacing - accumulated;

                if remaining_to_next_dot <= segment_length - pos_along {
                    pos_along += remaining_to_next_dot;
                    let dot_pos = start + direction * pos_along;
                    ui.painter().circle_filled(dot_pos, radius, color);
                    accumulated = 0.0;
                } else {
                    accumulated += segment_length - pos_along;
                    break;
                }
            }
        }
    }
}
