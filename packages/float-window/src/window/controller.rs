//! Controller window UI for managing flow windows

use super::commands::{CommandSender, WindowCommand, WindowRegistry};
use super::config::{Position, Size, WindowConfig};
use crate::content::Content;
use crate::effect::{PresetEffect, PresetEffectOptions};
use crate::menu_bar::{MenuBarIcon, MenuBarItem, MenuBarMenu};
use crate::shape::WindowShape;
use egui::{Context, Ui};
use std::path::PathBuf;

/// Available shapes for selection
const ALL_SHAPES: &[(&str, WindowShape)] = &[
    ("Circle", WindowShape::Circle),
    ("Rectangle", WindowShape::Rectangle),
];

/// All available preset effects
const ALL_EFFECTS: &[PresetEffect] = &[
    PresetEffect::RotatingHalo,
    PresetEffect::PulseRipple,
    PresetEffect::FlowingLight,
    PresetEffect::StardustScatter,
    PresetEffect::ElectricSpark,
    PresetEffect::SmokeWisp,
    PresetEffect::RainDrop,
    PresetEffect::LaserBeam,
    PresetEffect::LightningArc,
    PresetEffect::MeteorShower,
    PresetEffect::SonarPulse,
    PresetEffect::MatrixRain,
    PresetEffect::AuroraWave,
    PresetEffect::OrbitRings,
    PresetEffect::HeartbeatPulse,
    PresetEffect::CosmicStrings,
    PresetEffect::SilkRibbon,
];

/// Controller state for managing windows
pub struct ControllerState {
    /// Command sender to communicate with the event loop
    command_sender: CommandSender,
    /// Registry of managed windows
    registry: WindowRegistry,
    /// Selected effect for new window
    selected_effect: PresetEffect,
    /// Effect options for new window
    effect_options: PresetEffectOptions,
    /// Selected shape for new window
    selected_shape: WindowShape,
    /// Window size for new window
    new_window_size: u32,
    /// Position X for new window
    new_window_x: f32,
    /// Position Y for new window
    new_window_y: f32,
    /// Name for new menu bar item
    new_menu_bar_name: String,
    /// Tooltip for new menu bar item
    new_menu_bar_tooltip: String,
    /// Counter for menu bar items
    menu_bar_counter: u32,
    /// List of created menu bar item IDs
    menu_bar_items: Vec<String>,
    /// Selected tray icon color (RGB)
    tray_icon_color: [u8; 3],
    /// Selected image path for flow window content
    flow_window_image_path: Option<PathBuf>,
    /// Selected image path for tray icon
    tray_icon_image_path: Option<PathBuf>,
}

impl ControllerState {
    /// Create a new controller state
    pub fn new(command_sender: CommandSender, registry: WindowRegistry) -> Self {
        Self {
            command_sender,
            registry,
            selected_effect: PresetEffect::SilkRibbon,
            effect_options: PresetEffectOptions::default(),
            selected_shape: WindowShape::Circle,
            new_window_size: 50,
            new_window_x: 500.0,
            new_window_y: 300.0,
            new_menu_bar_name: "My App".to_string(),
            new_menu_bar_tooltip: "Click for menu".to_string(),
            menu_bar_counter: 1,
            menu_bar_items: Vec::new(),
            tray_icon_color: [100, 200, 255], // Cyan default
            flow_window_image_path: None,
            tray_icon_image_path: None,
        }
    }

    /// Get the window registry
    pub fn registry(&self) -> &WindowRegistry {
        &self.registry
    }

    /// Render the controller UI
    pub fn render(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Flow Window Controller");
            ui.separator();

            // Wrap content in a scroll area
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    // Create new window section
                    self.render_create_section(ui);

                    ui.separator();

                    // Manage existing windows section
                    self.render_manage_section(ui);

                    ui.separator();

                    // Menu bar section
                    self.render_menu_bar_section(ui);
                });
        });
    }

    /// Render the create new window section
    fn render_create_section(&mut self, ui: &mut Ui) {
        ui.heading("Create New Window");
        ui.add_space(8.0);

        // Effect selection
        ui.horizontal(|ui| {
            ui.label("Effect:");
            egui::ComboBox::from_id_salt("effect_selector")
                .selected_text(format!("{:?}", self.selected_effect))
                .show_ui(ui, |ui| {
                    for effect in ALL_EFFECTS {
                        ui.selectable_value(
                            &mut self.selected_effect,
                            *effect,
                            format!("{:?}", effect),
                        );
                    }
                });
        });

        ui.add_space(4.0);

        // Effect options based on selected effect
        ui.collapsing("Effect Options", |ui| {
            self.render_effect_options(ui);
        });

        ui.add_space(4.0);

        // Shape selection
        ui.horizontal(|ui| {
            ui.label("Shape:");
            egui::ComboBox::from_id_salt("shape_selector")
                .selected_text(format!("{:?}", self.selected_shape))
                .show_ui(ui, |ui| {
                    for (name, shape) in ALL_SHAPES {
                        ui.selectable_value(&mut self.selected_shape, shape.clone(), *name);
                    }
                });
        });

        ui.add_space(4.0);

        // Image Content Section
        ui.horizontal(|ui| {
            ui.label("Image:");
            if let Some(path) = &self.flow_window_image_path {
                let filename = path.file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                ui.label(&filename);
            } else {
                ui.label("None");
            }
            if ui.button("Browse...").clicked() {
                self.open_image_picker_for_flow_window();
            }
            if self.flow_window_image_path.is_some() && ui.button("Clear").clicked() {
                self.flow_window_image_path = None;
            }
        });

        ui.add_space(4.0);

        // Window size
        ui.horizontal(|ui| {
            ui.label("Size:");
            ui.add(egui::Slider::new(&mut self.new_window_size, 30..=200).suffix("px"));
        });

        // Position
        ui.horizontal(|ui| {
            ui.label("Position X:");
            ui.add(egui::DragValue::new(&mut self.new_window_x).range(0.0..=2000.0));
            ui.label("Y:");
            ui.add(egui::DragValue::new(&mut self.new_window_y).range(0.0..=2000.0));
        });

        ui.add_space(8.0);

        // Create button
        if ui.button("Create Window").clicked() {
            self.create_window();
        }
    }

    /// Render effect-specific options
    fn render_effect_options(&mut self, ui: &mut Ui) {
        // Common options
        ui.horizontal(|ui| {
            ui.label("Intensity:");
            ui.add(egui::Slider::new(&mut self.effect_options.intensity, 0.0..=1.0));
        });

        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut self.effect_options.speed, 0.1..=3.0));
        });

        // Effect-specific options
        match self.selected_effect {
            PresetEffect::SilkRibbon => {
                ui.horizontal(|ui| {
                    ui.label("Ribbon Count:");
                    let mut count = self.effect_options.ribbon_count as i32;
                    if ui.add(egui::Slider::new(&mut count, 1..=6)).changed() {
                        self.effect_options.ribbon_count = count as usize;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Petal Amplitude:");
                    ui.add(egui::Slider::new(&mut self.effect_options.petal_amplitude, 5.0..=50.0));
                });
            }
            _ => {
                // Generic options for other effects
                ui.horizontal(|ui| {
                    ui.label("Edge Width:");
                    ui.add(egui::Slider::new(&mut self.effect_options.edge_width, 5.0..=30.0));
                });
            }
        }

        // Color presets
        ui.horizontal(|ui| {
            ui.label("Colors:");
            if ui.button("Cyan/Purple").clicked() {
                self.effect_options.particle_colors = vec![
                    [0.4, 0.8, 1.0, 1.0],
                    [0.8, 0.4, 1.0, 1.0],
                ];
            }
            if ui.button("Fire").clicked() {
                self.effect_options.particle_colors = vec![
                    [1.0, 0.3, 0.0, 1.0],
                    [1.0, 0.6, 0.0, 1.0],
                    [1.0, 0.9, 0.2, 1.0],
                ];
            }
            if ui.button("Rainbow").clicked() {
                self.effect_options.particle_colors = vec![
                    [1.0, 0.0, 0.0, 1.0],
                    [1.0, 0.5, 0.0, 1.0],
                    [1.0, 1.0, 0.0, 1.0],
                    [0.0, 1.0, 0.0, 1.0],
                    [0.0, 0.5, 1.0, 1.0],
                    [0.5, 0.0, 1.0, 1.0],
                ];
            }
        });
    }

    /// Render the manage existing windows section
    fn render_manage_section(&mut self, ui: &mut Ui) {
        ui.heading("Managed Windows");
        ui.add_space(8.0);

        let windows = self.registry.list();

        if windows.is_empty() {
            ui.label("No windows created yet.");
        } else {
            // Table of windows
            egui::Grid::new("window_grid")
                .num_columns(3)
                .spacing([20.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Effect");
                    ui.label("Actions");
                    ui.end_row();

                    for window in &windows {
                        ui.label(&window.name);
                        ui.label(format!("{:?}", window.effect.unwrap_or(PresetEffect::RotatingHalo)));

                        if ui.button("Close").clicked() {
                            let _ = self.command_sender.send(WindowCommand::Close { id: window.id });
                        }
                        ui.end_row();
                    }
                });
        }

        ui.add_space(8.0);

        // Close all button
        if !windows.is_empty() && ui.button("Close All Windows").clicked() {
            let _ = self.command_sender.send(WindowCommand::CloseAll);
        }
    }

    /// Create a new window with current settings
    fn create_window(&mut self) {
        // Load image content if path is selected
        let content = if let Some(path) = &self.flow_window_image_path {
            match Content::from_path_sized(path, self.new_window_size, self.new_window_size) {
                Ok(content) => Some(content),
                Err(e) => {
                    log::error!("Failed to load image: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let config = WindowConfig {
            id: None,
            title: Some(self.registry.generate_name()),
            position: Position::new(self.new_window_x as f64, self.new_window_y as f64),
            size: Size::new(self.new_window_size, self.new_window_size),
            effect_margin: 0.0, // Will be calculated by builder
            shape: self.selected_shape.clone(),
            draggable: true,
            resizable: false,
            click_through: false,
            level: super::config::WindowLevel::AlwaysOnTop,
            opacity: 1.0,
            icon: None,
            content,
            effect: None, // Effect is passed separately
            show_animation: None,
            hide_animation: None,
        };

        let _ = self.command_sender.send(WindowCommand::Create {
            config,
            effect: Some((self.selected_effect, self.effect_options.clone())),
        });

        // Move position for next window
        self.new_window_x += 60.0;
        if self.new_window_x > 1000.0 {
            self.new_window_x = 500.0;
            self.new_window_y += 60.0;
        }
    }

    /// Render the menu bar section
    fn render_menu_bar_section(&mut self, ui: &mut Ui) {
        ui.heading("Menu Bar Items");
        ui.add_space(8.0);

        // Create new menu bar item section
        ui.group(|ui| {
            ui.label("Create Menu Bar Item");
            ui.add_space(4.0);

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
            ui.horizontal(|ui| {
                ui.label("Icon:");
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
                if ui.button("Image...").clicked() {
                    self.open_image_picker_for_tray();
                }
            });

            // Show selected image or color preview
            ui.horizontal(|ui| {
                ui.label("Selected:");
                if let Some(path) = &self.tray_icon_image_path {
                    let filename = path.file_name()
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
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                    ui.painter().circle_filled(rect.center(), 8.0, color);
                }
            });

            ui.add_space(4.0);

            if ui.button("Add Menu Bar Item").clicked() {
                self.create_menu_bar_item();
            }
        });

        ui.add_space(8.0);

        // List existing menu bar items
        if !self.menu_bar_items.is_empty() {
            ui.label("Active Menu Bar Items:");
            ui.add_space(4.0);

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
                let _ = self.command_sender.send(WindowCommand::RemoveMenuBarItem { id });
            }
        } else {
            ui.label("No menu bar items created.");
        }
    }

    /// Create a new menu bar item
    fn create_menu_bar_item(&mut self) {
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

        let _ = self.command_sender.send(WindowCommand::AddMenuBarItem { item });
        self.menu_bar_items.push(id);

        // Update name for next item
        self.new_menu_bar_name = format!("App {}", self.menu_bar_counter);
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

        MenuBarIcon::Rgba {
            data,
            width: size,
            height: size,
        }
    }

    /// Open file picker for flow window image
    fn open_image_picker_for_flow_window(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "svg", "jpg", "jpeg"])
            .pick_file()
        {
            self.flow_window_image_path = Some(path);
        }
    }

    /// Open file picker for tray icon image
    fn open_image_picker_for_tray(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "svg", "jpg", "jpeg"])
            .pick_file()
        {
            self.tray_icon_image_path = Some(path);
        }
    }
}
