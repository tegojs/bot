// Type definitions for screenshot module

use serde::{Deserialize, Serialize};

/// Screenshot tool configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotToolOptions {
    /// Default save path (optional)
    pub default_save_path: Option<String>,
    /// Auto copy to clipboard after capture
    pub auto_copy_to_clipboard: bool,
}

impl Default for ScreenshotToolOptions {
    fn default() -> Self {
        Self { default_save_path: None, auto_copy_to_clipboard: false }
    }
}

/// Interactive capture options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveCaptureOptions {
    /// Show grid overlay
    pub show_grid: bool,
    /// Show coordinate info
    pub show_coordinates: bool,
    /// Show size info
    pub show_size: bool,
    /// Hint text to display
    pub hint_text: Option<String>,
    /// Enable window snapping
    pub enable_window_snap: bool,
    /// Window snap threshold in pixels
    pub snap_threshold: u32,
}

impl Default for InteractiveCaptureOptions {
    fn default() -> Self {
        Self {
            show_grid: false,
            show_coordinates: true,
            show_size: true,
            hint_text: None,
            enable_window_snap: true,
            snap_threshold: 10,
        }
    }
}

/// Screen region definition
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ScreenRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl ScreenRegion {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if this region intersects with another
    pub fn intersects(&self, other: &ScreenRegion) -> bool {
        let self_right = self.x + self.width as i32;
        let self_bottom = self.y + self.height as i32;
        let other_right = other.x + other.width as i32;
        let other_bottom = other.y + other.height as i32;

        self.x < other_right
            && self_right > other.x
            && self.y < other_bottom
            && self_bottom > other.y
    }

    /// Check if a point is within threshold distance of this region's edges
    pub fn is_near_edge(&self, x: i32, y: i32, threshold: u32) -> bool {
        let threshold = threshold as i32;
        let right = self.x + self.width as i32;
        let bottom = self.y + self.height as i32;

        // Check if point is near any edge
        (x >= self.x - threshold && x <= self.x + threshold && y >= self.y && y <= bottom)
            || (x >= right - threshold && x <= right + threshold && y >= self.y && y <= bottom)
            || (y >= self.y - threshold && y <= self.y + threshold && x >= self.x && x <= right)
            || (y >= bottom - threshold && y <= bottom + threshold && x >= self.x && x <= right)
    }
}

/// Screenshot result containing captured image and metadata
#[derive(Debug, Clone)]
pub struct ScreenshotResult {
    /// PNG-encoded image data
    pub image: Vec<u8>,
    /// Capture region
    pub region: ScreenRegion,
    /// Timestamp of capture
    pub timestamp: i64,
}

impl ScreenshotResult {
    pub fn new(image: Vec<u8>, region: ScreenRegion) -> Self {
        Self { image, region, timestamp: chrono::Utc::now().timestamp() }
    }
}

/// Selection information (for callbacks)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionInfo {
    /// Current selection region
    pub region: ScreenRegion,
    /// Is selection snapped to window
    pub is_snapped: bool,
    /// Snapped window info (if snapped)
    pub snapped_window: Option<WindowSnapInfo>,
    /// Can resize the selection
    pub can_resize: bool,
}

/// Window snap information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSnapInfo {
    /// Window ID
    pub window_id: String,
    /// Window title
    pub title: String,
    /// Window region
    pub region: ScreenRegion,
    /// Which edge(s) are snapped
    pub snap_edge: String,
}

/// Color information in multiple formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorInfo {
    /// RGB values (0-255)
    pub rgb: RgbColor,
    /// RGBA values (0-255, alpha 0-1)
    pub rgba: RgbaColor,
    /// Hex color string (#RRGGBB)
    pub hex: String,
    /// HSL values (h: 0-360, s: 0-100, l: 0-100)
    pub hsl: HslColor,
    /// Position where color was picked
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HslColor {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

/// Color picker options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPickerOptions {
    /// Magnifier size in pixels
    pub magnifier_size: u32,
    /// Zoom level for magnifier
    pub zoom: u32,
    /// Show color history
    pub show_history: bool,
}

impl Default for ColorPickerOptions {
    fn default() -> Self {
        Self { magnifier_size: 150, zoom: 8, show_history: true }
    }
}

/// Internal selection state
#[derive(Debug, Clone)]
pub struct SelectionState {
    /// Current selection region
    pub region: ScreenRegion,
    /// Is currently dragging/resizing
    pub is_dragging: bool,
    /// Drag start position
    pub drag_start: Option<Position>,
    /// Currently snapped window (if any)
    pub snapped_window: Option<WindowSnapInfo>,
    /// Which resize handle is being dragged (if any)
    pub resize_handle: Option<ResizeHandle>,
}

impl SelectionState {
    pub fn to_selection_info(&self) -> SelectionInfo {
        SelectionInfo {
            region: self.region,
            is_snapped: self.snapped_window.is_some(),
            snapped_window: self.snapped_window.clone(),
            can_resize: true,
        }
    }
}

/// Resize handle positions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

/// Image save options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveImageOptions {
    /// Image format
    pub format: ImageFormat,
    /// Quality (1-100, for JPG/WebP)
    pub quality: u8,
}

impl Default for SaveImageOptions {
    fn default() -> Self {
        Self { format: ImageFormat::Png, quality: 90 }
    }
}

/// Supported image formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpg,
    Webp,
}

impl ImageFormat {
    /// Get format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(ImageFormat::Png),
            "jpg" | "jpeg" => Some(ImageFormat::Jpg),
            "webp" => Some(ImageFormat::Webp),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_intersects() {
        let r1 = ScreenRegion::new(0, 0, 100, 100);
        let r2 = ScreenRegion::new(50, 50, 100, 100);
        let r3 = ScreenRegion::new(200, 200, 100, 100);

        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_region_near_edge() {
        let r = ScreenRegion::new(100, 100, 200, 200);

        // Near left edge
        assert!(r.is_near_edge(105, 150, 10));
        // Not near any edge
        assert!(!r.is_near_edge(200, 200, 10));
    }

    #[test]
    fn test_image_format_from_extension() {
        assert_eq!(ImageFormat::from_extension("png"), Some(ImageFormat::Png));
        assert_eq!(ImageFormat::from_extension("jpg"), Some(ImageFormat::Jpg));
        assert_eq!(ImageFormat::from_extension("webp"), Some(ImageFormat::Webp));
        assert_eq!(ImageFormat::from_extension("gif"), None);
    }
}
