//! Screenshot appearance settings
//!
//! Controls corner radius, aspect ratio lock, border, and shadow.

/// Screenshot appearance settings
#[derive(Debug, Clone)]
pub struct ScreenshotSettings {
    /// Corner radius for final image (0 = square corners)
    pub corner_radius: f32,
    /// Lock aspect ratio during resize
    pub aspect_locked: bool,
    /// Locked aspect ratio (width/height), None means use current ratio
    pub aspect_ratio: Option<f32>,
    /// Border width (0 = no border)
    pub border_width: f32,
    /// Border color
    pub border_color: egui::Color32,
    /// Shadow enabled
    pub shadow_enabled: bool,
    /// Shadow blur radius
    pub shadow_blur: f32,
    /// Shadow offset (x, y)
    pub shadow_offset: (f32, f32),
    /// Shadow color
    pub shadow_color: egui::Color32,
}

impl Default for ScreenshotSettings {
    fn default() -> Self {
        Self {
            corner_radius: 0.0,
            aspect_locked: false,
            aspect_ratio: None,
            border_width: 0.0,
            border_color: egui::Color32::BLACK,
            shadow_enabled: false,
            shadow_blur: 10.0,
            shadow_offset: (4.0, 4.0),
            shadow_color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 128),
        }
    }
}

/// Preset corner radius values
pub const CORNER_RADIUS_PRESETS: &[f32] = &[0.0, 8.0, 16.0, 24.0, 32.0];

/// Preset aspect ratios: (ratio, label)
pub const ASPECT_PRESETS: &[(f32, &str)] = &[
    (1.0, "1:1"),
    (4.0 / 3.0, "4:3"),
    (16.0 / 9.0, "16:9"),
    (16.0 / 10.0, "16:10"),
    (21.0 / 9.0, "21:9"),
];

/// Preset border widths
pub const BORDER_WIDTH_PRESETS: &[f32] = &[0.0, 1.0, 2.0, 4.0, 8.0];
