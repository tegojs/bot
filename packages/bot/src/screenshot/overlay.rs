// Interactive overlay - Simplified placeholder implementation

use super::types::*;

/// Run interactive overlay and return selected region
///
/// Note: Full egui + winit interactive overlay implementation
/// requires refactoring for winit 0.30 API compatibility.
/// This will be completed in a future update using the new
/// ApplicationHandler trait and proper async event loop integration.
pub async fn run_interactive_overlay(
    _options: InteractiveCaptureOptions,
) -> Result<ScreenshotResult, String> {
    Err("Interactive overlay not yet fully implemented. Use capture_quick() for programmatic screenshots.".to_string())
}
