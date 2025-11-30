//! ActionRegistry - manages screenshot actions
//!
//! Handles registration, enabling/disabling, and execution of screen actions.

use std::collections::HashMap;

use super::action::{ActionContext, ActionInfo, ActionResult, ScreenAction};

/// Registry for managing screen actions
pub struct ActionRegistry {
    /// All registered actions
    actions: HashMap<String, Box<dyn ScreenAction>>,
    /// IDs of enabled actions (in display order)
    enabled_ids: Vec<String>,
}

impl ActionRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self { actions: HashMap::new(), enabled_ids: Vec::new() }
    }

    /// Register an action
    ///
    /// The action is registered but not enabled by default.
    pub fn register(&mut self, action: Box<dyn ScreenAction>) {
        let id = action.id().to_string();
        self.actions.insert(id, action);
    }

    /// Enable an action by ID
    ///
    /// Enabled actions are shown in the toolbar.
    pub fn enable(&mut self, id: &str) {
        if self.actions.contains_key(id) && !self.enabled_ids.contains(&id.to_string()) {
            self.enabled_ids.push(id.to_string());
        }
    }

    /// Disable an action by ID
    pub fn disable(&mut self, id: &str) {
        self.enabled_ids.retain(|i| i != id);
    }

    /// Enable multiple actions at once
    pub fn enable_all(&mut self, ids: &[String]) {
        for id in ids {
            self.enable(id);
        }
    }

    /// Check if an action is enabled
    pub fn is_enabled(&self, id: &str) -> bool {
        self.enabled_ids.contains(&id.to_string())
    }

    /// Get info for all enabled actions (in order)
    pub fn get_enabled(&self) -> Vec<ActionInfo> {
        self.enabled_ids.iter().filter_map(|id| self.actions.get(id).map(|a| a.info())).collect()
    }

    /// Get info for all registered actions
    pub fn get_all(&self) -> Vec<ActionInfo> {
        self.actions.values().map(|a| a.info()).collect()
    }

    /// Execute an action by ID
    pub fn execute(&mut self, id: &str, ctx: &ActionContext) -> ActionResult {
        if let Some(action) = self.actions.get_mut(id) {
            action.on_click(ctx)
        } else {
            ActionResult::Failure(format!("Action not found: {}", id))
        }
    }

    /// Get action IDs in the order they should be displayed
    pub fn enabled_ids(&self) -> &[String] {
        &self.enabled_ids
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a registry with all default actions registered
pub fn create_default_registry() -> ActionRegistry {
    use super::actions::{AnnotateAction, CancelAction, CopyAction, SaveAction, TextAction};

    let mut registry = ActionRegistry::new();

    // Register all built-in actions
    registry.register(Box::new(SaveAction::new()));
    registry.register(Box::new(CopyAction::new()));
    registry.register(Box::new(AnnotateAction::new()));
    registry.register(Box::new(TextAction::new()));
    registry.register(Box::new(CancelAction::new()));

    registry
}
