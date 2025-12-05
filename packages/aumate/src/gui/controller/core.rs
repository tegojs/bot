//! Core rendering utilities for the controller

use crate::gui::content::{Content, ImageDisplayOptions, ScaleMode};
use crate::gui::window::WindowCommand;
use egui::Ui;

use super::context::ControllerContext;
use super::registry::FeatureRegistry;

/// Default background image embedded at compile time
const DEFAULT_BACKGROUND: &[u8] = include_bytes!("../../assets/background.png");

/// Icon size for UI elements
pub const CONTROLLER_ICON_SIZE: f32 = 20.0;

/// Load the default controller background image
pub fn load_default_background() -> Option<Content> {
    match image::load_from_memory(DEFAULT_BACKGROUND) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            Some(Content::Image {
                data: rgba.into_raw(),
                width,
                height,
                options: ImageDisplayOptions::new().with_scale_mode(ScaleMode::Stretch),
            })
        }
        Err(e) => {
            log::warn!("Failed to load default background: {}", e);
            None
        }
    }
}

/// Render the title bar with app name and exit button
pub fn render_title_bar(ui: &mut Ui, ctx: &ControllerContext) {
    ui.horizontal(|ui| {
        ui.heading("aumate");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Allocate space for close button
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(28.0, 28.0), egui::Sense::click());

            // Draw background on hover
            if response.hovered() {
                ui.painter().rect_filled(
                    rect,
                    egui::CornerRadius::same(4),
                    egui::Color32::from_rgba_unmultiplied(200, 60, 60, 180),
                );
            }

            // Draw X using lines
            let padding = 8.0;
            let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
            ui.painter().line_segment(
                [
                    egui::pos2(rect.min.x + padding, rect.min.y + padding),
                    egui::pos2(rect.max.x - padding, rect.max.y - padding),
                ],
                stroke,
            );
            ui.painter().line_segment(
                [
                    egui::pos2(rect.max.x - padding, rect.min.y + padding),
                    egui::pos2(rect.min.x + padding, rect.max.y - padding),
                ],
                stroke,
            );

            if response.clicked() {
                let _ = ctx.command_sender.send(WindowCommand::ExitApplication);
            }
        });
    });
}

/// Render the navigation sidebar with dynamic tabs from registry
pub fn render_navigation(ui: &mut Ui, registry: &mut FeatureRegistry, ctx: &mut ControllerContext) {
    ui.add_space(8.0);

    let tabs = registry.tabs();
    let active_tab = registry.active_tab().cloned();

    for tab_info in tabs {
        let selected = active_tab.as_ref() == Some(&tab_info.id);
        let response = ui.add_sized(
            [ui.available_width(), 32.0],
            egui::Button::new(&tab_info.name).selected(selected),
        );
        if response.clicked() {
            registry.set_active(&tab_info.id, ctx);
        }
    }
}

/// Helper to render a section header
#[allow(dead_code)]
pub fn section_header(ui: &mut Ui, title: &str) {
    ui.heading(title);
    ui.add_space(8.0);
}

/// Helper to render a collapsible section
#[allow(dead_code)]
pub fn collapsible_section<R>(
    ui: &mut Ui,
    id: &str,
    title: &str,
    default_open: bool,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> egui::CollapsingResponse<R> {
    egui::CollapsingHeader::new(title).id_salt(id).default_open(default_open).show(ui, add_contents)
}

/// Format a timestamp as relative time (e.g., "2m ago", "1h ago")
pub fn format_time_ago(dt: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(dt);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{}d ago", duration.num_days())
    } else {
        dt.format("%Y-%m-%d").to_string()
    }
}
