//! Floating popup for annotation settings
//!
//! Shows stroke width, style, and color options.

use crate::screenshot::stroke::{PRESET_COLORS, PRESET_WIDTHS, StrokeSettings, StrokeStyle};

/// Floating popup for annotation settings
pub struct AnnotatePopup {
    /// Whether popup is visible
    pub visible: bool,
    /// Popup position (near the button)
    position: egui::Pos2,
    /// Current stroke settings
    pub settings: StrokeSettings,
}

impl Default for AnnotatePopup {
    fn default() -> Self {
        Self::new()
    }
}

impl AnnotatePopup {
    pub fn new() -> Self {
        Self { visible: false, position: egui::pos2(0.0, 0.0), settings: StrokeSettings::default() }
    }

    /// Show popup at given position
    pub fn show(&mut self, pos: egui::Pos2) {
        self.visible = true;
        self.position = pos;
    }

    /// Hide popup
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle popup visibility at given position
    pub fn toggle(&mut self, pos: egui::Pos2) {
        if self.visible {
            self.hide();
        } else {
            self.show(pos);
        }
    }

    /// Render the popup
    /// Returns true if settings were changed
    pub fn render(&mut self, ctx: &egui::Context) -> bool {
        if !self.visible {
            return false;
        }

        let mut settings_changed = false;

        egui::Area::new(egui::Id::new("annotate_popup"))
            .fixed_pos(self.position)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .fill(egui::Color32::from_rgba_unmultiplied(40, 40, 40, 240))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(80)))
                    .corner_radius(8.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_min_width(280.0);

                        // Title
                        ui.heading("Annotation Settings");
                        ui.add_space(8.0);

                        // Line width section
                        ui.horizontal(|ui| {
                            ui.label("Width:");
                            ui.add_space(8.0);
                            for &width in &PRESET_WIDTHS {
                                let selected = (self.settings.width - width).abs() < 0.1;
                                let text = format!("{}", width as i32);
                                let response = ui.selectable_label(selected, text);
                                if response.clicked() {
                                    self.settings.width = width;
                                    settings_changed = true;
                                }
                            }
                        });

                        ui.add_space(8.0);

                        // Line style section
                        ui.horizontal(|ui| {
                            ui.label("Style:");
                            ui.add_space(12.0);
                            for style in
                                [StrokeStyle::Solid, StrokeStyle::Dashed, StrokeStyle::Dotted]
                            {
                                let name = match style {
                                    StrokeStyle::Solid => "━━━",
                                    StrokeStyle::Dashed => "- - -",
                                    StrokeStyle::Dotted => "• • •",
                                };
                                let selected = self.settings.style == style;
                                if ui.selectable_label(selected, name).clicked() {
                                    self.settings.style = style;
                                    settings_changed = true;
                                }
                            }
                        });

                        ui.add_space(8.0);

                        // Color section
                        ui.horizontal(|ui| {
                            ui.label("Color:");
                            ui.add_space(8.0);
                            for &color in &PRESET_COLORS {
                                let selected = self.settings.color == color;
                                let size = if selected { 26.0 } else { 22.0 };
                                let (rect, response) = ui.allocate_exact_size(
                                    egui::vec2(size, size),
                                    egui::Sense::click(),
                                );

                                // Draw color swatch
                                ui.painter().rect_filled(rect, 3.0, color);

                                // Draw selection border
                                if selected {
                                    ui.painter().rect_stroke(
                                        rect.expand(2.0),
                                        3.0,
                                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                                        egui::StrokeKind::Outside,
                                    );
                                }

                                if response.clicked() {
                                    self.settings.color = color;
                                    settings_changed = true;
                                }
                            }
                        });
                    });
            });

        settings_changed
    }

    /// Check if a point is inside the popup area
    pub fn contains(&self, _pos: egui::Pos2) -> bool {
        // Approximate popup size (280 width + padding, ~120 height)
        // This is used to prevent closing when clicking inside
        if !self.visible {
            return false;
        }
        // For now, return false - egui handles this internally
        // The popup will consume clicks on its own
        false
    }
}
