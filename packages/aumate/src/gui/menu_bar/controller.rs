//! Menu bar feature for the controller
//!
//! Provides system tray / menu bar icon management functionality.

use std::path::PathBuf;

use crate::error::Result;
use crate::gui::controller::{ControllerContext, ControllerFeature, TabInfo};
use crate::gui::menu_bar::{MenuBarIcon, MenuBarItem, MenuBarMenu};
use crate::gui::window::WindowCommand;

/// Menu bar feature for system tray icons
pub struct MenuBarFeature {
    /// Name for new menu bar item
    new_menu_bar_name: String,
    /// Tooltip for new menu bar item
    new_menu_bar_tooltip: String,
    /// Counter for unique IDs
    menu_bar_counter: u32,
    /// List of active menu bar item IDs
    menu_bar_items: Vec<String>,
    /// Selected color for tray icon [R, G, B]
    tray_icon_color: [u8; 3],
    /// Optional path to icon image file
    tray_icon_image_path: Option<PathBuf>,
}

impl MenuBarFeature {
    pub fn new() -> Self {
        Self {
            new_menu_bar_name: "My App".to_string(),
            new_menu_bar_tooltip: "Click for menu".to_string(),
            menu_bar_counter: 1,
            menu_bar_items: Vec::new(),
            tray_icon_color: [100, 200, 255], // Cyan default
            tray_icon_image_path: None,
        }
    }

    /// Create a tray icon with the selected color
    fn create_tray_icon(&self) -> MenuBarIcon {
        let size = 22u32; // Standard macOS menu bar icon size
        let mut data = Vec::with_capacity((size * size * 4) as usize);
        let center = size as f32 / 2.0;
        let radius = center - 2.0;

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius {
                    // Inside circle - use selected color
                    data.extend_from_slice(&[
                        self.tray_icon_color[0],
                        self.tray_icon_color[1],
                        self.tray_icon_color[2],
                        255,
                    ]);
                } else {
                    // Outside - transparent
                    data.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }

        MenuBarIcon::Rgba { data, width: size, height: size }
    }

    /// Open file picker for tray icon image
    fn open_image_picker_for_tray(&mut self) {
        if let Some(path) =
            rfd::FileDialog::new().add_filter("Images", &["png", "svg", "jpg", "jpeg"]).pick_file()
        {
            self.tray_icon_image_path = Some(path);
        }
    }

    /// Create a new menu bar item
    fn create_menu_bar_item(&mut self, ctx: &mut ControllerContext) {
        let id = format!("tray_{}", self.menu_bar_counter);
        self.menu_bar_counter += 1;

        // Create a simple menu with some items
        let menu = MenuBarMenu::new()
            .add_item("show_windows", "Show All Windows")
            .add_item("hide_windows", "Hide All Windows")
            .add_separator()
            .add_item("settings", "Settings...")
            .add_separator()
            .add_quit();

        // Use image icon if selected, otherwise use color-based icon
        let icon = if let Some(path) = &self.tray_icon_image_path {
            MenuBarIcon::Path(path.clone())
        } else {
            self.create_tray_icon()
        };

        let item = MenuBarItem::builder(&self.new_menu_bar_name)
            .id(&id)
            .icon(icon)
            .tooltip(&self.new_menu_bar_tooltip)
            .menu(menu)
            .build();

        let _ = ctx.command_sender.send(WindowCommand::AddMenuBarItem { item });
        self.menu_bar_items.push(id);

        // Update name for next item
        self.new_menu_bar_name = format!("App {}", self.menu_bar_counter);
    }

    /// Render the create menu bar section
    fn render_create_section(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        ui.heading("Create Menu Bar Item");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.new_menu_bar_name);
        });

        ui.horizontal(|ui| {
            ui.label("Tooltip:");
            ui.text_edit_singleline(&mut self.new_menu_bar_tooltip);
        });

        ui.add_space(4.0);

        // Icon selection - color buttons or image file
        ui.label("Icon Color:");
        ui.horizontal(|ui| {
            if ui.button("Cyan").clicked() {
                self.tray_icon_color = [100, 200, 255];
                self.tray_icon_image_path = None;
            }
            if ui.button("Green").clicked() {
                self.tray_icon_color = [100, 255, 150];
                self.tray_icon_image_path = None;
            }
            if ui.button("Purple").clicked() {
                self.tray_icon_color = [200, 100, 255];
                self.tray_icon_image_path = None;
            }
            if ui.button("Orange").clicked() {
                self.tray_icon_color = [255, 150, 50];
                self.tray_icon_image_path = None;
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Image...").clicked() {
                self.open_image_picker_for_tray();
            }
            // Show selected image or color preview
            ui.label("Selected:");
            if let Some(path) = &self.tray_icon_image_path {
                let filename = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                ui.label(&filename);
                if ui.button("Clear").clicked() {
                    self.tray_icon_image_path = None;
                }
            } else {
                let color = egui::Color32::from_rgb(
                    self.tray_icon_color[0],
                    self.tray_icon_color[1],
                    self.tray_icon_color[2],
                );
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                ui.painter().circle_filled(rect.center(), 8.0, color);
            }
        });

        ui.add_space(8.0);

        if ui.button("Add Menu Bar Item").clicked() {
            self.create_menu_bar_item(ctx);
        }
    }

    /// Render the active menu bar items section
    fn render_active_section(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        ui.heading("Menu Bar Items");
        ui.add_space(8.0);

        if !self.menu_bar_items.is_empty() {
            let mut to_remove = None;
            for (idx, id) in self.menu_bar_items.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("• {}", id));
                    if ui.small_button("Remove").clicked() {
                        to_remove = Some(idx);
                    }
                });
            }

            if let Some(idx) = to_remove {
                let id = self.menu_bar_items.remove(idx);
                let _ = ctx.command_sender.send(WindowCommand::RemoveMenuBarItem { id });
            }
        } else {
            ui.label("No menu bar items created.");
        }
    }
}

impl Default for MenuBarFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for MenuBarFeature {
    fn id(&self) -> &'static str {
        "menu_bar"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("menu_bar", "Menu Bar", 20) // After screenshot (10)
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        self.render_create_section(ui, ctx);
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        self.render_active_section(ui, ctx);
    }

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Menu bar feature initialized");
        Ok(())
    }
}
