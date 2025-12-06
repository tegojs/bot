//! Command palette UI for window management
//!
//! A Raycast-style floating window with search input and action list.

use super::actions::{
    CurrentBounds, ScreenBounds, TargetBounds, WINDOW_ACTIONS, WindowAction, filter_actions,
};
use egui::{Color32, FontId, Key, Pos2, Rect, RichText, Sense, Vec2};

// Raycast-style colors - dark semi-transparent background
const BG_COLOR: Color32 = Color32::from_rgba_premultiplied(35, 30, 45, 230); // Dark purple-ish, mostly opaque
const SEARCH_BG: Color32 = Color32::from_rgba_premultiplied(60, 55, 70, 200); // Lighter for search
const SELECTED_BG: Color32 = Color32::from_rgba_premultiplied(90, 80, 120, 180); // Purple highlight
const HOVER_BG: Color32 = Color32::from_rgba_premultiplied(70, 65, 90, 150);
const TEXT_PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);
const TEXT_SECONDARY: Color32 = Color32::from_rgb(180, 180, 190);
const TEXT_HINT: Color32 = Color32::from_rgb(130, 130, 145);
const ICON_BG: Color32 = Color32::from_rgb(88, 86, 214); // Purple/blue icon background like Raycast
const SEPARATOR_COLOR: Color32 = Color32::from_rgba_premultiplied(100, 100, 120, 60);
const KEY_BADGE_BG: Color32 = Color32::from_rgba_premultiplied(80, 75, 95, 150);

/// Palette state
#[derive(Debug, Clone, Default)]
pub enum PaletteState {
    /// Palette is not visible
    #[default]
    Hidden,
    /// Palette is active
    Active {
        /// Process ID of the window to manipulate
        target_pid: i32,
        /// Current search text
        search_text: String,
        /// Currently selected index
        selected_index: usize,
    },
}

/// Result from palette interaction
#[derive(Debug, Clone)]
pub enum PaletteResult {
    /// Continue showing palette
    Continue,
    /// Execute an action
    Execute { action_id: &'static str, target_pid: i32 },
    /// Close the palette without action
    Cancel,
}

/// Command palette for window management
pub struct CommandPalette {
    /// Current state
    state: PaletteState,
    /// Whether the text input should request focus
    needs_focus: bool,
}

impl CommandPalette {
    /// Create a new command palette
    pub fn new() -> Self {
        Self { state: PaletteState::Hidden, needs_focus: false }
    }

    /// Show the palette for a specific window
    pub fn show(&mut self, target_pid: i32) {
        self.state =
            PaletteState::Active { target_pid, search_text: String::new(), selected_index: 0 };
        self.needs_focus = true;
    }

    /// Hide the palette
    pub fn hide(&mut self) {
        self.state = PaletteState::Hidden;
    }

    /// Check if palette is visible
    pub fn is_visible(&self) -> bool {
        matches!(self.state, PaletteState::Active { .. })
    }

    /// Get the target PID if active
    pub fn target_pid(&self) -> Option<i32> {
        match &self.state {
            PaletteState::Active { target_pid, .. } => Some(*target_pid),
            _ => None,
        }
    }

    /// Render the palette and handle input
    pub fn render(&mut self, ctx: &egui::Context) -> PaletteResult {
        // Extract state values to local variables to avoid borrow conflicts
        let (target_pid, mut search_text, mut selected_index) = match &self.state {
            PaletteState::Active { target_pid, search_text, selected_index } => {
                (*target_pid, search_text.clone(), *selected_index)
            }
            PaletteState::Hidden => return PaletteResult::Continue,
        };

        // Get filtered actions based on search text
        let filtered_actions = filter_actions(&search_text);

        // Ensure selected_index is valid
        if selected_index >= filtered_actions.len() && !filtered_actions.is_empty() {
            selected_index = filtered_actions.len() - 1;
        }

        let mut result = PaletteResult::Continue;

        // Handle global keyboard input first
        ctx.input(|i| {
            if i.key_pressed(Key::Escape) {
                result = PaletteResult::Cancel;
            } else if i.key_pressed(Key::ArrowDown) || (i.modifiers.ctrl && i.key_pressed(Key::N)) {
                // Arrow Down or Ctrl+N to move down
                if !filtered_actions.is_empty() {
                    selected_index = (selected_index + 1).min(filtered_actions.len() - 1);
                }
            } else if i.key_pressed(Key::ArrowUp) || (i.modifiers.ctrl && i.key_pressed(Key::P)) {
                // Arrow Up or Ctrl+P to move up
                selected_index = selected_index.saturating_sub(1);
            } else if i.key_pressed(Key::Enter) {
                if let Some(action) = filtered_actions.get(selected_index) {
                    result = PaletteResult::Execute { action_id: action.id, target_pid };
                }
            }
        });

        // If we got a result from keyboard, return early
        if !matches!(result, PaletteResult::Continue) {
            self.state = PaletteState::Active { target_pid, search_text, selected_index };
            return result;
        }

        let needs_focus = self.needs_focus;

        // Use CentralPanel to fill the entire native window
        // The native window is already sized to 1000x700
        let frame = egui::Frame::new()
            .fill(BG_COLOR)
            .corner_radius(12.0)
            .inner_margin(egui::Margin::same(0));

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            // Get available width from the ui
            let available_width = ui.available_width();

            ui.vertical(|ui| {
                // Search input section
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);

                    // Search input with custom styling
                    let search_frame = egui::Frame::new()
                        .fill(SEARCH_BG)
                        .corner_radius(8.0)
                        .inner_margin(egui::Margin::symmetric(12, 10));

                    search_frame.show(ui, |ui| {
                        ui.set_width(available_width - 48.0);
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut search_text)
                                .hint_text(
                                    RichText::new("Search window actions...").color(TEXT_HINT),
                                )
                                .font(FontId::proportional(18.0))
                                .text_color(TEXT_PRIMARY)
                                .frame(false)
                                .desired_width(available_width - 72.0),
                        );

                        if needs_focus {
                            response.request_focus();
                        }

                        if response.changed() {
                            selected_index = 0;
                        }
                    });

                    ui.add_space(16.0);
                });

                ui.add_space(12.0);

                // Separator
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    let rect = Rect::from_min_size(
                        ui.cursor().min,
                        Vec2::new(available_width - 32.0, 1.0),
                    );
                    ui.painter().rect_filled(rect, 0.0, SEPARATOR_COLOR);
                    ui.allocate_space(Vec2::new(available_width - 32.0, 1.0));
                });

                ui.add_space(8.0);

                // Results header
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(RichText::new("Results").color(TEXT_SECONDARY).size(12.0));
                });

                ui.add_space(8.0);

                // Action list with scroll area
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 60.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        if filtered_actions.is_empty() {
                            ui.add_space(40.0);
                            ui.horizontal(|ui| {
                                ui.add_space(20.0);
                                ui.label(
                                    RichText::new("No matching actions")
                                        .color(TEXT_HINT)
                                        .italics()
                                        .size(14.0),
                                );
                            });
                        } else {
                            for (i, action) in filtered_actions.iter().enumerate() {
                                let is_selected = i == selected_index;
                                let (clicked, item_response) =
                                    render_action_item(ui, action, is_selected, available_width);

                                // Auto-scroll to keep selected item visible
                                if is_selected {
                                    item_response.scroll_to_me(Some(egui::Align::Center));
                                }

                                if clicked {
                                    result =
                                        PaletteResult::Execute { action_id: action.id, target_pid };
                                }
                            }
                        }
                    });

                // Spacer
                ui.add_space(8.0);

                // Footer separator
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    let rect = Rect::from_min_size(
                        ui.cursor().min,
                        Vec2::new(available_width - 32.0, 1.0),
                    );
                    ui.painter().rect_filled(rect, 0.0, SEPARATOR_COLOR);
                    ui.allocate_space(Vec2::new(available_width - 32.0, 1.0));
                });

                ui.add_space(12.0);

                // Footer with keyboard hints (Raycast-style)
                ui.horizontal(|ui| {
                    ui.add_space(20.0);

                    // Open Command hint
                    render_key_badge(ui, "Enter");
                    ui.add_space(4.0);
                    ui.label(RichText::new("Open").color(TEXT_SECONDARY).size(12.0));

                    ui.add_space(20.0);

                    // Navigate hint
                    render_key_badge(ui, "Ctrl+N/P");
                    ui.add_space(4.0);
                    ui.label(RichText::new("Navigate").color(TEXT_SECONDARY).size(12.0));

                    ui.add_space(20.0);

                    // Cancel hint
                    render_key_badge(ui, "Esc");
                    ui.add_space(4.0);
                    ui.label(RichText::new("Cancel").color(TEXT_SECONDARY).size(12.0));
                });

                ui.add_space(20.0);
            });
        });

        // Update state with any changes
        self.state = PaletteState::Active { target_pid, search_text, selected_index };
        if needs_focus {
            self.needs_focus = false;
        }

        result
    }
}

/// Render a keyboard shortcut badge
fn render_key_badge(ui: &mut egui::Ui, text: &str) {
    let badge_frame = egui::Frame::new()
        .fill(KEY_BADGE_BG)
        .corner_radius(4.0)
        .inner_margin(egui::Margin::symmetric(6, 2));

    badge_frame.show(ui, |ui| {
        ui.label(RichText::new(text).color(TEXT_SECONDARY).size(11.0));
    });
}

/// Render a single action item (free function to avoid borrow conflicts)
/// Returns (clicked, response) tuple for scroll handling
fn render_action_item(
    ui: &mut egui::Ui,
    action: &WindowAction,
    is_selected: bool,
    window_width: f32,
) -> (bool, egui::Response) {
    let item_height = 44.0;
    let margin = 16.0;
    let item_width = window_width - margin * 2.0;

    // Create a frame for the item
    let bg_color = if is_selected { SELECTED_BG } else { Color32::TRANSPARENT };

    let frame = egui::Frame::new()
        .fill(bg_color)
        .corner_radius(8.0)
        .inner_margin(egui::Margin::symmetric(10, 0));

    let mut clicked = false;
    let mut item_response: Option<egui::Response> = None;

    ui.horizontal(|ui| {
        ui.add_space(margin);

        // Create a scope for the item
        let response = frame.show(ui, |ui| {
            ui.set_min_size(Vec2::new(item_width, item_height));
            ui.set_max_size(Vec2::new(item_width, item_height));

            ui.horizontal_centered(|ui| {
                // Icon (blue rectangle showing window position)
                let icon_size = 28.0;
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(icon_size), Sense::hover());

                // Draw icon background
                ui.painter().rect_filled(icon_rect, 6.0, ICON_BG);

                // Draw window position indicator inside icon
                draw_window_icon(ui.painter(), icon_rect, action.id);

                ui.add_space(12.0);

                // Action name - use a label for proper text rendering
                ui.label(RichText::new(action.name).color(TEXT_PRIMARY).size(15.0));

                // Flexible space to push category to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Category badge
                    ui.label(RichText::new("Window").color(TEXT_SECONDARY).size(12.0));
                });
            });
        });

        // Check for hover to change background
        let response = response.response.interact(Sense::click());
        if response.hovered() && !is_selected {
            ui.painter().rect_filled(response.rect, 8.0, HOVER_BG);
        }

        if response.clicked() {
            clicked = true;
        }

        item_response = Some(response);
    });

    ui.add_space(2.0);
    (clicked, item_response.unwrap())
}

/// Draw a window position indicator inside the icon
fn draw_window_icon(painter: &egui::Painter, icon_rect: Rect, action_id: &str) {
    let inner_margin = 4.0;
    let inner_rect = icon_rect.shrink(inner_margin);
    let w = inner_rect.width();
    let h = inner_rect.height();
    let x = inner_rect.min.x;
    let y = inner_rect.min.y;

    let highlight_color = Color32::from_rgb(200, 220, 255);
    let dim_color = Color32::from_rgba_premultiplied(150, 170, 200, 100);

    // Draw a small representation of the window position
    let (highlight_rect, has_highlight) = match action_id {
        "left_half" => (Rect::from_min_size(Pos2::new(x, y), Vec2::new(w / 2.0, h)), true),
        "right_half" => {
            (Rect::from_min_size(Pos2::new(x + w / 2.0, y), Vec2::new(w / 2.0, h)), true)
        }
        "top_half" => (Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h / 2.0)), true),
        "bottom_half" => {
            (Rect::from_min_size(Pos2::new(x, y + h / 2.0), Vec2::new(w, h / 2.0)), true)
        }
        "top_left" => (Rect::from_min_size(Pos2::new(x, y), Vec2::new(w / 2.0, h / 2.0)), true),
        "top_right" => {
            (Rect::from_min_size(Pos2::new(x + w / 2.0, y), Vec2::new(w / 2.0, h / 2.0)), true)
        }
        "bottom_left" => {
            (Rect::from_min_size(Pos2::new(x, y + h / 2.0), Vec2::new(w / 2.0, h / 2.0)), true)
        }
        "bottom_right" => (
            Rect::from_min_size(Pos2::new(x + w / 2.0, y + h / 2.0), Vec2::new(w / 2.0, h / 2.0)),
            true,
        ),
        "left_third" => (Rect::from_min_size(Pos2::new(x, y), Vec2::new(w / 3.0, h)), true),
        "center_third" => {
            (Rect::from_min_size(Pos2::new(x + w / 3.0, y), Vec2::new(w / 3.0, h)), true)
        }
        "right_third" => {
            (Rect::from_min_size(Pos2::new(x + w * 2.0 / 3.0, y), Vec2::new(w / 3.0, h)), true)
        }
        "left_two_thirds" => {
            (Rect::from_min_size(Pos2::new(x, y), Vec2::new(w * 2.0 / 3.0, h)), true)
        }
        "right_two_thirds" => {
            (Rect::from_min_size(Pos2::new(x + w / 3.0, y), Vec2::new(w * 2.0 / 3.0, h)), true)
        }
        "maximize" | "almost_maximize" => (inner_rect, true),
        "center" => {
            // Center: smaller rectangle in the middle
            let cw = w * 0.6;
            let ch = h * 0.6;
            (
                Rect::from_min_size(
                    Pos2::new(x + (w - cw) / 2.0, y + (h - ch) / 2.0),
                    Vec2::new(cw, ch),
                ),
                true,
            )
        }
        _ => (inner_rect, false),
    };

    // Draw dim background for the full screen
    painter.rect_filled(inner_rect, 2.0, dim_color);

    // Draw the highlighted portion
    if has_highlight {
        painter.rect_filled(highlight_rect, 2.0, highlight_color);
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute a window action
pub fn execute_action(
    action_id: &str,
    _target_pid: i32,
    screen_bounds: ScreenBounds,
    current_bounds: Option<CurrentBounds>,
) -> Result<TargetBounds, String> {
    let action = WINDOW_ACTIONS
        .iter()
        .find(|a| a.id == action_id)
        .ok_or_else(|| format!("Unknown action: {}", action_id))?;

    Ok(action.calculate(screen_bounds, current_bounds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_state() {
        let mut palette = CommandPalette::new();
        assert!(!palette.is_visible());

        palette.show(12345);
        assert!(palette.is_visible());
        assert_eq!(palette.target_pid(), Some(12345));

        palette.hide();
        assert!(!palette.is_visible());
    }

    #[test]
    fn test_execute_action() {
        let screen = ScreenBounds { x: 0, y: 0, width: 1920, height: 1080 };

        let result = execute_action("left_half", 0, screen, None).unwrap();
        assert_eq!(result.x, 0);
        assert_eq!(result.width, 960);
    }
}
