//! Screenshot feature for the controller
//!
//! Provides region capture functionality with configurable actions.

use std::collections::HashMap;

use egui::TextureHandle;

use crate::error::Result;
use crate::gui::controller::{CONTROLLER_ICON_SIZE, ControllerContext, ControllerFeature, TabInfo};
use crate::gui::window::WindowCommand;
use crate::screenshot::{create_default_registry, icons};

/// Screenshot feature for region capture
pub struct ScreenshotFeature {
    /// Which screenshot actions are enabled
    screenshot_actions_enabled: HashMap<String, bool>,
    /// Local icon cache for this feature
    icon_cache: HashMap<String, TextureHandle>,
}

impl ScreenshotFeature {
    pub fn new() -> Self {
        Self {
            screenshot_actions_enabled: Self::default_screenshot_actions(),
            icon_cache: HashMap::new(),
        }
    }

    /// Get default screenshot actions (enabled by default in Snipaste order)
    fn default_screenshot_actions() -> HashMap<String, bool> {
        let mut actions = HashMap::new();
        // Drawing tools - most enabled by default (in Snipaste order)
        actions.insert("rectangle".to_string(), true);
        actions.insert("ellipse".to_string(), true);
        actions.insert("polyline".to_string(), true);
        actions.insert("arrow".to_string(), true);
        actions.insert("annotate".to_string(), true);
        actions.insert("highlighter".to_string(), true);
        actions.insert("mosaic".to_string(), true);
        actions.insert("text".to_string(), true);
        actions.insert("sequence".to_string(), true);
        actions.insert("eraser".to_string(), true);
        // Privacy tools
        actions.insert("blur".to_string(), true);
        // Terminal actions
        actions.insert("cancel".to_string(), true);
        actions.insert("save".to_string(), true);
        actions.insert("copy".to_string(), true);
        actions
    }
}

impl Default for ScreenshotFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for ScreenshotFeature {
    fn id(&self) -> &'static str {
        "screenshot"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("screenshot", "Screenshot", 10) // After floating windows (0)
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        ui.heading("Region Capture");
        ui.add_space(8.0);

        // Get action info from registry
        let registry = create_default_registry();
        let actions = registry.get_all();

        // Action checkboxes - styled grid with icon and description
        ui.label("Enabled Actions:");
        ui.add_space(4.0);

        // Get scale factor for DPI-aware icon rendering
        let scale_factor = ui.ctx().pixels_per_point();
        // Render icons at 2x minimum for crisp display on Retina
        let render_scale = scale_factor.max(2.0);
        let render_size = (CONTROLLER_ICON_SIZE * render_scale).ceil() as u32;

        // Create styled action items in a grid layout
        egui::Grid::new("screenshot_actions_grid")
            .num_columns(3)
            .spacing([12.0, 6.0])
            .min_col_width(28.0)
            .show(ui, |ui| {
                for action in &actions {
                    if let Some(enabled) = self.screenshot_actions_enabled.get_mut(&action.id) {
                        // Checkbox with custom styling
                        ui.checkbox(enabled, "");

                        // Get or create icon texture at high resolution
                        let icon_id = action.icon_id.as_deref().unwrap_or(&action.id);
                        let cache_key = format!("{}_{}", icon_id, render_size);
                        if !self.icon_cache.contains_key(&cache_key) {
                            if let Some(texture) = icons::create_icon_texture(
                                ui.ctx(),
                                icon_id,
                                render_size,
                                egui::Color32::WHITE,
                            ) {
                                self.icon_cache.insert(cache_key.clone(), texture);
                            }
                        }

                        // Render icon in a styled container
                        let (rect, _response) =
                            ui.allocate_exact_size(egui::vec2(28.0, 28.0), egui::Sense::hover());

                        // Draw subtle background for icon
                        ui.painter().rect_filled(
                            rect,
                            egui::CornerRadius::same(4),
                            egui::Color32::from_rgba_unmultiplied(60, 65, 80, 180),
                        );

                        // Center and draw the icon at logical size (texture is high-res)
                        if let Some(texture) = self.icon_cache.get(&cache_key) {
                            let icon_rect = egui::Rect::from_center_size(
                                rect.center(),
                                egui::vec2(CONTROLLER_ICON_SIZE, CONTROLLER_ICON_SIZE),
                            );
                            ui.painter().image(
                                texture.id(),
                                icon_rect,
                                egui::Rect::from_min_max(
                                    egui::Pos2::ZERO,
                                    egui::Pos2::new(1.0, 1.0),
                                ),
                                egui::Color32::WHITE,
                            );
                        } else {
                            // Fallback text
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                action.id[..1].to_uppercase(),
                                egui::FontId::proportional(12.0),
                                egui::Color32::WHITE,
                            );
                        }

                        // Label with proper color
                        let label_color = if *enabled {
                            egui::Color32::WHITE
                        } else {
                            egui::Color32::from_gray(140)
                        };
                        ui.label(egui::RichText::new(&action.name).color(label_color));

                        ui.end_row();
                    }
                }
            });

        ui.add_space(12.0);

        // Region Capture button with prominent styling
        if ui
            .add_sized(
                [140.0, 32.0],
                egui::Button::new(egui::RichText::new("Region Capture").size(14.0)),
            )
            .clicked()
        {
            let enabled: Vec<String> = self
                .screenshot_actions_enabled
                .iter()
                .filter(|&(_, v)| *v)
                .map(|(k, _)| k.clone())
                .collect();

            let _ = ctx
                .command_sender
                .send(WindowCommand::StartScreenshotMode { enabled_actions: enabled });
        }
    }

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Screenshot feature initialized");
        Ok(())
    }
}
