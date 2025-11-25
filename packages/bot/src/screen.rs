use image::{ImageBuffer, ImageEncoder, RgbaImage};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use xcap::Monitor;

/// Screen capture result containing image data
#[napi(object)]
pub struct ScreenCapture {
    pub width: u32,
    pub height: u32,
    pub image: Buffer,
}

/// Screen size information
#[napi(object)]
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
}

/// Capture the entire screen (internal function)
///
/// # Returns
/// A ScreenCapture object containing the captured image as PNG buffer
#[allow(dead_code)]
pub(crate) async fn capture_screen() -> Result<ScreenCapture> {
    capture_screen_region(None, None, None, None).await
}

/// Capture a region of the screen (internal function)
///
/// # Arguments
/// * `x` - X coordinate of the top-left corner (optional)
/// * `y` - Y coordinate of the top-left corner (optional)
/// * `width` - Width of the region to capture (optional)
/// * `height` - Height of the region to capture (optional)
///
/// # Returns
/// A ScreenCapture object containing the captured image as PNG buffer
pub(crate) async fn capture_screen_region(
    x: Option<u32>,
    y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<ScreenCapture> {
    let monitors =
        Monitor::all().map_err(|e| Error::from_reason(format!("Failed to get monitors: {}", e)))?;

    if monitors.is_empty() {
        return Err(Error::from_reason("No monitors found"));
    }

    // Use the first monitor
    let monitor = &monitors[0];
    let image = monitor
        .capture_image()
        .map_err(|e| Error::from_reason(format!("Failed to capture screen: {}", e)))?;

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
    // xcap returns ImageBuffer<Rgba<u8>, Vec<u8>>, we can get the raw buffer
    let raw_buffer = image.as_raw();

    // Extract region if needed
    let mut region_buffer = Vec::new();
    if x == 0 && y == 0 && width == img_width && height == img_height {
        // Full screen - xcap already returns RGBA
        region_buffer.extend_from_slice(raw_buffer);
    } else {
        // Extract region - optimized for Rust 2024 edition
        // Using iterator chains for better performance and readability
        region_buffer.reserve((width * height * 4) as usize);
        for row in y..(y + height) {
            for col in x..(x + width) {
                let idx = ((row * img_width + col) * 4) as usize;
                if idx + 3 < raw_buffer.len() {
                    // Use slice pattern for better performance (Rust 2024 edition optimization)
                    region_buffer.extend_from_slice(&raw_buffer[idx..idx + 4]);
                }
            }
        }
    }

    // Create RGBA image
    let rgba_image: RgbaImage = ImageBuffer::from_raw(width, height, region_buffer)
        .ok_or_else(|| Error::from_reason("Failed to create image buffer"))?;

    // Convert to PNG bytes
    let mut png_bytes = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        // Use write_image with ExtendedColorType
        encoder
            .write_image(rgba_image.as_raw(), width, height, image::ExtendedColorType::Rgba8)
            .map_err(|e| Error::from_reason(format!("Failed to encode PNG: {}", e)))?;
    }

    Ok(ScreenCapture { width, height, image: Buffer::from(png_bytes) })
}

/// Get the screen size of the primary monitor (internal function)
///
/// # Returns
/// A ScreenSize object containing width and height
pub(crate) fn get_screen_size() -> Result<ScreenSize> {
    let monitors =
        Monitor::all().map_err(|e| Error::from_reason(format!("Failed to get monitors: {}", e)))?;

    if monitors.is_empty() {
        return Err(Error::from_reason("No monitors found"));
    }

    let monitor = &monitors[0];

    Ok(ScreenSize {
        width: monitor
            .width()
            .map_err(|e| Error::from_reason(format!("Failed to get monitor width: {}", e)))?,
        height: monitor
            .height()
            .map_err(|e| Error::from_reason(format!("Failed to get monitor height: {}", e)))?,
    })
}

/// Get the pixel color at the specified coordinates (internal function)
///
/// # Arguments
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
/// A PixelColor object containing RGBA values
pub(crate) async fn get_pixel_color(x: u32, y: u32) -> Result<PixelColor> {
    let monitors =
        Monitor::all().map_err(|e| Error::from_reason(format!("Failed to get monitors: {}", e)))?;

    if monitors.is_empty() {
        return Err(Error::from_reason("No monitors found"));
    }

    let monitor = &monitors[0];
    let image = monitor
        .capture_image()
        .map_err(|e| Error::from_reason(format!("Failed to capture screen: {}", e)))?;

    let img_width = image.width();
    let img_height = image.height();

    if x >= img_width || y >= img_height {
        return Err(Error::from_reason(format!(
            "Coordinates out of bounds: ({}, {}) for screen size {}x{}",
            x, y, img_width, img_height
        )));
    }

    let buffer = image.as_raw();
    let index = ((y * img_width + x) * 4) as usize;

    if index + 3 >= buffer.len() {
        return Err(Error::from_reason("Invalid buffer index"));
    }

    // xcap returns RGBA format
    let r = buffer[index];
    let g = buffer[index + 1];
    let b = buffer[index + 2];
    let a = buffer[index + 3];

    Ok(PixelColor { r: r as u32, g: g as u32, b: b as u32, a: a as u32 })
}

/// Pixel color information
#[napi(object)]
pub struct PixelColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}
