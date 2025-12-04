//! Widget event types

/// Unique identifier for widgets
pub type WidgetId = String;

/// Events that widgets can emit
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetEvent {
    /// Button was clicked
    ButtonClick { id: WidgetId },

    /// Text input value changed
    TextChanged { id: WidgetId, value: String },

    /// Text input submitted (Enter pressed)
    TextSubmit { id: WidgetId, value: String },

    /// Checkbox state changed
    CheckboxChanged { id: WidgetId, checked: bool },

    /// Slider value changed
    SliderChanged { id: WidgetId, value: f32 },

    /// Widget gained focus
    FocusGained { id: WidgetId },

    /// Widget lost focus
    FocusLost { id: WidgetId },

    /// Mouse entered widget area
    MouseEnter { id: WidgetId },

    /// Mouse left widget area
    MouseLeave { id: WidgetId },

    /// Dropdown selection changed
    SelectionChanged { id: WidgetId, index: usize, value: String },

    /// Radio button selection changed
    RadioChanged { id: WidgetId, index: usize, value: String },

    /// Tab selection changed
    TabChanged { id: WidgetId, index: usize, label: String },

    /// Link was clicked
    LinkClicked { id: WidgetId },

    /// SelectableLabel selection state changed
    SelectableLabelChanged { id: WidgetId, selected: bool },

    /// DragValue value changed
    DragValueChanged { id: WidgetId, value: f64 },

    /// Color picker value changed
    ColorChanged { id: WidgetId, color: [u8; 4] },

    /// Hyperlink was clicked (with URL)
    HyperlinkClicked { id: WidgetId, url: String },

    /// File dialog completed
    FileDialogCompleted { id: WidgetId, paths: Vec<String>, cancelled: bool },

    /// Font selection changed
    FontChanged { id: WidgetId, family: String },
}

impl WidgetEvent {
    /// Get the widget ID associated with this event
    pub fn widget_id(&self) -> &WidgetId {
        match self {
            WidgetEvent::ButtonClick { id } => id,
            WidgetEvent::TextChanged { id, .. } => id,
            WidgetEvent::TextSubmit { id, .. } => id,
            WidgetEvent::CheckboxChanged { id, .. } => id,
            WidgetEvent::SliderChanged { id, .. } => id,
            WidgetEvent::FocusGained { id } => id,
            WidgetEvent::FocusLost { id } => id,
            WidgetEvent::MouseEnter { id } => id,
            WidgetEvent::MouseLeave { id } => id,
            WidgetEvent::SelectionChanged { id, .. } => id,
            WidgetEvent::RadioChanged { id, .. } => id,
            WidgetEvent::TabChanged { id, .. } => id,
            WidgetEvent::LinkClicked { id } => id,
            WidgetEvent::SelectableLabelChanged { id, .. } => id,
            WidgetEvent::DragValueChanged { id, .. } => id,
            WidgetEvent::ColorChanged { id, .. } => id,
            WidgetEvent::HyperlinkClicked { id, .. } => id,
            WidgetEvent::FileDialogCompleted { id, .. } => id,
            WidgetEvent::FontChanged { id, .. } => id,
        }
    }

    /// Get the event type name as a string
    pub fn event_type(&self) -> &'static str {
        match self {
            WidgetEvent::ButtonClick { .. } => "button_click",
            WidgetEvent::TextChanged { .. } => "text_changed",
            WidgetEvent::TextSubmit { .. } => "text_submit",
            WidgetEvent::CheckboxChanged { .. } => "checkbox_changed",
            WidgetEvent::SliderChanged { .. } => "slider_changed",
            WidgetEvent::FocusGained { .. } => "focus_gained",
            WidgetEvent::FocusLost { .. } => "focus_lost",
            WidgetEvent::MouseEnter { .. } => "mouse_enter",
            WidgetEvent::MouseLeave { .. } => "mouse_leave",
            WidgetEvent::SelectionChanged { .. } => "selection_changed",
            WidgetEvent::RadioChanged { .. } => "radio_changed",
            WidgetEvent::TabChanged { .. } => "tab_changed",
            WidgetEvent::LinkClicked { .. } => "link_clicked",
            WidgetEvent::SelectableLabelChanged { .. } => "selectable_label_changed",
            WidgetEvent::DragValueChanged { .. } => "drag_value_changed",
            WidgetEvent::ColorChanged { .. } => "color_changed",
            WidgetEvent::HyperlinkClicked { .. } => "hyperlink_clicked",
            WidgetEvent::FileDialogCompleted { .. } => "file_dialog_completed",
            WidgetEvent::FontChanged { .. } => "font_changed",
        }
    }
}

/// Dialog result for modal dialogs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogResult {
    /// User clicked OK/Confirm
    Ok,
    /// User clicked Cancel/dismissed
    Cancel,
    /// User selected a custom option
    Custom(String),
}

/// File dialog result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDialogResult {
    /// Selected file path(s), empty if cancelled
    pub paths: Vec<String>,
    /// Whether the dialog was cancelled
    pub cancelled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_event_id() {
        let event = WidgetEvent::ButtonClick { id: "btn1".to_string() };
        assert_eq!(event.widget_id(), "btn1");
        assert_eq!(event.event_type(), "button_click");
    }

    #[test]
    fn test_text_changed_event() {
        let event =
            WidgetEvent::TextChanged { id: "input1".to_string(), value: "hello".to_string() };
        assert_eq!(event.widget_id(), "input1");
        if let WidgetEvent::TextChanged { value, .. } = event {
            assert_eq!(value, "hello");
        }
    }
}
