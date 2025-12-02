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

/// Sequence marker style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SequenceStyle {
    #[default]
    Number, // 1, 2, 3...
    Letter, // A, B, C...
    Roman,  // I, II, III...
}

/// Shape type for rectangle/ellipse drawing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShapeType {
    #[default]
    Rectangle,
    Ellipse,
}

/// Fill mode for shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillMode {
    #[default]
    Outline, // Stroke only
    Filled, // Solid fill
}

/// Arrow annotation
#[derive(Debug, Clone)]
pub struct Arrow {
    /// Start point (no arrowhead)
    pub start: egui::Pos2,
    /// End point (has arrowhead)
    pub end: egui::Pos2,
    /// Stroke settings
    pub settings: StrokeSettings,
    /// Whether arrow is selected for editing
    pub selected: bool,
}

impl Arrow {
    pub fn new(start: egui::Pos2, end: egui::Pos2, settings: &StrokeSettings) -> Self {
        Self { start, end, settings: settings.clone(), selected: false }
    }

    /// Snap to 45-degree intervals
    pub fn snap_angle(&mut self) {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let angle = dy.atan2(dx);
        let snapped = (angle / (std::f32::consts::PI / 4.0)).round() * (std::f32::consts::PI / 4.0);
        let length = (dx * dx + dy * dy).sqrt();
        self.end = egui::pos2(
            self.start.x + length * snapped.cos(),
            self.start.y + length * snapped.sin(),
        );
    }

    /// Get direction vector from start to end (normalized)
    pub fn direction(&self) -> egui::Vec2 {
        (self.end - self.start).normalized()
    }

    /// Get length of arrow
    pub fn length(&self) -> f32 {
        self.start.distance(self.end)
    }
}

/// Sequence marker annotation (numbered circles)
#[derive(Debug, Clone)]
pub struct SequenceMarker {
    /// Center position
    pub pos: egui::Pos2,
    /// Sequence number (1-based)
    pub number: u32,
    /// Display style
    pub style: SequenceStyle,
    /// Circle radius
    pub radius: f32,
    /// Color
    pub color: egui::Color32,
}

impl SequenceMarker {
    pub fn new(pos: egui::Pos2, number: u32, style: SequenceStyle, color: egui::Color32) -> Self {
        Self { pos, number, style, radius: 12.0, color }
    }

    /// Get display label for this marker
    pub fn label(&self) -> String {
        match self.style {
            SequenceStyle::Number => self.number.to_string(),
            SequenceStyle::Letter => {
                let c = (b'A' + ((self.number - 1) % 26) as u8) as char;
                c.to_string()
            }
            SequenceStyle::Roman => to_roman(self.number),
        }
    }
}

/// Convert number to Roman numerals
fn to_roman(n: u32) -> String {
    let numerals = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut result = String::new();
    let mut n = n;
    for (value, numeral) in numerals {
        while n >= value {
            result.push_str(numeral);
            n -= value;
        }
    }
    result
}

/// Shape annotation (rectangle or ellipse)
#[derive(Debug, Clone)]
pub struct Shape {
    /// Bounding rect
    pub rect: egui::Rect,
    /// Shape type
    pub shape_type: ShapeType,
    /// Fill mode
    pub fill_mode: FillMode,
    /// Stroke settings (for outline)
    pub settings: StrokeSettings,
    /// Selected for editing
    pub selected: bool,
}

impl Shape {
    pub fn new(
        rect: egui::Rect,
        shape_type: ShapeType,
        fill_mode: FillMode,
        settings: &StrokeSettings,
    ) -> Self {
        Self { rect, shape_type, fill_mode, settings: settings.clone(), selected: false }
    }

    /// Create from two corner points
    pub fn from_corners(
        start: egui::Pos2,
        end: egui::Pos2,
        shape_type: ShapeType,
        fill_mode: FillMode,
        settings: &StrokeSettings,
    ) -> Self {
        let rect = egui::Rect::from_two_pos(start, end);
        Self::new(rect, shape_type, fill_mode, settings)
    }
}

/// Polyline annotation (connected line segments, 折线绘制)
///
/// Unlike freehand strokes, polylines have discrete vertex points
/// that the user clicks to create. Good for technical diagrams.
#[derive(Debug, Clone)]
pub struct Polyline {
    /// Vertex points of the polyline
    pub points: Vec<egui::Pos2>,
    /// Stroke settings
    pub settings: StrokeSettings,
    /// Whether the polyline is closed (forms a polygon)
    pub closed: bool,
    /// Whether currently being edited
    pub selected: bool,
}

impl Polyline {
    pub fn new(settings: &StrokeSettings) -> Self {
        Self { points: Vec::new(), settings: settings.clone(), closed: false, selected: false }
    }

    /// Add a vertex point
    pub fn add_point(&mut self, pos: egui::Pos2) {
        self.points.push(pos);
    }

    /// Update the last point (for preview while drawing)
    pub fn update_last(&mut self, pos: egui::Pos2) {
        if let Some(last) = self.points.last_mut() {
            *last = pos;
        }
    }

    /// Check if polyline has enough points to be valid
    pub fn is_valid(&self) -> bool {
        self.points.len() >= 2
    }

    /// Close the polyline (connect last point to first)
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Get the total length of the polyline
    pub fn length(&self) -> f32 {
        self.points.windows(2).map(|w| w[0].distance(w[1])).sum::<f32>()
            + if self.closed && self.points.len() >= 2 {
                self.points.last().unwrap().distance(self.points[0])
            } else {
                0.0
            }
    }
}

/// Highlighter annotation (semi-transparent rectangle, 荧光笔)
///
/// Used to highlight areas of interest with a semi-transparent overlay.
/// Typically yellow or other bright colors at ~40% opacity.
#[derive(Debug, Clone)]
pub struct Highlighter {
    /// Bounding rectangle
    pub rect: egui::Rect,
    /// Highlight color (should be semi-transparent)
    pub color: egui::Color32,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self {
            rect: egui::Rect::NOTHING,
            color: egui::Color32::from_rgba_unmultiplied(255, 255, 0, 100), // Yellow at 40% opacity
        }
    }
}

impl Highlighter {
    /// Create a new highlighter with the given rect and color
    pub fn new(rect: egui::Rect, color: egui::Color32) -> Self {
        Self { rect, color }
    }

    /// Create from two corner points
    pub fn from_corners(start: egui::Pos2, end: egui::Pos2, color: egui::Color32) -> Self {
        Self { rect: egui::Rect::from_two_pos(start, end), color }
    }

    /// Create with default yellow highlight color
    pub fn yellow(rect: egui::Rect) -> Self {
        Self { rect, color: egui::Color32::from_rgba_unmultiplied(255, 255, 0, 100) }
    }

    /// Create with custom opacity (0-255)
    pub fn with_opacity(rect: egui::Rect, base_color: egui::Color32, opacity: u8) -> Self {
        Self {
            rect,
            color: egui::Color32::from_rgba_unmultiplied(
                base_color.r(),
                base_color.g(),
                base_color.b(),
                opacity,
            ),
        }
    }

    /// Check if highlighter has valid size
    pub fn is_valid(&self) -> bool {
        self.rect.width() > 2.0 && self.rect.height() > 2.0
    }
}

/// Get preset highlight colors (semi-transparent)
pub fn highlight_colors() -> [egui::Color32; 5] {
    [
        egui::Color32::from_rgba_unmultiplied(255, 255, 0, 100), // Yellow
        egui::Color32::from_rgba_unmultiplied(0, 255, 0, 100),   // Green
        egui::Color32::from_rgba_unmultiplied(255, 0, 255, 100), // Magenta
        egui::Color32::from_rgba_unmultiplied(0, 255, 255, 100), // Cyan
        egui::Color32::from_rgba_unmultiplied(255, 165, 0, 100), // Orange
    ]
}

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
    /// Completed freehand strokes
    pub strokes: Vec<Stroke>,
    /// Current stroke being drawn (if any)
    pub current_stroke: Option<Stroke>,
    /// Arrow annotations
    pub arrows: Vec<Arrow>,
    /// Current arrow being drawn (if any)
    pub current_arrow: Option<Arrow>,
    /// Shape annotations (rectangles, ellipses)
    pub shapes: Vec<Shape>,
    /// Current shape being drawn (if any)
    pub current_shape: Option<Shape>,
    /// Polyline annotations (connected line segments)
    pub polylines: Vec<Polyline>,
    /// Current polyline being drawn (if any)
    pub current_polyline: Option<Polyline>,
    /// Highlighter annotations (semi-transparent rectangles)
    pub highlighters: Vec<Highlighter>,
    /// Current highlighter being drawn (if any)
    pub current_highlighter: Option<Highlighter>,
    /// Sequence markers
    pub markers: Vec<SequenceMarker>,
    /// Next sequence number to use
    pub next_sequence: u32,
    /// Current sequence style
    pub sequence_style: SequenceStyle,
}

impl Annotations {
    pub fn new() -> Self {
        Self { next_sequence: 1, ..Self::default() }
    }

    // ==================== Freehand Strokes ====================

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

    /// Check if currently drawing a stroke
    pub fn is_drawing(&self) -> bool {
        self.current_stroke.is_some()
    }

    // ==================== Arrows ====================

    /// Start a new arrow at the given position
    pub fn start_arrow(&mut self, pos: egui::Pos2, settings: &StrokeSettings) {
        self.current_arrow = Some(Arrow::new(pos, pos, settings));
    }

    /// Update the end point of the current arrow
    pub fn update_arrow(&mut self, pos: egui::Pos2, snap: bool) {
        if let Some(ref mut arrow) = self.current_arrow {
            arrow.end = pos;
            if snap {
                arrow.snap_angle();
            }
        }
    }

    /// Finish current arrow
    pub fn finish_arrow(&mut self) {
        if let Some(arrow) = self.current_arrow.take() {
            // Only add if arrow has some length
            if arrow.length() > 5.0 {
                self.arrows.push(arrow);
            }
        }
    }

    /// Check if currently drawing an arrow
    pub fn is_drawing_arrow(&self) -> bool {
        self.current_arrow.is_some()
    }

    // ==================== Shapes ====================

    /// Start a new shape at the given position
    pub fn start_shape(
        &mut self,
        pos: egui::Pos2,
        shape_type: ShapeType,
        fill_mode: FillMode,
        settings: &StrokeSettings,
    ) {
        let rect = egui::Rect::from_two_pos(pos, pos);
        self.current_shape = Some(Shape::new(rect, shape_type, fill_mode, settings));
    }

    /// Update the shape's end corner
    pub fn update_shape(&mut self, pos: egui::Pos2) {
        if let Some(ref mut shape) = self.current_shape {
            // Keep the original start corner, update end corner
            let start = shape.rect.min;
            shape.rect = egui::Rect::from_two_pos(start, pos);
        }
    }

    /// Finish current shape
    pub fn finish_shape(&mut self) {
        if let Some(shape) = self.current_shape.take() {
            // Only add if shape has some size
            if shape.rect.width() > 5.0 && shape.rect.height() > 5.0 {
                self.shapes.push(shape);
            }
        }
    }

    /// Check if currently drawing a shape
    pub fn is_drawing_shape(&self) -> bool {
        self.current_shape.is_some()
    }

    // ==================== Sequence Markers ====================

    /// Add a sequence marker at the given position
    pub fn add_marker(&mut self, pos: egui::Pos2, color: egui::Color32) {
        let marker = SequenceMarker::new(pos, self.next_sequence, self.sequence_style, color);
        self.markers.push(marker);
        self.next_sequence += 1;
    }

    /// Set the sequence style for future markers
    pub fn set_sequence_style(&mut self, style: SequenceStyle) {
        self.sequence_style = style;
    }

    /// Increment the next sequence number
    pub fn increment_sequence(&mut self) {
        self.next_sequence = self.next_sequence.saturating_add(1);
    }

    /// Decrement the next sequence number (minimum 1)
    pub fn decrement_sequence(&mut self) {
        self.next_sequence = self.next_sequence.saturating_sub(1).max(1);
    }

    /// Reset sequence counter to 1
    pub fn reset_sequence(&mut self) {
        self.next_sequence = 1;
    }

    // ==================== Polylines ====================

    /// Start a new polyline
    pub fn start_polyline(&mut self, pos: egui::Pos2, settings: &StrokeSettings) {
        let mut polyline = Polyline::new(settings);
        polyline.add_point(pos);
        polyline.add_point(pos); // Second point for preview
        self.current_polyline = Some(polyline);
    }

    /// Add a vertex to the current polyline
    pub fn add_polyline_point(&mut self, pos: egui::Pos2) {
        if let Some(ref mut polyline) = self.current_polyline {
            polyline.add_point(pos);
        }
    }

    /// Update the preview point of the current polyline
    pub fn update_polyline_preview(&mut self, pos: egui::Pos2) {
        if let Some(ref mut polyline) = self.current_polyline {
            polyline.update_last(pos);
        }
    }

    /// Finish current polyline (double-click or press Enter)
    pub fn finish_polyline(&mut self) {
        if let Some(mut polyline) = self.current_polyline.take() {
            // Remove the preview point
            polyline.points.pop();
            if polyline.is_valid() {
                self.polylines.push(polyline);
            }
        }
    }

    /// Close and finish current polyline (Shift+double-click)
    pub fn close_polyline(&mut self) {
        if let Some(mut polyline) = self.current_polyline.take() {
            // Remove the preview point
            polyline.points.pop();
            if polyline.points.len() >= 3 {
                polyline.close();
                self.polylines.push(polyline);
            }
        }
    }

    /// Check if currently drawing a polyline
    pub fn is_drawing_polyline(&self) -> bool {
        self.current_polyline.is_some()
    }

    // ==================== Highlighters ====================

    /// Start a new highlighter at the given position
    pub fn start_highlighter(&mut self, pos: egui::Pos2, color: egui::Color32) {
        self.current_highlighter = Some(Highlighter::from_corners(pos, pos, color));
    }

    /// Update the highlighter's end corner
    pub fn update_highlighter(&mut self, pos: egui::Pos2) {
        if let Some(ref mut highlighter) = self.current_highlighter {
            let start = highlighter.rect.min;
            highlighter.rect = egui::Rect::from_two_pos(start, pos);
        }
    }

    /// Finish current highlighter
    pub fn finish_highlighter(&mut self) {
        if let Some(highlighter) = self.current_highlighter.take() {
            if highlighter.is_valid() {
                self.highlighters.push(highlighter);
            }
        }
    }

    /// Check if currently drawing a highlighter
    pub fn is_drawing_highlighter(&self) -> bool {
        self.current_highlighter.is_some()
    }

    // ==================== General ====================

    /// Check if any drawing is in progress
    pub fn is_any_drawing(&self) -> bool {
        self.current_stroke.is_some()
            || self.current_arrow.is_some()
            || self.current_shape.is_some()
            || self.current_polyline.is_some()
            || self.current_highlighter.is_some()
    }

    /// Clear all annotations
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.strokes.clear();
        self.current_stroke = None;
        self.arrows.clear();
        self.current_arrow = None;
        self.shapes.clear();
        self.current_shape = None;
        self.polylines.clear();
        self.current_polyline = None;
        self.highlighters.clear();
        self.current_highlighter = None;
        self.markers.clear();
        self.next_sequence = 1;
    }

    /// Check if there are any annotations
    pub fn is_empty(&self) -> bool {
        self.strokes.is_empty()
            && self.arrows.is_empty()
            && self.shapes.is_empty()
            && self.polylines.is_empty()
            && self.highlighters.is_empty()
            && self.markers.is_empty()
    }

    /// Create a snapshot of the current annotations state
    ///
    /// Used for undo/redo functionality.
    pub fn snapshot(&self) -> AnnotationsSnapshot {
        AnnotationsSnapshot {
            strokes: self.strokes.clone(),
            arrows: self.arrows.clone(),
            shapes: self.shapes.clone(),
            polylines: self.polylines.clone(),
            highlighters: self.highlighters.clone(),
            markers: self.markers.clone(),
            next_sequence: self.next_sequence,
            sequence_style: self.sequence_style,
        }
    }

    /// Restore annotations state from a snapshot
    ///
    /// Used for undo/redo functionality.
    pub fn restore(&mut self, snapshot: AnnotationsSnapshot) {
        self.strokes = snapshot.strokes;
        self.arrows = snapshot.arrows;
        self.shapes = snapshot.shapes;
        self.polylines = snapshot.polylines;
        self.highlighters = snapshot.highlighters;
        self.markers = snapshot.markers;
        self.next_sequence = snapshot.next_sequence;
        self.sequence_style = snapshot.sequence_style;
        // Clear any in-progress drawing state
        self.current_stroke = None;
        self.current_arrow = None;
        self.current_shape = None;
        self.current_polyline = None;
        self.current_highlighter = None;
    }
}

/// Snapshot of annotations state for undo/redo
///
/// Contains all completed annotations but not in-progress drawings.
#[derive(Debug, Clone, Default)]
pub struct AnnotationsSnapshot {
    /// Completed freehand strokes
    pub strokes: Vec<Stroke>,
    /// Arrow annotations
    pub arrows: Vec<Arrow>,
    /// Shape annotations (rectangles, ellipses)
    pub shapes: Vec<Shape>,
    /// Polyline annotations
    pub polylines: Vec<Polyline>,
    /// Highlighter annotations
    pub highlighters: Vec<Highlighter>,
    /// Sequence markers
    pub markers: Vec<SequenceMarker>,
    /// Next sequence number
    pub next_sequence: u32,
    /// Sequence style
    pub sequence_style: SequenceStyle,
}
