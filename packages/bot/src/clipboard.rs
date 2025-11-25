/**
 * Clipboard operations module
 */
use arboard::Clipboard;
use image::ImageEncoder;
use napi::bindgen_prelude::*;

/// Get text from clipboard
pub fn get_clipboard() -> Result<String> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::from_reason(format!("Failed to initialize clipboard: {}", e)))?;

    clipboard
        .get_text()
        .map_err(|e| Error::from_reason(format!("Failed to get clipboard text: {}", e)))
}

/// Set text to clipboard
pub fn set_clipboard(text: String) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::from_reason(format!("Failed to initialize clipboard: {}", e)))?;

    clipboard
        .set_text(text)
        .map_err(|e| Error::from_reason(format!("Failed to set clipboard text: {}", e)))
}

/// Get image from clipboard (returns PNG-encoded buffer)
pub fn get_clipboard_image() -> Result<Buffer> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::from_reason(format!("Failed to initialize clipboard: {}", e)))?;

    let image_data = clipboard
        .get_image()
        .map_err(|e| Error::from_reason(format!("Failed to get clipboard image: {}", e)))?;

    // Convert arboard ImageData to PNG buffer
    let rgba_data = image_data.bytes.to_vec();
    let width = image_data.width as u32;
    let height = image_data.height as u32;

    // Create image from raw RGBA data
    let img = image::RgbaImage::from_raw(width, height, rgba_data)
        .ok_or_else(|| Error::from_reason("Failed to create image from clipboard data"))?;

    // Encode as PNG
    let mut png_data = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder
        .write_image(img.as_raw(), width, height, image::ExtendedColorType::Rgba8)
        .map_err(|e| Error::from_reason(format!("Failed to encode image as PNG: {}", e)))?;

    Ok(Buffer::from(png_data))
}

/// Set image to clipboard (accepts PNG-encoded buffer)
pub fn set_clipboard_image(image_buffer: Buffer) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::from_reason(format!("Failed to initialize clipboard: {}", e)))?;

    // Decode PNG buffer
    let img = image::load_from_memory(&image_buffer)
        .map_err(|e| Error::from_reason(format!("Failed to decode image: {}", e)))?;

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
        .map_err(|e| Error::from_reason(format!("Failed to set clipboard image: {}", e)))
}

/// Clear clipboard
pub fn clear_clipboard() -> Result<()> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::from_reason(format!("Failed to initialize clipboard: {}", e)))?;

    clipboard.clear().map_err(|e| Error::from_reason(format!("Failed to clear clipboard: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_text() {
        // Test setting and getting text
        let test_text = "Hello, clipboard!";
        set_clipboard(test_text.to_string()).unwrap();

        let retrieved = get_clipboard().unwrap();
        assert_eq!(retrieved, test_text);
    }

    #[test]
    fn test_clear_clipboard() {
        // Set some text first
        set_clipboard("test".to_string()).unwrap();

        // Clear clipboard
        clear_clipboard().unwrap();

        // Getting clipboard might fail or return empty string after clear
        // This behavior varies by platform, so we just check it doesn't panic
        let _ = get_clipboard();
    }
}
