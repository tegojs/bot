//! ScreenshotMode - main state machine for screenshot functionality
//!
//! Coordinates selection, toolbar, and action execution.

use image::{ImageBuffer, Rgba};
use winit::event::{ElementState, MouseButton, WindowEvent};
use xcap::Monitor;

use super::action::{ActionContext, ActionResult};
use super::registry::{ActionRegistry, create_default_registry};
use super::selection::Selection;
use super::toolbar::Toolbar;
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
    /// Whether annotation mode is active (reserved for future use)
    #[allow(dead_code)]
    annotate_mode: bool,
    /// Whether text mode is active (reserved for future use)
    #[allow(dead_code)]
    text_mode: bool,
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

        Ok(Self {
            state: ModeState::Idle,
            selection: Selection::new(),
            toolbar: None,
            registry,
            screenshot: Some(screenshot),
            monitor: Some(monitor),
            scale_factor,
            screen_size: (
                screen_width as f32 / scale_factor as f32,
                screen_height as f32 / scale_factor as f32,
            ),
            hovered_button: None,
            annotate_mode: false,
            text_mode: false,
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
    /// Returns Some(ActionResult) if an action was executed
    pub fn handle_event(&mut self, event: &WindowEvent) -> Option<ActionResult> {
        match event {
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_button(*button, *state)
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos = (position.x as f32, position.y as f32);
                self.handle_cursor_move(pos);
                None
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
                        // Check if clicking a button
                        if self.toolbar.is_some() {
                            if let Some(id) = self.hovered_button.clone() {
                                return self.execute_action(&id);
                            }
                        }
                        // Click outside toolbar - reset selection
                        self.selection.reset();
                        self.toolbar = None;
                        self.state = ModeState::Idle;
                    }
                    _ => {}
                }
                None
            }
            ElementState::Released => {
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

    fn handle_cursor_move(&mut self, pos: (f32, f32)) {
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
        egui::Area::new(egui::Id::new("screenshot_overlay")).fixed_pos(egui::pos2(0.0, 0.0)).show(
            ctx,
            |ui| {
                let screen_rect = egui::Rect::from_min_size(
                    egui::pos2(0.0, 0.0),
                    egui::vec2(self.screen_size.0, self.screen_size.1),
                );

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
    }
}
