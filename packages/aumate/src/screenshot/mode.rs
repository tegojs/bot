//! ScreenshotMode - main state machine for screenshot functionality
//!
//! Coordinates selection, toolbar, and action execution.

use image::{ImageBuffer, Rgba};
use winit::event::{ElementState, MouseButton, WindowEvent};
use xcap::Monitor;

use super::action::{ActionContext, ActionResult, DrawingContext, ToolCategory};
use super::action_bar::ActionBar;
use super::history::History;
use super::options_panel::OptionsPanel;
use super::registry::{create_default_registry, ActionRegistry};
use super::selection::Selection;
use super::settings::ScreenshotSettings;
use super::stroke::{
    Annotations, Arrow, FillMode, Highlighter, Polyline, SequenceMarker, Shape, ShapeType, Stroke,
    StrokeStyle,
};
use super::ui::{AnnotatePopup, SettingsButton, SettingsToolbar};
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

/// Handle positions for selection resizing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlePosition {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl HandlePosition {
    /// Returns all 8 handle positions
    pub fn all() -> [HandlePosition; 8] {
        [
            Self::TopLeft,
            Self::TopCenter,
            Self::TopRight,
            Self::MiddleLeft,
            Self::MiddleRight,
            Self::BottomLeft,
            Self::BottomCenter,
            Self::BottomRight,
        ]
    }
}

/// Handle size in pixels
const HANDLE_SIZE: f32 = 10.0;

/// Main screenshot mode controller
pub struct ScreenshotMode {
    /// Current state
    state: ModeState,
    /// Selection state
    selection: Selection,
    /// New Snipaste-style action bar
    action_bar: Option<ActionBar>,
    /// Options panel for tool settings
    options_panel: Option<OptionsPanel>,
    /// History for undo/redo
    history: History,
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
    /// Annotation popup for stroke settings
    annotate_popup: AnnotatePopup,
    /// Annotations drawn on screenshot
    annotations: Annotations,
    /// Last cursor position (for drawing)
    last_cursor_pos: Option<(f32, f32)>,
    /// Currently hovered handle (for cursor change)
    hovered_handle: Option<HandlePosition>,
    /// Handle being dragged for resize
    dragging_handle: Option<HandlePosition>,
    /// Screenshot appearance settings
    settings: ScreenshotSettings,
    /// Top-left settings toolbar
    settings_toolbar: SettingsToolbar,
    /// Whether the action bar is being dragged
    action_bar_dragging: bool,
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
            action_bar: None,
            options_panel: None,
            history: History::new(),
            registry,
            screenshot: Some(screenshot),
            monitor: Some(monitor),
            scale_factor,
            screen_size,
            hovered_button: None,
            annotate_active: false,
            annotate_popup: AnnotatePopup::new(),
            annotations: Annotations::new(),
            last_cursor_pos: None,
            hovered_handle: None,
            dragging_handle: None,
            settings: ScreenshotSettings::default(),
            settings_toolbar: SettingsToolbar::new(),
            action_bar_dragging: false,
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

                        // Check if clicking on a resize handle
                        if let Some(pos) = self.last_cursor_pos {
                            if let Some(handle) = self.handle_at_pos(pos) {
                                self.dragging_handle = Some(handle);
                                return None; // Handle drag started
                            }
                        }

                        // Check if clicking on drag handle of action bar
                        if let Some(ref mut bar) = self.action_bar {
                            if let Some(pos) = self.last_cursor_pos {
                                let egui_pos = egui::pos2(pos.0, pos.1);
                                if bar.is_in_drag_handle(egui_pos) {
                                    bar.start_drag(egui_pos);
                                    self.action_bar_dragging = true;
                                    return None;
                                }
                            }
                        }

                        // Check if clicking an action bar button
                        if let Some(ref bar) = self.action_bar {
                            if let Some(pos) = self.last_cursor_pos {
                                if let Some(id) = bar.check_click(egui::pos2(pos.0, pos.1)) {
                                    let id_owned = id.to_string();
                                    return self.execute_action(&id_owned);
                                }
                            }
                        }

                        // If any drawing tool is active and clicking in selection area, start drawing
                        // Use the new action-based drawing lifecycle
                        if self.registry.has_active_drawing_tool() {
                            if let Some(pos) = self.last_cursor_pos {
                                if self.is_inside_selection(pos) {
                                    // Record snapshot before starting draw for undo
                                    let snapshot = self.annotations.snapshot();
                                    self.history.record(snapshot);

                                    let egui_pos = egui::pos2(pos.0, pos.1);
                                    let selection_bounds = self.selection.bounds();
                                    let mut draw_ctx = DrawingContext::new(
                                        &mut self.annotations,
                                        &self.annotate_popup.settings,
                                        selection_bounds,
                                        self.scale_factor as f32,
                                    );
                                    self.registry.on_draw_start(egui_pos, &mut draw_ctx);
                                    return None;
                                }
                            }
                        }

                        // Close popup if clicking outside
                        if self.annotate_popup.visible {
                            self.annotate_popup.hide();
                        }

                        // Click outside action bar and selection - reset
                        if !self.is_inside_selection(self.last_cursor_pos.unwrap_or((0.0, 0.0))) {
                            // Check if clicking inside action bar
                            let inside_action_bar = self
                                .action_bar
                                .as_ref()
                                .map(|bar| bar.contains(egui::pos2(
                                    self.last_cursor_pos.unwrap_or((0.0, 0.0)).0,
                                    self.last_cursor_pos.unwrap_or((0.0, 0.0)).1,
                                )))
                                .unwrap_or(false);
                            if !inside_action_bar {
                                self.selection.reset();
                                self.action_bar = None;
                                self.options_panel = None;
                                self.annotate_active = false;
                                self.annotate_popup.hide();
                                self.state = ModeState::Idle;
                            }
                        }
                    }
                    _ => {}
                }
                None
            }
            ElementState::Released => {
                // Finish handle dragging
                if self.dragging_handle.is_some() {
                    self.dragging_handle = None;
                    self.update_toolbar_position();
                }

                // Finish action bar dragging
                if self.action_bar_dragging {
                    if let Some(ref mut bar) = self.action_bar {
                        bar.stop_drag();
                    }
                    self.action_bar_dragging = false;
                }

                // Finish any ongoing drawing using the action-based lifecycle
                if self.registry.has_active_drawing_tool() {
                    let selection_bounds = self.selection.bounds();
                    let mut draw_ctx = DrawingContext::new(
                        &mut self.annotations,
                        &self.annotate_popup.settings,
                        selection_bounds,
                        self.scale_factor as f32,
                    );
                    self.registry.on_draw_end(&mut draw_ctx);
                }

                if self.state == ModeState::Selecting {
                    self.selection.finish();

                    // Check if selection is large enough
                    if let Some((w, h)) = self.selection.size() {
                        if w > 5.0 && h > 5.0 {
                            // Create action bar
                            if let Some(bounds) = self.selection.bounds() {
                                let actions = self.registry.get_enabled();
                                // Create new ActionBar
                                let min_pos = egui::pos2(bounds.0 .0, bounds.0 .1);
                                let max_pos = egui::pos2(bounds.1 .0, bounds.1 .1);
                                let screen_size_vec =
                                    egui::vec2(self.screen_size.0, self.screen_size.1);
                                self.action_bar = Some(ActionBar::new(
                                    &actions,
                                    (min_pos, max_pos),
                                    screen_size_vec,
                                    self.scale_factor as f32,
                                ));
                                // Also create options panel
                                if let Some(ref bar) = self.action_bar {
                                    self.options_panel =
                                        Some(OptionsPanel::new(bar.position(), bar.size()));
                                }
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

    /// Get the rect for a handle at given position
    fn handle_rect(&self, handle: HandlePosition) -> Option<egui::Rect> {
        let ((min_x, min_y), (max_x, max_y)) = self.selection.bounds()?;
        let mid_x = (min_x + max_x) / 2.0;
        let mid_y = (min_y + max_y) / 2.0;

        let center = match handle {
            HandlePosition::TopLeft => egui::pos2(min_x, min_y),
            HandlePosition::TopCenter => egui::pos2(mid_x, min_y),
            HandlePosition::TopRight => egui::pos2(max_x, min_y),
            HandlePosition::MiddleLeft => egui::pos2(min_x, mid_y),
            HandlePosition::MiddleRight => egui::pos2(max_x, mid_y),
            HandlePosition::BottomLeft => egui::pos2(min_x, max_y),
            HandlePosition::BottomCenter => egui::pos2(mid_x, max_y),
            HandlePosition::BottomRight => egui::pos2(max_x, max_y),
        };

        Some(egui::Rect::from_center_size(
            center,
            egui::vec2(HANDLE_SIZE, HANDLE_SIZE),
        ))
    }

    /// Check which handle (if any) is at the given position
    fn handle_at_pos(&self, pos: (f32, f32)) -> Option<HandlePosition> {
        let point = egui::pos2(pos.0, pos.1);
        for handle in HandlePosition::all() {
            if let Some(rect) = self.handle_rect(handle) {
                if rect.contains(point) {
                    return Some(handle);
                }
            }
        }
        None
    }

    /// Update toolbar position after selection resize
    fn update_toolbar_position(&mut self) {
        if let Some(bounds) = self.selection.bounds() {
            let actions = self.registry.get_enabled();
            // Update ActionBar position
            let min_pos = egui::pos2(bounds.0 .0, bounds.0 .1);
            let max_pos = egui::pos2(bounds.1 .0, bounds.1 .1);
            let screen_size_vec = egui::vec2(self.screen_size.0, self.screen_size.1);
            self.action_bar = Some(ActionBar::new(
                &actions,
                (min_pos, max_pos),
                screen_size_vec,
                self.scale_factor as f32,
            ));
            // Update options panel position
            if let Some(ref bar) = self.action_bar {
                self.options_panel = Some(OptionsPanel::new(bar.position(), bar.size()));
            }
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
                // Handle resize drag
                if let Some(handle) = self.dragging_handle {
                    self.selection.resize(handle, pos);
                    return;
                }

                // Handle action bar dragging
                if self.action_bar_dragging {
                    if let Some(ref mut bar) = self.action_bar {
                        let screen_size = egui::vec2(self.screen_size.0, self.screen_size.1);
                        bar.update_drag(egui::pos2(pos.0, pos.1), screen_size);
                    }
                    return;
                }

                // Update hovered handle for cursor
                self.hovered_handle = self.handle_at_pos(pos);

                // If any drawing tool is active, update the current drawing using action lifecycle
                if self.registry.has_active_drawing_tool() {
                    let egui_pos = egui::pos2(pos.0, pos.1);
                    let selection_bounds = self.selection.bounds();
                    let mut draw_ctx = DrawingContext::new(
                        &mut self.annotations,
                        &self.annotate_popup.settings,
                        selection_bounds,
                        self.scale_factor as f32,
                    );
                    self.registry.on_draw_move(egui_pos, &mut draw_ctx);
                }

                // Update hovered button via action bar
                if let Some(ref mut bar) = self.action_bar {
                    bar.update_hover(egui::pos2(pos.0, pos.1));
                    self.hovered_button = bar.hovered_button().map(|s| s.to_string());
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
        // Create action context with both physical and logical selection bounds
        let selection_physical = self.selection.bounds_physical(self.scale_factor as f32);
        let selection_logical = self.selection.bounds();
        let ctx = ActionContext::new(
            selection_physical,
            selection_logical,
            self.screenshot.as_ref(),
            Some(&self.annotations),
            self.monitor.as_ref(),
            self.scale_factor,
        );

        // Execute action (registry handles mutual exclusion)
        let result = self.registry.execute(id, &ctx);

        // Handle result
        match &result {
            ActionResult::Exit => {
                self.state = ModeState::Exiting;
            }
            ActionResult::Undo => {
                // Perform undo operation
                let current = self.annotations.snapshot();
                if let Some(prev) = self.history.undo(current) {
                    self.annotations.restore(prev);
                    log::info!("Undo performed");
                }
            }
            ActionResult::Redo => {
                // Perform redo operation
                let current = self.annotations.snapshot();
                if let Some(next) = self.history.redo(current) {
                    self.annotations.restore(next);
                    log::info!("Redo performed");
                }
            }
            ActionResult::Continue => {
                // Update annotate_active based on registry state
                self.annotate_active = self.registry.is_tool_active("annotate");

                // Handle annotate popup visibility
                if id == "annotate" {
                    if self.annotate_active {
                        // Find annotate button position from action bar
                        if let Some(ref bar) = self.action_bar {
                            // Position popup above the action bar
                            let bar_pos = bar.position();
                            self.annotate_popup.show(egui::pos2(bar_pos.x, bar_pos.y - 150.0));
                        }
                    } else {
                        self.annotate_popup.hide();
                    }
                } else if !self.annotate_active {
                    // If another tool was activated, hide annotate popup
                    self.annotate_popup.hide();
                }
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

                    // Note: Size indicator moved to top-left settings toolbar

                    // Render annotations inside selection area
                    self.render_annotations(ui);

                    // Render resize handles when toolbar is visible
                    if self.state == ModeState::ToolbarVisible {
                        self.render_handles(ui);
                    }
                } else {
                    // No selection - full overlay
                    ui.painter().rect_filled(screen_rect, egui::CornerRadius::ZERO, overlay_color);
                }

                // Render new action bar if visible
                if let Some(ref mut bar) = self.action_bar {
                    // Get the active drawing/privacy tool from registry
                    let active_tool = self
                        .registry
                        .get_active_tool(ToolCategory::Drawing)
                        .or_else(|| self.registry.get_active_tool(ToolCategory::Privacy));
                    let can_undo = self.history.can_undo();
                    let can_redo = self.history.can_redo();
                    bar.render(ui, active_tool, can_undo, can_redo);
                }

                // Render options panel if a drawing tool is active
                if let Some(ref mut panel) = self.options_panel {
                    let active_tool = self
                        .registry
                        .get_active_tool(ToolCategory::Drawing)
                        .or_else(|| self.registry.get_active_tool(ToolCategory::Privacy));
                    if let Some(tool_id) = active_tool {
                        panel.set_tool(tool_id);
                        panel.render(ui);
                    }
                }
            },
        );

        // Render annotation popup (outside the Area so it's on top)
        self.annotate_popup.render(ctx);

        // Render settings toolbar (top-left, above selection) with coords and resolution
        if self.state == ModeState::ToolbarVisible {
            if let Some(((min_x, min_y), (max_x, max_y))) = self.selection.bounds() {
                // Calculate start coordinates in physical pixels
                let start_x = (min_x * self.scale_factor as f32) as i32;
                let start_y = (min_y * self.scale_factor as f32) as i32;

                // Calculate resolution in physical pixels
                let width = ((max_x - min_x) * self.scale_factor as f32) as u32;
                let height = ((max_y - min_y) * self.scale_factor as f32) as u32;

                self.settings_toolbar.update_position((min_x, min_y));
                if let Some(button) = self.settings_toolbar.render(
                    ctx,
                    &mut self.settings,
                    (start_x, start_y),
                    (width, height),
                ) {
                    match button {
                        SettingsButton::Refresh => {
                            self.refresh_screenshot();
                        }
                    }
                }
            }
        }
    }

    /// Render all annotations
    fn render_annotations(&self, ui: &egui::Ui) {
        // Render highlighters (below other annotations)
        for highlighter in &self.annotations.highlighters {
            self.render_highlighter(ui, highlighter);
        }
        if let Some(ref highlighter) = self.annotations.current_highlighter {
            self.render_highlighter(ui, highlighter);
        }

        // Render completed shapes
        for shape in &self.annotations.shapes {
            self.render_shape(ui, shape);
        }

        // Render current shape being drawn
        if let Some(ref shape) = self.annotations.current_shape {
            self.render_shape(ui, shape);
        }

        // Render completed strokes
        for stroke in &self.annotations.strokes {
            self.render_stroke(ui, stroke);
        }

        // Render current stroke being drawn
        if let Some(ref stroke) = self.annotations.current_stroke {
            self.render_stroke(ui, stroke);
        }

        // Render completed arrows
        for arrow in &self.annotations.arrows {
            self.render_arrow(ui, arrow);
        }

        // Render current arrow being drawn
        if let Some(ref arrow) = self.annotations.current_arrow {
            self.render_arrow(ui, arrow);
        }

        // Render polylines
        for polyline in &self.annotations.polylines {
            self.render_polyline(ui, polyline);
        }
        if let Some(ref polyline) = self.annotations.current_polyline {
            self.render_polyline(ui, polyline);
        }

        // Render sequence markers (on top of everything)
        for marker in &self.annotations.markers {
            self.render_marker(ui, marker);
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

    /// Re-capture the screen
    fn refresh_screenshot(&mut self) {
        if let Some(ref monitor) = self.monitor {
            if let Ok(new_screenshot) = monitor.capture_image() {
                self.screenshot = Some(new_screenshot);
                log::info!("Screenshot refreshed");
            } else {
                log::warn!("Failed to refresh screenshot");
            }
        }
    }

    /// Render resize handles at selection corners and edges
    fn render_handles(&self, ui: &egui::Ui) {
        for handle in HandlePosition::all() {
            if let Some(rect) = self.handle_rect(handle) {
                let is_hovered = self.hovered_handle == Some(handle);
                let fill = if is_hovered {
                    egui::Color32::from_rgb(0, 150, 255)
                } else {
                    egui::Color32::WHITE
                };
                ui.painter().rect_filled(rect, 2.0, fill);
                ui.painter().rect_stroke(
                    rect,
                    2.0,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 120, 255)),
                    egui::StrokeKind::Outside,
                );
            }
        }
    }

    /// Render an arrow annotation
    fn render_arrow(&self, ui: &egui::Ui, arrow: &Arrow) {
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

    /// Render a shape annotation (rectangle or ellipse)
    fn render_shape(&self, ui: &egui::Ui, shape: &Shape) {
        let stroke = egui::Stroke::new(shape.settings.width, shape.settings.color);

        match (shape.shape_type, shape.fill_mode) {
            (ShapeType::Rectangle, FillMode::Filled) => {
                ui.painter()
                    .rect_filled(shape.rect, 0.0, shape.settings.color);
            }
            (ShapeType::Rectangle, FillMode::Outline) => {
                ui.painter()
                    .rect_stroke(shape.rect, 0.0, stroke, egui::StrokeKind::Inside);
            }
            (ShapeType::Ellipse, FillMode::Filled) => {
                let center = shape.rect.center();
                let radius = shape.rect.size() / 2.0;
                // egui doesn't have native ellipse, approximate with circle for now
                // TODO: Use custom path for true ellipse
                let avg_radius = (radius.x + radius.y) / 2.0;
                ui.painter()
                    .circle_filled(center, avg_radius, shape.settings.color);
            }
            (ShapeType::Ellipse, FillMode::Outline) => {
                let center = shape.rect.center();
                let radius = shape.rect.size() / 2.0;
                let avg_radius = (radius.x + radius.y) / 2.0;
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

    /// Render a sequence marker
    fn render_marker(&self, ui: &egui::Ui, marker: &SequenceMarker) {
        // Draw filled circle
        ui.painter().circle_filled(marker.pos, marker.radius, marker.color);

        // Draw border
        ui.painter()
            .circle_stroke(marker.pos, marker.radius, egui::Stroke::new(2.0, egui::Color32::WHITE));

        // Draw label
        let label = marker.label();
        let font = egui::FontId::proportional(marker.radius * 1.2);
        ui.painter().text(
            marker.pos,
            egui::Align2::CENTER_CENTER,
            label,
            font,
            egui::Color32::WHITE,
        );
    }

    /// Render a polyline annotation
    fn render_polyline(&self, ui: &egui::Ui, polyline: &Polyline) {
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
            ui.painter().line_segment(
                [*polyline.points.last().unwrap(), polyline.points[0]],
                stroke,
            );
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

    /// Render a highlighter annotation (semi-transparent rectangle)
    fn render_highlighter(&self, ui: &egui::Ui, highlighter: &Highlighter) {
        // Draw a semi-transparent filled rectangle
        ui.painter().rect_filled(highlighter.rect, 0.0, highlighter.color);
    }
}
