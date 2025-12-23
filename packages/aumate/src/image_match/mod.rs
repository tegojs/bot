//! Image template matching module
//!
//! Provides template matching functionality for finding UI elements on screen.
//!
//! # Example
//!
//! ```no_run
//! use aumate::image_match::{find_on_screen, MatchConfig};
//!
//! let template = image::open("button.png").unwrap();
//! let config = MatchConfig::new().with_confidence(0.8);
//!
//! if let Ok(Some(result)) = find_on_screen(&template, Some(config)) {
//!     println!("Found at ({}, {})", result.x, result.y);
//! }
//! ```

mod config;
mod engine;
mod result;

pub use config::MatchConfig;
pub use engine::ImageMatcher;
pub use result::MatchResult;

use crate::error::Result;
use image::DynamicImage;

/// Find first match of template in current screen
///
/// # Arguments
/// * `template` - Template image to search for
/// * `config` - Optional matching configuration
///
/// # Returns
/// * `Ok(Some(MatchResult))` - If template found
/// * `Ok(None)` - If template not found
/// * `Err(_)` - If screen capture or matching failed
pub fn find_on_screen(
    template: &DynamicImage,
    config: Option<MatchConfig>,
) -> Result<Option<MatchResult>> {
    let screen_capture = crate::screen::capture_screen()?;
    let screen = image::load_from_memory(&screen_capture.image)
        .map_err(|e| crate::error::AumateError::Other(format!("Failed to decode screen: {}", e)))?;
    ImageMatcher::find(&screen, template, &config.unwrap_or_default())
}

/// Find all matches of template in current screen
///
/// # Arguments
/// * `template` - Template image to search for
/// * `config` - Optional matching configuration
///
/// # Returns
/// * `Ok(Vec<MatchResult>)` - All matches found, sorted by confidence
/// * `Err(_)` - If screen capture or matching failed
pub fn find_all_on_screen(
    template: &DynamicImage,
    config: Option<MatchConfig>,
) -> Result<Vec<MatchResult>> {
    let screen_capture = crate::screen::capture_screen()?;
    let screen = image::load_from_memory(&screen_capture.image)
        .map_err(|e| crate::error::AumateError::Other(format!("Failed to decode screen: {}", e)))?;
    ImageMatcher::find_all(&screen, template, &config.unwrap_or_default())
}

/// Find first match of template in a region of the screen
///
/// # Arguments
/// * `template` - Template image to search for
/// * `x`, `y`, `width`, `height` - Region to search in
/// * `config` - Optional matching configuration
///
/// # Returns
/// * `Ok(Some(MatchResult))` - If template found (coordinates are relative to screen, not region)
/// * `Ok(None)` - If template not found
/// * `Err(_)` - If screen capture or matching failed
pub fn find_in_region(
    template: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    config: Option<MatchConfig>,
) -> Result<Option<MatchResult>> {
    let screen_capture =
        crate::screen::capture_screen_region(Some(x), Some(y), Some(width), Some(height))?;
    let screen = image::load_from_memory(&screen_capture.image)
        .map_err(|e| crate::error::AumateError::Other(format!("Failed to decode screen: {}", e)))?;

    let mut result = ImageMatcher::find(&screen, template, &config.unwrap_or_default())?;

    // Adjust coordinates to screen space
    if let Some(ref mut r) = result {
        r.x += x;
        r.y += y;
    }

    Ok(result)
}

/// Find all matches of template in a region of the screen
///
/// # Arguments
/// * `template` - Template image to search for
/// * `x`, `y`, `width`, `height` - Region to search in
/// * `config` - Optional matching configuration
///
/// # Returns
/// * `Ok(Vec<MatchResult>)` - All matches found (coordinates are relative to screen, not region)
/// * `Err(_)` - If screen capture or matching failed
pub fn find_all_in_region(
    template: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    config: Option<MatchConfig>,
) -> Result<Vec<MatchResult>> {
    let screen_capture =
        crate::screen::capture_screen_region(Some(x), Some(y), Some(width), Some(height))?;
    let screen = image::load_from_memory(&screen_capture.image)
        .map_err(|e| crate::error::AumateError::Other(format!("Failed to decode screen: {}", e)))?;

    let mut results = ImageMatcher::find_all(&screen, template, &config.unwrap_or_default())?;

    // Adjust coordinates to screen space
    for r in &mut results {
        r.x += x;
        r.y += y;
    }

    Ok(results)
}
