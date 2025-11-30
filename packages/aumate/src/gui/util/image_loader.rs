//! Image loading utilities for PNG/SVG files

use std::path::Path;

/// Loaded image data in RGBA format
pub struct LoadedImage {
    /// RGBA pixel data
    pub data: Vec<u8>,
    /// Image width in pixels
    pub width: u32,
    /// Image height in pixels
    pub height: u32,
}

/// Load an image from file path (PNG, JPG, or SVG)
///
/// # Arguments
/// * `path` - Path to the image file
/// * `target_width` - Optional target width for resizing
/// * `target_height` - Optional target height for resizing
///
/// # Returns
/// `LoadedImage` with RGBA pixel data
pub fn load_image(
    path: &Path,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> Result<LoadedImage, String> {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    match ext.to_lowercase().as_str() {
        "svg" => load_svg(path, target_width, target_height),
        _ => load_raster(path, target_width, target_height),
    }
}

/// Load an SVG file and render to RGBA pixels
fn load_svg(
    path: &Path,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> Result<LoadedImage, String> {
    let svg_data =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read SVG: {}", e))?;

    let tree = resvg::usvg::Tree::from_str(&svg_data, &resvg::usvg::Options::default())
        .map_err(|e| format!("Failed to parse SVG: {}", e))?;

    let size = tree.size();
    let width = target_width.unwrap_or(size.width() as u32);
    let height = target_height.unwrap_or(size.height() as u32);

    // Ensure minimum size
    let width = width.max(1);
    let height = height.max(1);

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| "Failed to create pixmap".to_string())?;

    let scale_x = width as f32 / size.width() as f32;
    let scale_y = height as f32 / size.height() as f32;
    let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Ok(LoadedImage { data: pixmap.take(), width, height })
}

/// Load a raster image (PNG, JPG, etc.) and convert to RGBA
fn load_raster(
    path: &Path,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> Result<LoadedImage, String> {
    let img = image::open(path).map_err(|e| format!("Failed to open image: {}", e))?;

    let img = if let (Some(w), Some(h)) = (target_width, target_height) {
        img.resize_exact(w, h, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    Ok(LoadedImage { data: rgba.into_raw(), width, height })
}
