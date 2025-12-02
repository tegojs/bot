//! SVG icon loading and rendering for screenshot toolbar
//!
//! This module provides functionality to load SVG icons at compile time
//! and render them to RGBA pixel data for use with egui.

use egui::{Color32, ColorImage, TextureHandle, TextureOptions};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Icon size in logical pixels (will be scaled by DPI)
pub const ICON_SIZE: u32 = 18;

/// Get the actual render size for an icon based on scale factor
pub fn icon_render_size(scale_factor: f32) -> u32 {
    // Render at 2x for crisp icons on retina displays
    ((ICON_SIZE as f32) * scale_factor.max(2.0)).ceil() as u32
}

/// Static storage for loaded icons
static ICONS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

/// Get all embedded SVG icons
fn get_svg_icons() -> &'static HashMap<&'static str, &'static str> {
    ICONS.get_or_init(|| {
        let mut icons = HashMap::new();
        // Drawing tools
        icons.insert("drag_handle", include_str!("icons/drag_handle.svg"));
        icons.insert("rectangle", include_str!("icons/rectangle.svg"));
        icons.insert("ellipse", include_str!("icons/ellipse.svg"));
        icons.insert("polyline", include_str!("icons/polyline.svg"));
        icons.insert("arrow", include_str!("icons/arrow.svg"));
        icons.insert("annotate", include_str!("icons/annotate.svg"));
        icons.insert("highlighter", include_str!("icons/highlighter.svg"));
        icons.insert("mosaic", include_str!("icons/mosaic.svg"));
        icons.insert("blur", include_str!("icons/blur.svg"));
        icons.insert("text", include_str!("icons/text.svg"));
        icons.insert("sequence", include_str!("icons/sequence.svg"));
        icons.insert("eraser", include_str!("icons/eraser.svg"));
        icons.insert("smart", include_str!("icons/smart.svg"));
        // Edit tools
        icons.insert("undo", include_str!("icons/undo.svg"));
        icons.insert("redo", include_str!("icons/redo.svg"));
        // Action tools
        icons.insert("crop", include_str!("icons/crop.svg"));
        icons.insert("fullscreen", include_str!("icons/fullscreen.svg"));
        icons.insert("cancel", include_str!("icons/cancel.svg"));
        icons.insert("pin", include_str!("icons/pin.svg"));
        icons.insert("save", include_str!("icons/save.svg"));
        icons.insert("copy", include_str!("icons/copy.svg"));
        icons.insert("menu", include_str!("icons/menu.svg"));
        icons
    })
}

/// Get an SVG icon string by ID
pub fn get_svg(id: &str) -> Option<&'static str> {
    get_svg_icons().get(id).copied()
}

/// Render an SVG icon to RGBA pixel data
///
/// # Arguments
/// * `id` - The icon identifier
/// * `size` - Target size in pixels
/// * `color` - The color to render the icon in
///
/// # Returns
/// RGBA pixel data as a Vec<u8>, or None if the icon doesn't exist
pub fn render_svg_icon(id: &str, size: u32, color: Color32) -> Option<Vec<u8>> {
    let svg_str = get_svg(id)?;

    // Replace currentColor with the target color
    let hex_color = format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b());
    let svg_with_color = svg_str
        .replace("currentColor", &hex_color)
        .replace("stroke=\"currentColor\"", &format!("stroke=\"{}\"", hex_color))
        .replace("fill=\"currentColor\"", &format!("fill=\"{}\"", hex_color));

    // Parse SVG
    let options = resvg::usvg::Options::default();
    let tree = match resvg::usvg::Tree::from_str(&svg_with_color, &options) {
        Ok(tree) => tree,
        Err(e) => {
            log::warn!("Failed to parse SVG '{}': {}", id, e);
            return None;
        }
    };

    // Create render target
    let size_f = size as f32;
    let tree_size = tree.size();
    let scale_x = size_f / tree_size.width();
    let scale_y = size_f / tree_size.height();
    let scale = scale_x.min(scale_y);

    let mut pixmap = match tiny_skia::Pixmap::new(size, size) {
        Some(p) => p,
        None => {
            log::warn!("Failed to create pixmap for icon '{}'", id);
            return None;
        }
    };

    // Center the icon
    let offset_x = (size_f - tree_size.width() * scale) / 2.0;
    let offset_y = (size_f - tree_size.height() * scale) / 2.0;

    let transform =
        tiny_skia::Transform::from_scale(scale, scale).post_translate(offset_x, offset_y);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // Apply alpha from color
    let alpha = color.a() as f32 / 255.0;
    let data = pixmap.data_mut();
    for chunk in data.chunks_exact_mut(4) {
        chunk[3] = (chunk[3] as f32 * alpha) as u8;
    }

    Some(pixmap.take())
}

/// Create an egui texture from an SVG icon
pub fn create_icon_texture(
    ctx: &egui::Context,
    id: &str,
    size: u32,
    color: Color32,
) -> Option<TextureHandle> {
    let pixels = render_svg_icon(id, size, color)?;

    let image = ColorImage::from_rgba_unmultiplied([size as usize, size as usize], &pixels);

    Some(ctx.load_texture(
        format!("icon_{}_{}", id, size),
        image,
        TextureOptions::NEAREST, // Use nearest for crisp pixel-perfect icons
    ))
}

/// Icon texture cache for efficient rendering
pub struct IconCache {
    textures: HashMap<(String, u32, u32), TextureHandle>,
}

impl IconCache {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    /// Get or create an icon texture
    pub fn get_or_create(
        &mut self,
        ctx: &egui::Context,
        id: &str,
        size: u32,
        color: Color32,
    ) -> Option<TextureHandle> {
        let key = (
            id.to_string(),
            size,
            color.to_array().into_iter().fold(0u32, |acc, b| acc * 256 + b as u32),
        );

        if let Some(texture) = self.textures.get(&key) {
            return Some(texture.clone());
        }

        let texture = create_icon_texture(ctx, id, size, color)?;
        self.textures.insert(key, texture.clone());
        Some(texture)
    }

    /// Clear all cached textures
    pub fn clear(&mut self) {
        self.textures.clear();
    }
}

impl Default for IconCache {
    fn default() -> Self {
        Self::new()
    }
}

/// List of all available icon IDs
pub const ICON_IDS: &[&str] = &[
    "drag_handle",
    "rectangle",
    "ellipse",
    "polyline",
    "arrow",
    "annotate",
    "highlighter",
    "mosaic",
    "blur",
    "text",
    "sequence",
    "eraser",
    "smart",
    "undo",
    "redo",
    "crop",
    "fullscreen",
    "cancel",
    "pin",
    "save",
    "copy",
    "menu",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_icons_load() {
        for id in ICON_IDS {
            assert!(get_svg(id).is_some(), "Icon '{}' should exist", id);
        }
    }

    #[test]
    fn test_render_icon() {
        let pixels = render_svg_icon("rectangle", 24, Color32::WHITE);
        assert!(pixels.is_some());
        let data = pixels.unwrap();
        assert_eq!(data.len(), 24 * 24 * 4);
    }
}
