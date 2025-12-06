//! Window Manager feature for the controller
//!
//! Provides window positioning functionality with a command palette UI
//! triggered by a global hotkey.

use crate::error::Result;
use crate::gui::controller::{ControllerContext, ControllerFeature, TabInfo};
use crate::gui::window::WindowCommand;
use crate::window_manager::{
    Modifier, WINDOW_ACTIONS, WindowManagerConfig, WindowManagerHotkeyManager,
};

#[cfg(target_os = "macos")]
use crate::click_helper::accessibility::{
    is_input_monitoring_enabled, open_input_monitoring_settings,
};
#[cfg(target_os = "macos")]
use crate::window_manager::is_accessibility_trusted;

/// Window Manager feature for window positioning
pub struct WindowManagerFeature {
    /// Window Manager configuration
    config: WindowManagerConfig,
    /// Whether accessibility permission is granted
    #[cfg(target_os = "macos")]
    accessibility_trusted: bool,
    /// Hotkey manager for global hotkeys
    hotkey_manager: Option<WindowManagerHotkeyManager>,
    /// Whether hotkey is currently enabled/running
    hotkey_enabled: bool,
}

impl WindowManagerFeature {
    pub fn new() -> Self {
        Self {
            config: WindowManagerConfig::load().unwrap_or_default(),
            #[cfg(target_os = "macos")]
            accessibility_trusted: false,
            hotkey_manager: None,
            hotkey_enabled: false,
        }
    }

    /// Initialize Window Manager hotkey manager with the command sender
    fn init_hotkey(&mut self, command_sender: &crate::gui::window::CommandSender) {
        let config = self.config.hotkey.clone();
        let sender = command_sender.clone();

        let mut manager = WindowManagerHotkeyManager::new();
        manager.set_config(config);
        manager.set_callback(move || {
            log::info!("Window Manager hotkey callback triggered, sending command");
            if let Err(e) = sender.send(WindowCommand::StartWindowManagerPalette) {
                log::error!("Failed to send Window Manager command: {}", e);
            }
        });

        if let Err(e) = manager.start() {
            log::error!("Failed to start Window Manager hotkey manager: {}", e);
        } else {
            log::info!("Window Manager hotkey manager started");
            self.hotkey_manager = Some(manager);
        }
    }

    /// Toggle a modifier in the config
    fn toggle_modifier(&mut self, modifier: Modifier) {
        if let Some(pos) = self.config.hotkey.modifiers.iter().position(|m| *m == modifier) {
            self.config.hotkey.modifiers.remove(pos);
        } else {
            self.config.hotkey.modifiers.push(modifier);
        }
    }
}

impl Default for WindowManagerFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for WindowManagerFeature {
    fn id(&self) -> &'static str {
        "window_manager"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("window_manager", "Window Manager", 65) // After Click Helper (60)
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        // Check accessibility permission status
        #[cfg(target_os = "macos")]
        {
            self.accessibility_trusted = is_accessibility_trusted();
        }

        ui.heading("Window Manager");
        ui.add_space(8.0);

        // Status section
        ui.group(|ui| {
            #[cfg(target_os = "macos")]
            {
                ui.horizontal(|ui| {
                    ui.label("Accessibility Permission:");
                    if self.accessibility_trusted {
                        ui.label(egui::RichText::new("Granted").color(egui::Color32::GREEN));
                    } else {
                        ui.label(egui::RichText::new("Not Granted").color(egui::Color32::RED));
                        if ui.button("Request").clicked() {
                            // Request accessibility permission
                            use crate::click_helper::ClickHelperMode;
                            let mode = ClickHelperMode::new();
                            mode.request_accessibility_permission();
                        }
                    }
                });

                ui.add_space(4.0);

                // Input Monitoring permission status (required for global hotkeys)
                let input_monitoring_enabled = is_input_monitoring_enabled();
                ui.horizontal(|ui| {
                    ui.label("Input Monitoring:");
                    if input_monitoring_enabled {
                        ui.label(egui::RichText::new("Granted").color(egui::Color32::GREEN));
                    } else {
                        ui.label(egui::RichText::new("Not Granted").color(egui::Color32::RED));
                        if ui.button("Open Settings").clicked() {
                            open_input_monitoring_settings();
                        }
                    }
                });
            }

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Hotkey:");
                ui.label(self.config.hotkey.display_string());
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Global Hotkey:");
                let is_running = self.hotkey_manager.as_ref().is_some_and(|m| m.is_running());

                // Enabled checkbox
                let mut enabled = self.config.hotkey_enabled;
                #[cfg(target_os = "macos")]
                let input_monitoring_enabled = is_input_monitoring_enabled();
                #[cfg(not(target_os = "macos"))]
                let input_monitoring_enabled = true;

                let checkbox = ui.add_enabled(
                    input_monitoring_enabled || enabled,
                    egui::Checkbox::new(&mut enabled, "Enabled"),
                );

                if checkbox.changed() {
                    self.config.hotkey_enabled = enabled;
                    if let Err(e) = self.config.save() {
                        log::error!("Failed to save Window Manager config: {}", e);
                    }

                    if enabled && !is_running {
                        self.init_hotkey(ctx.command_sender);
                        self.hotkey_enabled = true;
                    } else if !enabled && is_running {
                        if let Some(ref mut manager) = self.hotkey_manager {
                            manager.stop();
                        }
                        self.hotkey_manager = None;
                        self.hotkey_enabled = false;
                    }
                }

                // Status indicator
                if is_running {
                    ui.label(egui::RichText::new("Running").color(egui::Color32::GREEN));
                } else {
                    ui.label(egui::RichText::new("Stopped").color(egui::Color32::GRAY));
                }
            });

            // Warning about Input Monitoring
            #[cfg(target_os = "macos")]
            if !is_input_monitoring_enabled() {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(
                        "Warning: Input Monitoring permission required for global hotkeys.\n\
                         Click 'Open Settings', add this app, then RESTART the app.",
                    )
                    .small()
                    .color(egui::Color32::from_rgb(255, 180, 100)),
                );
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // Test button to manually trigger Window Manager
            if ui.button("Test Command Palette (Manual Trigger)").clicked() {
                log::info!("Manual Window Manager trigger from UI");
                if let Err(e) = ctx.command_sender.send(WindowCommand::StartWindowManagerPalette) {
                    log::error!("Failed to send Window Manager command: {}", e);
                }
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Settings section
        ui.heading("Hotkey Settings");
        ui.add_space(8.0);

        let mut config_changed = false;

        ui.group(|ui| {
            // Hotkey configuration
            ui.horizontal(|ui| {
                ui.label("Activation Key:");
                let mut key = self.config.hotkey.key.clone();
                if ui.text_edit_singleline(&mut key).changed() {
                    self.config.hotkey.key = key;
                    config_changed = true;
                }
            });

            ui.add_space(8.0);

            // Modifier checkboxes
            ui.label("Modifiers:");
            ui.horizontal(|ui| {
                let mut has_ctrl = self.config.hotkey.modifiers.contains(&Modifier::Ctrl);
                if ui.checkbox(&mut has_ctrl, "Ctrl").changed() {
                    self.toggle_modifier(Modifier::Ctrl);
                    config_changed = true;
                }

                let mut has_alt = self.config.hotkey.modifiers.contains(&Modifier::Alt);
                if ui.checkbox(&mut has_alt, "Alt").changed() {
                    self.toggle_modifier(Modifier::Alt);
                    config_changed = true;
                }

                let mut has_shift = self.config.hotkey.modifiers.contains(&Modifier::Shift);
                if ui.checkbox(&mut has_shift, "Shift").changed() {
                    self.toggle_modifier(Modifier::Shift);
                    config_changed = true;
                }

                let mut has_meta = self.config.hotkey.modifiers.contains(&Modifier::Meta);
                if ui.checkbox(&mut has_meta, "Cmd").changed() {
                    self.toggle_modifier(Modifier::Meta);
                    config_changed = true;
                }
            });
        });

        // Save config if changed
        if config_changed {
            if let Err(e) = self.config.save() {
                log::error!("Failed to save Window Manager config: {}", e);
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Available Actions
        ui.heading("Available Actions");
        ui.add_space(8.0);

        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            ui.group(|ui| {
                for action in WINDOW_ACTIONS.iter() {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(action.name).strong());
                        ui.label(
                            egui::RichText::new(format!("[{}]", action.category))
                                .small()
                                .color(egui::Color32::GRAY),
                        );
                    });
                }
            });
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Instructions
        ui.heading("Usage");
        ui.add_space(8.0);
        ui.label(format!(
            "Press {} to open the command palette.",
            self.config.hotkey.display_string()
        ));
        ui.label("Type to filter actions, use arrow keys to navigate.");
        ui.label("Press Enter to execute, Escape to cancel.");
    }

    fn initialize(&mut self, ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Window Manager feature initialized");

        // Auto-start hotkey if enabled in config
        if self.config.hotkey_enabled {
            self.init_hotkey(ctx.command_sender);
            self.hotkey_enabled = true;
        }

        Ok(())
    }

    fn shutdown(&mut self) {
        // Stop hotkey manager if running
        if let Some(ref mut manager) = self.hotkey_manager {
            manager.stop();
        }
        log::info!("Window Manager feature shutdown");
    }
}
