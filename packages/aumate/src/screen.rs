//! Screen capture module
//!
//! Provides screen capture and pixel operations for desktop automation.

use crate::error::{AumateError, Result};
use image::{ImageBuffer, ImageEncoder, RgbaImage};
use xcap::Monitor;

/// Screen capture result containing image data
#[derive(Debug, Clone)]
pub struct ScreenCapture {
    pub width: u32,
    pub height: u32,
    pub image: Vec<u8>,
}

/// Screen size information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
}

/// Pixel color information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl PixelColor {
    /// Create a new PixelColor
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to hex string (e.g., "#FF0000")
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to hex string with alpha (e.g., "#FF0000FF")
    pub fn to_hex_with_alpha(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

/// Capture the entire screen
///
/// # Returns
/// A ScreenCapture object containing the captured image as PNG buffer
pub fn capture_screen() -> Result<ScreenCapture> {
    capture_screen_region(None, None, None, None)
}

/// Capture a region of the screen
///
/// # Arguments
/// * `x` - X coordinate of the top-left corner (optional)
/// * `y` - Y coordinate of the top-left corner (optional)
/// * `width` - Width of the region to capture (optional)
/// * `height` - Height of the region to capture (optional)
///
/// # Returns
/// A ScreenCapture object containing the captured image as PNG buffer
pub fn capture_screen_region(
    x: Option<u32>,
    y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<ScreenCapture> {
    let monitors = Monitor::all()
        .map_err(|e| AumateError::Screen(format!("Failed to get monitors: {}", e)))?;

    if monitors.is_empty() {
        return Err(AumateError::Screen("No monitors found".to_string()));
    }

    // Use the first monitor
    let monitor = &monitors[0];
    let image = monitor
        .capture_image()
        .map_err(|e| AumateError::Screen(format!("Failed to capture screen: {}", e)))?;

    let img_width = image.width();
    let img_height = image.height();

    // Handle region capture
    let (x, y, width, height) = match (x, y, width, height) {
        (Some(x), Some(y), Some(w), Some(h)) => {
            let x = x.min(img_width);
            let y = y.min(img_height);
            let w = w.min(img_width - x);
            let h = h.min(img_height - y);
            (x, y, w, h)
        }
        _ => (0, 0, img_width, img_height),
    };

    // Get the raw RGBA buffer from xcap
    let raw_buffer = image.as_raw();

    // Extract region if needed
    let mut region_buffer = Vec::new();
    if x == 0 && y == 0 && width == img_width && height == img_height {
        // Full screen - xcap already returns RGBA
        region_buffer.extend_from_slice(raw_buffer);
    } else {
        // Extract region
        region_buffer.reserve((width * height * 4) as usize);
        for row in y..(y + height) {
            for col in x..(x + width) {
                let idx = ((row * img_width + col) * 4) as usize;
                if idx + 3 < raw_buffer.len() {
                    region_buffer.extend_from_slice(&raw_buffer[idx..idx + 4]);
                }
            }
        }
    }

    // Create RGBA image
    let rgba_image: RgbaImage = ImageBuffer::from_raw(width, height, region_buffer)
        .ok_or_else(|| AumateError::Screen("Failed to create image buffer".to_string()))?;

    // Convert to PNG bytes
    let mut png_bytes = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        encoder
            .write_image(rgba_image.as_raw(), width, height, image::ExtendedColorType::Rgba8)
            .map_err(|e| AumateError::Screen(format!("Failed to encode PNG: {}", e)))?;
    }

    Ok(ScreenCapture { width, height, image: png_bytes })
}

/// Get the screen size of the primary monitor
///
/// # Returns
/// A ScreenSize object containing width and height
pub fn get_screen_size() -> Result<ScreenSize> {
    let monitors = Monitor::all()
        .map_err(|e| AumateError::Screen(format!("Failed to get monitors: {}", e)))?;

    if monitors.is_empty() {
        return Err(AumateError::Screen("No monitors found".to_string()));
    }

    let monitor = &monitors[0];

    Ok(ScreenSize {
        width: monitor
            .width()
            .map_err(|e| AumateError::Screen(format!("Failed to get monitor width: {}", e)))?,
        height: monitor
            .height()
            .map_err(|e| AumateError::Screen(format!("Failed to get monitor height: {}", e)))?,
    })
}

/// Get the pixel color at the specified coordinates
///
/// # Arguments
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
/// A PixelColor object containing RGBA values
pub fn get_pixel_color(x: u32, y: u32) -> Result<PixelColor> {
    let monitors = Monitor::all()
        .map_err(|e| AumateError::Screen(format!("Failed to get monitors: {}", e)))?;

    if monitors.is_empty() {
        return Err(AumateError::Screen("No monitors found".to_string()));
    }

    let monitor = &monitors[0];
    let image = monitor
        .capture_image()
        .map_err(|e| AumateError::Screen(format!("Failed to capture screen: {}", e)))?;

    let img_width = image.width();
    let img_height = image.height();

    if x >= img_width || y >= img_height {
        return Err(AumateError::Screen(format!(
            "Coordinates out of bounds: ({}, {}) for screen size {}x{}",
            x, y, img_width, img_height
        )));
    }

    let buffer = image.as_raw();
    let index = ((y * img_width + x) * 4) as usize;

    if index + 3 >= buffer.len() {
        return Err(AumateError::Screen("Invalid buffer index".to_string()));
    }

    // xcap returns RGBA format
    Ok(PixelColor {
        r: buffer[index],
        g: buffer[index + 1],
        b: buffer[index + 2],
        a: buffer[index + 3],
    })
}

/// Get all monitors
pub fn get_monitors() -> Result<Vec<MonitorInfo>> {
    let monitors = Monitor::all()
        .map_err(|e| AumateError::Screen(format!("Failed to get monitors: {}", e)))?;

    monitors
        .iter()
        .enumerate()
        .map(|(i, m)| {
            Ok(MonitorInfo {
                id: i as u32,
                name: m.name().unwrap_or_else(|_| format!("Monitor {}", i)),
                width: m.width().map_err(|e| {
                    AumateError::Screen(format!("Failed to get monitor width: {}", e))
                })?,
                height: m.height().map_err(|e| {
                    AumateError::Screen(format!("Failed to get monitor height: {}", e))
                })?,
                x: m.x()
                    .map_err(|e| AumateError::Screen(format!("Failed to get monitor x: {}", e)))?,
                y: m.y()
                    .map_err(|e| AumateError::Screen(format!("Failed to get monitor y: {}", e)))?,
                is_primary: i == 0,
            })
        })
        .collect()
}

/// Monitor information
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub is_primary: bool,
}
