// Screenshot module - Interactive screenshot tool with annotations
//
// This module provides:
// - Interactive region selection with window snapping
// - Color picker
// - Basic annotations (arrow, rectangle, brush)
// - Multi-format export (PNG, JPG, WebP)

mod annotations;
mod capture;
mod color_picker;
pub mod napi_bindings;
mod overlay;
mod selection;
mod types;

pub use annotations::*;
pub use capture::*;
pub use color_picker::*;
pub use napi_bindings::*;
pub use selection::{check_window_snap, detect_windows, snap_to_window};
pub use types::*;

use std::sync::{Arc, Mutex};

/// Main screenshot tool instance
pub struct ScreenshotTool {
    /// Tool configuration options
    options: ScreenshotToolOptions,
    /// Current selection state (if in interactive mode)
    selection_state: Arc<Mutex<Option<SelectionState>>>,
}

impl ScreenshotTool {
    /// Create a new screenshot tool instance
    pub fn new(options: Option<ScreenshotToolOptions>) -> Self {
        Self { options: options.unwrap_or_default(), selection_state: Arc::new(Mutex::new(None)) }
    }

    /// Start interactive screenshot capture
    /// Returns the captured screenshot after user confirms selection
    pub async fn capture_interactive(
        &self,
        options: Option<InteractiveCaptureOptions>,
    ) -> Result<ScreenshotResult, String> {
        // Will be implemented in selection module
        selection::start_interactive_capture(
            options.unwrap_or_default(),
            self.selection_state.clone(),
        )
        .await
    }

    /// Quick screenshot without interaction
    /// If region is None, captures the entire screen
    pub async fn capture_quick(
        &self,
        region: Option<ScreenRegion>,
    ) -> Result<ScreenshotResult, String> {
        capture::capture_screen_region(region).await
    }

    /// Get pixel color at specific coordinates
    pub async fn get_pixel_color(&self, x: u32, y: u32) -> Result<ColorInfo, String> {
        color_picker::get_pixel_color_at(x, y).await
    }

    /// Start interactive color picker mode
    pub async fn pick_color(
        &self,
        options: Option<ColorPickerOptions>,
    ) -> Result<ColorInfo, String> {
        color_picker::start_color_picker(options.unwrap_or_default()).await
    }

    /// Get current selection info (if in interactive mode)
    pub fn get_current_selection(&self) -> Option<SelectionInfo> {
        let state = self.selection_state.lock().unwrap();
        state.as_ref().map(|s| s.to_selection_info())
    }

    /// Close and cleanup resources
    pub async fn close(&self) {
        let mut state = self.selection_state.lock().unwrap();
        *state = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_tool_creation() {
        let tool = ScreenshotTool::new(None);
        assert!(tool.get_current_selection().is_none());
    }
}
