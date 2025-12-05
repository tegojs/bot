//! Settings feature for controller configuration

use super::context::ControllerContext;
use super::feature::ControllerFeature;
use super::types::TabInfo;
use crate::error::Result;

/// Settings feature for controller configuration
pub struct SettingsFeature {
    // Currently no state - settings are managed through ControllerState
    // TODO: Move settings state here in future refactoring
}

impl SettingsFeature {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SettingsFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerFeature for SettingsFeature {
    fn id(&self) -> &'static str {
        "settings"
    }

    fn tab_info(&self) -> TabInfo {
        TabInfo::new("settings", "Settings", 100) // High order = last tab
    }

    fn render(&mut self, ui: &mut egui::Ui, _ctx: &mut ControllerContext) {
        ui.heading("Controller Settings");
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label("Settings will be migrated from the old controller.");
            ui.label("Background image, dock icon visibility, etc.");
        });

        // TODO: Migrate settings logic from old controller:
        // - Background image management
        // - macOS dock icon visibility
    }

    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        log::info!("Settings feature initialized");
        Ok(())
    }
}
