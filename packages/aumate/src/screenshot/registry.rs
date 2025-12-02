//! ActionRegistry - manages screenshot actions
//!
//! Handles registration, enabling/disabling, and execution of screen actions.

use std::collections::HashMap;

use egui::Pos2;

use super::action::{
    ActionContext, ActionInfo, ActionResult, DrawingContext, RenderContext, ScreenAction,
    ToolCategory,
};

/// Display order for actions in UI (controller settings, etc.)
/// This ensures consistent ordering across renders.
const DISPLAY_ORDER: &[&str] = &[
    // Drawing tools (Snipaste-like order)
    "rectangle",
    "ellipse",
    "polyline",
    "arrow",
    "annotate",
    "highlighter",
    "mosaic",
    "blur",
    "text",
    "sequence",
    "eraser",
    // Terminal actions
    "undo",
    "redo",
    "cancel",
    "save",
    "copy",
];

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

    /// Get info for all registered actions in consistent display order
    pub fn get_all(&self) -> Vec<ActionInfo> {
        // Return actions in DISPLAY_ORDER to ensure consistent UI ordering
        let mut result = Vec::new();
        for id in DISPLAY_ORDER {
            if let Some(action) = self.actions.get(*id) {
                result.push(action.info());
            }
        }
        // Add any actions not in DISPLAY_ORDER (shouldn't happen, but be safe)
        for (id, action) in &self.actions {
            if !DISPLAY_ORDER.contains(&id.as_str()) {
                result.push(action.info());
            }
        }
        result
    }

    /// Execute an action by ID
    ///
    /// For toggle tools (Drawing and Privacy categories), this handles
    /// mutual exclusion - activating one tool deactivates others in the same category.
    pub fn execute(&mut self, id: &str, ctx: &ActionContext) -> ActionResult {
        // Get the category before executing
        let category = self.actions.get(id).map(|a| a.category());

        // Execute the action
        let result = if let Some(action) = self.actions.get_mut(id) {
            action.on_click(ctx)
        } else {
            return ActionResult::Failure(format!("Action not found: {}", id));
        };

        // Handle mutual exclusion for toggle tools
        if let Some(cat) = category {
            if cat != ToolCategory::Action {
                // Check if the tool is now active
                let is_now_active = self.actions.get(id).is_some_and(|a| a.is_active());
                if is_now_active {
                    // Deactivate other tools in the same category
                    self.deactivate_category_except(cat, id);
                }
            }
        }

        result
    }

    /// Deactivate all tools in a category except the specified one
    fn deactivate_category_except(&mut self, category: ToolCategory, except_id: &str) {
        for (id, action) in self.actions.iter_mut() {
            if id != except_id && action.category() == category {
                action.set_active(false);
            }
        }
    }

    /// Get the currently active tool ID in a category (if any)
    pub fn get_active_tool(&self, category: ToolCategory) -> Option<&str> {
        for (id, action) in &self.actions {
            if action.category() == category && action.is_active() {
                return Some(id);
            }
        }
        None
    }

    /// Check if a specific tool is active
    pub fn is_tool_active(&self, id: &str) -> bool {
        self.actions.get(id).is_some_and(|a| a.is_active())
    }

    /// Get action IDs in the order they should be displayed
    pub fn enabled_ids(&self) -> &[String] {
        &self.enabled_ids
    }

    // ==================== Drawing Lifecycle Methods ====================

    /// Get the ID of the currently active drawing tool (if any)
    ///
    /// Checks both Drawing and Privacy categories.
    pub fn get_active_drawing_tool_id(&self) -> Option<String> {
        self.get_active_tool(ToolCategory::Drawing)
            .or_else(|| self.get_active_tool(ToolCategory::Privacy))
            .map(|s| s.to_string())
    }

    /// Forward draw start event to the active drawing tool
    ///
    /// Returns true if the event was handled by a drawing tool.
    pub fn on_draw_start(&mut self, pos: Pos2, ctx: &mut DrawingContext) -> bool {
        // Get active tool ID first (avoids borrow issues)
        let active_id = self.get_active_drawing_tool_id();

        if let Some(id) = active_id {
            if let Some(action) = self.actions.get_mut(&id) {
                if action.is_drawing_tool() {
                    action.on_draw_start(pos, ctx);
                    return true;
                }
            }
        }
        false
    }

    /// Forward draw move event to the active drawing tool
    ///
    /// Returns true if the event was handled by a drawing tool.
    pub fn on_draw_move(&mut self, pos: Pos2, ctx: &mut DrawingContext) -> bool {
        let active_id = self.get_active_drawing_tool_id();

        if let Some(id) = active_id {
            if let Some(action) = self.actions.get_mut(&id) {
                if action.is_drawing_tool() {
                    action.on_draw_move(pos, ctx);
                    return true;
                }
            }
        }
        false
    }

    /// Forward draw end event to the active drawing tool
    ///
    /// Returns true if the event was handled by a drawing tool.
    pub fn on_draw_end(&mut self, ctx: &mut DrawingContext) -> bool {
        let active_id = self.get_active_drawing_tool_id();

        if let Some(id) = active_id {
            if let Some(action) = self.actions.get_mut(&id) {
                if action.is_drawing_tool() {
                    action.on_draw_end(ctx);
                    return true;
                }
            }
        }
        false
    }

    /// Check if there's an active drawing tool
    pub fn has_active_drawing_tool(&self) -> bool {
        self.get_active_drawing_tool_id().is_some()
    }

    // ==================== Rendering ====================

    /// Render all annotations by calling each action's render method
    ///
    /// The rendering order is determined by the annotation type to ensure
    /// proper layering (e.g., highlighters below shapes, markers on top).
    /// Order: highlighters -> shapes -> strokes -> arrows -> polylines -> markers
    pub fn render_all_annotations(&self, ctx: &RenderContext) {
        // Each action renders its own annotation type
        // The order is enforced by iterating in a specific order
        const RENDER_ORDER: &[&str] = &[
            "highlighter", // Bottom layer (semi-transparent)
            "rectangle",   // Shapes
            "ellipse",
            "annotate", // Freehand strokes
            "arrow",
            "polyline",
            "sequence", // Top layer (markers with numbers)
        ];

        for action_id in RENDER_ORDER {
            if let Some(action) = self.actions.get(*action_id) {
                action.render_annotations(ctx);
            }
        }
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a registry with all default actions registered
pub fn create_default_registry() -> ActionRegistry {
    use super::actions::{
        AnnotateAction, ArrowAction, BlurAction, CancelAction, CopyAction, EllipseAction,
        EraserAction, HighlighterAction, MosaicAction, PolylineAction, RectangleAction, RedoAction,
        SaveAction, SequenceAction, TextAction, UndoAction,
    };

    let mut registry = ActionRegistry::new();

    // Drawing tools (mutually exclusive)
    registry.register(Box::new(RectangleAction::new()));
    registry.register(Box::new(EllipseAction::new()));
    registry.register(Box::new(PolylineAction::new()));
    registry.register(Box::new(ArrowAction::new()));
    registry.register(Box::new(AnnotateAction::new()));
    registry.register(Box::new(HighlighterAction::new()));
    registry.register(Box::new(SequenceAction::new()));
    registry.register(Box::new(TextAction::new()));
    registry.register(Box::new(EraserAction::new()));

    // Privacy tools (mutually exclusive)
    registry.register(Box::new(MosaicAction::new()));
    registry.register(Box::new(BlurAction::new()));

    // Edit actions
    registry.register(Box::new(UndoAction::new()));
    registry.register(Box::new(RedoAction::new()));

    // Terminal actions
    registry.register(Box::new(CancelAction::new()));
    registry.register(Box::new(SaveAction::new()));
    registry.register(Box::new(CopyAction::new()));

    registry
}
