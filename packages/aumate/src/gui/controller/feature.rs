//! Controller feature trait definitions

use super::context::ControllerContext;
use super::types::TabInfo;
use crate::error::Result;
use std::path::PathBuf;

/// Trait for controller features that can be registered and rendered as tabs.
///
/// Each feature implements this trait to integrate with the controller's
/// dynamic tab system. Features are responsible for:
/// - Providing tab information for navigation
/// - Rendering their UI section
/// - Handling lifecycle events (activate/deactivate)
/// - Processing async callbacks in update()
pub trait ControllerFeature: Send {
    /// Unique feature identifier
    fn id(&self) -> &'static str;

    /// Tab information for navigation display
    fn tab_info(&self) -> TabInfo;

    /// Render the feature's UI section.
    /// Called each frame when this feature's tab is active.
    fn render(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext);

    /// Called when this tab becomes active.
    /// Use for lazy initialization or refresh.
    fn on_activate(&mut self, _ctx: &mut ControllerContext) {}

    /// Called when switching away from this tab.
    /// Use for cleanup or saving state.
    fn on_deactivate(&mut self, _ctx: &mut ControllerContext) {}

    /// Called each frame for all features (even inactive ones).
    /// Use for processing async task callbacks.
    fn update(&mut self, _ctx: &mut ControllerContext) {}

    /// Initialize the feature. Called once at startup.
    fn initialize(&mut self, _ctx: &mut ControllerContext) -> Result<()> {
        Ok(())
    }

    /// Cleanup on application shutdown.
    fn shutdown(&mut self) {}
}

/// Trait for features with persistent configuration state.
///
/// Provides a uniform interface for saving and loading feature configs,
/// typically to files in ~/.aumate/
pub trait FeatureState: Sized {
    /// Load state from the default config file
    fn load() -> Result<Self>;

    /// Save state to the default config file
    fn save(&self) -> Result<()>;

    /// Get the config file path
    fn config_path() -> PathBuf;

    /// Load with fallback to default on error
    fn load_or_default() -> Self
    where
        Self: Default,
    {
        Self::load().unwrap_or_default()
    }
}
