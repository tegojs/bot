use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::codecs::webp::WebPEncoder;
use image::DynamicImage;
use std::io::Cursor;
use std::path::Path;
use tokio::fs;

use super::types::ImageFormat;

/// Encode an image to bytes in the specified format
pub fn encode_image(image: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>, String> {
    let mut buf = Vec::with_capacity(image.as_bytes().len() / 4);

    match format {
        ImageFormat::Png => {
            image
                .write_with_encoder(PngEncoder::new_with_quality(
                    &mut buf,
                    CompressionType::Fast,
                    FilterType::Paeth,
                ))
                .map_err(|e| format!("Failed to encode PNG: {}", e))?;
        }
        ImageFormat::Webp => {
            image
                .write_with_encoder(WebPEncoder::new_lossless(&mut buf))
                .map_err(|e| format!("Failed to encode WebP: {}", e))?;
        }
        ImageFormat::Jpeg => {
            // Convert to RGB for JPEG (no alpha channel)
            let rgb_image = DynamicImage::ImageRgb8(image.to_rgb8());
            rgb_image
                .write_with_encoder(JpegEncoder::new_with_quality(&mut buf, 90))
                .map_err(|e| format!("Failed to encode JPEG: {}", e))?;
        }
    }

    Ok(buf)
}

/// Save an image to a file
pub async fn save_image_to_file(
    image: &DynamicImage,
    file_path: &Path,
    format: ImageFormat,
) -> Result<(), String> {
    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let encoded = encode_image(image, format)?;

    fs::write(file_path, encoded)
        .await
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(())
}

/// Decode image bytes to DynamicImage
pub fn decode_image(bytes: &[u8]) -> Result<DynamicImage, String> {
    let cursor = Cursor::new(bytes);
    image::load(cursor, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to decode image: {}", e))
}

/// Get the file extension for an image format
pub fn get_extension(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "png",
        ImageFormat::Webp => "webp",
        ImageFormat::Jpeg => "jpg",
    }
}

/// Determine image format from file extension
pub fn format_from_extension(extension: &str) -> ImageFormat {
    match extension.to_lowercase().as_str() {
        "png" => ImageFormat::Png,
        "webp" => ImageFormat::Webp,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        _ => ImageFormat::Png,
    }
}
