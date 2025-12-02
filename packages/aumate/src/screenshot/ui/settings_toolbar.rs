//! Top-left settings toolbar for screenshot
//!
//! Provides buttons for corner radius, aspect lock, border/shadow, and refresh.

use crate::screenshot::settings::{
    ScreenshotSettings, ASPECT_PRESETS, BORDER_WIDTH_PRESETS, CORNER_RADIUS_PRESETS,
};

/// Which settings popup is currently open
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsPopup {
    CornerRadius,
    AspectRatio,
    BorderShadow,
}

/// Button clicked on settings toolbar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsButton {
    Refresh,
}

/// Top-left settings toolbar
pub struct SettingsToolbar {
    /// Toolbar position
    position: egui::Pos2,
    /// Which popup is currently open
    active_popup: Option<SettingsPopup>,
}

impl Default for SettingsToolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsToolbar {
    const BUTTON_SIZE: f32 = 28.0;
    const BUTTON_SPACING: f32 = 4.0;
    const PADDING: f32 = 6.0;

    pub fn new() -> Self {
        Self { position: egui::pos2(0.0, 0.0), active_popup: None }
    }

    /// Update position based on selection bounds
    pub fn update_position(&mut self, selection_min: (f32, f32)) {
        // Position above the selection
        self.position = egui::pos2(selection_min.0, selection_min.1 - Self::toolbar_height() - 8.0);
    }

    fn toolbar_height() -> f32 {
        Self::BUTTON_SIZE + Self::PADDING * 2.0
    }

    /// Render toolbar and return clicked button (if any)
    ///
    /// # Arguments
    /// * `ctx` - egui context
    /// * `settings` - Screenshot settings
    /// * `start_coords` - Start coordinates (x, y) in physical pixels
    /// * `resolution` - Selection resolution (width, height) in physical pixels
    pub fn render(
        &mut self,
        ctx: &egui::Context,
        settings: &mut ScreenshotSettings,
        start_coords: (i32, i32),
        resolution: (u32, u32),
    ) -> Option<SettingsButton> {
        let mut clicked = None;

        egui::Area::new(egui::Id::new("settings_toolbar"))
            .fixed_pos(self.position)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .fill(egui::Color32::from_rgba_unmultiplied(40, 40, 40, 230))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(80)))
                    .corner_radius(6.0)
                    .inner_margin(Self::PADDING)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = Self::BUTTON_SPACING;

                            // Start coordinates label
                            ui.label(
                                egui::RichText::new(format!(
                                    "({}, {})",
                                    start_coords.0, start_coords.1
                                ))
                                .color(egui::Color32::from_gray(200))
                                .size(13.0),
                            );

                            ui.separator();

                            // Resolution label
                            ui.label(
                                egui::RichText::new(format!("{}×{}", resolution.0, resolution.1))
                                    .color(egui::Color32::from_gray(200))
                                    .size(13.0),
                            );

                            ui.separator();

                            // Corner radius button
                            if self.icon_button(
                                ui,
                                "⌓",
                                settings.corner_radius > 0.0,
                                "Corner radius",
                            ) {
                                self.toggle_popup(SettingsPopup::CornerRadius);
                            }

                            // Aspect lock button
                            if self.icon_button(
                                ui,
                                "⛶",
                                settings.aspect_locked,
                                "Aspect ratio lock",
                            ) {
                                self.toggle_popup(SettingsPopup::AspectRatio);
                            }

                            // Border/shadow button
                            if self.icon_button(
                                ui,
                                "▣",
                                settings.border_width > 0.0 || settings.shadow_enabled,
                                "Border & shadow",
                            ) {
                                self.toggle_popup(SettingsPopup::BorderShadow);
                            }

                            // Refresh button
                            if self.icon_button(ui, "↻", false, "Re-capture") {
                                clicked = Some(SettingsButton::Refresh);
                            }
                        });
                    });
            });

        // Render active popup
        match self.active_popup {
            Some(SettingsPopup::CornerRadius) => {
                self.render_corner_radius_popup(ctx, settings);
            }
            Some(SettingsPopup::AspectRatio) => {
                self.render_aspect_ratio_popup(ctx, settings);
            }
            Some(SettingsPopup::BorderShadow) => {
                self.render_border_shadow_popup(ctx, settings);
            }
            None => {}
        }

        clicked
    }

    fn icon_button(&self, ui: &mut egui::Ui, icon: &str, active: bool, tooltip: &str) -> bool {
        let size = egui::vec2(Self::BUTTON_SIZE, Self::BUTTON_SIZE);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        let bg = if active {
            egui::Color32::from_rgb(0, 100, 200)
        } else if response.hovered() {
            egui::Color32::from_gray(80)
        } else {
            egui::Color32::from_gray(60)
        };

        ui.painter().rect_filled(rect, 4.0, bg);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(16.0),
            egui::Color32::WHITE,
        );

        response.on_hover_text(tooltip).clicked()
    }

    fn toggle_popup(&mut self, popup: SettingsPopup) {
        if self.active_popup == Some(popup) {
            self.active_popup = None;
        } else {
            self.active_popup = Some(popup);
        }
    }

    /// Close any open popup
    pub fn close_popup(&mut self) {
        self.active_popup = None;
    }

    fn render_corner_radius_popup(&mut self, ctx: &egui::Context, settings: &mut ScreenshotSettings) {
        let popup_pos = egui::pos2(self.position.x, self.position.y + Self::toolbar_height() + 4.0);

        egui::Area::new(egui::Id::new("corner_radius_popup"))
            .fixed_pos(popup_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .fill(egui::Color32::from_rgba_unmultiplied(40, 40, 40, 240))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(80)))
                    .corner_radius(8.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_min_width(200.0);

                        ui.label("Corner Radius");
                        ui.add_space(4.0);
                        ui.add(
                            egui::Slider::new(&mut settings.corner_radius, 0.0..=50.0)
                                .suffix("px")
                                .clamping(egui::SliderClamping::Always),
                        );

                        ui.add_space(8.0);

                        // Preset buttons
                        ui.horizontal(|ui| {
                            for &radius in CORNER_RADIUS_PRESETS {
                                let selected = (settings.corner_radius - radius).abs() < 0.1;
                                if ui.selectable_label(selected, format!("{}", radius as i32)).clicked()
                                {
                                    settings.corner_radius = radius;
                                }
                            }
                        });
                    });
            });
    }

    fn render_aspect_ratio_popup(&mut self, ctx: &egui::Context, settings: &mut ScreenshotSettings) {
        let popup_pos = egui::pos2(
            self.position.x + Self::BUTTON_SIZE + Self::BUTTON_SPACING,
            self.position.y + Self::toolbar_height() + 4.0,
        );

        egui::Area::new(egui::Id::new("aspect_ratio_popup"))
            .fixed_pos(popup_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .fill(egui::Color32::from_rgba_unmultiplied(40, 40, 40, 240))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(80)))
                    .corner_radius(8.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_min_width(180.0);

                        // Lock toggle
                        ui.horizontal(|ui| {
                            ui.label("Lock Aspect Ratio:");
                            if ui.checkbox(&mut settings.aspect_locked, "").changed() {
                                // When locking, capture current ratio if none set
                                if settings.aspect_locked && settings.aspect_ratio.is_none() {
                                    // Will be set by the caller based on current selection
                                }
                            }
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        ui.label("Preset Ratios:");
                        ui.add_space(4.0);

                        // Preset ratio buttons
                        for &(ratio, label) in ASPECT_PRESETS {
                            let selected =
                                settings.aspect_ratio.is_some_and(|r| (r - ratio).abs() < 0.01);
                            if ui.selectable_label(selected, label).clicked() {
                                settings.aspect_ratio = Some(ratio);
                                settings.aspect_locked = true;
                            }
                        }

                        ui.add_space(8.0);

                        // Free ratio option
                        if ui
                            .selectable_label(
                                settings.aspect_locked && settings.aspect_ratio.is_none(),
                                "Current",
                            )
                            .clicked()
                        {
                            settings.aspect_ratio = None;
                            settings.aspect_locked = true;
                        }

                        // Unlock option
                        if ui.selectable_label(!settings.aspect_locked, "Free").clicked() {
                            settings.aspect_locked = false;
                        }
                    });
            });
    }

    fn render_border_shadow_popup(&mut self, ctx: &egui::Context, settings: &mut ScreenshotSettings) {
        let popup_pos = egui::pos2(
            self.position.x + 2.0 * (Self::BUTTON_SIZE + Self::BUTTON_SPACING),
            self.position.y + Self::toolbar_height() + 4.0,
        );

        egui::Area::new(egui::Id::new("border_shadow_popup"))
            .fixed_pos(popup_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .fill(egui::Color32::from_rgba_unmultiplied(40, 40, 40, 240))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(80)))
                    .corner_radius(8.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.set_min_width(220.0);

                        // Border section
                        ui.label("Border");
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label("Width:");
                            for &width in BORDER_WIDTH_PRESETS {
                                let selected = (settings.border_width - width).abs() < 0.1;
                                if ui.selectable_label(selected, format!("{}", width as i32)).clicked()
                                {
                                    settings.border_width = width;
                                }
                            }
                        });

                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label("Color:");
                            let mut color_arr = [
                                settings.border_color.r(),
                                settings.border_color.g(),
                                settings.border_color.b(),
                            ];
                            if ui.color_edit_button_srgb(&mut color_arr).changed() {
                                settings.border_color =
                                    egui::Color32::from_rgb(color_arr[0], color_arr[1], color_arr[2]);
                            }
                        });

                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Shadow section
                        ui.horizontal(|ui| {
                            ui.label("Shadow");
                            ui.checkbox(&mut settings.shadow_enabled, "");
                        });

                        if settings.shadow_enabled {
                            ui.add_space(4.0);

                            ui.horizontal(|ui| {
                                ui.label("Blur:");
                                ui.add(
                                    egui::Slider::new(&mut settings.shadow_blur, 0.0..=30.0)
                                        .suffix("px"),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Offset X:");
                                ui.add(
                                    egui::Slider::new(&mut settings.shadow_offset.0, -20.0..=20.0)
                                        .suffix("px"),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Offset Y:");
                                ui.add(
                                    egui::Slider::new(&mut settings.shadow_offset.1, -20.0..=20.0)
                                        .suffix("px"),
                                );
                            });

                            ui.add_space(4.0);

                            ui.horizontal(|ui| {
                                ui.label("Color:");
                                let mut color_arr = [
                                    settings.shadow_color.r(),
                                    settings.shadow_color.g(),
                                    settings.shadow_color.b(),
                                    settings.shadow_color.a(),
                                ];
                                if ui.color_edit_button_srgba_unmultiplied(&mut color_arr).changed() {
                                    settings.shadow_color = egui::Color32::from_rgba_unmultiplied(
                                        color_arr[0],
                                        color_arr[1],
                                        color_arr[2],
                                        color_arr[3],
                                    );
                                }
                            });
                        }
                    });
            });
    }
}
