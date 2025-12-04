//! Widget renderer - renders WidgetDef to egui
//!
//! This module provides the rendering logic that converts widget definitions
//! into actual egui UI elements.

use super::definition::{WidgetDef, WidgetProps};
use super::events::{WidgetEvent, WidgetId};
use super::fonts::FontManager;
use super::style::{TextAlign, WidgetStyle};
use egui::{Color32, FontId, RichText, Ui, Vec2};
use std::collections::HashMap;

/// Mutable state for widgets that need it (text inputs, checkboxes, etc.)
#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    /// Text value for text inputs
    pub text: String,
    /// Boolean value for checkboxes
    pub checked: bool,
    /// Numeric value for sliders
    pub value: f32,
    /// Selected index for dropdowns, radio groups, and tabs
    pub selected: usize,
}

/// Widget renderer context
pub struct WidgetRenderer {
    /// Mutable state for interactive widgets, keyed by widget ID
    pub state: HashMap<WidgetId, WidgetState>,
    /// Font manager for loading system fonts
    font_manager: FontManager,
}

impl Default for WidgetRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetRenderer {
    /// Create a new widget renderer
    pub fn new() -> Self {
        Self { state: HashMap::new(), font_manager: FontManager::new() }
    }

    /// Render a widget definition to egui, collecting events
    pub fn render(&mut self, ui: &mut Ui, widget: &WidgetDef, events: &mut Vec<WidgetEvent>) {
        let props = widget.props();

        // Skip invisible widgets
        if !props.visible {
            return;
        }

        // Apply style constraints
        self.apply_style_constraints(ui, &props.style);

        match widget {
            WidgetDef::Label { text, props } => {
                self.render_label(ui, text, props);
            }
            WidgetDef::Button { text, props } => {
                self.render_button(ui, text, props, events);
            }
            WidgetDef::TextInput { value, placeholder, password, props } => {
                self.render_text_input(ui, value, placeholder.as_deref(), *password, props, events);
            }
            WidgetDef::Checkbox { checked, label, props } => {
                self.render_checkbox(ui, *checked, label, props, events);
            }
            WidgetDef::Slider { value, min, max, step, props } => {
                self.render_slider(ui, *value, *min, *max, *step, props, events);
            }
            WidgetDef::ProgressBar { value, show_percentage, props } => {
                self.render_progress_bar(ui, *value, *show_percentage, props);
            }
            WidgetDef::Image { data, width, height, props } => {
                self.render_image(ui, data, *width, *height, props);
            }
            WidgetDef::Separator { .. } => {
                ui.separator();
            }
            WidgetDef::Spacer { size, .. } => {
                ui.add_space(*size);
            }
            WidgetDef::HBox { children, spacing, props } => {
                self.render_hbox(ui, children, *spacing, props, events);
            }
            WidgetDef::VBox { children, spacing, props } => {
                self.render_vbox(ui, children, *spacing, props, events);
            }
            WidgetDef::Grid { children, row_spacing, col_spacing, props } => {
                self.render_grid(ui, children, *row_spacing, *col_spacing, props, events);
            }
            WidgetDef::Panel { child, props } => {
                self.render_panel(ui, child, props, events);
            }
            WidgetDef::ScrollArea { child, max_height, props } => {
                self.render_scroll_area(ui, child, *max_height, props, events);
            }
            WidgetDef::Group { title, child, collapsed, props } => {
                self.render_group(ui, title.as_deref(), child, *collapsed, props, events);
            }
            WidgetDef::Dropdown { options, selected, placeholder, props } => {
                self.render_dropdown(ui, options, *selected, placeholder.as_deref(), props, events);
            }
            WidgetDef::RadioGroup { options, selected, horizontal, props } => {
                self.render_radio_group(ui, options, *selected, *horizontal, props, events);
            }
            WidgetDef::TextArea { value, placeholder, rows, props } => {
                self.render_text_area(ui, value, placeholder.as_deref(), *rows, props, events);
            }

            WidgetDef::Tabs { tabs, active, props } => {
                self.render_tabs(ui, tabs, *active, props, events);
            }
            WidgetDef::Link { text, props } => {
                self.render_link(ui, text, props, events);
            }
            WidgetDef::SelectableLabel { text, selected, props } => {
                self.render_selectable_label(ui, text, *selected, props, events);
            }
            WidgetDef::DragValue { value, min, max, speed, prefix, suffix, decimals, props } => {
                self.render_drag_value(
                    ui,
                    *value,
                    *min,
                    *max,
                    *speed,
                    prefix.as_deref(),
                    suffix.as_deref(),
                    *decimals,
                    props,
                    events,
                );
            }
            WidgetDef::ColorPicker { color, alpha, props } => {
                self.render_color_picker(ui, *color, *alpha, props, events);
            }
            WidgetDef::Hyperlink { text, url, new_tab, props } => {
                self.render_hyperlink(ui, text, url, *new_tab, props, events);
            }
            WidgetDef::ImageButton { data, width, height, frame, selected, tint, props } => {
                self.render_image_button(
                    ui, data, *width, *height, *frame, *selected, *tint, props, events,
                );
            }
        }
    }

    fn apply_style_constraints(&self, ui: &mut Ui, style: &WidgetStyle) {
        if let Some(min_width) = style.min_width {
            ui.set_min_width(min_width);
        }
        if let Some(min_height) = style.min_height {
            ui.set_min_height(min_height);
        }
        if let Some(max_width) = style.max_width {
            ui.set_max_width(max_width);
        }
        if let Some(max_height) = style.max_height {
            ui.set_max_height(max_height);
        }
    }

    fn render_label(&self, ui: &mut Ui, text: &str, props: &WidgetProps) {
        let mut rich_text = RichText::new(text);

        // Apply text color
        if let Some([r, g, b, a]) = props.style.text_color {
            rich_text = rich_text.color(Color32::from_rgba_unmultiplied(r, g, b, a));
        }

        // Apply font family and size
        let size = props.style.font_size.unwrap_or(14.0);
        if let Some(ref family_name) = props.style.font_family {
            let family = self.font_manager.ensure_font_loaded(ui.ctx(), family_name);
            rich_text = rich_text.font(FontId::new(size, family));
        } else if props.style.font_size.is_some() {
            rich_text = rich_text.size(size);
        }

        // Create label with alignment
        let label = egui::Label::new(rich_text);

        // Handle text alignment
        match props.style.text_align {
            Some(TextAlign::Center) => {
                ui.centered_and_justified(|ui| {
                    ui.add(label);
                });
            }
            Some(TextAlign::Right) => {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(label);
                });
            }
            _ => {
                ui.add(label);
            }
        }

        // Note: Labels don't support hover tooltips directly in egui
        // Tooltips for labels would require wrapping in a sense area
        let _ = &props.tooltip; // Acknowledge but don't use for labels
    }

    fn render_button(
        &self,
        ui: &mut Ui,
        text: &str,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let mut button = egui::Button::new(text);

        // Apply min size from style
        if props.style.min_width.is_some() || props.style.min_height.is_some() {
            let min_size = Vec2::new(
                props.style.min_width.unwrap_or(0.0),
                props.style.min_height.unwrap_or(0.0),
            );
            button = button.min_size(min_size);
        }

        let response = ui.add_enabled(props.enabled, button);

        if response.clicked() {
            if let Some(id) = &props.id {
                events.push(WidgetEvent::ButtonClick { id: id.clone() });
            }
        }

        // Tooltip
        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_text_input(
        &mut self,
        ui: &mut Ui,
        initial_value: &str,
        placeholder: Option<&str>,
        password: bool,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        // Get or initialize state
        let id = props.id.clone().unwrap_or_default();
        let state = self.state.entry(id.clone()).or_insert_with(|| WidgetState {
            text: initial_value.to_string(),
            ..Default::default()
        });

        let mut text_edit = egui::TextEdit::singleline(&mut state.text);

        if let Some(hint) = placeholder {
            text_edit = text_edit.hint_text(hint);
        }

        if password {
            text_edit = text_edit.password(true);
        }

        // Apply min width
        if let Some(min_width) = props.style.min_width {
            text_edit = text_edit.desired_width(min_width);
        }

        let response = ui.add_enabled(props.enabled, text_edit);

        // Emit text changed event
        if response.changed() && !id.is_empty() {
            events.push(WidgetEvent::TextChanged { id: id.clone(), value: state.text.clone() });
        }

        // Emit submit event on Enter
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) && !id.is_empty()
        {
            events.push(WidgetEvent::TextSubmit { id: id.clone(), value: state.text.clone() });
        }

        // Tooltip
        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_checkbox(
        &mut self,
        ui: &mut Ui,
        initial_checked: bool,
        label: &str,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();
        let state = self
            .state
            .entry(id.clone())
            .or_insert_with(|| WidgetState { checked: initial_checked, ..Default::default() });

        let response =
            ui.add_enabled(props.enabled, egui::Checkbox::new(&mut state.checked, label));

        if response.changed() && !id.is_empty() {
            events.push(WidgetEvent::CheckboxChanged { id: id.clone(), checked: state.checked });
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_slider(
        &mut self,
        ui: &mut Ui,
        initial_value: f32,
        min: f32,
        max: f32,
        step: Option<f32>,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();
        let state = self
            .state
            .entry(id.clone())
            .or_insert_with(|| WidgetState { value: initial_value, ..Default::default() });

        let mut slider = egui::Slider::new(&mut state.value, min..=max);

        if let Some(s) = step {
            slider = slider.step_by(s as f64);
        }

        let response = ui.add_enabled(props.enabled, slider);

        if response.changed() && !id.is_empty() {
            events.push(WidgetEvent::SliderChanged { id: id.clone(), value: state.value });
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_progress_bar(
        &self,
        ui: &mut Ui,
        value: f32,
        show_percentage: bool,
        props: &WidgetProps,
    ) {
        let progress = value.clamp(0.0, 1.0);
        let mut bar = egui::ProgressBar::new(progress);

        if show_percentage {
            bar = bar.show_percentage();
        }

        let response = ui.add(bar);

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_image(&self, ui: &mut Ui, data: &[u8], width: u32, height: u32, props: &WidgetProps) {
        // Create texture from RGBA data
        let size = [width as usize, height as usize];
        let image = egui::ColorImage::from_rgba_unmultiplied(size, data);
        let texture = ui.ctx().load_texture(
            format!("widget_image_{}", props.id.as_deref().unwrap_or("default")),
            image,
            egui::TextureOptions::default(),
        );

        let display_size = Vec2::new(width as f32, height as f32);
        let response = ui.add(egui::Image::new(&texture).fit_to_exact_size(display_size));

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_hbox(
        &mut self,
        ui: &mut Ui,
        children: &[WidgetDef],
        spacing: f32,
        _props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = spacing;
            for child in children {
                self.render(ui, child, events);
            }
        });
    }

    fn render_vbox(
        &mut self,
        ui: &mut Ui,
        children: &[WidgetDef],
        spacing: f32,
        _props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = spacing;
            for child in children {
                self.render(ui, child, events);
            }
        });
    }

    fn render_grid(
        &mut self,
        ui: &mut Ui,
        children: &[Vec<WidgetDef>],
        row_spacing: f32,
        col_spacing: f32,
        _props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        egui::Grid::new(ui.id().with("grid")).spacing(Vec2::new(col_spacing, row_spacing)).show(
            ui,
            |ui| {
                for row in children {
                    for cell in row {
                        self.render(ui, cell, events);
                    }
                    ui.end_row();
                }
            },
        );
    }

    fn render_panel(
        &mut self,
        ui: &mut Ui,
        child: &WidgetDef,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let padding = props.style.padding.left;
        let frame = if let Some([r, g, b, a]) = props.style.background_color {
            egui::Frame::default()
                .fill(Color32::from_rgba_unmultiplied(r, g, b, a))
                .inner_margin(padding)
        } else {
            egui::Frame::default().inner_margin(padding)
        };

        frame.show(ui, |ui| {
            self.render(ui, child, events);
        });
    }

    fn render_scroll_area(
        &mut self,
        ui: &mut Ui,
        child: &WidgetDef,
        max_height: Option<f32>,
        _props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let mut scroll = egui::ScrollArea::vertical();

        if let Some(height) = max_height {
            scroll = scroll.max_height(height);
        }

        scroll.show(ui, |ui| {
            self.render(ui, child, events);
        });
    }

    fn render_group(
        &mut self,
        ui: &mut Ui,
        title: Option<&str>,
        child: &WidgetDef,
        collapsed: bool,
        _props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        if let Some(title) = title {
            egui::CollapsingHeader::new(title).default_open(!collapsed).show(ui, |ui| {
                self.render(ui, child, events);
            });
        } else {
            ui.group(|ui| {
                self.render(ui, child, events);
            });
        }
    }

    fn render_dropdown(
        &mut self,
        ui: &mut Ui,
        options: &[String],
        initial_selected: Option<usize>,
        placeholder: Option<&str>,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();
        let state = self.state.entry(id.clone()).or_insert_with(|| WidgetState {
            value: initial_selected.map(|i| i as f32).unwrap_or(-1.0),
            ..Default::default()
        });

        let selected_idx = if state.value >= 0.0 { Some(state.value as usize) } else { None };
        let display_text =
            selected_idx.and_then(|i| options.get(i)).cloned().unwrap_or_else(|| {
                placeholder.map(|s| s.to_string()).unwrap_or_else(|| "Select...".to_string())
            });

        let response = egui::ComboBox::from_id_salt(ui.id().with(&id))
            .selected_text(display_text)
            .show_ui(ui, |ui| {
                for (idx, option) in options.iter().enumerate() {
                    let is_selected = selected_idx == Some(idx);
                    if ui.selectable_label(is_selected, option).clicked() {
                        state.value = idx as f32;
                        if !id.is_empty() {
                            events.push(WidgetEvent::SelectionChanged {
                                id: id.clone(),
                                index: idx,
                                value: option.clone(),
                            });
                        }
                    }
                }
            });

        if let Some(tooltip) = &props.tooltip {
            response.response.on_hover_text(tooltip);
        }
    }

    fn render_radio_group(
        &mut self,
        ui: &mut Ui,
        options: &[String],
        initial_selected: Option<usize>,
        horizontal: bool,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();
        let state = self.state.entry(id.clone()).or_insert_with(|| WidgetState {
            value: initial_selected.map(|i| i as f32).unwrap_or(-1.0),
            ..Default::default()
        });

        let selected_idx = if state.value >= 0.0 { Some(state.value as usize) } else { None };

        let render_options = |ui: &mut Ui| {
            for (idx, option) in options.iter().enumerate() {
                let is_selected = selected_idx == Some(idx);
                if ui.radio(is_selected, option).clicked() {
                    state.value = idx as f32;
                    if !id.is_empty() {
                        events.push(WidgetEvent::RadioChanged {
                            id: id.clone(),
                            index: idx,
                            value: option.clone(),
                        });
                    }
                }
            }
        };

        if horizontal {
            ui.horizontal(render_options);
        } else {
            ui.vertical(render_options);
        }
    }

    fn render_text_area(
        &mut self,
        ui: &mut Ui,
        initial_value: &str,
        placeholder: Option<&str>,
        rows: u32,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();
        let state = self.state.entry(id.clone()).or_insert_with(|| WidgetState {
            text: initial_value.to_string(),
            ..Default::default()
        });

        let mut text_edit = egui::TextEdit::multiline(&mut state.text).desired_rows(rows as usize);

        if let Some(hint) = placeholder {
            text_edit = text_edit.hint_text(hint);
        }

        if let Some(min_width) = props.style.min_width {
            text_edit = text_edit.desired_width(min_width);
        }

        let response = ui.add_enabled(props.enabled, text_edit);

        if response.changed() && !id.is_empty() {
            events.push(WidgetEvent::TextChanged { id: id.clone(), value: state.text.clone() });
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_tabs(
        &mut self,
        ui: &mut Ui,
        tabs: &[(String, Box<WidgetDef>)],
        initial_active: usize,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();
        let state = self
            .state
            .entry(id.clone())
            .or_insert_with(|| WidgetState { selected: initial_active, ..Default::default() });

        // Render tab bar
        ui.horizontal(|ui| {
            for (i, (label, _)) in tabs.iter().enumerate() {
                let is_active = state.selected == i;
                let tab_text = if is_active {
                    egui::RichText::new(label).strong()
                } else {
                    egui::RichText::new(label)
                };

                if ui.selectable_label(is_active, tab_text).clicked() && !is_active {
                    state.selected = i;
                    if !id.is_empty() {
                        events.push(WidgetEvent::TabChanged {
                            id: id.clone(),
                            index: i,
                            label: label.clone(),
                        });
                    }
                }
            }
        });

        ui.separator();

        // Render active tab content
        if let Some((_, content)) = tabs.get(state.selected) {
            self.render(ui, content, events);
        }
    }

    fn render_link(
        &self,
        ui: &mut Ui,
        text: &str,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let response = ui.add_enabled(props.enabled, egui::Link::new(text));

        if response.clicked() {
            if let Some(id) = &props.id {
                events.push(WidgetEvent::LinkClicked { id: id.clone() });
            }
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_selectable_label(
        &mut self,
        ui: &mut Ui,
        text: &str,
        initial_selected: bool,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();
        let state = self
            .state
            .entry(id.clone())
            .or_insert_with(|| WidgetState { checked: initial_selected, ..Default::default() });

        // Use Button::selected() instead of deprecated SelectableLabel
        let button = egui::Button::new(text).selected(state.checked);
        let response = ui.add_enabled(props.enabled, button);

        if response.clicked() {
            state.checked = !state.checked;
            if !id.is_empty() {
                events.push(WidgetEvent::SelectableLabelChanged {
                    id: id.clone(),
                    selected: state.checked,
                });
            }
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_drag_value(
        &mut self,
        ui: &mut Ui,
        initial_value: f64,
        min: Option<f64>,
        max: Option<f64>,
        speed: f64,
        prefix: Option<&str>,
        suffix: Option<&str>,
        decimals: Option<usize>,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();
        let state = self
            .state
            .entry(id.clone())
            .or_insert_with(|| WidgetState { value: initial_value as f32, ..Default::default() });

        // Store as f64 for precision
        let mut value = state.value as f64;
        let mut drag = egui::DragValue::new(&mut value).speed(speed);

        if let Some(min_val) = min {
            drag = drag.range(min_val..=max.unwrap_or(f64::MAX));
        } else if let Some(max_val) = max {
            drag = drag.range(f64::MIN..=max_val);
        }

        if let Some(prefix_str) = prefix {
            drag = drag.prefix(prefix_str);
        }

        if let Some(suffix_str) = suffix {
            drag = drag.suffix(suffix_str);
        }

        if let Some(dec) = decimals {
            drag = drag.max_decimals(dec);
        }

        let response = ui.add_enabled(props.enabled, drag);

        if response.changed() {
            state.value = value as f32;
            if !id.is_empty() {
                events.push(WidgetEvent::DragValueChanged { id: id.clone(), value });
            }
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_color_picker(
        &mut self,
        ui: &mut Ui,
        initial_color: [u8; 4],
        alpha: bool,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let id = props.id.clone().unwrap_or_default();

        // Store color in state - we need to use a separate storage approach
        // since WidgetState doesn't have a color field
        let state = self.state.entry(id.clone()).or_insert_with(|| {
            // Encode color in value field as 32-bit float
            let color_u32 = u32::from_le_bytes(initial_color);
            WidgetState { value: f32::from_bits(color_u32), ..Default::default() }
        });

        // Decode color from value
        let color_u32 = state.value.to_bits();
        let color_bytes = color_u32.to_le_bytes();
        let mut color = egui::Color32::from_rgba_unmultiplied(
            color_bytes[0],
            color_bytes[1],
            color_bytes[2],
            color_bytes[3],
        );

        let response = if alpha {
            ui.color_edit_button_srgba(&mut color)
        } else {
            // For no-alpha, use RGB only
            let mut rgb = [color.r(), color.g(), color.b()];
            let response = ui.color_edit_button_srgb(&mut rgb);
            color = egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
            response
        };

        if response.changed() {
            let new_color = [color.r(), color.g(), color.b(), color.a()];
            let new_color_u32 = u32::from_le_bytes(new_color);
            state.value = f32::from_bits(new_color_u32);

            if !id.is_empty() {
                events.push(WidgetEvent::ColorChanged { id: id.clone(), color: new_color });
            }
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    fn render_hyperlink(
        &self,
        ui: &mut Ui,
        text: &str,
        url: &str,
        _new_tab: bool,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        let response =
            ui.add_enabled(props.enabled, egui::Hyperlink::from_label_and_url(text, url));

        // Note: egui's Hyperlink automatically opens the URL on click
        // We emit an event for tracking purposes
        if response.clicked() {
            if let Some(id) = &props.id {
                events.push(WidgetEvent::HyperlinkClicked { id: id.clone(), url: url.to_string() });
            }
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_image_button(
        &self,
        ui: &mut Ui,
        data: &[u8],
        width: u32,
        height: u32,
        frame: bool,
        selected: bool,
        tint: Option<[u8; 4]>,
        props: &WidgetProps,
        events: &mut Vec<WidgetEvent>,
    ) {
        // Create texture from RGBA data
        let size = [width as usize, height as usize];
        let image_data = egui::ColorImage::from_rgba_unmultiplied(size, data);
        let texture = ui.ctx().load_texture(
            format!("image_button_{}", props.id.as_deref().unwrap_or("default")),
            image_data,
            egui::TextureOptions::default(),
        );

        let display_size = Vec2::new(width as f32, height as f32);
        let mut img = egui::Image::new(&texture).fit_to_exact_size(display_size);

        if let Some([r, g, b, a]) = tint {
            img = img.tint(Color32::from_rgba_unmultiplied(r, g, b, a));
        }

        // Use Button::image() instead of deprecated ImageButton
        let mut button = egui::Button::image(img).frame(frame).selected(selected);

        // For image buttons without frame, use minimal styling
        if !frame {
            button = button.fill(Color32::TRANSPARENT);
        }

        let response = ui.add_enabled(props.enabled, button);

        if response.clicked() {
            if let Some(id) = &props.id {
                events.push(WidgetEvent::ButtonClick { id: id.clone() });
            }
        }

        if let Some(tooltip) = &props.tooltip {
            response.on_hover_text(tooltip);
        }
    }

    /// Update widget state from external source (e.g., command)
    pub fn update_state(&mut self, id: &WidgetId, update: WidgetStateUpdate) {
        if let Some(state) = self.state.get_mut(id) {
            match update {
                WidgetStateUpdate::SetText(text) => state.text = text,
                WidgetStateUpdate::SetChecked(checked) => state.checked = checked,
                WidgetStateUpdate::SetValue(value) => state.value = value,
            }
        }
    }

    /// Clear all widget state
    pub fn clear_state(&mut self) {
        self.state.clear();
    }
}

/// Updates that can be applied to widget state
#[derive(Debug, Clone)]
pub enum WidgetStateUpdate {
    SetText(String),
    SetChecked(bool),
    SetValue(f32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = WidgetRenderer::new();
        assert!(renderer.state.is_empty());
    }

    #[test]
    fn test_state_update() {
        let mut renderer = WidgetRenderer::new();
        renderer.state.insert(
            "test".to_string(),
            WidgetState { text: "initial".to_string(), ..Default::default() },
        );

        renderer
            .update_state(&"test".to_string(), WidgetStateUpdate::SetText("updated".to_string()));

        assert_eq!(renderer.state.get("test").unwrap().text, "updated");
    }
}
