//! ScreenAction trait and related types
//!
//! Defines the interface for screenshot actions (save, copy, annotate, etc.)

use image::{ImageBuffer, Rgba};

/// Context passed to each action when executed
pub struct ActionContext<'a> {
    /// Selection coordinates in physical pixels: ((min_x, min_y), (max_x, max_y))
    pub selection: Option<((u32, u32), (u32, u32))>,
    /// The captured screenshot image (full screen)
    pub screenshot: Option<&'a ImageBuffer<Rgba<u8>, Vec<u8>>>,
    /// The monitor used for capture
    pub monitor: Option<&'a xcap::Monitor>,
    /// Scale factor for DPI conversion
    pub scale_factor: f64,
}

impl<'a> ActionContext<'a> {
    /// Create a new action context
    pub fn new(
        selection: Option<((u32, u32), (u32, u32))>,
        screenshot: Option<&'a ImageBuffer<Rgba<u8>, Vec<u8>>>,
        monitor: Option<&'a xcap::Monitor>,
        scale_factor: f64,
    ) -> Self {
        Self { selection, screenshot, monitor, scale_factor }
    }

    /// Get the selected region from the screenshot
    pub fn get_selected_region(&self) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
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
}

/// Information about an action for UI display
#[derive(Debug, Clone)]
pub struct ActionInfo {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Icon PNG bytes
    pub icon: Option<Vec<u8>>,
}

/// The ScreenAction trait - defines the interface for screenshot actions
///
/// Actions are registered with the ActionRegistry and displayed in the toolbar
/// after the user completes a selection.
pub trait ScreenAction: Send + Sync {
    /// Unique identifier for this action (e.g., "save", "copy")
    fn id(&self) -> &str;

    /// Human-readable name for display (e.g., "Save", "Copy to Clipboard")
    fn name(&self) -> &str;

    /// Icon as PNG-encoded bytes (typically 32x32)
    fn icon(&self) -> Option<&[u8]>;

    /// Execute the action
    ///
    /// Called when the user clicks the action's toolbar button.
    /// Returns an ActionResult indicating what should happen next.
    fn on_click(&mut self, ctx: &ActionContext) -> ActionResult;

    /// Get action info for UI display
    fn info(&self) -> ActionInfo {
        ActionInfo {
            id: self.id().to_string(),
            name: self.name().to_string(),
            icon: self.icon().map(|b| b.to_vec()),
        }
    }
}
