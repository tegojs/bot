//! Stroke types for screenshot annotation
//!
//! Provides stroke settings (width, style, color) and annotation storage.

/// Stroke line style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StrokeStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

/// Stroke settings for annotation
#[derive(Debug, Clone)]
pub struct StrokeSettings {
    /// Line width in pixels
    pub width: f32,
    /// Line style
    pub style: StrokeStyle,
    /// Stroke color (RGBA)
    pub color: egui::Color32,
}

impl Default for StrokeSettings {
    fn default() -> Self {
        Self { width: 4.0, style: StrokeStyle::Solid, color: egui::Color32::RED }
    }
}

/// Preset colors for quick selection
pub const PRESET_COLORS: [egui::Color32; 7] = [
    egui::Color32::RED,
    egui::Color32::from_rgb(255, 165, 0), // Orange
    egui::Color32::YELLOW,
    egui::Color32::GREEN,
    egui::Color32::from_rgb(0, 120, 255), // Blue
    egui::Color32::BLACK,
    egui::Color32::WHITE,
];

/// Preset line widths
pub const PRESET_WIDTHS: [f32; 6] = [2.0, 4.0, 8.0, 12.0, 16.0, 24.0];

/// A single stroke (line segment collection)
#[derive(Debug, Clone)]
pub struct Stroke {
    /// Points in the stroke (logical coordinates)
    pub points: Vec<egui::Pos2>,
    /// Stroke settings when drawn
    pub settings: StrokeSettings,
}

/// Collection of annotations on the screenshot
#[derive(Debug, Clone, Default)]
pub struct Annotations {
    /// Completed strokes
    pub strokes: Vec<Stroke>,
    /// Current stroke being drawn (if any)
    pub current_stroke: Option<Stroke>,
}

impl Annotations {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new stroke at the given position
    pub fn start_stroke(&mut self, pos: egui::Pos2, settings: &StrokeSettings) {
        self.current_stroke = Some(Stroke { points: vec![pos], settings: settings.clone() });
    }

    /// Add point to current stroke
    pub fn add_point(&mut self, pos: egui::Pos2) {
        if let Some(ref mut stroke) = self.current_stroke {
            stroke.points.push(pos);
        }
    }

    /// Finish current stroke
    pub fn finish_stroke(&mut self) {
        if let Some(stroke) = self.current_stroke.take() {
            if stroke.points.len() > 1 {
                self.strokes.push(stroke);
            }
        }
    }

    /// Check if currently drawing
    pub fn is_drawing(&self) -> bool {
        self.current_stroke.is_some()
    }

    /// Clear all annotations
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.strokes.clear();
        self.current_stroke = None;
    }
}
