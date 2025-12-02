//! ScreenAction trait and related types
//!
//! Defines the interface for screenshot actions (save, copy, annotate, etc.)

use egui::Pos2;
use image::{ImageBuffer, Rgba};

use super::composite::composite_annotations;
use super::stroke::{Annotations, StrokeSettings};

/// Context passed to each action when executed
pub struct ActionContext<'a> {
    /// Selection coordinates in physical pixels: ((min_x, min_y), (max_x, max_y))
    pub selection: Option<((u32, u32), (u32, u32))>,
    /// Selection coordinates in logical pixels for coordinate conversion
    pub selection_logical: Option<((f32, f32), (f32, f32))>,
    /// The captured screenshot image (full screen)
    pub screenshot: Option<&'a ImageBuffer<Rgba<u8>, Vec<u8>>>,
    /// Annotations drawn on the screenshot
    pub annotations: Option<&'a Annotations>,
    /// The monitor used for capture
    pub monitor: Option<&'a xcap::Monitor>,
    /// Scale factor for DPI conversion
    pub scale_factor: f64,
}

impl<'a> ActionContext<'a> {
    /// Create a new action context
    pub fn new(
        selection: Option<((u32, u32), (u32, u32))>,
        selection_logical: Option<((f32, f32), (f32, f32))>,
        screenshot: Option<&'a ImageBuffer<Rgba<u8>, Vec<u8>>>,
        annotations: Option<&'a Annotations>,
        monitor: Option<&'a xcap::Monitor>,
        scale_factor: f64,
    ) -> Self {
        Self {
            selection,
            selection_logical,
            screenshot,
            annotations,
            monitor,
            scale_factor,
        }
    }

    /// Get the selected region from the screenshot (without annotations)
    ///
    /// This returns only the base screenshot region without any drawn annotations.
    /// Used internally by `get_composited_region()` as a fallback.
    fn get_selected_region(&self) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let ((x1, y1), (x2, y2)) = self.selection?;
        let screenshot = self.screenshot?;

        let width = x2.saturating_sub(x1);
        let height = y2.saturating_sub(y1);

        if width == 0 || height == 0 {
            return None;
        }

        // Extract the selected region
        let mut region = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let src_x = x1 + x;
                let src_y = y1 + y;
                if src_x < screenshot.width() && src_y < screenshot.height() {
                    let pixel = screenshot.get_pixel(src_x, src_y);
                    region.put_pixel(x, y, *pixel);
                }
            }
        }

        Some(region)
    }

    /// Get the selected region with annotations composited onto it
    ///
    /// This is the main method for getting the final screenshot image that includes
    /// all user-drawn annotations (strokes, arrows, shapes, etc.).
    pub fn get_composited_region(&self) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let selection = self.selection?;
        let selection_logical = self.selection_logical?;
        let screenshot = self.screenshot?;

        // If no annotations, just return the base region (more efficient)
        let annotations = match self.annotations {
            Some(a) if !a.is_empty() => a,
            _ => return self.get_selected_region(),
        };

        Some(composite_annotations(
            screenshot,
            annotations,
            selection,
            selection_logical,
            self.scale_factor as f32,
        ))
    }
}

/// Result from action execution
#[derive(Debug, Clone)]
pub enum ActionResult {
    /// Action completed successfully
    Success,
    /// Action failed with error message
    Failure(String),
    /// Exit screenshot mode (e.g., after save or cancel)
    Exit,
    /// Continue in screenshot mode (e.g., for toggle actions like annotate)
    Continue,
    /// Request undo operation
    Undo,
    /// Request redo operation
    Redo,
}

/// Tool category for mutual exclusion
///
/// Drawing and Privacy tools are mutually exclusive within their category.
/// Only one tool can be active at a time in each category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolCategory {
    /// Drawing tools (annotate, arrow, polyline, rectangle, ellipse, sequence, text, highlighter)
    /// Mutually exclusive - only one can be active at a time
    Drawing,
    /// Privacy tools (mosaic, blur)
    /// Mutually exclusive - only one can be active at a time
    Privacy,
    /// Terminal actions (cancel, copy, save)
    /// These don't toggle - they execute and exit or complete
    #[default]
    Action,
}

/// Information about an action for UI display
#[derive(Debug, Clone)]
pub struct ActionInfo {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Icon ID for looking up in icons module (e.g., "arrow", "rectangle")
    pub icon_id: Option<String>,
    /// Tool category
    pub category: ToolCategory,
}

/// Context for drawing operations
///
/// Provides actions with access to annotations storage and settings
/// during drawing lifecycle events.
pub struct DrawingContext<'a> {
    /// Mutable access to annotations storage
    pub annotations: &'a mut Annotations,
    /// Current stroke settings (color, width, style)
    pub settings: &'a StrokeSettings,
    /// Selection bounds in logical pixels (for clipping)
    pub selection_bounds: Option<((f32, f32), (f32, f32))>,
    /// DPI scale factor
    pub scale_factor: f32,
}

impl<'a> DrawingContext<'a> {
    /// Create a new drawing context
    pub fn new(
        annotations: &'a mut Annotations,
        settings: &'a StrokeSettings,
        selection_bounds: Option<((f32, f32), (f32, f32))>,
        scale_factor: f32,
    ) -> Self {
        Self {
            annotations,
            settings,
            selection_bounds,
            scale_factor,
        }
    }

    /// Check if a position is within the selection bounds
    pub fn is_in_bounds(&self, pos: Pos2) -> bool {
        match self.selection_bounds {
            Some(((min_x, min_y), (max_x, max_y))) => {
                pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y
            }
            None => true, // No bounds = everything is valid
        }
    }

    /// Clamp a position to selection bounds
    pub fn clamp_to_bounds(&self, pos: Pos2) -> Pos2 {
        match self.selection_bounds {
            Some(((min_x, min_y), (max_x, max_y))) => {
                Pos2::new(pos.x.clamp(min_x, max_x), pos.y.clamp(min_y, max_y))
            }
            None => pos,
        }
    }
}

/// The ScreenAction trait - defines the interface for screenshot actions
///
/// Actions are registered with the ActionRegistry and displayed in the toolbar
/// after the user completes a selection.
///
/// # Drawing Actions
///
/// Actions that are drawing tools (category = Drawing or Privacy) can implement
/// the drawing lifecycle methods to handle mouse interactions:
///
/// - `is_drawing_tool()` - Return true if this action handles drawing
/// - `on_draw_start()` - Called when mouse is pressed inside selection
/// - `on_draw_move()` - Called when mouse moves while drawing
/// - `on_draw_end()` - Called when mouse is released
/// - `render_preview()` - Optional: render current drawing state preview
///
/// This allows each action to independently manage its drawing logic without
/// requiring changes to mode.rs.
pub trait ScreenAction: Send + Sync {
    /// Unique identifier for this action (e.g., "save", "copy")
    fn id(&self) -> &str;

    /// Human-readable name for display (e.g., "Save", "Copy to Clipboard")
    fn name(&self) -> &str;

    /// Icon ID for looking up in shared icons module (e.g., "arrow", "rectangle")
    ///
    /// Returns the icon ID that can be used with `icons::get_svg()` to get the SVG data.
    /// If None, the action's ID will be used as a fallback.
    fn icon_id(&self) -> Option<&str> {
        None
    }

    /// Execute the action
    ///
    /// Called when the user clicks the action's toolbar button.
    /// Returns an ActionResult indicating what should happen next.
    fn on_click(&mut self, ctx: &ActionContext) -> ActionResult;

    /// Get the tool category for mutual exclusion
    ///
    /// Drawing and Privacy tools are mutually exclusive within their category.
    /// Default is Action (terminal actions that don't toggle).
    fn category(&self) -> ToolCategory {
        ToolCategory::Action
    }

    /// Check if this tool is currently active
    ///
    /// Only applicable for toggle tools (Drawing and Privacy categories).
    fn is_active(&self) -> bool {
        false
    }

    /// Set the active state of this tool
    ///
    /// Called by the registry to deactivate tools when another tool
    /// in the same category is activated.
    fn set_active(&mut self, _active: bool) {}

    // ==================== Drawing Lifecycle Methods ====================

    /// Check if this action is a drawing tool
    ///
    /// Drawing tools handle mouse events through the drawing lifecycle methods.
    /// Return true to indicate that `on_draw_start`, `on_draw_move`, and
    /// `on_draw_end` should be called for this action.
    fn is_drawing_tool(&self) -> bool {
        false
    }

    /// Called when mouse is pressed to start drawing
    ///
    /// The position is in logical pixels (screen coordinates).
    /// Use the DrawingContext to access annotations and settings.
    fn on_draw_start(&mut self, _pos: Pos2, _ctx: &mut DrawingContext) {}

    /// Called when mouse moves during drawing
    ///
    /// This is called continuously while drawing (mouse button held).
    /// Update the current annotation based on the new position.
    fn on_draw_move(&mut self, _pos: Pos2, _ctx: &mut DrawingContext) {}

    /// Called when mouse is released to finish drawing
    ///
    /// Finalize the current annotation and add it to the history.
    fn on_draw_end(&mut self, _ctx: &mut DrawingContext) {}

    /// Render a preview of the current drawing state (optional)
    ///
    /// Called during render to show the in-progress drawing.
    /// Default implementation does nothing.
    fn render_preview(&self, _ui: &egui::Ui) {}

    // ==================== Info ====================

    /// Get action info for UI display
    fn info(&self) -> ActionInfo {
        ActionInfo {
            id: self.id().to_string(),
            name: self.name().to_string(),
            icon_id: self.icon_id().map(|s| s.to_string()),
            category: self.category(),
        }
    }
}
