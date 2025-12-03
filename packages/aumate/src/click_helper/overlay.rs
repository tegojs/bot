//! Click Helper overlay rendering
//!
//! Renders a dimmed overlay with hint labels for clickable elements.

use super::accessibility::ClickableElement;
use super::config::ClickHelperConfig;
use super::hints::{HintGenerator, HintLabel};
use egui::{Color32, FontId, Pos2, Rect, Vec2};

/// Overlay state for Click Helper
pub struct ClickHelperOverlay {
    /// Overlay background color
    overlay_color: Color32,
    /// Elements with their hints
    pub hints: Vec<(ClickableElement, HintLabel)>,
    /// Current input buffer
    input_buffer: String,
    /// Indices of visible hints (filtered by input)
    visible_indices: Vec<usize>,
    /// Hint styling
    hint_font_size: f32,
    hint_bg_color: Color32,
    hint_fg_color: Color32,
}

impl ClickHelperOverlay {
    /// Create a new overlay with elements and generated hints
    pub fn new(elements: Vec<ClickableElement>, config: &ClickHelperConfig) -> Self {
        let hint_gen = HintGenerator::new(config);
        let labels = hint_gen.generate(elements.len());

        let hints: Vec<_> = elements.into_iter().zip(labels).collect();
        let visible_indices: Vec<_> = (0..hints.len()).collect();

        Self {
            overlay_color: Color32::from_rgba_unmultiplied(0, 0, 0, config.overlay_opacity),
            hints,
            input_buffer: String::new(),
            visible_indices,
            hint_font_size: config.hint_font_size,
            hint_bg_color: Color32::from_rgba_unmultiplied(
                config.hint_bg_color[0],
                config.hint_bg_color[1],
                config.hint_bg_color[2],
                config.hint_bg_color[3],
            ),
            hint_fg_color: Color32::from_rgba_unmultiplied(
                config.hint_fg_color[0],
                config.hint_fg_color[1],
                config.hint_fg_color[2],
                config.hint_fg_color[3],
            ),
        }
    }

    /// Get current input buffer
    pub fn input_buffer(&self) -> &str {
        &self.input_buffer
    }

    /// Add a character to the input buffer and filter hints
    pub fn add_input(&mut self, c: char) {
        self.input_buffer.push(c);
        self.filter_hints();
    }

    /// Remove last character from input buffer
    pub fn backspace(&mut self) {
        self.input_buffer.pop();
        self.filter_hints();
    }

    /// Clear input buffer and show all hints
    pub fn reset_input(&mut self) {
        self.input_buffer.clear();
        self.visible_indices = (0..self.hints.len()).collect();
    }

    /// Filter hints based on current input buffer
    fn filter_hints(&mut self) {
        if self.input_buffer.is_empty() {
            self.visible_indices = (0..self.hints.len()).collect();
        } else {
            self.visible_indices = self
                .hints
                .iter()
                .enumerate()
                .filter(|(_, (_, hint))| hint.label.starts_with(&self.input_buffer))
                .map(|(i, _)| i)
                .collect();
        }
    }

    /// Get matching hints count
    pub fn matching_count(&self) -> usize {
        self.visible_indices.len()
    }

    /// Get the exact match if input matches a single hint exactly
    pub fn get_exact_match(&self) -> Option<&ClickableElement> {
        if self.visible_indices.len() == 1 {
            let idx = self.visible_indices[0];
            let (element, hint) = &self.hints[idx];
            if hint.label == self.input_buffer {
                return Some(element);
            }
        }

        // Also check for single match when input length matches hint length
        let matching: Vec<_> =
            self.hints.iter().filter(|(_, hint)| hint.label == self.input_buffer).collect();

        if matching.len() == 1 {
            return Some(&matching[0].0);
        }

        None
    }

    /// Get the unique match if only one hint matches the prefix
    pub fn get_unique_match(&self) -> Option<&ClickableElement> {
        if self.visible_indices.len() == 1 {
            let idx = self.visible_indices[0];
            return Some(&self.hints[idx].0);
        }
        None
    }

    /// Render the overlay
    #[allow(deprecated)]
    pub fn render(&self, ctx: &egui::Context) {
        let screen_rect = ctx.screen_rect();

        egui::Area::new(egui::Id::new("click_helper_overlay"))
            .fixed_pos(Pos2::ZERO)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                // Draw dimmed overlay
                ui.painter().rect_filled(screen_rect, 0.0, self.overlay_color);

                // Draw hint labels for visible hints
                for &idx in &self.visible_indices {
                    let (element, hint) = &self.hints[idx];
                    self.render_hint_label(ui, element, hint);
                }

                // Draw input buffer display at top center
                if !self.input_buffer.is_empty() {
                    self.render_input_display(ui, screen_rect);
                }
            });
    }

    /// Render a single hint label at element position
    fn render_hint_label(&self, ui: &egui::Ui, element: &ClickableElement, hint: &HintLabel) {
        let pos = Pos2::new(element.bounds.0, element.bounds.1);

        // Calculate text display
        let display_text = if self.input_buffer.is_empty() {
            hint.label.clone()
        } else {
            // Show remaining characters in different style
            hint.label.clone()
        };

        let font_id = FontId::monospace(self.hint_font_size);
        let galley =
            ui.painter().layout_no_wrap(display_text.clone(), font_id.clone(), self.hint_fg_color);

        let padding = Vec2::new(6.0, 3.0);
        let label_size = galley.size() + padding * 2.0;
        let label_rect = Rect::from_min_size(pos, label_size);

        // Draw background with rounded corners
        ui.painter().rect_filled(label_rect, 4.0, self.hint_bg_color);

        // Draw border
        ui.painter().rect_stroke(
            label_rect,
            4.0,
            egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 100)),
            egui::StrokeKind::Outside,
        );

        // Draw text
        let text_pos = label_rect.min + padding;

        // If we have input, highlight the matched prefix
        if !self.input_buffer.is_empty() && hint.label.starts_with(&self.input_buffer) {
            let matched_part = &hint.label[..self.input_buffer.len()];
            let remaining_part = &hint.label[self.input_buffer.len()..];

            // Draw matched part in dimmer color
            let matched_galley = ui.painter().layout_no_wrap(
                matched_part.to_string(),
                font_id.clone(),
                Color32::from_rgba_unmultiplied(100, 100, 100, 255),
            );
            ui.painter().galley(text_pos, matched_galley.clone(), Color32::PLACEHOLDER);

            // Draw remaining part in full color
            if !remaining_part.is_empty() {
                let offset = matched_galley.size().x;
                let remaining_galley = ui.painter().layout_no_wrap(
                    remaining_part.to_string(),
                    font_id,
                    self.hint_fg_color,
                );
                ui.painter().galley(
                    text_pos + Vec2::new(offset, 0.0),
                    remaining_galley,
                    Color32::PLACEHOLDER,
                );
            }
        } else {
            ui.painter().galley(text_pos, galley, Color32::PLACEHOLDER);
        }
    }

    /// Render the input display at top of screen
    fn render_input_display(&self, ui: &egui::Ui, screen_rect: Rect) {
        let display_text = format!("Input: {}", self.input_buffer);
        let font_id = FontId::proportional(18.0);
        let galley = ui.painter().layout_no_wrap(display_text, font_id, Color32::WHITE);

        let padding = Vec2::new(16.0, 8.0);
        let label_size = galley.size() + padding * 2.0;

        // Center at top of screen
        let pos = Pos2::new((screen_rect.width() - label_size.x) / 2.0, 20.0);
        let label_rect = Rect::from_min_size(pos, label_size);

        // Draw background
        ui.painter().rect_filled(label_rect, 8.0, Color32::from_rgba_unmultiplied(50, 50, 50, 220));

        // Draw text
        ui.painter().galley(label_rect.min + padding, galley, Color32::PLACEHOLDER);
    }
}
