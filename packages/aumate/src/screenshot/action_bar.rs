//! Snipaste-style action bar for screenshot tools
//!
//! This module implements a two-row toolbar design:
//! - Row 1: Main toolbar with drawing tools, edit tools, and action buttons
//! - Row 2: Options panel that changes based on the selected tool

use super::action::ActionInfo;
use super::icons::{icon_render_size, IconCache, ICON_SIZE};
use egui::{Color32, Pos2, Rect, Ui, Vec2};

// ============================================================================
// Style Constants (matching Snipaste)
// ============================================================================

/// Button size (square)
const BUTTON_SIZE: f32 = 28.0;
/// Padding inside buttons
#[allow(dead_code)]
const BUTTON_PADDING: f32 = 4.0;
/// Spacing between buttons
const BUTTON_SPACING: f32 = 2.0;
/// Padding around the toolbar
const BAR_PADDING: f32 = 6.0;
/// Corner radius for toolbar background
const BAR_CORNER_RADIUS: f32 = 6.0;
/// Separator line width
const SEPARATOR_WIDTH: f32 = 1.0;
/// Separator vertical margin
const SEPARATOR_MARGIN: f32 = 6.0;
/// Drag handle width
const DRAG_HANDLE_WIDTH: f32 = 16.0;
/// Gap between rows
#[allow(dead_code)]
const ROW_GAP: f32 = 4.0;
/// Margin from selection edge
const TOOLBAR_MARGIN: f32 = 8.0;

// Colors
const BAR_BG_COLOR: Color32 = Color32::from_rgba_premultiplied(35, 35, 35, 245);
const BUTTON_DEFAULT_COLOR: Color32 = Color32::TRANSPARENT;
const BUTTON_HOVER_COLOR: Color32 = Color32::from_rgba_premultiplied(60, 60, 60, 255);
const BUTTON_ACTIVE_COLOR: Color32 = Color32::from_rgb(0, 120, 215);
const SEPARATOR_COLOR: Color32 = Color32::from_gray(60);
const ICON_COLOR: Color32 = Color32::WHITE;
const ICON_DISABLED_COLOR: Color32 = Color32::from_gray(100);

// ============================================================================
// Tool Groups (matching Snipaste layout)
// ============================================================================

/// Drawing tools (first group)
pub const DRAWING_TOOLS: &[&str] = &[
    "rectangle",
    "ellipse",
    "polyline",
    "arrow",
    "annotate",
    "highlighter",
    "mosaic",
    "text",
    "sequence",
];

/// Edit tools (second group, after separator)
pub const EDIT_TOOLS: &[&str] = &["undo", "redo"];

/// Action tools (third group, after separator)
pub const ACTION_TOOLS: &[&str] = &["cancel", "save", "copy"];

// ============================================================================
// ToolButton
// ============================================================================

/// A single tool button in the action bar
#[derive(Clone)]
pub struct ToolButton {
    /// Tool ID (matches action ID)
    pub id: String,
    /// Tooltip text
    pub tooltip: String,
    /// Whether this is a toggle button (stays active when clicked)
    pub is_toggle: bool,
    /// Button bounds (computed during layout)
    pub bounds: Rect,
}

impl ToolButton {
    pub fn new(id: &str, tooltip: &str, is_toggle: bool) -> Self {
        Self {
            id: id.to_string(),
            tooltip: tooltip.to_string(),
            is_toggle,
            bounds: Rect::NOTHING,
        }
    }
}

// ============================================================================
// ActionBar
// ============================================================================

/// Snipaste-style action bar with drag support and grouped tools
pub struct ActionBar {
    /// Current position (top-left corner)
    position: Pos2,
    /// Drag handle bounds
    drag_handle_bounds: Rect,
    /// Drawing tool buttons
    drawing_buttons: Vec<ToolButton>,
    /// Edit tool buttons (undo/redo)
    edit_buttons: Vec<ToolButton>,
    /// Action buttons (cancel/save/copy)
    action_buttons: Vec<ToolButton>,
    /// Total size of the main bar
    main_bar_size: Vec2,
    /// Options panel (shown below main bar when a tool is active)
    #[allow(dead_code)]
    options_panel_visible: bool,
    /// Icon texture cache
    icon_cache: IconCache,
    /// Is currently being dragged
    is_dragging: bool,
    /// Drag offset from position when drag started
    drag_offset: Vec2,
    /// Currently hovered button ID
    hovered_button: Option<String>,
    /// Scale factor for high-DPI rendering
    scale_factor: f32,
}

impl ActionBar {
    /// Create a new action bar
    pub fn new(actions: &[ActionInfo], selection_bounds: (Pos2, Pos2), screen_size: Vec2, scale_factor: f32) -> Self {
        // Create drawing buttons
        let mut drawing_buttons: Vec<ToolButton> = DRAWING_TOOLS
            .iter()
            .filter(|id| actions.iter().any(|a| a.id == **id))
            .map(|id| {
                let tooltip = actions
                    .iter()
                    .find(|a| a.id == *id)
                    .map(|a| a.name.clone())
                    .unwrap_or_default();
                ToolButton::new(id, &tooltip, true)
            })
            .collect();

        // Create edit buttons
        let edit_buttons: Vec<ToolButton> = EDIT_TOOLS
            .iter()
            .map(|id| ToolButton::new(id, id, false))
            .collect();

        // Create action buttons
        let mut action_buttons: Vec<ToolButton> = ACTION_TOOLS
            .iter()
            .filter(|id| actions.iter().any(|a| a.id == **id))
            .map(|id| {
                let tooltip = actions
                    .iter()
                    .find(|a| a.id == *id)
                    .map(|a| a.name.clone())
                    .unwrap_or_default();
                ToolButton::new(id, &tooltip, false)
            })
            .collect();

        // Add any actions not in predefined groups
        for action in actions {
            let id = &action.id;
            if !DRAWING_TOOLS.contains(&id.as_str())
                && !EDIT_TOOLS.contains(&id.as_str())
                && !ACTION_TOOLS.contains(&id.as_str())
            {
                // Determine if it's a toggle tool based on category
                let is_toggle = matches!(
                    action.category,
                    super::action::ToolCategory::Drawing | super::action::ToolCategory::Privacy
                );
                if is_toggle {
                    drawing_buttons.push(ToolButton::new(id, &action.name, true));
                } else {
                    action_buttons.push(ToolButton::new(id, &action.name, false));
                }
            }
        }

        // Calculate main bar dimensions
        let drawing_width = drawing_buttons.len() as f32 * BUTTON_SIZE
            + (drawing_buttons.len().saturating_sub(1)) as f32 * BUTTON_SPACING;
        let edit_width = edit_buttons.len() as f32 * BUTTON_SIZE
            + (edit_buttons.len().saturating_sub(1)) as f32 * BUTTON_SPACING;
        let action_width = action_buttons.len() as f32 * BUTTON_SIZE
            + (action_buttons.len().saturating_sub(1)) as f32 * BUTTON_SPACING;

        // Add separators between groups
        let separator_count = 2; // Between drawing/edit and edit/action
        let separators_width = separator_count as f32 * (SEPARATOR_WIDTH + 2.0 * SEPARATOR_MARGIN);

        let total_width = DRAG_HANDLE_WIDTH
            + BAR_PADDING
            + drawing_width
            + separators_width
            + edit_width
            + action_width
            + BAR_PADDING;

        let bar_height = BUTTON_SIZE + 2.0 * BAR_PADDING;
        let main_bar_size = Vec2::new(total_width, bar_height);

        // Position toolbar centered below selection
        let (min_pos, max_pos) = selection_bounds;
        let selection_center_x = (min_pos.x + max_pos.x) / 2.0;
        let mut toolbar_x = selection_center_x - total_width / 2.0;

        // Debug logging
        log::debug!(
            "ActionBar position calc: selection=({:.0},{:.0})-({:.0},{:.0}), screen=({:.0},{:.0}), bar_height={:.0}",
            min_pos.x, min_pos.y, max_pos.x, max_pos.y,
            screen_size.x, screen_size.y, bar_height
        );

        // Position below selection with margin (default)
        // Calculate space available below and above selection
        let space_below = screen_size.y - max_pos.y;
        let space_above = min_pos.y;
        let selection_height = max_pos.y - min_pos.y;
        let required_space = bar_height + TOOLBAR_MARGIN;

        log::debug!(
            "ActionBar: space_below={:.0}, space_above={:.0}, selection_height={:.0}, required={:.0}",
            space_below, space_above, selection_height, required_space
        );

        // Priority: 1. Below selection, 2. Inside at bottom, 3. Above selection
        let toolbar_y = if space_below >= required_space {
            // Place below selection (outside) - DEFAULT
            max_pos.y + TOOLBAR_MARGIN
        } else if selection_height >= required_space * 2.0 {
            // Place inside selection at bottom (with margin from bottom edge)
            max_pos.y - bar_height - TOOLBAR_MARGIN
        } else if space_above >= required_space {
            // Place above selection (outside)
            min_pos.y - bar_height - TOOLBAR_MARGIN
        } else {
            // Fallback: place at bottom of screen
            screen_size.y - bar_height - TOOLBAR_MARGIN
        };

        // Keep on screen horizontally
        toolbar_x = toolbar_x.max(TOOLBAR_MARGIN);
        if screen_size.x > total_width + TOOLBAR_MARGIN * 2.0 {
            toolbar_x = toolbar_x.min(screen_size.x - total_width - TOOLBAR_MARGIN);
        }

        // Keep on screen vertically - but don't override placement if it was intentional
        let mut toolbar_y = toolbar_y;
        toolbar_y = toolbar_y.max(TOOLBAR_MARGIN);
        // Only apply max constraint if we have valid screen size
        if screen_size.y > bar_height + TOOLBAR_MARGIN * 2.0 {
            toolbar_y = toolbar_y.min(screen_size.y - bar_height - TOOLBAR_MARGIN);
        }

        log::debug!("ActionBar final position: ({:.0}, {:.0})", toolbar_x, toolbar_y);

        let position = Pos2::new(toolbar_x, toolbar_y);

        let mut bar = Self {
            position,
            drag_handle_bounds: Rect::NOTHING,
            drawing_buttons,
            edit_buttons,
            action_buttons,
            main_bar_size,
            options_panel_visible: false,
            icon_cache: IconCache::new(),
            is_dragging: false,
            drag_offset: Vec2::ZERO,
            hovered_button: None,
            scale_factor,
        };

        bar.update_button_bounds();
        bar
    }

    /// Update button bounds based on current position
    fn update_button_bounds(&mut self) {
        let mut x = self.position.x + DRAG_HANDLE_WIDTH + BAR_PADDING;
        let y = self.position.y + BAR_PADDING;

        // Drag handle bounds
        self.drag_handle_bounds = Rect::from_min_size(
            Pos2::new(self.position.x, self.position.y),
            Vec2::new(DRAG_HANDLE_WIDTH, self.main_bar_size.y),
        );

        // Drawing buttons
        for button in &mut self.drawing_buttons {
            button.bounds = Rect::from_min_size(Pos2::new(x, y), Vec2::splat(BUTTON_SIZE));
            x += BUTTON_SIZE + BUTTON_SPACING;
        }

        // First separator
        x += SEPARATOR_MARGIN;
        x += SEPARATOR_WIDTH;
        x += SEPARATOR_MARGIN;

        // Edit buttons
        for button in &mut self.edit_buttons {
            button.bounds = Rect::from_min_size(Pos2::new(x, y), Vec2::splat(BUTTON_SIZE));
            x += BUTTON_SIZE + BUTTON_SPACING;
        }

        // Second separator
        x += SEPARATOR_MARGIN;
        x += SEPARATOR_WIDTH;
        x += SEPARATOR_MARGIN;

        // Action buttons
        for button in &mut self.action_buttons {
            button.bounds = Rect::from_min_size(Pos2::new(x, y), Vec2::splat(BUTTON_SIZE));
            x += BUTTON_SIZE + BUTTON_SPACING;
        }
    }

    /// Get the main bar bounds
    pub fn bounds(&self) -> Rect {
        Rect::from_min_size(self.position, self.main_bar_size)
    }

    /// Check if a point is inside the action bar
    pub fn contains(&self, pos: Pos2) -> bool {
        self.bounds().contains(pos)
    }

    /// Check if point is in drag handle
    pub fn is_in_drag_handle(&self, pos: Pos2) -> bool {
        self.drag_handle_bounds.contains(pos)
    }

    /// Start dragging
    pub fn start_drag(&mut self, mouse_pos: Pos2) {
        self.is_dragging = true;
        self.drag_offset = mouse_pos - self.position;
    }

    /// Update drag position
    pub fn update_drag(&mut self, mouse_pos: Pos2, screen_size: Vec2) {
        if self.is_dragging {
            let mut new_pos = mouse_pos - self.drag_offset;

            // Keep on screen
            new_pos.x = new_pos.x.max(0.0);
            new_pos.y = new_pos.y.max(0.0);
            new_pos.x = new_pos.x.min(screen_size.x - self.main_bar_size.x);
            new_pos.y = new_pos.y.min(screen_size.y - self.main_bar_size.y);

            self.position = new_pos;
            self.update_button_bounds();
        }
    }

    /// Stop dragging
    pub fn stop_drag(&mut self) {
        self.is_dragging = false;
    }

    /// Check if currently dragging
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// Update hover state based on mouse position
    pub fn update_hover(&mut self, pos: Pos2) {
        self.hovered_button = None;

        for button in self
            .drawing_buttons
            .iter()
            .chain(self.edit_buttons.iter())
            .chain(self.action_buttons.iter())
        {
            if button.bounds.contains(pos) {
                self.hovered_button = Some(button.id.clone());
                break;
            }
        }
    }

    /// Check if a button was clicked, return the button ID
    pub fn check_click(&self, pos: Pos2) -> Option<&str> {
        for button in self
            .drawing_buttons
            .iter()
            .chain(self.edit_buttons.iter())
            .chain(self.action_buttons.iter())
        {
            if button.bounds.contains(pos) {
                return Some(&button.id);
            }
        }
        None
    }

    /// Get currently hovered button ID
    pub fn hovered_button(&self) -> Option<&str> {
        self.hovered_button.as_deref()
    }

    /// Render the action bar
    pub fn render(&mut self, ui: &mut Ui, active_tool: Option<&str>, undo_enabled: bool, redo_enabled: bool) {
        let main_rect = self.bounds();

        // Draw main bar background
        ui.painter().rect_filled(
            main_rect,
            BAR_CORNER_RADIUS,
            BAR_BG_COLOR,
        );

        // Draw drag handle
        Self::render_drag_handle_static(ui, self.drag_handle_bounds);

        // Collect button render info to avoid borrow issues
        let drawing_render_info: Vec<_> = self.drawing_buttons.iter().map(|button| {
            let is_active = active_tool == Some(button.id.as_str());
            let is_hovered = self.hovered_button.as_deref() == Some(button.id.as_str());
            (button.id.clone(), button.bounds, is_active, is_hovered, true)
        }).collect();

        let edit_render_info: Vec<_> = self.edit_buttons.iter().map(|button| {
            let enabled = if button.id == "undo" {
                undo_enabled
            } else if button.id == "redo" {
                redo_enabled
            } else {
                true
            };
            let is_hovered = self.hovered_button.as_deref() == Some(button.id.as_str());
            (button.id.clone(), button.bounds, false, is_hovered && enabled, enabled)
        }).collect();

        let action_render_info: Vec<_> = self.action_buttons.iter().map(|button| {
            let is_hovered = self.hovered_button.as_deref() == Some(button.id.as_str());
            (button.id.clone(), button.bounds, false, is_hovered, true)
        }).collect();

        // Draw drawing tool buttons
        for (id, bounds, is_active, is_hovered, enabled) in &drawing_render_info {
            self.render_button_by_info(ui, id, *bounds, *is_active, *is_hovered, *enabled);
        }

        // Draw first separator
        let sep1_x = self.drawing_buttons.last().map(|b| b.bounds.max.x + SEPARATOR_MARGIN).unwrap_or(self.position.x);
        Self::render_separator_static(ui, sep1_x, self.position.y, self.main_bar_size.y);

        // Draw edit buttons
        for (id, bounds, is_active, is_hovered, enabled) in &edit_render_info {
            self.render_button_by_info(ui, id, *bounds, *is_active, *is_hovered, *enabled);
        }

        // Draw second separator
        let sep2_x = self.edit_buttons.last().map(|b| b.bounds.max.x + SEPARATOR_MARGIN).unwrap_or(sep1_x);
        Self::render_separator_static(ui, sep2_x, self.position.y, self.main_bar_size.y);

        // Draw action buttons
        for (id, bounds, is_active, is_hovered, enabled) in &action_render_info {
            self.render_button_by_info(ui, id, *bounds, *is_active, *is_hovered, *enabled);
        }

        // Draw tooltip for hovered button
        if let Some(ref hovered_id) = self.hovered_button.clone() {
            if let Some(button) = self.find_button(&hovered_id) {
                Self::render_tooltip_static(ui, button.bounds, &button.tooltip, &button.id);
            }
        }
    }

    /// Find a button by ID
    fn find_button(&self, id: &str) -> Option<&ToolButton> {
        self.drawing_buttons
            .iter()
            .chain(self.edit_buttons.iter())
            .chain(self.action_buttons.iter())
            .find(|b| b.id == id)
    }

    /// Render drag handle (6 dots in 2 columns) - static version
    fn render_drag_handle_static(ui: &mut Ui, rect: Rect) {
        let center = rect.center();
        let dot_radius = 1.5;
        let dot_spacing_x = 4.0;
        let dot_spacing_y = 5.0;
        let dot_color = Color32::from_gray(120);

        // Draw 6 dots (2 columns, 3 rows)
        for col in 0..2 {
            for row in 0..3 {
                let x = center.x + (col as f32 - 0.5) * dot_spacing_x;
                let y = center.y + (row as f32 - 1.0) * dot_spacing_y;
                ui.painter().circle_filled(Pos2::new(x, y), dot_radius, dot_color);
            }
        }
    }

    /// Render a separator line - static version
    fn render_separator_static(ui: &mut Ui, x: f32, position_y: f32, bar_height: f32) {
        let top = position_y + BAR_PADDING + 4.0;
        let bottom = position_y + bar_height - BAR_PADDING - 4.0;
        ui.painter().line_segment(
            [Pos2::new(x, top), Pos2::new(x, bottom)],
            egui::Stroke::new(SEPARATOR_WIDTH, SEPARATOR_COLOR),
        );
    }

    /// Render a single button using collected info
    fn render_button_by_info(&mut self, ui: &mut Ui, id: &str, bounds: Rect, is_active: bool, is_hovered: bool, enabled: bool) {
        let bg_color = if is_active {
            BUTTON_ACTIVE_COLOR
        } else if is_hovered {
            BUTTON_HOVER_COLOR
        } else {
            BUTTON_DEFAULT_COLOR
        };

        // Draw button background
        if bg_color != BUTTON_DEFAULT_COLOR {
            ui.painter().rect_filled(
                bounds,
                4.0,
                bg_color,
            );
        }

        // Draw icon at high resolution for crisp rendering
        let icon_color = if enabled { ICON_COLOR } else { ICON_DISABLED_COLOR };
        let render_size = icon_render_size(self.scale_factor);
        if let Some(texture) = self.icon_cache.get_or_create(ui.ctx(), id, render_size, icon_color) {
            // Display at logical size (ICON_SIZE) but render at high resolution
            let display_size = Vec2::splat(ICON_SIZE as f32);
            let icon_pos = bounds.center() - display_size / 2.0;
            let icon_rect = Rect::from_min_size(icon_pos, display_size);
            ui.painter().image(
                texture.id(),
                icon_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        } else {
            // Fallback to text if icon not available
            ui.painter().text(
                bounds.center(),
                egui::Align2::CENTER_CENTER,
                &id[..1].to_uppercase(),
                egui::FontId::proportional(14.0),
                icon_color,
            );
        }
    }

    /// Render tooltip for a button - static version
    fn render_tooltip_static(ui: &mut Ui, bounds: Rect, tooltip: &str, id: &str) {
        let tooltip_text = if tooltip.is_empty() {
            id
        } else {
            tooltip
        };

        let tooltip_pos = Pos2::new(bounds.center().x, bounds.max.y + 4.0);
        let font = egui::FontId::proportional(12.0);
        let galley = ui.painter().layout_no_wrap(tooltip_text.to_string(), font.clone(), Color32::WHITE);
        let text_size = galley.size();

        let padding = Vec2::new(6.0, 3.0);
        let bg_rect = Rect::from_min_size(
            Pos2::new(tooltip_pos.x - text_size.x / 2.0 - padding.x, tooltip_pos.y),
            text_size + padding * 2.0,
        );

        ui.painter().rect_filled(bg_rect, 4.0, Color32::from_rgba_premultiplied(20, 20, 20, 230));
        ui.painter().text(
            bg_rect.center(),
            egui::Align2::CENTER_CENTER,
            tooltip_text,
            font,
            Color32::WHITE,
        );
    }

    /// Get position
    pub fn position(&self) -> Pos2 {
        self.position
    }

    /// Get size
    pub fn size(&self) -> Vec2 {
        self.main_bar_size
    }
}
