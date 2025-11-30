//! Clipboard operations module
//!
//! Provides clipboard text and image operations for desktop automation.

use crate::error::{AumateError, Result};
use arboard::Clipboard;
use image::ImageEncoder;
use std::sync::Mutex;

// Thread-safe clipboard instance
static CLIPBOARD: Mutex<Option<Clipboard>> = Mutex::new(None);

fn get_or_init_clipboard() -> Result<std::sync::MutexGuard<'static, Option<Clipboard>>> {
    let mut guard =
        CLIPBOARD.lock().map_err(|e| AumateError::Clipboard(format!("Lock error: {}", e)))?;

    if guard.is_none() {
        let clipboard = Clipboard::new().map_err(|e| {
            AumateError::Clipboard(format!("Failed to initialize clipboard: {}", e))
        })?;
        *guard = Some(clipboard);
    }

    Ok(guard)
}

/// Get text from clipboard
pub fn get_text() -> Result<String> {
    let mut guard = get_or_init_clipboard()?;
    let clipboard = guard
        .as_mut()
        .ok_or_else(|| AumateError::Clipboard("Clipboard not initialized".to_string()))?;

    clipboard
        .get_text()
        .map_err(|e| AumateError::Clipboard(format!("Failed to get clipboard text: {}", e)))
}

/// Set text to clipboard
pub fn set_text(text: &str) -> Result<()> {
    let mut guard = get_or_init_clipboard()?;
    let clipboard = guard
        .as_mut()
        .ok_or_else(|| AumateError::Clipboard("Clipboard not initialized".to_string()))?;

    clipboard
        .set_text(text.to_string())
        .map_err(|e| AumateError::Clipboard(format!("Failed to set clipboard text: {}", e)))
}

/// Get image from clipboard (returns PNG-encoded buffer)
pub fn get_image() -> Result<Vec<u8>> {
    let mut guard = get_or_init_clipboard()?;
    let clipboard = guard
        .as_mut()
        .ok_or_else(|| AumateError::Clipboard("Clipboard not initialized".to_string()))?;

    let image_data = clipboard
        .get_image()
        .map_err(|e| AumateError::Clipboard(format!("Failed to get clipboard image: {}", e)))?;

    // Convert arboard ImageData to PNG buffer
    let rgba_data = image_data.bytes.to_vec();
    let width = image_data.width as u32;
    let height = image_data.height as u32;

    // Create image from raw RGBA data
    let img = image::RgbaImage::from_raw(width, height, rgba_data).ok_or_else(|| {
        AumateError::Clipboard("Failed to create image from clipboard data".to_string())
    })?;

    // Encode as PNG
    let mut png_data = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder
        .write_image(img.as_raw(), width, height, image::ExtendedColorType::Rgba8)
        .map_err(|e| AumateError::Clipboard(format!("Failed to encode image as PNG: {}", e)))?;

    Ok(png_data)
}

/// Set image to clipboard (accepts PNG-encoded buffer)
pub fn set_image(image_buffer: &[u8]) -> Result<()> {
    let mut guard = get_or_init_clipboard()?;
    let clipboard = guard
        .as_mut()
        .ok_or_else(|| AumateError::Clipboard("Clipboard not initialized".to_string()))?;

    // Decode PNG buffer
    let img = image::load_from_memory(image_buffer)
        .map_err(|e| AumateError::Clipboard(format!("Failed to decode image: {}", e)))?;

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    // Convert to arboard ImageData
    let image_data = arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: rgba.into_raw().into(),
    };

    clipboard
        .set_image(image_data)
        .map_err(|e| AumateError::Clipboard(format!("Failed to set clipboard image: {}", e)))
}

/// Set image to clipboard from raw RGBA data
pub fn set_image_raw(width: u32, height: u32, rgba_data: Vec<u8>) -> Result<()> {
    let mut guard = get_or_init_clipboard()?;
    let clipboard = guard
        .as_mut()
        .ok_or_else(|| AumateError::Clipboard("Clipboard not initialized".to_string()))?;

    let image_data = arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: rgba_data.into(),
    };

    clipboard
        .set_image(image_data)
        .map_err(|e| AumateError::Clipboard(format!("Failed to set clipboard image: {}", e)))
}

/// Clear clipboard
pub fn clear() -> Result<()> {
    let mut guard = get_or_init_clipboard()?;
    let clipboard = guard
        .as_mut()
        .ok_or_else(|| AumateError::Clipboard("Clipboard not initialized".to_string()))?;

    clipboard
        .clear()
        .map_err(|e| AumateError::Clipboard(format!("Failed to clear clipboard: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_text() {
        let test_text = "Hello, clipboard!";
        set_text(test_text).unwrap();

        let retrieved = get_text().unwrap();
        assert_eq!(retrieved, test_text);
    }

    #[test]
    fn test_clear_clipboard() {
        set_text("test").unwrap();
        clear().unwrap();
        // Getting clipboard might fail or return empty string after clear
        let _ = get_text();
    }
}
