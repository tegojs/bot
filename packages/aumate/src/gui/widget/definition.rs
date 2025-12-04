//! Widget definition types
//!
//! This module defines the `WidgetDef` enum which represents all possible widgets
//! in a declarative manner. These definitions are serializable and can be sent
//! across thread boundaries.

use super::events::WidgetId;
use super::style::WidgetStyle;

/// Base properties shared by all widgets
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WidgetProps {
    /// Optional unique identifier for event targeting
    pub id: Option<WidgetId>,
    /// Whether the widget is visible
    pub visible: bool,
    /// Whether the widget is enabled (interactive)
    pub enabled: bool,
    /// Optional tooltip text
    pub tooltip: Option<String>,
    /// Widget styling
    pub style: WidgetStyle,
}

impl WidgetProps {
    /// Create default widget properties (visible and enabled)
    pub fn new() -> Self {
        Self {
            id: None,
            visible: true,
            enabled: true,
            tooltip: None,
            style: WidgetStyle::default(),
        }
    }

    /// Set the widget ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set visibility
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set style
    pub fn with_style(mut self, style: WidgetStyle) -> Self {
        self.style = style;
        self
    }
}

/// Widget definition - describes a UI widget declaratively
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetDef {
    // ==================== Basic Widgets ====================
    /// Static text label
    Label { text: String, props: WidgetProps },

    /// Clickable button
    Button { text: String, props: WidgetProps },

    /// Single-line text input
    TextInput { value: String, placeholder: Option<String>, password: bool, props: WidgetProps },

    /// Checkbox with label
    Checkbox { checked: bool, label: String, props: WidgetProps },

    /// Numeric slider
    Slider { value: f32, min: f32, max: f32, step: Option<f32>, props: WidgetProps },

    /// Progress bar
    ProgressBar { value: f32, show_percentage: bool, props: WidgetProps },

    /// Image display
    Image { data: Vec<u8>, width: u32, height: u32, props: WidgetProps },

    /// Horizontal separator line
    Separator { props: WidgetProps },

    /// Empty spacer
    Spacer { size: f32, props: WidgetProps },

    // ==================== Layout Widgets ====================
    /// Horizontal layout (children arranged left to right)
    HBox { children: Vec<WidgetDef>, spacing: f32, props: WidgetProps },

    /// Vertical layout (children arranged top to bottom)
    VBox { children: Vec<WidgetDef>, spacing: f32, props: WidgetProps },

    /// Grid layout (rows and columns)
    Grid { children: Vec<Vec<WidgetDef>>, row_spacing: f32, col_spacing: f32, props: WidgetProps },

    // ==================== Container Widgets ====================
    /// Panel with optional background
    Panel { child: Box<WidgetDef>, props: WidgetProps },

    /// Scrollable area
    ScrollArea { child: Box<WidgetDef>, max_height: Option<f32>, props: WidgetProps },

    /// Collapsible group with title
    Group { title: Option<String>, child: Box<WidgetDef>, collapsed: bool, props: WidgetProps },

    // ==================== Advanced Widgets ====================
    /// Dropdown select widget
    Dropdown {
        options: Vec<String>,
        selected: Option<usize>,
        placeholder: Option<String>,
        props: WidgetProps,
    },

    /// Radio button group
    RadioGroup {
        options: Vec<String>,
        selected: Option<usize>,
        horizontal: bool,
        props: WidgetProps,
    },

    /// Multi-line text area
    TextArea { value: String, placeholder: Option<String>, rows: u32, props: WidgetProps },

    /// Tab container with multiple pages
    Tabs { tabs: Vec<(String, Box<WidgetDef>)>, active: usize, props: WidgetProps },

    /// Clickable text link (fires event on click)
    Link { text: String, props: WidgetProps },

    /// Selectable label (toggle state)
    SelectableLabel { text: String, selected: bool, props: WidgetProps },

    /// Numeric input with drag-to-change
    DragValue {
        value: f64,
        min: Option<f64>,
        max: Option<f64>,
        speed: f64,
        prefix: Option<String>,
        suffix: Option<String>,
        decimals: Option<usize>,
        props: WidgetProps,
    },

    /// Color picker widget
    ColorPicker { color: [u8; 4], alpha: bool, props: WidgetProps },

    /// Clickable URL hyperlink
    Hyperlink { text: String, url: String, new_tab: bool, props: WidgetProps },

    /// Button with an image
    ImageButton {
        data: Vec<u8>,
        width: u32,
        height: u32,
        frame: bool,
        selected: bool,
        tint: Option<[u8; 4]>,
        props: WidgetProps,
    },
}

impl WidgetDef {
    // ==================== Basic Widget Constructors ====================

    /// Create a label widget
    pub fn label(text: impl Into<String>) -> Self {
        Self::Label { text: text.into(), props: WidgetProps::new() }
    }

    /// Create a button widget
    pub fn button(text: impl Into<String>) -> Self {
        Self::Button { text: text.into(), props: WidgetProps::new() }
    }

    /// Create a text input widget
    pub fn text_input() -> Self {
        Self::TextInput {
            value: String::new(),
            placeholder: None,
            password: false,
            props: WidgetProps::new(),
        }
    }

    /// Create a text input with initial value
    pub fn text_input_with_value(value: impl Into<String>) -> Self {
        Self::TextInput {
            value: value.into(),
            placeholder: None,
            password: false,
            props: WidgetProps::new(),
        }
    }

    /// Create a checkbox widget
    pub fn checkbox(label: impl Into<String>, checked: bool) -> Self {
        Self::Checkbox { checked, label: label.into(), props: WidgetProps::new() }
    }

    /// Create a slider widget
    pub fn slider(value: f32, min: f32, max: f32) -> Self {
        Self::Slider { value, min, max, step: None, props: WidgetProps::new() }
    }

    /// Create a progress bar widget
    pub fn progress_bar(value: f32) -> Self {
        Self::ProgressBar { value, show_percentage: true, props: WidgetProps::new() }
    }

    /// Create an image widget from RGBA data
    pub fn image(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self::Image { data, width, height, props: WidgetProps::new() }
    }

    /// Create a horizontal separator
    pub fn separator() -> Self {
        Self::Separator { props: WidgetProps::new() }
    }

    /// Create a spacer
    pub fn spacer(size: f32) -> Self {
        Self::Spacer { size, props: WidgetProps::new() }
    }

    // ==================== Layout Widget Constructors ====================

    /// Create a horizontal box layout
    pub fn hbox(children: Vec<WidgetDef>) -> Self {
        Self::HBox { children, spacing: 4.0, props: WidgetProps::new() }
    }

    /// Create a vertical box layout
    pub fn vbox(children: Vec<WidgetDef>) -> Self {
        Self::VBox { children, spacing: 4.0, props: WidgetProps::new() }
    }

    /// Create a grid layout
    pub fn grid(children: Vec<Vec<WidgetDef>>) -> Self {
        Self::Grid { children, row_spacing: 4.0, col_spacing: 4.0, props: WidgetProps::new() }
    }

    // ==================== Container Widget Constructors ====================

    /// Create a panel container
    pub fn panel(child: WidgetDef) -> Self {
        Self::Panel { child: Box::new(child), props: WidgetProps::new() }
    }

    /// Create a scroll area container
    pub fn scroll_area(child: WidgetDef) -> Self {
        Self::ScrollArea { child: Box::new(child), max_height: None, props: WidgetProps::new() }
    }

    /// Create a collapsible group
    pub fn group(title: impl Into<String>, child: WidgetDef) -> Self {
        Self::Group {
            title: Some(title.into()),
            child: Box::new(child),
            collapsed: false,
            props: WidgetProps::new(),
        }
    }

    // ==================== Advanced Widget Constructors ====================

    /// Create a dropdown select widget
    pub fn dropdown(options: Vec<String>) -> Self {
        Self::Dropdown { options, selected: None, placeholder: None, props: WidgetProps::new() }
    }

    /// Create a radio button group
    pub fn radio_group(options: Vec<String>) -> Self {
        Self::RadioGroup { options, selected: None, horizontal: false, props: WidgetProps::new() }
    }

    /// Create a multi-line text area
    pub fn text_area() -> Self {
        Self::TextArea {
            value: String::new(),
            placeholder: None,
            rows: 4,
            props: WidgetProps::new(),
        }
    }

    /// Create a multi-line text area with initial value
    pub fn text_area_with_value(value: impl Into<String>) -> Self {
        Self::TextArea {
            value: value.into(),
            placeholder: None,
            rows: 4,
            props: WidgetProps::new(),
        }
    }

    /// Create a tab container
    pub fn tabs(tabs: Vec<(String, WidgetDef)>) -> Self {
        let boxed_tabs: Vec<(String, Box<WidgetDef>)> =
            tabs.into_iter().map(|(label, content)| (label, Box::new(content))).collect();
        Self::Tabs { tabs: boxed_tabs, active: 0, props: WidgetProps::new() }
    }

    /// Create a link widget (clickable text that fires an event)
    pub fn link(text: impl Into<String>) -> Self {
        Self::Link { text: text.into(), props: WidgetProps::new() }
    }

    /// Create a selectable label widget
    pub fn selectable_label(text: impl Into<String>, selected: bool) -> Self {
        Self::SelectableLabel { text: text.into(), selected, props: WidgetProps::new() }
    }

    /// Create a drag value widget for numeric input
    pub fn drag_value(value: f64) -> Self {
        Self::DragValue {
            value,
            min: None,
            max: None,
            speed: 1.0,
            prefix: None,
            suffix: None,
            decimals: None,
            props: WidgetProps::new(),
        }
    }

    /// Create a color picker widget
    pub fn color_picker(color: [u8; 4]) -> Self {
        Self::ColorPicker { color, alpha: true, props: WidgetProps::new() }
    }

    /// Create a hyperlink widget (opens URL in browser)
    pub fn hyperlink(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self::Hyperlink {
            text: text.into(),
            url: url.into(),
            new_tab: true,
            props: WidgetProps::new(),
        }
    }

    /// Create a hyperlink widget with URL as both text and link
    pub fn hyperlink_url(url: impl Into<String>) -> Self {
        let url_str = url.into();
        Self::Hyperlink {
            text: url_str.clone(),
            url: url_str,
            new_tab: true,
            props: WidgetProps::new(),
        }
    }

    /// Create an image button widget
    pub fn image_button(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self::ImageButton {
            data,
            width,
            height,
            frame: true,
            selected: false,
            tint: None,
            props: WidgetProps::new(),
        }
    }

    // ==================== Builder Methods ====================

    /// Set the widget ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.props_mut().id = Some(id.into());
        self
    }

    /// Set visibility
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.props_mut().visible = visible;
        self
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.props_mut().enabled = enabled;
        self
    }

    /// Set tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.props_mut().tooltip = Some(tooltip.into());
        self
    }

    /// Set style
    pub fn with_style(mut self, style: WidgetStyle) -> Self {
        self.props_mut().style = style;
        self
    }

    /// Set spacing for layout widgets (HBox, VBox)
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        match &mut self {
            WidgetDef::HBox { spacing: s, .. } => *s = spacing,
            WidgetDef::VBox { spacing: s, .. } => *s = spacing,
            WidgetDef::Grid { row_spacing, col_spacing, .. } => {
                *row_spacing = spacing;
                *col_spacing = spacing;
            }
            _ => {}
        }
        self
    }

    /// Set placeholder for text input
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        if let WidgetDef::TextInput { placeholder: p, .. } = &mut self {
            *p = Some(placeholder.into());
        }
        self
    }

    /// Set password mode for text input
    pub fn with_password(mut self, password: bool) -> Self {
        if let WidgetDef::TextInput { password: p, .. } = &mut self {
            *p = password;
        }
        self
    }

    /// Set step for slider
    pub fn with_step(mut self, step: f32) -> Self {
        if let WidgetDef::Slider { step: s, .. } = &mut self {
            *s = Some(step);
        }
        self
    }

    /// Set max height for scroll area
    pub fn with_max_height(mut self, height: f32) -> Self {
        if let WidgetDef::ScrollArea { max_height, .. } = &mut self {
            *max_height = Some(height);
        }
        self
    }

    /// Set collapsed state for group
    pub fn with_collapsed(mut self, collapsed: bool) -> Self {
        if let WidgetDef::Group { collapsed: c, .. } = &mut self {
            *c = collapsed;
        }
        self
    }

    /// Set selected index for dropdown or radio group
    pub fn with_selected(mut self, index: usize) -> Self {
        match &mut self {
            WidgetDef::Dropdown { selected, .. } => *selected = Some(index),
            WidgetDef::RadioGroup { selected, .. } => *selected = Some(index),
            _ => {}
        }
        self
    }

    /// Set horizontal layout for radio group
    pub fn with_horizontal(mut self, horizontal: bool) -> Self {
        if let WidgetDef::RadioGroup { horizontal: h, .. } = &mut self {
            *h = horizontal;
        }
        self
    }

    /// Set number of rows for text area
    pub fn with_rows(mut self, rows: u32) -> Self {
        if let WidgetDef::TextArea { rows: r, .. } = &mut self {
            *r = rows;
        }
        self
    }

    /// Set active tab index for tabs widget
    pub fn with_active(mut self, index: usize) -> Self {
        if let WidgetDef::Tabs { active, .. } = &mut self {
            *active = index;
        }
        self
    }

    /// Set range for drag value widget
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        if let WidgetDef::DragValue { min: mi, max: ma, .. } = &mut self {
            *mi = Some(min);
            *ma = Some(max);
        }
        self
    }

    /// Set speed for drag value widget
    pub fn with_speed(mut self, speed: f64) -> Self {
        if let WidgetDef::DragValue { speed: s, .. } = &mut self {
            *s = speed;
        }
        self
    }

    /// Set prefix for drag value widget
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        if let WidgetDef::DragValue { prefix: p, .. } = &mut self {
            *p = Some(prefix.into());
        }
        self
    }

    /// Set suffix for drag value widget
    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        if let WidgetDef::DragValue { suffix: s, .. } = &mut self {
            *s = Some(suffix.into());
        }
        self
    }

    /// Set decimal places for drag value widget
    pub fn with_decimals(mut self, decimals: usize) -> Self {
        if let WidgetDef::DragValue { decimals: d, .. } = &mut self {
            *d = Some(decimals);
        }
        self
    }

    /// Set alpha channel display for color picker
    pub fn with_alpha(mut self, alpha: bool) -> Self {
        if let WidgetDef::ColorPicker { alpha: a, .. } = &mut self {
            *a = alpha;
        }
        self
    }

    /// Set new tab behavior for hyperlink
    pub fn with_new_tab(mut self, new_tab: bool) -> Self {
        if let WidgetDef::Hyperlink { new_tab: n, .. } = &mut self {
            *n = new_tab;
        }
        self
    }

    /// Set frame visibility for image button
    pub fn with_frame(mut self, frame: bool) -> Self {
        if let WidgetDef::ImageButton { frame: f, .. } = &mut self {
            *f = frame;
        }
        self
    }

    /// Set selected state for image button
    pub fn with_image_selected(mut self, selected: bool) -> Self {
        if let WidgetDef::ImageButton { selected: s, .. } = &mut self {
            *s = selected;
        }
        self
    }

    /// Set tint color for image button
    pub fn with_tint(mut self, tint: [u8; 4]) -> Self {
        if let WidgetDef::ImageButton { tint: t, .. } = &mut self {
            *t = Some(tint);
        }
        self
    }

    // ==================== Helper Methods ====================

    /// Get mutable reference to widget props
    fn props_mut(&mut self) -> &mut WidgetProps {
        match self {
            WidgetDef::Label { props, .. } => props,
            WidgetDef::Button { props, .. } => props,
            WidgetDef::TextInput { props, .. } => props,
            WidgetDef::Checkbox { props, .. } => props,
            WidgetDef::Slider { props, .. } => props,
            WidgetDef::ProgressBar { props, .. } => props,
            WidgetDef::Image { props, .. } => props,
            WidgetDef::Separator { props } => props,
            WidgetDef::Spacer { props, .. } => props,
            WidgetDef::HBox { props, .. } => props,
            WidgetDef::VBox { props, .. } => props,
            WidgetDef::Grid { props, .. } => props,
            WidgetDef::Panel { props, .. } => props,
            WidgetDef::ScrollArea { props, .. } => props,
            WidgetDef::Group { props, .. } => props,
            WidgetDef::Dropdown { props, .. } => props,
            WidgetDef::RadioGroup { props, .. } => props,
            WidgetDef::TextArea { props, .. } => props,
            WidgetDef::Tabs { props, .. } => props,
            WidgetDef::Link { props, .. } => props,
            WidgetDef::SelectableLabel { props, .. } => props,
            WidgetDef::DragValue { props, .. } => props,
            WidgetDef::ColorPicker { props, .. } => props,
            WidgetDef::Hyperlink { props, .. } => props,
            WidgetDef::ImageButton { props, .. } => props,
        }
    }

    /// Get immutable reference to widget props
    pub fn props(&self) -> &WidgetProps {
        match self {
            WidgetDef::Label { props, .. } => props,
            WidgetDef::Button { props, .. } => props,
            WidgetDef::TextInput { props, .. } => props,
            WidgetDef::Checkbox { props, .. } => props,
            WidgetDef::Slider { props, .. } => props,
            WidgetDef::ProgressBar { props, .. } => props,
            WidgetDef::Image { props, .. } => props,
            WidgetDef::Separator { props } => props,
            WidgetDef::Spacer { props, .. } => props,
            WidgetDef::HBox { props, .. } => props,
            WidgetDef::VBox { props, .. } => props,
            WidgetDef::Grid { props, .. } => props,
            WidgetDef::Panel { props, .. } => props,
            WidgetDef::ScrollArea { props, .. } => props,
            WidgetDef::Group { props, .. } => props,
            WidgetDef::Dropdown { props, .. } => props,
            WidgetDef::RadioGroup { props, .. } => props,
            WidgetDef::TextArea { props, .. } => props,
            WidgetDef::Tabs { props, .. } => props,
            WidgetDef::Link { props, .. } => props,
            WidgetDef::SelectableLabel { props, .. } => props,
            WidgetDef::DragValue { props, .. } => props,
            WidgetDef::ColorPicker { props, .. } => props,
            WidgetDef::Hyperlink { props, .. } => props,
            WidgetDef::ImageButton { props, .. } => props,
        }
    }

    /// Get the widget ID if set
    pub fn id(&self) -> Option<&WidgetId> {
        self.props().id.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_creation() {
        let label = WidgetDef::label("Hello");
        if let WidgetDef::Label { text, props } = label {
            assert_eq!(text, "Hello");
            assert!(props.visible);
            assert!(props.enabled);
        } else {
            panic!("Expected Label widget");
        }
    }

    #[test]
    fn test_button_with_id() {
        let button = WidgetDef::button("Click").with_id("btn1");
        assert_eq!(button.id(), Some(&"btn1".to_string()));
    }

    #[test]
    fn test_vbox_with_spacing() {
        let vbox =
            WidgetDef::vbox(vec![WidgetDef::label("A"), WidgetDef::label("B")]).with_spacing(10.0);

        if let WidgetDef::VBox { spacing, children, .. } = vbox {
            assert_eq!(spacing, 10.0);
            assert_eq!(children.len(), 2);
        } else {
            panic!("Expected VBox widget");
        }
    }

    #[test]
    fn test_text_input_with_placeholder() {
        let input = WidgetDef::text_input().with_placeholder("Enter name").with_password(false);

        if let WidgetDef::TextInput { placeholder, password, .. } = input {
            assert_eq!(placeholder, Some("Enter name".to_string()));
            assert!(!password);
        } else {
            panic!("Expected TextInput widget");
        }
    }

    #[test]
    fn test_nested_layout() {
        let ui = WidgetDef::vbox(vec![
            WidgetDef::label("Title"),
            WidgetDef::hbox(vec![WidgetDef::button("OK"), WidgetDef::button("Cancel")]),
        ]);

        if let WidgetDef::VBox { children, .. } = ui {
            assert_eq!(children.len(), 2);
            assert!(matches!(children[0], WidgetDef::Label { .. }));
            assert!(matches!(children[1], WidgetDef::HBox { .. }));
        } else {
            panic!("Expected VBox widget");
        }
    }
}
