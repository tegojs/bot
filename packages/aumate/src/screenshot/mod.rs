//! Screenshot module
//!
//! Provides interactive screenshot functionality with region selection.
//! This module is integrated with the GUI controller.

pub mod action;
pub mod actions;
pub mod mode;
pub mod registry;
pub mod selection;
pub mod stroke;
pub mod toolbar;
pub mod ui;

pub use action::{ActionContext, ActionInfo, ActionResult, ScreenAction};
pub use mode::{ModeState, ScreenshotMode};
pub use registry::{ActionRegistry, create_default_registry};
pub use selection::Selection;
pub use toolbar::Toolbar;

use crate::error::Result;

/// Screen region for screenshot
#[derive(Debug, Clone, Copy)]
pub struct ScreenRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl ScreenRegion {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

/// Screenshot result
#[derive(Debug, Clone)]
pub struct ScreenshotResult {
    /// The captured region
    pub region: ScreenRegion,
    /// PNG-encoded image data
    pub image: Vec<u8>,
}

impl ScreenshotResult {
    pub fn new(region: ScreenRegion, image: Vec<u8>) -> Self {
        Self { region, image }
    }
}

/// Start interactive screenshot mode
///
/// This will be integrated with the GUI controller to allow
/// region selection with visual feedback.
pub fn start_interactive() -> Result<Option<ScreenshotResult>> {
    // TODO: Integrate with GUI controller
    // For now, this is a placeholder
    Ok(None)
}

/// Capture a specific region of the screen
pub fn capture_region(region: ScreenRegion) -> Result<ScreenshotResult> {
    #[cfg(feature = "screen")]
    {
        let capture = crate::screen::capture_screen_region(
            Some(region.x as u32),
            Some(region.y as u32),
            Some(region.width),
            Some(region.height),
        )?;
        Ok(ScreenshotResult::new(region, capture.image))
    }

    #[cfg(not(feature = "screen"))]
    {
        Err(crate::error::AumateError::Screenshot("Screen capture feature not enabled".to_string()))
    }
}
