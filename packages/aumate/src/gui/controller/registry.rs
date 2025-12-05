//! Feature registry for dynamic tab management

use super::context::ControllerContext;
use super::feature::ControllerFeature;
use super::types::{TabId, TabInfo};
use crate::error::Result;

/// Registry that manages controller features and their tabs.
///
/// Features are registered at startup and can be dynamically enabled/disabled.
/// The registry handles tab switching, lifecycle callbacks, and rendering.
pub struct FeatureRegistry {
    features: Vec<Box<dyn ControllerFeature>>,
    active_tab: Option<TabId>,
}

impl FeatureRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self { features: Vec::new(), active_tab: None }
    }

    /// Register a feature. Features are automatically sorted by tab order.
    pub fn register(&mut self, feature: Box<dyn ControllerFeature>) {
        self.features.push(feature);
        // Sort by display order
        self.features.sort_by_key(|f| f.tab_info().order);
    }

    /// Get all available tabs (sorted by order)
    pub fn tabs(&self) -> Vec<TabInfo> {
        self.features.iter().map(|f| f.tab_info()).collect()
    }

    /// Get the currently active tab ID
    pub fn active_tab(&self) -> Option<&TabId> {
        self.active_tab.as_ref()
    }

    /// Set the active tab, handling activate/deactivate callbacks
    pub fn set_active(&mut self, tab_id: &TabId, ctx: &mut ControllerContext) {
        // Skip if already active
        if self.active_tab.as_ref() == Some(tab_id) {
            return;
        }

        // Deactivate previous tab
        if let Some(ref old_id) = self.active_tab {
            if let Some(feature) = self.features.iter_mut().find(|f| &f.tab_info().id == old_id) {
                feature.on_deactivate(ctx);
            }
        }

        // Activate new tab
        if let Some(feature) = self.features.iter_mut().find(|f| &f.tab_info().id == tab_id) {
            feature.on_activate(ctx);
        }

        self.active_tab = Some(tab_id.clone());
    }

    /// Set the first tab as active (if any)
    pub fn activate_first(&mut self, ctx: &mut ControllerContext) {
        if let Some(first_tab) = self.tabs().first().map(|t| t.id.clone()) {
            self.set_active(&first_tab, ctx);
        }
    }

    /// Get a mutable reference to a feature by tab ID
    pub fn get_mut(&mut self, tab_id: &TabId) -> Option<&mut Box<dyn ControllerFeature>> {
        self.features.iter_mut().find(|f| &f.tab_info().id == tab_id)
    }

    /// Get an immutable reference to a feature by tab ID
    pub fn get(&self, tab_id: &TabId) -> Option<&dyn ControllerFeature> {
        self.features.iter().find(|f| &f.tab_info().id == tab_id).map(|b| b.as_ref())
    }

    /// Update all features (for async callbacks).
    /// Called every frame.
    pub fn update_all(&mut self, ctx: &mut ControllerContext) {
        for feature in &mut self.features {
            feature.update(ctx);
        }
    }

    /// Render the active feature's UI
    pub fn render_active(&mut self, ui: &mut egui::Ui, ctx: &mut ControllerContext) {
        if let Some(ref tab_id) = self.active_tab.clone() {
            if let Some(feature) = self.get_mut(tab_id) {
                feature.render(ui, ctx);
            }
        }
    }

    /// Initialize all features
    pub fn initialize_all(&mut self, ctx: &mut ControllerContext) -> Result<()> {
        for feature in &mut self.features {
            feature.initialize(ctx)?;
        }
        Ok(())
    }

    /// Shutdown all features
    pub fn shutdown_all(&mut self) {
        for feature in &mut self.features {
            feature.shutdown();
        }
    }

    /// Get the number of registered features
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }
}

impl Default for FeatureRegistry {
    fn default() -> Self {
        Self::new()
    }
}
