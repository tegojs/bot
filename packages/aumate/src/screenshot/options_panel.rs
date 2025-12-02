//! Tool options panels for screenshot tools
//!
//! Each drawing tool can have an options panel that appears below the main toolbar,
//! allowing users to customize stroke width, color, fill mode, etc.

use egui::{Color32, Pos2, Rect, Ui, Vec2};

// ============================================================================
// Style Constants
// ============================================================================

const PANEL_HEIGHT: f32 = 32.0;
const PANEL_PADDING: f32 = 6.0;
const PANEL_CORNER_RADIUS: f32 = 6.0;
const PANEL_GAP: f32 = 4.0;

const PANEL_BG_COLOR: Color32 = Color32::from_rgba_premultiplied(35, 35, 35, 245);
const OPTION_BUTTON_SIZE: f32 = 24.0;
const OPTION_SPACING: f32 = 4.0;

/// Predefined color palette
pub const COLOR_PALETTE: &[Color32] = &[
    Color32::from_rgb(255, 0, 0),     // Red
    Color32::from_rgb(255, 165, 0),   // Orange
    Color32::from_rgb(255, 255, 0),   // Yellow
    Color32::from_rgb(0, 255, 0),     // Green
    Color32::from_rgb(0, 255, 255),   // Cyan
    Color32::from_rgb(0, 0, 255),     // Blue
    Color32::from_rgb(128, 0, 128),   // Purple
    Color32::from_rgb(255, 192, 203), // Pink
    Color32::WHITE,
    Color32::BLACK,
];

/// Predefined stroke widths
pub const STROKE_WIDTHS: &[f32] = &[1.0, 2.0, 3.0, 4.0, 6.0, 8.0];

// ============================================================================
// Common Tool Options
// ============================================================================

/// Fill mode for shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillMode {
    /// Stroke only (outline)
    Stroke,
    /// Fill only (solid)
    Fill,
    /// Both stroke and fill
    Both,
}

impl Default for FillMode {
    fn default() -> Self {
        Self::Stroke
    }
}

/// Line style for strokes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
}

impl Default for LineStyle {
    fn default() -> Self {
        Self::Solid
    }
}

/// Arrow style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowStyle {
    /// Arrow at end only
    Single,
    /// Arrows at both ends
    Double,
    /// No arrow (just a line)
    None,
}

impl Default for ArrowStyle {
    fn default() -> Self {
        Self::Single
    }
}

/// Common options shared by many tools
#[derive(Debug, Clone)]
pub struct CommonOptions {
    /// Stroke color
    pub color: Color32,
    /// Stroke width
    pub stroke_width: f32,
    /// Fill mode
    pub fill_mode: FillMode,
    /// Line style
    pub line_style: LineStyle,
}

impl Default for CommonOptions {
    fn default() -> Self {
        Self {
            color: Color32::RED,
            stroke_width: 2.0,
            fill_mode: FillMode::Stroke,
            line_style: LineStyle::Solid,
        }
    }
}

// ============================================================================
// Tool-Specific Options
// ============================================================================

/// Options for rectangle tool
#[derive(Debug, Clone, Default)]
pub struct RectangleOptions {
    pub common: CommonOptions,
}

/// Options for ellipse tool
#[derive(Debug, Clone, Default)]
pub struct EllipseOptions {
    pub common: CommonOptions,
}

/// Options for arrow tool
#[derive(Debug, Clone)]
pub struct ArrowOptions {
    pub common: CommonOptions,
    pub arrow_style: ArrowStyle,
}

impl Default for ArrowOptions {
    fn default() -> Self {
        Self {
            common: CommonOptions::default(),
            arrow_style: ArrowStyle::Single,
        }
    }
}

/// Options for annotate (freehand) tool
#[derive(Debug, Clone, Default)]
pub struct AnnotateOptions {
    pub common: CommonOptions,
}

/// Options for highlighter tool
#[derive(Debug, Clone)]
pub struct HighlighterOptions {
    pub color: Color32,
    pub width: f32,
    pub opacity: f32,
}

impl Default for HighlighterOptions {
    fn default() -> Self {
        Self {
            color: Color32::YELLOW,
            width: 20.0,
            opacity: 0.5,
        }
    }
}

/// Options for mosaic tool
#[derive(Debug, Clone)]
pub struct MosaicOptions {
    /// Block size for mosaic effect
    pub block_size: u32,
    /// Use blur instead of mosaic
    pub use_blur: bool,
    /// Blur strength (if use_blur is true)
    pub blur_strength: f32,
}

impl Default for MosaicOptions {
    fn default() -> Self {
        Self {
            block_size: 10,
            use_blur: false,
            blur_strength: 5.0,
        }
    }
}

/// Options for text tool
#[derive(Debug, Clone)]
pub struct TextOptions {
    pub color: Color32,
    pub font_size: f32,
    pub bold: bool,
    pub italic: bool,
    pub font_family: String,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            color: Color32::RED,
            font_size: 16.0,
            bold: false,
            italic: false,
            font_family: "sans-serif".to_string(),
        }
    }
}

/// Options for sequence (numbered markers) tool
#[derive(Debug, Clone)]
pub struct SequenceOptions {
    pub color: Color32,
    pub size: f32,
    pub start_number: u32,
}

impl Default for SequenceOptions {
    fn default() -> Self {
        Self {
            color: Color32::RED,
            size: 24.0,
            start_number: 1,
        }
    }
}

// ============================================================================
// Options Panel
// ============================================================================

/// The options panel that appears below the main toolbar
pub struct OptionsPanel {
    /// Position (computed from action bar position)
    position: Pos2,
    /// Size
    size: Vec2,
    /// Current tool's options type
    tool_id: String,
    /// Common options (shared state)
    pub common: CommonOptions,
    /// Arrow-specific options
    pub arrow: ArrowOptions,
    /// Highlighter-specific options
    pub highlighter: HighlighterOptions,
    /// Mosaic-specific options
    pub mosaic: MosaicOptions,
    /// Text-specific options
    pub text: TextOptions,
    /// Sequence-specific options
    pub sequence: SequenceOptions,
}

impl OptionsPanel {
    /// Create a new options panel positioned below the action bar
    pub fn new(action_bar_pos: Pos2, action_bar_size: Vec2) -> Self {
        let position = Pos2::new(action_bar_pos.x, action_bar_pos.y + action_bar_size.y + PANEL_GAP);
        let size = Vec2::new(action_bar_size.x, PANEL_HEIGHT + 2.0 * PANEL_PADDING);

        Self {
            position,
            size,
            tool_id: String::new(),
            common: CommonOptions::default(),
            arrow: ArrowOptions::default(),
            highlighter: HighlighterOptions::default(),
            mosaic: MosaicOptions::default(),
            text: TextOptions::default(),
            sequence: SequenceOptions::default(),
        }
    }

    /// Update position based on action bar
    pub fn update_position(&mut self, action_bar_pos: Pos2, action_bar_size: Vec2) {
        self.position = Pos2::new(action_bar_pos.x, action_bar_pos.y + action_bar_size.y + PANEL_GAP);
        self.size.x = action_bar_size.x;
    }

    /// Set the current tool
    pub fn set_tool(&mut self, tool_id: &str) {
        self.tool_id = tool_id.to_string();
    }

    /// Get bounds
    pub fn bounds(&self) -> Rect {
        Rect::from_min_size(self.position, self.size)
    }

    /// Check if point is inside panel
    pub fn contains(&self, pos: Pos2) -> bool {
        self.bounds().contains(pos)
    }

    /// Render the options panel for the current tool
    pub fn render(&mut self, ui: &mut Ui) {
        let rect = self.bounds();

        // Draw background
        ui.painter().rect_filled(rect, PANEL_CORNER_RADIUS, PANEL_BG_COLOR);

        // Render tool-specific options
        let content_rect = rect.shrink(PANEL_PADDING);
        match self.tool_id.as_str() {
            "rectangle" | "ellipse" | "polyline" => {
                self.render_shape_options(ui, content_rect);
            }
            "arrow" => {
                self.render_arrow_options(ui, content_rect);
            }
            "annotate" => {
                self.render_annotate_options(ui, content_rect);
            }
            "highlighter" => {
                self.render_highlighter_options(ui, content_rect);
            }
            "mosaic" | "blur" => {
                self.render_mosaic_options(ui, content_rect);
            }
            "text" => {
                self.render_text_options(ui, content_rect);
            }
            "sequence" => {
                self.render_sequence_options(ui, content_rect);
            }
            _ => {
                // No options for this tool
            }
        }
    }

    /// Render shape options (line style, width, fill, color)
    fn render_shape_options(&mut self, ui: &mut Ui, rect: Rect) {
        let mut x = rect.min.x;
        let y = rect.center().y;

        // Line style buttons
        x = self.render_line_style_selector(ui, x, y);
        x += OPTION_SPACING * 2.0;

        // Stroke width selector
        x = self.render_stroke_width_selector(ui, x, y);
        x += OPTION_SPACING * 2.0;

        // Fill mode selector
        x = self.render_fill_mode_selector(ui, x, y);
        x += OPTION_SPACING * 2.0;

        // Color picker
        self.render_color_palette(ui, x, y);
    }

    /// Render arrow-specific options
    fn render_arrow_options(&mut self, ui: &mut Ui, rect: Rect) {
        let mut x = rect.min.x;
        let y = rect.center().y;

        // Arrow style selector
        x = self.render_arrow_style_selector(ui, x, y);
        x += OPTION_SPACING * 2.0;

        // Stroke width
        x = self.render_stroke_width_selector(ui, x, y);
        x += OPTION_SPACING * 2.0;

        // Color
        self.render_color_palette(ui, x, y);
    }

    /// Render annotate (freehand) options
    fn render_annotate_options(&mut self, ui: &mut Ui, rect: Rect) {
        let mut x = rect.min.x;
        let y = rect.center().y;

        // Stroke width
        x = self.render_stroke_width_selector(ui, x, y);
        x += OPTION_SPACING * 2.0;

        // Color
        self.render_color_palette(ui, x, y);
    }

    /// Render highlighter options
    fn render_highlighter_options(&mut self, ui: &mut Ui, rect: Rect) {
        let mut x = rect.min.x;
        let y = rect.center().y;

        // Width selector
        for &width in [10.0, 20.0, 30.0, 40.0].iter() {
            let btn_rect = Rect::from_center_size(
                Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            let is_selected = (self.highlighter.width - width).abs() < 0.1;
            self.render_width_button(ui, btn_rect, width, is_selected);
            if is_selected {
                ui.painter().rect_stroke(
                    btn_rect,
                    4.0,
                    egui::Stroke::new(1.0, Color32::WHITE),
                    egui::StrokeKind::Outside,
                );
            }
            x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }

        x += OPTION_SPACING * 2.0;

        // Color palette (with transparency)
        self.render_color_palette(ui, x, y);
    }

    /// Render mosaic options
    fn render_mosaic_options(&mut self, ui: &mut Ui, rect: Rect) {
        let mut x = rect.min.x;
        let y = rect.center().y;

        // Mosaic/Blur toggle
        let mosaic_rect = Rect::from_center_size(
            Pos2::new(x + 30.0, y),
            Vec2::new(60.0, OPTION_BUTTON_SIZE),
        );
        let blur_rect = Rect::from_center_size(
            Pos2::new(x + 95.0, y),
            Vec2::new(60.0, OPTION_BUTTON_SIZE),
        );

        // Mosaic button
        ui.painter().rect_filled(
            mosaic_rect,
            4.0,
            if !self.mosaic.use_blur { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
        );
        ui.painter().text(
            mosaic_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Mosaic",
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );

        // Blur button
        ui.painter().rect_filled(
            blur_rect,
            4.0,
            if self.mosaic.use_blur { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
        );
        ui.painter().text(
            blur_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Blur",
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );

        x += 160.0;

        // Intensity selector
        for &size in [5, 10, 15, 20].iter() {
            let btn_rect = Rect::from_center_size(
                Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            let is_selected = self.mosaic.block_size == size;
            let bg_color = if is_selected {
                Color32::from_rgb(0, 120, 215)
            } else {
                Color32::from_gray(60)
            };
            ui.painter().rect_filled(btn_rect, 4.0, bg_color);
            ui.painter().text(
                btn_rect.center(),
                egui::Align2::CENTER_CENTER,
                &size.to_string(),
                egui::FontId::proportional(11.0),
                Color32::WHITE,
            );
            x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }
    }

    /// Render text options
    fn render_text_options(&mut self, ui: &mut Ui, rect: Rect) {
        let mut x = rect.min.x;
        let y = rect.center().y;

        // Bold button
        let bold_rect = Rect::from_center_size(
            Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
            Vec2::splat(OPTION_BUTTON_SIZE),
        );
        ui.painter().rect_filled(
            bold_rect,
            4.0,
            if self.text.bold { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
        );
        ui.painter().text(
            bold_rect.center(),
            egui::Align2::CENTER_CENTER,
            "B",
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
            Color32::WHITE,
        );
        x += OPTION_BUTTON_SIZE + OPTION_SPACING;

        // Italic button
        let italic_rect = Rect::from_center_size(
            Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
            Vec2::splat(OPTION_BUTTON_SIZE),
        );
        ui.painter().rect_filled(
            italic_rect,
            4.0,
            if self.text.italic { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
        );
        ui.painter().text(
            italic_rect.center(),
            egui::Align2::CENTER_CENTER,
            "I",
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
            Color32::WHITE,
        );
        x += OPTION_BUTTON_SIZE + OPTION_SPACING * 3.0;

        // Font size selector
        for &size in &[12.0, 16.0, 20.0, 24.0, 32.0] {
            let btn_rect = Rect::from_center_size(
                Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            let is_selected = (self.text.font_size - size).abs() < 0.1;
            ui.painter().rect_filled(
                btn_rect,
                4.0,
                if is_selected { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
            );
            ui.painter().text(
                btn_rect.center(),
                egui::Align2::CENTER_CENTER,
                &format!("{}", size as i32),
                egui::FontId::proportional(10.0),
                Color32::WHITE,
            );
            x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }

        x += OPTION_SPACING * 2.0;

        // Color palette
        self.render_color_palette(ui, x, y);
    }

    /// Render sequence (numbered marker) options
    fn render_sequence_options(&mut self, ui: &mut Ui, rect: Rect) {
        let mut x = rect.min.x;
        let y = rect.center().y;

        // Size selector
        for &size in &[20.0, 24.0, 28.0, 32.0] {
            let btn_rect = Rect::from_center_size(
                Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            let is_selected = (self.sequence.size - size).abs() < 0.1;
            ui.painter().rect_filled(
                btn_rect,
                4.0,
                if is_selected { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
            );
            // Draw a circle to represent the marker
            ui.painter().circle_stroke(
                btn_rect.center(),
                size / 3.0,
                egui::Stroke::new(1.5, Color32::WHITE),
            );
            x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }

        x += OPTION_SPACING * 2.0;

        // Color palette
        self.render_color_palette(ui, x, y);
    }

    // ========================================================================
    // Helper renderers
    // ========================================================================

    fn render_line_style_selector(&mut self, ui: &mut Ui, start_x: f32, y: f32) -> f32 {
        let styles = [LineStyle::Solid, LineStyle::Dashed, LineStyle::Dotted];
        let mut x = start_x;

        for style in styles {
            let btn_rect = Rect::from_center_size(
                Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            let is_selected = self.common.line_style == style;
            ui.painter().rect_filled(
                btn_rect,
                4.0,
                if is_selected { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
            );

            // Draw line style preview
            let line_y = btn_rect.center().y;
            let line_start = Pos2::new(btn_rect.min.x + 4.0, line_y);
            let line_end = Pos2::new(btn_rect.max.x - 4.0, line_y);

            match style {
                LineStyle::Solid => {
                    ui.painter().line_segment(
                        [line_start, line_end],
                        egui::Stroke::new(2.0, Color32::WHITE),
                    );
                }
                LineStyle::Dashed => {
                    // Draw dashed line
                    let dash_len = 4.0;
                    let gap_len = 2.0;
                    let mut px = line_start.x;
                    while px < line_end.x {
                        let end_px = (px + dash_len).min(line_end.x);
                        ui.painter().line_segment(
                            [Pos2::new(px, line_y), Pos2::new(end_px, line_y)],
                            egui::Stroke::new(2.0, Color32::WHITE),
                        );
                        px += dash_len + gap_len;
                    }
                }
                LineStyle::Dotted => {
                    // Draw dotted line
                    let dot_spacing = 4.0;
                    let mut px = line_start.x;
                    while px < line_end.x {
                        ui.painter().circle_filled(Pos2::new(px, line_y), 1.5, Color32::WHITE);
                        px += dot_spacing;
                    }
                }
            }

            x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }

        x
    }

    fn render_stroke_width_selector(&mut self, ui: &mut Ui, start_x: f32, y: f32) -> f32 {
        let mut x = start_x;

        for &width in STROKE_WIDTHS {
            let btn_rect = Rect::from_center_size(
                Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            let is_selected = (self.common.stroke_width - width).abs() < 0.1;
            self.render_width_button(ui, btn_rect, width, is_selected);
            x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }

        x
    }

    fn render_width_button(&self, ui: &mut Ui, rect: Rect, width: f32, is_selected: bool) {
        ui.painter().rect_filled(
            rect,
            4.0,
            if is_selected { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
        );
        // Draw a line representing the width
        let line_y = rect.center().y;
        ui.painter().line_segment(
            [
                Pos2::new(rect.min.x + 4.0, line_y),
                Pos2::new(rect.max.x - 4.0, line_y),
            ],
            egui::Stroke::new(width.min(6.0), Color32::WHITE),
        );
    }

    fn render_fill_mode_selector(&mut self, ui: &mut Ui, start_x: f32, y: f32) -> f32 {
        let modes = [FillMode::Stroke, FillMode::Fill, FillMode::Both];
        let mut x = start_x;

        for mode in modes {
            let btn_rect = Rect::from_center_size(
                Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            let is_selected = self.common.fill_mode == mode;
            ui.painter().rect_filled(
                btn_rect,
                4.0,
                if is_selected { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
            );

            // Draw fill mode preview (small square)
            let preview_size = 10.0;
            let preview_rect = Rect::from_center_size(btn_rect.center(), Vec2::splat(preview_size));

            match mode {
                FillMode::Stroke => {
                    ui.painter().rect_stroke(
                        preview_rect,
                        0.0,
                        egui::Stroke::new(1.5, Color32::WHITE),
                        egui::StrokeKind::Inside,
                    );
                }
                FillMode::Fill => {
                    ui.painter().rect_filled(preview_rect, 0.0, Color32::WHITE);
                }
                FillMode::Both => {
                    ui.painter().rect_filled(preview_rect, 0.0, Color32::from_gray(150));
                    ui.painter().rect_stroke(
                        preview_rect,
                        0.0,
                        egui::Stroke::new(1.5, Color32::WHITE),
                        egui::StrokeKind::Inside,
                    );
                }
            }

            x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }

        x
    }

    fn render_arrow_style_selector(&mut self, ui: &mut Ui, start_x: f32, y: f32) -> f32 {
        let styles = [ArrowStyle::Single, ArrowStyle::Double, ArrowStyle::None];
        let mut x = start_x;

        for style in styles {
            let btn_rect = Rect::from_center_size(
                Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            let is_selected = self.arrow.arrow_style == style;
            ui.painter().rect_filled(
                btn_rect,
                4.0,
                if is_selected { Color32::from_rgb(0, 120, 215) } else { Color32::from_gray(60) },
            );

            // Draw arrow preview
            let center = btn_rect.center();
            let left = Pos2::new(center.x - 6.0, center.y);
            let right = Pos2::new(center.x + 6.0, center.y);

            ui.painter().line_segment(
                [left, right],
                egui::Stroke::new(1.5, Color32::WHITE),
            );

            match style {
                ArrowStyle::Single => {
                    // Arrow at right
                    ui.painter().line_segment(
                        [Pos2::new(right.x - 4.0, center.y - 3.0), right],
                        egui::Stroke::new(1.5, Color32::WHITE),
                    );
                    ui.painter().line_segment(
                        [Pos2::new(right.x - 4.0, center.y + 3.0), right],
                        egui::Stroke::new(1.5, Color32::WHITE),
                    );
                }
                ArrowStyle::Double => {
                    // Arrows at both ends
                    ui.painter().line_segment(
                        [Pos2::new(right.x - 4.0, center.y - 3.0), right],
                        egui::Stroke::new(1.5, Color32::WHITE),
                    );
                    ui.painter().line_segment(
                        [Pos2::new(right.x - 4.0, center.y + 3.0), right],
                        egui::Stroke::new(1.5, Color32::WHITE),
                    );
                    ui.painter().line_segment(
                        [Pos2::new(left.x + 4.0, center.y - 3.0), left],
                        egui::Stroke::new(1.5, Color32::WHITE),
                    );
                    ui.painter().line_segment(
                        [Pos2::new(left.x + 4.0, center.y + 3.0), left],
                        egui::Stroke::new(1.5, Color32::WHITE),
                    );
                }
                ArrowStyle::None => {
                    // Just the line, no arrows
                }
            }

            x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }

        x
    }

    fn render_color_palette(&mut self, ui: &mut Ui, start_x: f32, y: f32) {
        let mut x = start_x;
        let color_size = 16.0;

        for &color in COLOR_PALETTE {
            let color_rect = Rect::from_center_size(
                Pos2::new(x + color_size / 2.0, y),
                Vec2::splat(color_size),
            );

            ui.painter().rect_filled(color_rect, 2.0, color);

            // Draw selection indicator
            if self.common.color == color {
                ui.painter().rect_stroke(
                    color_rect.expand(2.0),
                    2.0,
                    egui::Stroke::new(2.0, Color32::WHITE),
                    egui::StrokeKind::Outside,
                );
            }

            x += color_size + 2.0;
        }
    }

    /// Handle click on options panel, returns true if something was clicked
    pub fn handle_click(&mut self, pos: Pos2) -> bool {
        if !self.contains(pos) {
            return false;
        }

        let rect = self.bounds().shrink(PANEL_PADDING);
        let y = rect.center().y;
        let mut x = rect.min.x;

        // Check which element was clicked based on current tool
        match self.tool_id.as_str() {
            "rectangle" | "ellipse" | "polyline" => {
                // Line style
                if let Some(style) = self.check_line_style_click(pos, &mut x, y) {
                    self.common.line_style = style;
                    return true;
                }
                x += OPTION_SPACING * 2.0;
                // Stroke width
                if let Some(width) = self.check_stroke_width_click(pos, &mut x, y) {
                    self.common.stroke_width = width;
                    return true;
                }
                x += OPTION_SPACING * 2.0;
                // Fill mode
                if let Some(mode) = self.check_fill_mode_click(pos, &mut x, y) {
                    self.common.fill_mode = mode;
                    return true;
                }
                x += OPTION_SPACING * 2.0;
                // Color
                if let Some(color) = self.check_color_click(pos, x, y) {
                    self.common.color = color;
                    return true;
                }
            }
            "arrow" => {
                // Arrow style
                if let Some(style) = self.check_arrow_style_click(pos, &mut x, y) {
                    self.arrow.arrow_style = style;
                    return true;
                }
                x += OPTION_SPACING * 2.0;
                // Stroke width
                if let Some(width) = self.check_stroke_width_click(pos, &mut x, y) {
                    self.common.stroke_width = width;
                    return true;
                }
                x += OPTION_SPACING * 2.0;
                // Color
                if let Some(color) = self.check_color_click(pos, x, y) {
                    self.common.color = color;
                    return true;
                }
            }
            "annotate" => {
                // Stroke width
                if let Some(width) = self.check_stroke_width_click(pos, &mut x, y) {
                    self.common.stroke_width = width;
                    return true;
                }
                x += OPTION_SPACING * 2.0;
                // Color
                if let Some(color) = self.check_color_click(pos, x, y) {
                    self.common.color = color;
                    return true;
                }
            }
            "mosaic" | "blur" => {
                // Mosaic/blur toggle
                let mosaic_rect = Rect::from_center_size(
                    Pos2::new(x + 30.0, y),
                    Vec2::new(60.0, OPTION_BUTTON_SIZE),
                );
                let blur_rect = Rect::from_center_size(
                    Pos2::new(x + 95.0, y),
                    Vec2::new(60.0, OPTION_BUTTON_SIZE),
                );
                if mosaic_rect.contains(pos) {
                    self.mosaic.use_blur = false;
                    return true;
                }
                if blur_rect.contains(pos) {
                    self.mosaic.use_blur = true;
                    return true;
                }
                x += 160.0;
                // Block size
                for &size in [5u32, 10, 15, 20].iter() {
                    let btn_rect = Rect::from_center_size(
                        Pos2::new(x + OPTION_BUTTON_SIZE / 2.0, y),
                        Vec2::splat(OPTION_BUTTON_SIZE),
                    );
                    if btn_rect.contains(pos) {
                        self.mosaic.block_size = size;
                        return true;
                    }
                    x += OPTION_BUTTON_SIZE + OPTION_SPACING;
                }
            }
            _ => {}
        }

        false
    }

    fn check_line_style_click(&self, pos: Pos2, x: &mut f32, y: f32) -> Option<LineStyle> {
        for style in [LineStyle::Solid, LineStyle::Dashed, LineStyle::Dotted] {
            let btn_rect = Rect::from_center_size(
                Pos2::new(*x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            if btn_rect.contains(pos) {
                return Some(style);
            }
            *x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }
        None
    }

    fn check_stroke_width_click(&self, pos: Pos2, x: &mut f32, y: f32) -> Option<f32> {
        for &width in STROKE_WIDTHS {
            let btn_rect = Rect::from_center_size(
                Pos2::new(*x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            if btn_rect.contains(pos) {
                return Some(width);
            }
            *x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }
        None
    }

    fn check_fill_mode_click(&self, pos: Pos2, x: &mut f32, y: f32) -> Option<FillMode> {
        for mode in [FillMode::Stroke, FillMode::Fill, FillMode::Both] {
            let btn_rect = Rect::from_center_size(
                Pos2::new(*x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            if btn_rect.contains(pos) {
                return Some(mode);
            }
            *x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }
        None
    }

    fn check_arrow_style_click(&self, pos: Pos2, x: &mut f32, y: f32) -> Option<ArrowStyle> {
        for style in [ArrowStyle::Single, ArrowStyle::Double, ArrowStyle::None] {
            let btn_rect = Rect::from_center_size(
                Pos2::new(*x + OPTION_BUTTON_SIZE / 2.0, y),
                Vec2::splat(OPTION_BUTTON_SIZE),
            );
            if btn_rect.contains(pos) {
                return Some(style);
            }
            *x += OPTION_BUTTON_SIZE + OPTION_SPACING;
        }
        None
    }

    fn check_color_click(&self, pos: Pos2, start_x: f32, y: f32) -> Option<Color32> {
        let mut x = start_x;
        let color_size = 16.0;

        for &color in COLOR_PALETTE {
            let color_rect = Rect::from_center_size(
                Pos2::new(x + color_size / 2.0, y),
                Vec2::splat(color_size),
            );
            if color_rect.contains(pos) {
                return Some(color);
            }
            x += color_size + 2.0;
        }
        None
    }
}

impl Default for OptionsPanel {
    fn default() -> Self {
        Self::new(Pos2::ZERO, Vec2::ZERO)
    }
}
