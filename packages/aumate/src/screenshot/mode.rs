//! ScreenshotMode - main state machine for screenshot functionality
//!
//! Coordinates selection, toolbar, and action execution.

use image::{ImageBuffer, Rgba};
use winit::event::{ElementState, MouseButton, WindowEvent};
use xcap::Monitor;

use super::action::{ActionContext, ActionResult};
use super::registry::{ActionRegistry, create_default_registry};
use super::selection::Selection;
use super::stroke::{Annotations, Stroke, StrokeStyle};
use super::toolbar::Toolbar;
use super::ui::AnnotatePopup;
use crate::error::{AumateError, Result};

/// Screenshot mode state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeState {
    /// Waiting for user to start selection
    Idle,
    /// User is dragging to select region
    Selecting,
    /// Selection complete, showing toolbar
    ToolbarVisible,
    /// Screenshot mode should exit
    Exiting,
}

/// Main screenshot mode controller
pub struct ScreenshotMode {
    /// Current state
    state: ModeState,
    /// Selection state
    selection: Selection,
    /// Action toolbar (created after selection completes)
    toolbar: Option<Toolbar>,
    /// Action registry
    registry: ActionRegistry,
    /// Captured screenshot image
    screenshot: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    /// Monitor used for capture
    monitor: Option<Monitor>,
    /// Scale factor for DPI
    scale_factor: f64,
    /// Screen size in logical coordinates
    screen_size: (f32, f32),
    /// Currently hovered button ID
    hovered_button: Option<String>,
    /// Whether annotation mode is active
    annotate_active: bool,
    /// Whether text mode is active (reserved for future use)
    #[allow(dead_code)]
    text_mode: bool,
    /// Annotation popup for stroke settings
    annotate_popup: AnnotatePopup,
    /// Annotations drawn on screenshot
    annotations: Annotations,
    /// Last cursor position (for drawing)
    last_cursor_pos: Option<(f32, f32)>,
}

impl ScreenshotMode {
    /// Create a new screenshot mode with specified enabled actions
    ///
    /// Captures the current screen and initializes the registry.
    pub fn new(enabled_actions: Vec<String>, scale_factor: f64) -> Result<Self> {
        // Get monitors
        let monitors = Monitor::all()
            .map_err(|e| AumateError::Screenshot(format!("Failed to get monitors: {}", e)))?;

        if monitors.is_empty() {
            return Err(AumateError::Screenshot("No monitors found".to_string()));
        }

        // Use primary monitor
        let monitor = monitors.into_iter().next().unwrap();

        // Capture screenshot
        let screenshot = monitor
            .capture_image()
            .map_err(|e| AumateError::Screenshot(format!("Failed to capture screen: {}", e)))?;

        let screen_width = monitor
            .width()
            .map_err(|e| AumateError::Screenshot(format!("Failed to get width: {}", e)))?;
        let screen_height = monitor
            .height()
            .map_err(|e| AumateError::Screenshot(format!("Failed to get height: {}", e)))?;

        // Create and configure registry
        let mut registry = create_default_registry();
        registry.enable_all(&enabled_actions);

        let screen_size =
            (screen_width as f32 / scale_factor as f32, screen_height as f32 / scale_factor as f32);

        log::info!(
            "ScreenshotMode: xcap physical={}x{}, scale_factor={}, logical screen_size=({}, {})",
            screen_width,
            screen_height,
            scale_factor,
            screen_size.0,
            screen_size.1
        );

        Ok(Self {
            state: ModeState::Idle,
            selection: Selection::new(),
            toolbar: None,
            registry,
            screenshot: Some(screenshot),
            monitor: Some(monitor),
            scale_factor,
            screen_size,
            hovered_button: None,
            annotate_active: false,
            text_mode: false,
            annotate_popup: AnnotatePopup::new(),
            annotations: Annotations::new(),
            last_cursor_pos: None,
        })
    }

    /// Get current state
    pub fn state(&self) -> ModeState {
        self.state
    }

    /// Check if screenshot mode should exit
    pub fn should_exit(&self) -> bool {
        self.state == ModeState::Exiting
    }

    /// Get the captured screenshot
    pub fn screenshot(&self) -> Option<&ImageBuffer<Rgba<u8>, Vec<u8>>> {
        self.screenshot.as_ref()
    }

    /// Get the selection bounds in logical coordinates
    pub fn selection_bounds(&self) -> Option<((f32, f32), (f32, f32))> {
        self.selection.bounds()
    }

    /// Get the toolbar if visible
    pub fn toolbar(&self) -> Option<&Toolbar> {
        self.toolbar.as_ref()
    }

    /// Get currently hovered button
    pub fn hovered_button(&self) -> Option<&str> {
        self.hovered_button.as_deref()
    }

    /// Handle a window event
    ///
    /// Returns Some(ActionResult) if an action was executed.
    /// Note: CursorMoved events should be handled separately via `handle_cursor_move()`
    /// with pre-converted logical coordinates.
    pub fn handle_event(&mut self, event: &WindowEvent) -> Option<ActionResult> {
        match event {
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_button(*button, *state)
            }
            WindowEvent::KeyboardInput { event, .. } => self.handle_keyboard(event),
            _ => None,
        }
    }

    fn handle_mouse_button(
        &mut self,
        button: MouseButton,
        state: ElementState,
    ) -> Option<ActionResult> {
        if button != MouseButton::Left {
            return None;
        }

        match state {
            ElementState::Pressed => {
                match self.state {
                    ModeState::Idle => {
                        // Start selection - position will be set by cursor move
                        self.state = ModeState::Selecting;
                    }
                    ModeState::ToolbarVisible => {
                        // Check if clicking inside the popup area (egui handles its own clicks)
                        if self.annotate_popup.visible {
                            if let Some(pos) = self.last_cursor_pos {
                                // Check if inside popup (approximate bounds)
                                if self.annotate_popup.contains(egui::pos2(pos.0, pos.1)) {
                                    return None; // Let popup handle the click
                                }
                            }
                        }

                        // Check if clicking a toolbar button
                        if self.toolbar.is_some() {
                            if let Some(id) = self.hovered_button.clone() {
                                return self.execute_action(&id);
                            }
                        }

                        // If in annotate mode and clicking in selection area, start drawing
                        if self.annotate_active {
                            if let Some(pos) = self.last_cursor_pos {
                                if self.is_inside_selection(pos) {
                                    self.annotations.start_stroke(
                                        egui::pos2(pos.0, pos.1),
                                        &self.annotate_popup.settings,
                                    );
                                    return None;
                                }
                            }
                        }

                        // Close popup if clicking outside
                        if self.annotate_popup.visible {
                            self.annotate_popup.hide();
                        }

                        // Click outside toolbar and selection - reset
                        if !self.is_inside_selection(self.last_cursor_pos.unwrap_or((0.0, 0.0))) {
                            self.selection.reset();
                            self.toolbar = None;
                            self.annotate_active = false;
                            self.annotate_popup.hide();
                            self.state = ModeState::Idle;
                        }
                    }
                    _ => {}
                }
                None
            }
            ElementState::Released => {
                // Finish any ongoing stroke
                if self.annotations.is_drawing() {
                    self.annotations.finish_stroke();
                }

                if self.state == ModeState::Selecting {
                    self.selection.finish();

                    // Check if selection is large enough
                    if let Some((w, h)) = self.selection.size() {
                        if w > 5.0 && h > 5.0 {
                            // Create toolbar
                            if let Some(bounds) = self.selection.bounds() {
                                let actions = self.registry.get_enabled();
                                self.toolbar =
                                    Some(Toolbar::new(&actions, bounds, self.screen_size));
                                self.state = ModeState::ToolbarVisible;
                            }
                        } else {
                            // Selection too small, reset
                            self.selection.reset();
                            self.state = ModeState::Idle;
                        }
                    } else {
                        self.selection.reset();
                        self.state = ModeState::Idle;
                    }
                }
                None
            }
        }
    }

    /// Check if a position is inside the current selection
    fn is_inside_selection(&self, pos: (f32, f32)) -> bool {
        if let Some(((min_x, min_y), (max_x, max_y))) = self.selection.bounds() {
            pos.0 >= min_x && pos.0 <= max_x && pos.1 >= min_y && pos.1 <= max_y
        } else {
            false
        }
    }

    /// Handle cursor movement with logical coordinates
    ///
    /// This method expects coordinates already converted to logical pixels
    /// (i.e., physical pixels divided by scale factor).
    pub fn handle_cursor_move(&mut self, pos: (f32, f32)) {
        // Always track cursor position
        self.last_cursor_pos = Some(pos);

        match self.state {
            ModeState::Idle => {
                // Track position for when selection starts
            }
            ModeState::Selecting => {
                if !self.selection.has_selection() {
                    self.selection.start(pos);
                } else {
                    self.selection.update(pos);
                }
            }
            ModeState::ToolbarVisible => {
                // If drawing, add points to stroke
                if self.annotate_active && self.annotations.is_drawing() {
                    self.annotations.add_point(egui::pos2(pos.0, pos.1));
                }

                // Update hovered button
                if let Some(ref toolbar) = self.toolbar {
                    self.hovered_button = toolbar.check_click(pos).map(|s| s.to_string());
                }
            }
            _ => {}
        }
    }

    fn handle_keyboard(&mut self, event: &winit::event::KeyEvent) -> Option<ActionResult> {
        use winit::keyboard::{Key, NamedKey};

        if event.state != ElementState::Pressed {
            return None;
        }

        match &event.logical_key {
            Key::Named(NamedKey::Escape) => {
                self.state = ModeState::Exiting;
                Some(ActionResult::Exit)
            }
            _ => None,
        }
    }

    fn execute_action(&mut self, id: &str) -> Option<ActionResult> {
        // Handle annotate action specially
        if id == "annotate" {
            // Toggle annotate mode
            self.annotate_active = !self.annotate_active;

            // Show/hide popup when toggling mode
            if self.annotate_active {
                // Find annotate button position for popup placement
                if let Some(ref toolbar) = self.toolbar {
                    if let Some(button) = toolbar.buttons().iter().find(|b| b.id == "annotate") {
                        let (bx, by, bw, _) = button.bounds;
                        // Position popup above the button
                        self.annotate_popup.show(egui::pos2(bx + bw / 2.0 - 140.0, by - 150.0));
                    }
                }
            } else {
                self.annotate_popup.hide();
            }

            return Some(ActionResult::Continue);
        }

        // Create action context
        let selection_physical = self.selection.bounds_physical(self.scale_factor as f32);
        let ctx = ActionContext::new(
            selection_physical,
            self.screenshot.as_ref(),
            self.monitor.as_ref(),
            self.scale_factor,
        );

        // Execute action
        let result = self.registry.execute(id, &ctx);

        // Handle result
        match &result {
            ActionResult::Exit => {
                self.state = ModeState::Exiting;
            }
            ActionResult::Continue => {
                // Check for mode toggles
                // This is a simplified approach - in practice you'd want to track
                // which action was toggled
            }
            _ => {}
        }

        Some(result)
    }

    /// Render the screenshot mode overlay
    pub fn render(&mut self, ctx: &egui::Context) {
        // Use egui's actual screen rect instead of calculated screen_size
        // This ensures the overlay covers the full window regardless of size differences
        #[allow(deprecated)]
        let screen_rect = ctx.input(|i| i.screen_rect());

        egui::Area::new(egui::Id::new("screenshot_overlay")).fixed_pos(egui::pos2(0.0, 0.0)).show(
            ctx,
            |ui| {
                // Draw semi-transparent overlay
                let overlay_color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100);

                if let Some(((min_x, min_y), (max_x, max_y))) = self.selection.bounds() {
                    // Draw overlay around selection (4 rectangles)
                    let selection_rect = egui::Rect::from_min_max(
                        egui::pos2(min_x, min_y),
                        egui::pos2(max_x, max_y),
                    );

                    // Top
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(
                            screen_rect.min,
                            egui::pos2(screen_rect.max.x, selection_rect.min.y),
                        ),
                        egui::CornerRadius::ZERO,
                        overlay_color,
                    );
                    // Bottom
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(
                            egui::pos2(screen_rect.min.x, selection_rect.max.y),
                            screen_rect.max,
                        ),
                        egui::CornerRadius::ZERO,
                        overlay_color,
                    );
                    // Left
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(
                            egui::pos2(screen_rect.min.x, selection_rect.min.y),
                            egui::pos2(selection_rect.min.x, selection_rect.max.y),
                        ),
                        egui::CornerRadius::ZERO,
                        overlay_color,
                    );
                    // Right
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(
                            egui::pos2(selection_rect.max.x, selection_rect.min.y),
                            egui::pos2(screen_rect.max.x, selection_rect.max.y),
                        ),
                        egui::CornerRadius::ZERO,
                        overlay_color,
                    );

                    // Draw selection border
                    ui.painter().rect_stroke(
                        selection_rect,
                        egui::CornerRadius::ZERO,
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 120, 255)),
                        egui::StrokeKind::Outside,
                    );

                    // Draw size indicator
                    if let Some((w, h)) = self.selection.size_physical(self.scale_factor as f32) {
                        let size_text = format!("{}×{}", w, h);
                        let text_pos =
                            egui::pos2(selection_rect.min.x + 4.0, selection_rect.min.y - 20.0);
                        ui.painter().text(
                            text_pos,
                            egui::Align2::LEFT_BOTTOM,
                            size_text,
                            egui::FontId::proportional(12.0),
                            egui::Color32::WHITE,
                        );
                    }

                    // Render annotations inside selection area
                    self.render_annotations(ui);
                } else {
                    // No selection - full overlay
                    ui.painter().rect_filled(screen_rect, egui::CornerRadius::ZERO, overlay_color);
                }

                // Render toolbar if visible
                if let Some(ref toolbar) = self.toolbar {
                    toolbar.render(ui, self.hovered_button.as_deref());
                }
            },
        );

        // Render annotation popup (outside the Area so it's on top)
        self.annotate_popup.render(ctx);
    }

    /// Render all annotations
    fn render_annotations(&self, ui: &egui::Ui) {
        // Render completed strokes
        for stroke in &self.annotations.strokes {
            self.render_stroke(ui, stroke);
        }

        // Render current stroke being drawn
        if let Some(ref stroke) = self.annotations.current_stroke {
            self.render_stroke(ui, stroke);
        }
    }

    /// Render a single stroke
    fn render_stroke(&self, ui: &egui::Ui, stroke: &Stroke) {
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
                // Draw dashed line (simplified: skip every other segment)
                let dash_length = stroke.settings.width * 3.0;
                let gap_length = stroke.settings.width * 2.0;
                self.render_dashed_line(ui, &stroke.points, egui_stroke, dash_length, gap_length);
            }
            StrokeStyle::Dotted => {
                // Draw dotted line (circles at intervals)
                let dot_spacing = stroke.settings.width * 2.5;
                self.render_dotted_line(
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
        &self,
        ui: &egui::Ui,
        points: &[egui::Pos2],
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
        &self,
        ui: &egui::Ui,
        points: &[egui::Pos2],
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
