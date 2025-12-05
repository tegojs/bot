//! Tool options panels for screenshot tools
//!
//! Each drawing tool can have an options panel that appears below the main toolbar,
//! allowing users to customize stroke width, color, fill mode, etc.

use egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2};

// ============================================================================
// Style Constants (matching action_bar.rs)
// ============================================================================

const BUTTON_SIZE: f32 = 28.0;
const BAR_PADDING: f32 = 6.0;
const PANEL_CORNER_RADIUS: f32 = 6.0;
const PANEL_GAP: f32 = 4.0;
const WIDGET_HEIGHT: f32 = 24.0;
const COLOR_SWATCH_SIZE: f32 = 20.0;

const PANEL_BG_COLOR: Color32 = Color32::from_rgba_premultiplied(35, 35, 35, 245);
const WIDGET_BG_COLOR: Color32 = Color32::from_rgba_premultiplied(50, 50, 50, 255);
const WIDGET_HOVER_COLOR: Color32 = Color32::from_rgba_premultiplied(70, 70, 70, 255);
const WIDGET_BORDER_COLOR: Color32 = Color32::from_rgba_premultiplied(80, 80, 80, 255);

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

// ============================================================================
// Common Tool Options
// ============================================================================

/// Fill mode for shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillMode {
    /// Stroke only (outline)
    #[default]
    Stroke,
    /// Fill only (solid)
    Fill,
    /// Both stroke and fill
    Both,
}

/// Line style for strokes (5 variants like reference image)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
    DashDot,
    DashDotDot,
}

impl LineStyle {
    /// Get all line styles
    pub fn all() -> &'static [LineStyle] {
        &[Self::Solid, Self::Dashed, Self::Dotted, Self::DashDot, Self::DashDotDot]
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Solid => "Solid",
            Self::Dashed => "Dashed",
            Self::Dotted => "Dotted",
            Self::DashDot => "Dash-Dot",
            Self::DashDotDot => "Dash-Dot-Dot",
        }
    }
}

/// Arrow style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArrowStyle {
    /// Arrow at end only
    #[default]
    Single,
    /// Arrows at both ends
    Double,
    /// No arrow (just a line)
    None,
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
        Self { common: CommonOptions::default(), arrow_style: ArrowStyle::Single }
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
        Self { color: Color32::YELLOW, width: 20.0, opacity: 0.5 }
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
        Self { block_size: 10, use_blur: false, blur_strength: 5.0 }
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
        Self { color: Color32::RED, size: 24.0, start_number: 1 }
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
    /// Whether line style dropdown is open
    line_style_open: bool,
    /// Dropdown button rect (for popup positioning)
    dropdown_button_rect: Option<Rect>,
    /// Whether color picker popup is open
    color_picker_open: bool,
}

impl OptionsPanel {
    /// Create a new options panel positioned below the action bar
    pub fn new(action_bar_pos: Pos2, action_bar_size: Vec2) -> Self {
        let position =
            Pos2::new(action_bar_pos.x, action_bar_pos.y + action_bar_size.y + PANEL_GAP);
        // Match action bar height: BUTTON_SIZE + 2 * BAR_PADDING
        let height = BUTTON_SIZE + 2.0 * BAR_PADDING;
        let size = Vec2::new(action_bar_size.x, height);

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
            line_style_open: false,
            dropdown_button_rect: None,
            color_picker_open: false,
        }
    }

    /// Update position based on action bar
    pub fn update_position(&mut self, action_bar_pos: Pos2, action_bar_size: Vec2) {
        self.position =
            Pos2::new(action_bar_pos.x, action_bar_pos.y + action_bar_size.y + PANEL_GAP);
        self.size.x = action_bar_size.x;
    }

    /// Set the current tool
    pub fn set_tool(&mut self, tool_id: &str) {
        self.tool_id = tool_id.to_string();
        // Close dropdown when switching tools
        self.line_style_open = false;
    }

    /// Get bounds
    pub fn bounds(&self) -> Rect {
        Rect::from_min_size(self.position, self.size)
    }

    /// Check if point is inside panel (including open popups)
    pub fn contains(&self, pos: Pos2) -> bool {
        // Check main panel bounds
        if self.bounds().contains(pos) {
            return true;
        }

        // Check dropdown popup bounds if open
        if self.line_style_open {
            if let Some(btn_rect) = self.dropdown_button_rect {
                let dropdown_width = 100.0;
                let popup_height = LineStyle::all().len() as f32 * 32.0 + 8.0;
                let popup_rect = Rect::from_min_size(
                    Pos2::new(btn_rect.min.x, btn_rect.max.y + 2.0),
                    Vec2::new(dropdown_width, popup_height),
                );
                if popup_rect.contains(pos) {
                    return true;
                }
            }
        }

        // Check color picker popup area (approximate - egui handles internally)
        // Color picker popup appears below/near the color button
        if self.color_picker_open {
            // Allow a generous area below and to the right of the panel for color picker
            let extended_rect = Rect::from_min_max(
                self.position,
                Pos2::new(
                    self.position.x + self.size.x + 250.0,
                    self.position.y + self.size.y + 300.0,
                ),
            );
            if extended_rect.contains(pos) {
                return true;
            }
        }

        false
    }

    /// Check if any popup is currently open
    pub fn has_open_popup(&self) -> bool {
        self.line_style_open || self.color_picker_open
    }

    /// Render the options panel for the current tool using egui widgets
    pub fn render(&mut self, ui: &mut Ui) {
        let rect = self.bounds();

        // Draw background
        ui.painter().rect_filled(rect, PANEL_CORNER_RADIUS, PANEL_BG_COLOR);

        // Create a child UI within the panel bounds for proper widget interaction
        let content_rect = rect.shrink(BAR_PADDING);
        let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));

        // Use horizontal_centered for vertical centering
        child_ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 0.0);

            match self.tool_id.as_str() {
                "rectangle" | "ellipse" | "polyline" => {
                    self.render_shape_options(ui);
                }
                "arrow" => {
                    self.render_arrow_options(ui);
                }
                "annotate" => {
                    self.render_annotate_options(ui);
                }
                "highlighter" => {
                    self.render_highlighter_options(ui);
                }
                "mosaic" | "blur" => {
                    self.render_mosaic_options(ui);
                }
                "text" => {
                    self.render_text_options(ui);
                }
                "sequence" => {
                    self.render_sequence_options(ui);
                }
                _ => {}
            }
        });
    }

    /// Render shape options
    fn render_shape_options(&mut self, ui: &mut Ui) {
        self.render_line_style_dropdown(ui);
        self.render_separator(ui);
        self.render_stroke_width_drag(ui);
        self.render_separator(ui);
        self.render_fill_mode_buttons(ui);
        self.render_separator(ui);
        self.render_color_picker(ui);
    }

    /// Render arrow options
    fn render_arrow_options(&mut self, ui: &mut Ui) {
        self.render_arrow_style_buttons(ui);
        self.render_separator(ui);
        self.render_stroke_width_drag(ui);
        self.render_separator(ui);
        self.render_color_picker(ui);
    }

    /// Render annotate options
    fn render_annotate_options(&mut self, ui: &mut Ui) {
        self.render_line_style_dropdown(ui);
        self.render_separator(ui);
        self.render_stroke_width_drag(ui);
        self.render_separator(ui);
        self.render_color_picker(ui);
    }

    /// Render highlighter options
    fn render_highlighter_options(&mut self, ui: &mut Ui) {
        // Width drag value
        ui.add(
            egui::DragValue::new(&mut self.highlighter.width)
                .range(5.0..=50.0)
                .speed(0.5)
                .suffix(" px"),
        );
        self.render_separator(ui);
        self.render_color_picker(ui);
    }

    /// Render mosaic options
    fn render_mosaic_options(&mut self, ui: &mut Ui) {
        // Mosaic/Blur toggle buttons
        let btn_size = Vec2::new(60.0, WIDGET_HEIGHT);
        if self.render_toggle_button(ui, "Mosaic", !self.mosaic.use_blur, btn_size) {
            self.mosaic.use_blur = false;
        }
        if self.render_toggle_button(ui, "Blur", self.mosaic.use_blur, btn_size) {
            self.mosaic.use_blur = true;
        }
        self.render_separator(ui);

        // Block size drag value
        let mut block_size_f = self.mosaic.block_size as f32;
        if ui
            .add(egui::DragValue::new(&mut block_size_f).range(5.0..=30.0).speed(0.5).suffix(" px"))
            .changed()
        {
            self.mosaic.block_size = block_size_f as u32;
        }
    }

    /// Render text options
    fn render_text_options(&mut self, ui: &mut Ui) {
        // Bold/Italic toggle buttons
        let btn_size = Vec2::new(WIDGET_HEIGHT, WIDGET_HEIGHT);
        if self.render_toggle_button(ui, "B", self.text.bold, btn_size) {
            self.text.bold = !self.text.bold;
        }
        if self.render_toggle_button(ui, "I", self.text.italic, btn_size) {
            self.text.italic = !self.text.italic;
        }
        self.render_separator(ui);

        // Font size drag value
        ui.add(
            egui::DragValue::new(&mut self.text.font_size)
                .range(8.0..=72.0)
                .speed(0.5)
                .suffix(" pt"),
        );
        self.render_separator(ui);
        self.render_color_picker(ui);
    }

    /// Render sequence options
    fn render_sequence_options(&mut self, ui: &mut Ui) {
        // Size drag value
        ui.add(
            egui::DragValue::new(&mut self.sequence.size)
                .range(16.0..=48.0)
                .speed(0.5)
                .suffix(" px"),
        );
        self.render_separator(ui);
        self.render_color_picker(ui);
    }

    // ========================================================================
    // Widget helpers
    // ========================================================================

    /// Render vertical separator
    fn render_separator(&self, ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(1.0, WIDGET_HEIGHT), egui::Sense::hover());
        ui.painter().line_segment(
            [rect.center_top(), rect.center_bottom()],
            Stroke::new(1.0, WIDGET_BORDER_COLOR),
        );
    }

    /// Render a toggle button, returns true if clicked
    fn render_toggle_button(&self, ui: &mut Ui, text: &str, selected: bool, size: Vec2) -> bool {
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        let bg_color = if selected {
            Color32::from_rgb(70, 130, 180) // Steel blue when selected
        } else if response.hovered() {
            WIDGET_HOVER_COLOR
        } else {
            WIDGET_BG_COLOR
        };

        ui.painter().rect_filled(rect, 4.0, bg_color);
        ui.painter().rect_stroke(
            rect,
            4.0,
            Stroke::new(1.0, WIDGET_BORDER_COLOR),
            egui::StrokeKind::Inside,
        );

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );

        response.clicked()
    }

    /// Render line style dropdown with SVG-like icons
    fn render_line_style_dropdown(&mut self, ui: &mut Ui) {
        let dropdown_width = 100.0;
        let dropdown_height = WIDGET_HEIGHT;

        // Main dropdown button
        let (rect, response) = ui
            .allocate_exact_size(Vec2::new(dropdown_width, dropdown_height), egui::Sense::click());

        // Store button rect for bounds checking
        self.dropdown_button_rect = Some(rect);

        let bg_color = if response.hovered() || self.line_style_open {
            WIDGET_HOVER_COLOR
        } else {
            WIDGET_BG_COLOR
        };

        ui.painter().rect_filled(rect, 4.0, bg_color);
        ui.painter().rect_stroke(
            rect,
            4.0,
            Stroke::new(1.0, WIDGET_BORDER_COLOR),
            egui::StrokeKind::Inside,
        );

        // Draw current line style preview
        let preview_rect = Rect::from_min_size(
            rect.min + Vec2::new(8.0, (dropdown_height - 2.0) / 2.0),
            Vec2::new(dropdown_width - 28.0, 2.0),
        );
        self.draw_line_style_preview(ui, preview_rect, self.common.line_style, Color32::WHITE);

        // Draw dropdown arrow
        let arrow_center = Pos2::new(rect.max.x - 12.0, rect.center().y);
        let arrow_points = [
            Pos2::new(arrow_center.x - 4.0, arrow_center.y - 2.0),
            Pos2::new(arrow_center.x + 4.0, arrow_center.y - 2.0),
            Pos2::new(arrow_center.x, arrow_center.y + 3.0),
        ];
        ui.painter().add(egui::Shape::convex_polygon(
            arrow_points.to_vec(),
            Color32::WHITE,
            Stroke::NONE,
        ));

        if response.clicked() {
            self.line_style_open = !self.line_style_open;
        }

        // Dropdown popup
        if self.line_style_open {
            let popup_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, rect.max.y + 2.0),
                Vec2::new(dropdown_width, LineStyle::all().len() as f32 * 32.0 + 8.0),
            );

            // Draw popup background
            ui.painter().rect_filled(popup_rect, 4.0, PANEL_BG_COLOR);
            ui.painter().rect_stroke(
                popup_rect,
                4.0,
                Stroke::new(1.0, WIDGET_BORDER_COLOR),
                egui::StrokeKind::Inside,
            );

            // Draw style options
            for (i, &style) in LineStyle::all().iter().enumerate() {
                let item_rect = Rect::from_min_size(
                    popup_rect.min + Vec2::new(4.0, 4.0 + i as f32 * 32.0),
                    Vec2::new(dropdown_width - 8.0, 28.0),
                );

                let item_response =
                    ui.interact(item_rect, ui.id().with(("line_style", i)), egui::Sense::click());

                let is_selected = self.common.line_style == style;
                let item_bg = if is_selected {
                    Color32::from_rgb(70, 130, 180)
                } else if item_response.hovered() {
                    WIDGET_HOVER_COLOR
                } else {
                    Color32::TRANSPARENT
                };

                if item_bg != Color32::TRANSPARENT {
                    ui.painter().rect_filled(item_rect, 3.0, item_bg);
                }

                // Draw line style preview
                let preview_rect = Rect::from_min_size(
                    item_rect.min + Vec2::new(8.0, (28.0 - 2.0) / 2.0),
                    Vec2::new(item_rect.width() - 16.0, 2.0),
                );
                self.draw_line_style_preview(ui, preview_rect, style, Color32::WHITE);

                if item_response.clicked() {
                    self.common.line_style = style;
                    self.line_style_open = false;
                }
            }
        }
    }

    /// Draw line style preview (SVG-like rendering)
    fn draw_line_style_preview(&self, ui: &mut Ui, rect: Rect, style: LineStyle, color: Color32) {
        let y = rect.center().y;
        let start_x = rect.min.x;
        let end_x = rect.max.x;

        match style {
            LineStyle::Solid => {
                ui.painter().line_segment(
                    [Pos2::new(start_x, y), Pos2::new(end_x, y)],
                    Stroke::new(2.0, color),
                );
            }
            LineStyle::Dashed => {
                let dash_len = 8.0;
                let gap_len = 4.0;
                let mut x = start_x;
                while x < end_x {
                    let dash_end = (x + dash_len).min(end_x);
                    ui.painter().line_segment(
                        [Pos2::new(x, y), Pos2::new(dash_end, y)],
                        Stroke::new(2.0, color),
                    );
                    x += dash_len + gap_len;
                }
            }
            LineStyle::Dotted => {
                let dot_spacing = 6.0;
                let mut x = start_x;
                while x <= end_x {
                    ui.painter().circle_filled(Pos2::new(x, y), 1.5, color);
                    x += dot_spacing;
                }
            }
            LineStyle::DashDot => {
                let dash_len = 8.0;
                let gap = 4.0;
                let mut x = start_x;
                let mut is_dash = true;
                while x < end_x {
                    if is_dash {
                        let dash_end = (x + dash_len).min(end_x);
                        ui.painter().line_segment(
                            [Pos2::new(x, y), Pos2::new(dash_end, y)],
                            Stroke::new(2.0, color),
                        );
                        x += dash_len + gap;
                    } else {
                        ui.painter().circle_filled(Pos2::new(x, y), 1.5, color);
                        x += gap;
                    }
                    is_dash = !is_dash;
                }
            }
            LineStyle::DashDotDot => {
                let dash_len = 8.0;
                let gap = 3.0;
                let mut x = start_x;
                let mut state = 0; // 0=dash, 1=dot, 2=dot
                while x < end_x {
                    match state {
                        0 => {
                            let dash_end = (x + dash_len).min(end_x);
                            ui.painter().line_segment(
                                [Pos2::new(x, y), Pos2::new(dash_end, y)],
                                Stroke::new(2.0, color),
                            );
                            x += dash_len + gap;
                        }
                        _ => {
                            ui.painter().circle_filled(Pos2::new(x, y), 1.5, color);
                            x += gap + 3.0;
                        }
                    }
                    state = (state + 1) % 3;
                }
            }
        }
    }

    /// Render stroke width drag value (slider + editable number)
    fn render_stroke_width_drag(&mut self, ui: &mut Ui) {
        ui.add(
            egui::DragValue::new(&mut self.common.stroke_width)
                .range(1.0..=50.0)
                .speed(0.5)
                .suffix(" px"),
        );
    }

    /// Render fill mode toggle buttons
    fn render_fill_mode_buttons(&mut self, ui: &mut Ui) {
        let btn_size = Vec2::splat(WIDGET_HEIGHT);

        // Stroke only (outline square)
        let (rect, response) = ui.allocate_exact_size(btn_size, egui::Sense::click());
        let selected = self.common.fill_mode == FillMode::Stroke;
        self.draw_fill_mode_icon(ui, rect, response.hovered(), selected, FillMode::Stroke);
        if response.clicked() {
            self.common.fill_mode = FillMode::Stroke;
        }

        // Fill only (solid square)
        let (rect, response) = ui.allocate_exact_size(btn_size, egui::Sense::click());
        let selected = self.common.fill_mode == FillMode::Fill;
        self.draw_fill_mode_icon(ui, rect, response.hovered(), selected, FillMode::Fill);
        if response.clicked() {
            self.common.fill_mode = FillMode::Fill;
        }

        // Both (outline + fill square)
        let (rect, response) = ui.allocate_exact_size(btn_size, egui::Sense::click());
        let selected = self.common.fill_mode == FillMode::Both;
        self.draw_fill_mode_icon(ui, rect, response.hovered(), selected, FillMode::Both);
        if response.clicked() {
            self.common.fill_mode = FillMode::Both;
        }
    }

    /// Draw fill mode icon
    fn draw_fill_mode_icon(
        &self,
        ui: &mut Ui,
        rect: Rect,
        hovered: bool,
        selected: bool,
        mode: FillMode,
    ) {
        let bg_color = if selected {
            Color32::from_rgb(70, 130, 180)
        } else if hovered {
            WIDGET_HOVER_COLOR
        } else {
            WIDGET_BG_COLOR
        };

        ui.painter().rect_filled(rect, 4.0, bg_color);

        let icon_rect = rect.shrink(6.0);
        match mode {
            FillMode::Stroke => {
                ui.painter().rect_stroke(
                    icon_rect,
                    2.0,
                    Stroke::new(2.0, Color32::WHITE),
                    egui::StrokeKind::Inside,
                );
            }
            FillMode::Fill => {
                ui.painter().rect_filled(icon_rect, 2.0, Color32::WHITE);
            }
            FillMode::Both => {
                ui.painter().rect_filled(icon_rect, 2.0, Color32::from_gray(180));
                ui.painter().rect_stroke(
                    icon_rect,
                    2.0,
                    Stroke::new(2.0, Color32::WHITE),
                    egui::StrokeKind::Inside,
                );
            }
        }
    }

    /// Render arrow style toggle buttons
    fn render_arrow_style_buttons(&mut self, ui: &mut Ui) {
        let btn_size = Vec2::new(32.0, WIDGET_HEIGHT);

        // Single arrow
        let (rect, response) = ui.allocate_exact_size(btn_size, egui::Sense::click());
        let selected = self.arrow.arrow_style == ArrowStyle::Single;
        self.draw_arrow_icon(ui, rect, response.hovered(), selected, ArrowStyle::Single);
        if response.clicked() {
            self.arrow.arrow_style = ArrowStyle::Single;
        }

        // Double arrow
        let (rect, response) = ui.allocate_exact_size(btn_size, egui::Sense::click());
        let selected = self.arrow.arrow_style == ArrowStyle::Double;
        self.draw_arrow_icon(ui, rect, response.hovered(), selected, ArrowStyle::Double);
        if response.clicked() {
            self.arrow.arrow_style = ArrowStyle::Double;
        }

        // No arrow (line)
        let (rect, response) = ui.allocate_exact_size(btn_size, egui::Sense::click());
        let selected = self.arrow.arrow_style == ArrowStyle::None;
        self.draw_arrow_icon(ui, rect, response.hovered(), selected, ArrowStyle::None);
        if response.clicked() {
            self.arrow.arrow_style = ArrowStyle::None;
        }
    }

    /// Draw arrow icon
    fn draw_arrow_icon(
        &self,
        ui: &mut Ui,
        rect: Rect,
        hovered: bool,
        selected: bool,
        style: ArrowStyle,
    ) {
        let bg_color = if selected {
            Color32::from_rgb(70, 130, 180)
        } else if hovered {
            WIDGET_HOVER_COLOR
        } else {
            WIDGET_BG_COLOR
        };

        ui.painter().rect_filled(rect, 4.0, bg_color);

        let center = rect.center();
        let half_width = rect.width() / 2.0 - 6.0;

        // Draw line
        ui.painter().line_segment(
            [
                Pos2::new(center.x - half_width, center.y),
                Pos2::new(center.x + half_width, center.y),
            ],
            Stroke::new(2.0, Color32::WHITE),
        );

        let arrow_size = 5.0;

        match style {
            ArrowStyle::Single => {
                // Arrow at end
                let tip = Pos2::new(center.x + half_width, center.y);
                ui.painter().add(egui::Shape::convex_polygon(
                    vec![
                        tip,
                        Pos2::new(tip.x - arrow_size, tip.y - arrow_size),
                        Pos2::new(tip.x - arrow_size, tip.y + arrow_size),
                    ],
                    Color32::WHITE,
                    Stroke::NONE,
                ));
            }
            ArrowStyle::Double => {
                // Arrow at both ends
                let tip_right = Pos2::new(center.x + half_width, center.y);
                let tip_left = Pos2::new(center.x - half_width, center.y);

                ui.painter().add(egui::Shape::convex_polygon(
                    vec![
                        tip_right,
                        Pos2::new(tip_right.x - arrow_size, tip_right.y - arrow_size),
                        Pos2::new(tip_right.x - arrow_size, tip_right.y + arrow_size),
                    ],
                    Color32::WHITE,
                    Stroke::NONE,
                ));
                ui.painter().add(egui::Shape::convex_polygon(
                    vec![
                        tip_left,
                        Pos2::new(tip_left.x + arrow_size, tip_left.y - arrow_size),
                        Pos2::new(tip_left.x + arrow_size, tip_left.y + arrow_size),
                    ],
                    Color32::WHITE,
                    Stroke::NONE,
                ));
            }
            ArrowStyle::None => {
                // Just a line, no arrow heads
            }
        }
    }

    /// Render color picker with palette and custom color button
    fn render_color_picker(&mut self, ui: &mut Ui) {
        // Color palette
        for &color in COLOR_PALETTE {
            let is_selected = self.common.color == color;
            let size = COLOR_SWATCH_SIZE;

            let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());

            // Draw color swatch
            ui.painter().rect_filled(rect, 3.0, color);

            // Draw selection border
            if is_selected {
                ui.painter().rect_stroke(
                    rect,
                    3.0,
                    Stroke::new(2.0, Color32::WHITE),
                    egui::StrokeKind::Outside,
                );
            } else if response.hovered() {
                ui.painter().rect_stroke(
                    rect,
                    3.0,
                    Stroke::new(1.0, Color32::from_gray(150)),
                    egui::StrokeKind::Outside,
                );
            }

            if response.clicked() {
                self.common.color = color;
            }
        }

        ui.add_space(4.0);

        // Custom color picker button
        let picker_response = egui::color_picker::color_edit_button_srgba(
            ui,
            &mut self.common.color,
            egui::color_picker::Alpha::Opaque,
        );

        // Track if color picker popup is open (the button is "open" when popup is shown)
        // We detect this by checking if the response indicates the popup is active
        self.color_picker_open =
            picker_response.has_focus() || egui::Popup::is_id_open(ui.ctx(), picker_response.id);
    }

    /// Handle click on options panel, returns true if click is inside panel
    pub fn handle_click(&mut self, pos: Pos2) -> bool {
        self.contains(pos)
    }

    /// Close any open popups (called when clicking outside the panel)
    pub fn close_popups(&mut self) {
        self.line_style_open = false;
        // Note: color_picker_open is managed by egui internally
    }
}

impl Default for OptionsPanel {
    fn default() -> Self {
        Self::new(Pos2::ZERO, Vec2::ZERO)
    }
}
