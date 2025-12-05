//! Click Helper feature for the controller
//!
//! Provides keyboard-driven UI element clicking (EasyMotion/Vimium-style)
//! using accessibility APIs and global hotkeys.

use crate::click_helper::{
    ClickHelperConfig, ClickHelperHotkeyManager, ClickHelperMode, Modifier as ClickHelperModifier,
    accessibility::{is_input_monitoring_enabled, open_input_monitoring_settings},
};
use crate::error::Result;
use crate::gui::controller::{ControllerContext, ControllerFeature, TabInfo};
use crate::gui::window::WindowCommand;

/// Click Helper feature for keyboard-driven UI element clicking
pub struct ClickHelperFeature {
    /// Click helper configuration
    click_helper_config: ClickHelperConfig,
    /// Whether accessibility permission is granted
    click_helper_trusted: bool,
    /// Hotkey manager for global hotkeys
    click_helper_hotkey_manager: Option<ClickHelperHotkeyManager>,
    /// Whether hotkey is currently enabled/running
    click_helper_hotkey_enabled: bool,
}

impl ClickHelperFeature {
    pub fn new() -> Self {
        Self {
            click_helper_config: ClickHelperConfig::load().unwrap_or_default(),
            click_helper_trusted: false,
            click_helper_hotkey_manager: None,
            click_helper_hotkey_enabled: false,
        }
    }

    /// Initialize Click Helper hotkey manager with the command sender
    fn init_click_helper_hotkey(&mut self, command_sender: &crate::gui::window::CommandSender) {
        let config = self.click_helper_config.hotkey.clone();
        let sender = command_sender.clone();

        let mut manager = ClickHelperHotkeyManager::new();
        manager.set_config(config);
        manager.set_callback(move || {
            log::info!("Click Helper hotkey callback triggered, sending command");
            if let Err(e) = sender.send(WindowCommand::StartClickHelperMode) {
                log::error!("Failed to send Click Helper command: {}", e);
            }
        });

        if let Err(e) = manager.start() {
            log::error!("Failed to start Click Helper hotkey manager: {}", e);
        } else {
            log::info!("Click Helper hotkey manager started");
            self.click_helper_hotkey_manager = Some(manager);
        }
    }

    /// Toggle a modifier in the click helper config
    fn toggle_click_helper_modifier(&mut self, modifier: ClickHelperModifier) {
        if let Some(pos) =
            self.click_helper_config.hotkey.modifiers.iter().position(|m| *m == modifier)
        {
            self.click_helper_config.hotkey.modifiers.remove(pos);
        } else {
            self.click_helper_config.hotkey.modifiers.push(modifier);
        }
    }
}

impl Default for ClickHelperFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for ClickHelperFeature {
    fn id(&self) -> &'static str {
        "click_helper"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("click_helper", "Click Helper", 60) // After OCR (50)
    }

    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        // Check accessibility permission status
        let mode = ClickHelperMode::new();
        self.click_helper_trusted = mode.is_accessibility_trusted();

        ui.heading("Click Helper");
        ui.add_space(8.0);

        // Status section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Accessibility Permission:");
                if self.click_helper_trusted {
                    ui.label(egui::RichText::new("Granted").color(egui::Color32::GREEN));
                } else {
                    ui.label(egui::RichText::new("Not Granted").color(egui::Color32::RED));
                    if ui.button("Request").clicked() {
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

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Hotkey:");
                ui.label(self.click_helper_config.hotkey.display_string());
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Global Hotkey:");
                let is_running =
                    self.click_helper_hotkey_manager.as_ref().is_some_and(|m| m.is_running());

                // Enabled checkbox
                let mut enabled = self.click_helper_config.hotkey_enabled;
                let checkbox = ui.add_enabled(
                    input_monitoring_enabled || enabled,
                    egui::Checkbox::new(&mut enabled, "Enabled"),
                );

                if checkbox.changed() {
                    self.click_helper_config.hotkey_enabled = enabled;
                    if let Err(e) = self.click_helper_config.save() {
                        log::error!("Failed to save Click Helper config: {}", e);
                    }

                    if enabled && !is_running {
                        self.init_click_helper_hotkey(ctx.command_sender);
                        self.click_helper_hotkey_enabled = true;
                    } else if !enabled && is_running {
                        if let Some(ref mut manager) = self.click_helper_hotkey_manager {
                            manager.stop();
                        }
                        self.click_helper_hotkey_manager = None;
                        self.click_helper_hotkey_enabled = false;
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
            if !input_monitoring_enabled {
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

            // Test button to manually trigger Click Helper
            if ui.button("Test Click Helper (Manual Trigger)").clicked() {
                log::info!("Manual Click Helper trigger from UI");
                if let Err(e) = ctx.command_sender.send(WindowCommand::StartClickHelperMode) {
                    log::error!("Failed to send Click Helper command: {}", e);
                }
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Settings section
        ui.heading("Settings");
        ui.add_space(8.0);

        let mut config_changed = false;

        ui.group(|ui| {
            // Hotkey configuration
            ui.horizontal(|ui| {
                ui.label("Activation Key:");
                let mut key = self.click_helper_config.hotkey.key.clone();
                if ui.text_edit_singleline(&mut key).changed() {
                    self.click_helper_config.hotkey.key = key;
                    config_changed = true;
                }
            });

            ui.add_space(8.0);

            // Modifier checkboxes
            ui.label("Modifiers:");
            ui.horizontal(|ui| {
                let mut has_ctrl =
                    self.click_helper_config.hotkey.modifiers.contains(&ClickHelperModifier::Ctrl);
                if ui.checkbox(&mut has_ctrl, "Ctrl").changed() {
                    self.toggle_click_helper_modifier(ClickHelperModifier::Ctrl);
                    config_changed = true;
                }

                let mut has_alt =
                    self.click_helper_config.hotkey.modifiers.contains(&ClickHelperModifier::Alt);
                if ui.checkbox(&mut has_alt, "Alt").changed() {
                    self.toggle_click_helper_modifier(ClickHelperModifier::Alt);
                    config_changed = true;
                }

                let mut has_shift =
                    self.click_helper_config.hotkey.modifiers.contains(&ClickHelperModifier::Shift);
                if ui.checkbox(&mut has_shift, "Shift").changed() {
                    self.toggle_click_helper_modifier(ClickHelperModifier::Shift);
                    config_changed = true;
                }

                let mut has_meta =
                    self.click_helper_config.hotkey.modifiers.contains(&ClickHelperModifier::Meta);
                if ui.checkbox(&mut has_meta, "Cmd").changed() {
                    self.toggle_click_helper_modifier(ClickHelperModifier::Meta);
                    config_changed = true;
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Hint characters configuration
            ui.label("Hint Characters:");
            ui.horizontal(|ui| {
                ui.label("Tier 1 (groups):");
                if ui.text_edit_singleline(&mut self.click_helper_config.tier1_chars).changed() {
                    config_changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Tier 2 (selection):");
                if ui.text_edit_singleline(&mut self.click_helper_config.tier2_chars).changed() {
                    config_changed = true;
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Appearance settings
            ui.label("Appearance:");

            ui.horizontal(|ui| {
                ui.label("Font Size:");
                if ui
                    .add(egui::Slider::new(
                        &mut self.click_helper_config.hint_font_size,
                        10.0..=24.0,
                    ))
                    .changed()
                {
                    config_changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Overlay Opacity:");
                if ui
                    .add(egui::Slider::new(&mut self.click_helper_config.overlay_opacity, 50..=200))
                    .changed()
                {
                    config_changed = true;
                }
            });

            // Color pickers
            ui.horizontal(|ui| {
                ui.label("Hint Background:");
                let mut color = egui::Color32::from_rgba_unmultiplied(
                    self.click_helper_config.hint_bg_color[0],
                    self.click_helper_config.hint_bg_color[1],
                    self.click_helper_config.hint_bg_color[2],
                    self.click_helper_config.hint_bg_color[3],
                );
                if ui.color_edit_button_srgba(&mut color).changed() {
                    self.click_helper_config.hint_bg_color =
                        [color.r(), color.g(), color.b(), color.a()];
                    config_changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Hint Text:");
                let mut color = egui::Color32::from_rgba_unmultiplied(
                    self.click_helper_config.hint_fg_color[0],
                    self.click_helper_config.hint_fg_color[1],
                    self.click_helper_config.hint_fg_color[2],
                    self.click_helper_config.hint_fg_color[3],
                );
                if ui.color_edit_button_srgba(&mut color).changed() {
                    self.click_helper_config.hint_fg_color =
                        [color.r(), color.g(), color.b(), color.a()];
                    config_changed = true;
                }
            });
        });

        // Save config if changed
        if config_changed {
            if let Err(e) = self.click_helper_config.save() {
                log::error!("Failed to save Click Helper config: {}", e);
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Instructions
        ui.heading("Usage");
        ui.add_space(8.0);
        ui.label(format!(
            "Press {} to activate Click Helper mode.",
            self.click_helper_config.hotkey.display_string()
        ));
        ui.label("Type hint characters to click elements.");
        ui.label("Press ESC or Backspace to cancel.");
    }

    fn initialize(&mut self, ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Click Helper feature initialized");

        // Auto-start hotkey if enabled in config
        if self.click_helper_config.hotkey_enabled {
            self.init_click_helper_hotkey(ctx.command_sender);
            self.click_helper_hotkey_enabled = true;
        }

        Ok(())
    }

    fn shutdown(&mut self) {
        // Stop hotkey manager if running
        if let Some(ref mut manager) = self.click_helper_hotkey_manager {
            manager.stop();
        }
        log::info!("Click Helper feature shutdown");
    }
}
