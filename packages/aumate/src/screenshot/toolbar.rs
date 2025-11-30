//! Toolbar for screenshot actions
//!
//! Renders action buttons below the selection area.

use super::action::ActionInfo;

/// Button dimensions
const BUTTON_WIDTH: f32 = 80.0;
const BUTTON_HEIGHT: f32 = 32.0;
const BUTTON_SPACING: f32 = 8.0;
const TOOLBAR_PADDING: f32 = 8.0;
const TOOLBAR_MARGIN: f32 = 10.0;

/// A toolbar button
#[derive(Debug, Clone)]
pub struct ToolbarButton {
    /// Action ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Button bounds (x, y, width, height)
    pub bounds: (f32, f32, f32, f32),
    /// Icon texture (if available)
    pub icon: Option<Vec<u8>>,
}

/// Toolbar for displaying action buttons
#[derive(Debug, Clone)]
pub struct Toolbar {
    /// Buttons in the toolbar
    buttons: Vec<ToolbarButton>,
    /// Toolbar position (top-left corner)
    position: (f32, f32),
    /// Toolbar size (width, height)
    size: (f32, f32),
}

impl Toolbar {
    /// Create a new toolbar for the given actions
    ///
    /// Positions the toolbar below the selection bounds
    pub fn new(
        actions: &[ActionInfo],
        selection_bounds: ((f32, f32), (f32, f32)),
        screen_size: (f32, f32),
    ) -> Self {
        let ((min_x, _min_y), (max_x, max_y)) = selection_bounds;

        // Calculate toolbar dimensions
        let button_count = actions.len() as f32;
        let toolbar_width = button_count * BUTTON_WIDTH
            + (button_count - 1.0) * BUTTON_SPACING
            + 2.0 * TOOLBAR_PADDING;
        let toolbar_height = BUTTON_HEIGHT + 2.0 * TOOLBAR_PADDING;

        // Center toolbar horizontally below selection
        let selection_center_x = (min_x + max_x) / 2.0;
        let mut toolbar_x = selection_center_x - toolbar_width / 2.0;

        // Position below selection with margin
        let mut toolbar_y = max_y + TOOLBAR_MARGIN;

        // Keep toolbar on screen
        toolbar_x = toolbar_x.max(TOOLBAR_MARGIN);
        toolbar_x = toolbar_x.min(screen_size.0 - toolbar_width - TOOLBAR_MARGIN);

        // If toolbar would go off bottom, position it above selection
        if toolbar_y + toolbar_height > screen_size.1 - TOOLBAR_MARGIN {
            toolbar_y = selection_bounds.0.1 - toolbar_height - TOOLBAR_MARGIN;
        }

        // Create buttons
        let mut buttons = Vec::with_capacity(actions.len());
        let mut button_x = toolbar_x + TOOLBAR_PADDING;
        let button_y = toolbar_y + TOOLBAR_PADDING;

        for action in actions {
            buttons.push(ToolbarButton {
                id: action.id.clone(),
                name: action.name.clone(),
                bounds: (button_x, button_y, BUTTON_WIDTH, BUTTON_HEIGHT),
                icon: action.icon.clone(),
            });
            button_x += BUTTON_WIDTH + BUTTON_SPACING;
        }

        Self { buttons, position: (toolbar_x, toolbar_y), size: (toolbar_width, toolbar_height) }
    }

    /// Get all buttons
    pub fn buttons(&self) -> &[ToolbarButton] {
        &self.buttons
    }

    /// Get toolbar position
    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    /// Get toolbar size
    pub fn size(&self) -> (f32, f32) {
        self.size
    }

    /// Get toolbar bounds (x, y, width, height)
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.position.0, self.position.1, self.size.0, self.size.1)
    }

    /// Check if a point is inside the toolbar
    pub fn contains(&self, pos: (f32, f32)) -> bool {
        let (x, y) = pos;
        x >= self.position.0
            && x <= self.position.0 + self.size.0
            && y >= self.position.1
            && y <= self.position.1 + self.size.1
    }

    /// Check if a point clicks a button, return action ID if so
    pub fn check_click(&self, pos: (f32, f32)) -> Option<&str> {
        let (px, py) = pos;
        for button in &self.buttons {
            let (bx, by, bw, bh) = button.bounds;
            if px >= bx && px <= bx + bw && py >= by && py <= by + bh {
                return Some(&button.id);
            }
        }
        None
    }

    /// Render the toolbar using egui
    pub fn render(&self, ui: &mut egui::Ui, hovered_button: Option<&str>) {
        let (tx, ty, tw, th) = self.bounds();

        // Draw toolbar background
        let toolbar_rect = egui::Rect::from_min_size(egui::pos2(tx, ty), egui::vec2(tw, th));

        ui.painter().rect_filled(
            toolbar_rect,
            egui::CornerRadius::same(8),
            egui::Color32::from_rgba_unmultiplied(40, 40, 40, 230),
        );

        // Draw buttons
        for button in &self.buttons {
            let (bx, by, bw, bh) = button.bounds;
            let button_rect = egui::Rect::from_min_size(egui::pos2(bx, by), egui::vec2(bw, bh));

            let is_hovered = hovered_button == Some(button.id.as_str());
            let bg_color = if is_hovered {
                egui::Color32::from_rgba_unmultiplied(80, 80, 80, 255)
            } else {
                egui::Color32::from_rgba_unmultiplied(60, 60, 60, 255)
            };

            ui.painter().rect_filled(button_rect, egui::CornerRadius::same(4), bg_color);

            // Draw button text
            ui.painter().text(
                button_rect.center(),
                egui::Align2::CENTER_CENTER,
                &button.name,
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );
        }
    }
}
