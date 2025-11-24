// Screen capture functionality

use super::types::*;
use image::{ImageBuffer, ImageEncoder, RgbaImage};
use xcap::Monitor;

/// Capture screen region
/// If region is None, captures the entire primary monitor
pub async fn capture_screen_region(
    region: Option<ScreenRegion>,
) -> Result<ScreenshotResult, String> {
    tokio::task::spawn_blocking(move || capture_screen_region_sync(region))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

/// Synchronous screen capture (runs in blocking task)
fn capture_screen_region_sync(region: Option<ScreenRegion>) -> Result<ScreenshotResult, String> {
    // Get all monitors
    let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;

    if monitors.is_empty() {
        return Err("No monitors found".to_string());
    }

    // Use primary monitor
    let monitor = &monitors[0];

    // Capture the monitor
    let image = monitor.capture_image().map_err(|e| format!("Failed to capture screen: {}", e))?;

    let full_width = image.width();
    let full_height = image.height();

    // Determine region to capture
    let capture_region = region.unwrap_or_else(|| ScreenRegion {
        x: 0,
        y: 0,
        width: full_width,
        height: full_height,
    });

    // Validate region
    if capture_region.x < 0
        || capture_region.y < 0
        || capture_region.x as u32 + capture_region.width > full_width
        || capture_region.y as u32 + capture_region.height > full_height
    {
        return Err(format!(
            "Invalid region: {}x{} at ({}, {}), screen size: {}x{}",
            capture_region.width,
            capture_region.height,
            capture_region.x,
            capture_region.y,
            full_width,
            full_height
        ));
    }

    // Extract region from full capture
    let region_buffer =
        if region.is_some() { extract_region(&image, &capture_region)? } else { image.to_vec() };

    // Encode as PNG
    let png_data = encode_png(&region_buffer, capture_region.width, capture_region.height)?;

    Ok(ScreenshotResult::new(png_data, capture_region))
}

/// Extract a region from captured image buffer
fn extract_region(image: &[u8], region: &ScreenRegion) -> Result<Vec<u8>, String> {
    // xcap returns BGRA format, need to convert to RGBA
    let full_width = (image.len() / 4) as u32;
    let _full_height = (image.len() as u32) / (full_width * 4);

    let mut region_buffer = Vec::with_capacity((region.width * region.height * 4) as usize);

    for y in 0..region.height {
        for x in 0..region.width {
            let src_x = (region.x as u32 + x) as usize;
            let src_y = (region.y as u32 + y) as usize;
            let src_idx = (src_y * full_width as usize + src_x) * 4;

            if src_idx + 3 < image.len() {
                // Convert BGRA to RGBA
                region_buffer.push(image[src_idx + 2]); // R
                region_buffer.push(image[src_idx + 1]); // G
                region_buffer.push(image[src_idx]); // B
                region_buffer.push(image[src_idx + 3]); // A
            }
        }
    }

    Ok(region_buffer)
}

/// Encode image buffer as PNG
pub fn encode_png(buffer: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let rgba_image: RgbaImage = ImageBuffer::from_raw(width, height, buffer.to_vec())
        .ok_or_else(|| "Failed to create image buffer".to_string())?;

    let mut png_bytes = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        encoder
            .write_image(rgba_image.as_raw(), width, height, image::ExtendedColorType::Rgba8)
            .map_err(|e| format!("Failed to encode PNG: {}", e))?;
    }

    Ok(png_bytes)
}

/// Encode image buffer as JPEG
pub fn encode_jpeg(buffer: &[u8], width: u32, height: u32, quality: u8) -> Result<Vec<u8>, String> {
    // Convert RGBA to RGB (JPEG doesn't support alpha)
    let mut rgb_buffer = Vec::with_capacity((width * height * 3) as usize);
    for chunk in buffer.chunks(4) {
        if chunk.len() >= 3 {
            rgb_buffer.push(chunk[0]); // R
            rgb_buffer.push(chunk[1]); // G
            rgb_buffer.push(chunk[2]); // B
        }
    }

    let mut jpeg_bytes = Vec::new();
    {
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, quality);
        encoder
            .write_image(&rgb_buffer, width, height, image::ExtendedColorType::Rgb8)
            .map_err(|e| format!("Failed to encode JPEG: {}", e))?;
    }

    Ok(jpeg_bytes)
}

/// Encode image buffer as WebP
pub fn encode_webp(
    buffer: &[u8],
    width: u32,
    height: u32,
    _quality: u8,
) -> Result<Vec<u8>, String> {
    // For now, fall back to PNG encoding
    // Full WebP support would require additional dependencies
    // This is a placeholder for future implementation
    encode_png(buffer, width, height)
}

/// Save screenshot result to file
pub fn save_to_file(
    result: &ScreenshotResult,
    path: &str,
    options: Option<SaveImageOptions>,
) -> Result<(), String> {
    let options = options.unwrap_or_default();

    // Determine format from extension if not specified
    let format = if let Some(ext) = std::path::Path::new(path).extension() {
        ImageFormat::from_extension(ext.to_str().unwrap_or("png")).unwrap_or(options.format)
    } else {
        options.format
    };

    // For PNG, we already have the data
    let data = if format == ImageFormat::Png {
        result.image.clone()
    } else {
        // Decode PNG first
        let img = image::load_from_memory(&result.image)
            .map_err(|e| format!("Failed to decode image: {}", e))?;
        let rgba = img.to_rgba8();

        match format {
            ImageFormat::Jpg => encode_jpeg(
                rgba.as_raw(),
                result.region.width,
                result.region.height,
                options.quality,
            )?,
            ImageFormat::Webp => encode_webp(
                rgba.as_raw(),
                result.region.width,
                result.region.height,
                options.quality,
            )?,
            _ => result.image.clone(),
        }
    };

    std::fs::write(path, data).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

/// Copy screenshot to clipboard
pub fn copy_to_clipboard(result: &ScreenshotResult) -> Result<(), String> {
    use arboard::Clipboard;

    // Decode PNG to raw image
    let img = image::load_from_memory(&result.image)
        .map_err(|e| format!("Failed to decode image: {}", e))?;
    let rgba = img.to_rgba8();

    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to initialize clipboard: {}", e))?;

    let img_data = arboard::ImageData {
        width: result.region.width as usize,
        height: result.region.height as usize,
        bytes: rgba.as_raw().into(),
    };

    clipboard.set_image(img_data).map_err(|e| format!("Failed to set clipboard: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_png() {
        let buffer = vec![255u8; 100 * 100 * 4]; // White 100x100 image
        let result = encode_png(&buffer, 100, 100);
        assert!(result.is_ok());
        let png_data = result.unwrap();
        assert!(!png_data.is_empty());
    }

    #[test]
    fn test_encode_jpeg() {
        let buffer = vec![255u8; 100 * 100 * 4]; // White 100x100 image
        let result = encode_jpeg(&buffer, 100, 100, 90);
        assert!(result.is_ok());
        let jpeg_data = result.unwrap();
        assert!(!jpeg_data.is_empty());
    }
}
