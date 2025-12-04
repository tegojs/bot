//! GUI module - napi bindings for aumate widget system
//!
//! This module provides JavaScript bindings for the aumate GUI widget system,
//! allowing creation of declarative UIs from JavaScript/TypeScript.

// Include all GUI code inline to avoid napi re-export issues
#![allow(dead_code)]

use aumate::gui::prelude::*;
use aumate::gui::widget::WidgetDef;
use aumate::gui::window::commands::{CommandSender, WidgetEventSender, WindowCommand};
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

// ============================================================================
// Global GUI State
// ============================================================================

struct GuiState {
    sender: CommandSender,
    handle: Option<JoinHandle<()>>,
    next_window_id: u64,
    /// Callbacks by window name
    window_callbacks: HashMap<String, Arc<ThreadsafeFunction<JsWidgetEvent>>>,
    /// Event receiver for widget events from GUI thread
    event_receiver: Option<mpsc::Receiver<(String, WidgetEvent)>>,
    /// Event sender to pass to GUI thread
    event_sender: Option<WidgetEventSender>,
    /// Queue of pending events for polling API
    pending_events: Vec<(String, WidgetEvent)>,
}

static GUI_STATE: once_cell::sync::Lazy<Mutex<Option<GuiState>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(None));

// ============================================================================
// GuiApp Class
// ============================================================================

/// GUI Application that manages the GUI thread and windows.
///
/// Create a single GuiApp instance to spawn the GUI thread, then use it
/// to create windows with widget content.
///
/// @example
/// ```javascript
/// const app = new GuiApp();
/// const win = app.createWindow({ title: 'Hello', width: 300, height: 200 });
/// win.setContent(vbox([label('Hello World!'), button('Close').withId('close')]));
/// win.show();
/// app.run(); // Blocks until all windows are closed
/// ```
#[napi]
pub struct GuiApp {}

#[napi]
impl GuiApp {
    /// Create a new GUI application.
    /// This spawns the GUI thread in the background.
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        let mut state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;

        if state.is_some() {
            return Err(Error::from_reason(
                "GuiApp already initialized. Only one instance is allowed.",
            ));
        }

        let (sender, handle) = FloatingWindow::spawn_controller().map_err(Error::from_reason)?;

        // Create event channel for widget events
        let (event_tx, event_rx) = mpsc::channel();

        *state = Some(GuiState {
            sender,
            handle: Some(handle),
            next_window_id: 1,
            window_callbacks: HashMap::new(),
            event_receiver: Some(event_rx),
            event_sender: Some(event_tx),
            pending_events: Vec::new(),
        });

        Ok(GuiApp {})
    }

    /// Create a new window with the given configuration.
    #[napi]
    pub fn create_window(&self, config: JsWindowConfig) -> Result<GuiWindow> {
        GuiWindow::new(config)
    }

    /// Run the GUI event loop. This blocks until all windows are closed
    /// or `exit()` is called.
    ///
    /// On macOS, this runs the event loop on the current (main) thread.
    /// On other platforms, the event loop runs in a background thread.
    ///
    /// **Note**: For event callbacks to work, use `runAsync()` instead which
    /// integrates with Node.js's event loop.
    #[napi]
    pub fn run(&self) -> Result<()> {
        // On macOS, run the event loop on the main thread (blocking)
        FloatingWindow::run_event_loop().map_err(Error::from_reason)?;

        // Wait for the handle thread to finish
        let handle = {
            let mut state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
            state.as_mut().and_then(|s| s.handle.take())
        };

        if let Some(h) = handle {
            let _ = h.join();
        }

        Ok(())
    }

    /// Initialize the event loop for non-blocking mode.
    ///
    /// Call this once before using `runOnce()`. This sets up the GUI
    /// event loop for incremental pumping.
    #[napi]
    pub fn init(&self) -> Result<()> {
        FloatingWindow::init_event_loop().map_err(Error::from_reason)
    }

    /// Pump the GUI event loop once (non-blocking).
    ///
    /// Returns `true` if the app should continue running, `false` if it should exit.
    /// Call `init()` once before using this method.
    ///
    /// Usage pattern:
    /// ```javascript
    /// app.init();
    /// function pump() {
    ///   if (app.runOnce()) {
    ///     const events = app.pollEvents();
    ///     events.forEach(handleEvent);
    ///     setImmediate(pump);
    ///   }
    /// }
    /// pump();
    /// ```
    #[napi]
    pub fn run_once(&self) -> Result<bool> {
        FloatingWindow::run_event_loop_once().map_err(Error::from_reason)
    }

    /// Poll for pending widget events.
    ///
    /// This drains the event receiver and returns all pending events.
    /// Call this after `runOnce()` to get events that occurred.
    #[napi]
    pub fn poll_events(&self) -> Result<Vec<JsWidgetEvent>> {
        let mut events = Vec::new();

        let mut state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
        if let Some(ref mut s) = *state {
            // First, drain any events from the receiver into pending_events
            if let Some(ref rx) = s.event_receiver {
                while let Ok((window_name, event)) = rx.try_recv() {
                    s.pending_events.push((window_name, event));
                }
            }

            // Now convert pending events to JS events
            for (_window_name, event) in s.pending_events.drain(..) {
                let js_event: JsWidgetEvent = event.into();
                events.push(js_event);
            }
        }

        Ok(events)
    }

    /// Exit the GUI application and close all windows.
    #[napi]
    pub fn exit(&self) -> Result<()> {
        let state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
        if let Some(ref s) = *state {
            let _ = s.sender.send(WindowCommand::ExitApplication);
        }
        Ok(())
    }
}

// ============================================================================
// Window Configuration
// ============================================================================

/// Window configuration options
#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct JsWindowConfig {
    /// Window title
    pub title: Option<String>,
    /// Window width in pixels
    pub width: Option<u32>,
    /// Window height in pixels
    pub height: Option<u32>,
    /// X position on screen
    pub x: Option<f64>,
    /// Y position on screen
    pub y: Option<f64>,
    /// Whether window is always on top
    pub always_on_top: Option<bool>,
    /// Whether window is resizable
    pub resizable: Option<bool>,
    /// Whether window has decorations (title bar, borders)
    pub decorations: Option<bool>,
    /// Whether window is transparent
    pub transparent: Option<bool>,
}

// ============================================================================
// Widget Event
// ============================================================================

/// Widget event from UI interactions
#[napi(object)]
#[derive(Debug, Clone)]
pub struct JsWidgetEvent {
    /// Event type: "button_click", "text_changed", "text_submit", "checkbox_changed", "slider_changed", etc.
    pub event_type: String,
    /// Widget ID that triggered the event
    pub widget_id: String,
    /// String value (for text events)
    pub value: Option<String>,
    /// Boolean value (for checkbox, selectable_label events)
    pub checked: Option<bool>,
    /// Numeric value (for slider, drag_value events)
    pub number_value: Option<f64>,
    /// Color value as [r, g, b, a] (for color_changed events)
    pub color: Option<Vec<u8>>,
    /// File paths (for file_dialog_completed events)
    pub paths: Option<Vec<String>>,
    /// Whether a dialog was cancelled (for file_dialog_completed events)
    pub cancelled: Option<bool>,
}

impl From<WidgetEvent> for JsWidgetEvent {
    fn from(event: WidgetEvent) -> Self {
        match event {
            WidgetEvent::ButtonClick { id } => JsWidgetEvent {
                event_type: "button_click".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::TextChanged { id, value } => JsWidgetEvent {
                event_type: "text_changed".to_string(),
                widget_id: id,
                value: Some(value),
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::TextSubmit { id, value } => JsWidgetEvent {
                event_type: "text_submit".to_string(),
                widget_id: id,
                value: Some(value),
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::CheckboxChanged { id, checked } => JsWidgetEvent {
                event_type: "checkbox_changed".to_string(),
                widget_id: id,
                value: None,
                checked: Some(checked),
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::SliderChanged { id, value } => JsWidgetEvent {
                event_type: "slider_changed".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: Some(value as f64),
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::FocusGained { id } => JsWidgetEvent {
                event_type: "focus_gained".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::FocusLost { id } => JsWidgetEvent {
                event_type: "focus_lost".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::MouseEnter { id } => JsWidgetEvent {
                event_type: "mouse_enter".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::MouseLeave { id } => JsWidgetEvent {
                event_type: "mouse_leave".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::SelectionChanged { id, index, value } => JsWidgetEvent {
                event_type: "selection_changed".to_string(),
                widget_id: id,
                value: Some(value),
                checked: None,
                number_value: Some(index as f64),
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::RadioChanged { id, index, value } => JsWidgetEvent {
                event_type: "radio_changed".to_string(),
                widget_id: id,
                value: Some(value),
                checked: None,
                number_value: Some(index as f64),
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::TabChanged { id, index, label } => JsWidgetEvent {
                event_type: "tab_changed".to_string(),
                widget_id: id,
                value: Some(label),
                checked: None,
                number_value: Some(index as f64),
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::LinkClicked { id } => JsWidgetEvent {
                event_type: "link_clicked".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::SelectableLabelChanged { id, selected } => JsWidgetEvent {
                event_type: "selectable_label_changed".to_string(),
                widget_id: id,
                value: None,
                checked: Some(selected),
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::DragValueChanged { id, value } => JsWidgetEvent {
                event_type: "drag_value_changed".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: Some(value),
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::ColorChanged { id, color } => JsWidgetEvent {
                event_type: "color_changed".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: None,
                color: Some(color.to_vec()),
                paths: None,
                cancelled: None,
            },
            WidgetEvent::HyperlinkClicked { id, url } => JsWidgetEvent {
                event_type: "hyperlink_clicked".to_string(),
                widget_id: id,
                value: Some(url),
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
            WidgetEvent::FileDialogCompleted { id, paths, cancelled } => JsWidgetEvent {
                event_type: "file_dialog_completed".to_string(),
                widget_id: id,
                value: None,
                checked: None,
                number_value: None,
                color: None,
                paths: Some(paths),
                cancelled: Some(cancelled),
            },
            WidgetEvent::FontChanged { id, family } => JsWidgetEvent {
                event_type: "font_changed".to_string(),
                widget_id: id,
                value: Some(family),
                checked: None,
                number_value: None,
                color: None,
                paths: None,
                cancelled: None,
            },
        }
    }
}

// ============================================================================
// GuiWindow Class
// ============================================================================

/// A GUI window that can display widget content.
///
/// Windows are created via `GuiApp.createWindow()` and can display
/// widget trees built with the widget builder functions.
#[napi]
pub struct GuiWindow {
    window_id: u64,
    config: WindowConfig,
    content: Option<WidgetDef>,
    shown: bool,
    /// Whether this window has an event callback registered
    has_event_callback: bool,
}

#[napi]
impl GuiWindow {
    pub(crate) fn new(js_config: JsWindowConfig) -> Result<Self> {
        let mut state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
        let state = state.as_mut().ok_or_else(|| Error::from_reason("GuiApp not initialized"))?;

        let window_id = state.next_window_id;
        state.next_window_id += 1;

        let level = if js_config.always_on_top.unwrap_or(false) {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        };

        let config = WindowConfig {
            id: Some(format!("js-window-{}", window_id)),
            title: js_config.title,
            position: Position::new(js_config.x.unwrap_or(100.0), js_config.y.unwrap_or(100.0)),
            size: Size::new(js_config.width.unwrap_or(400), js_config.height.unwrap_or(300)),
            level,
            resizable: js_config.resizable.unwrap_or(true),
            draggable: true,
            ..Default::default()
        };

        Ok(GuiWindow { window_id, config, content: None, shown: false, has_event_callback: false })
    }

    /// Set the widget content for this window.
    #[napi]
    pub fn set_content(&mut self, widget: &Widget) -> Result<&Self> {
        self.content = Some(widget.inner.clone());

        // Note: If the window is already shown, we would need to send a command
        // to update the widget content. For now, content must be set before show().
        // TODO: Add support for dynamic content updates via CloseByName + Create

        Ok(self)
    }

    /// Register a callback for widget events.
    ///
    /// Events are dispatched when users interact with widgets (button clicks,
    /// text changes, checkbox toggles, etc.).
    ///
    /// @example
    /// ```javascript
    /// win.onEvent((event) => {
    ///   if (event.eventType === 'button_click') {
    ///     console.log('Button clicked:', event.widgetId);
    ///   }
    /// });
    /// ```
    #[napi]
    pub fn on_event(
        &mut self,
        #[napi(ts_arg_type = "(event: JsWidgetEvent) => void")] callback: ThreadsafeFunction<
            JsWidgetEvent,
        >,
    ) -> Result<&Self> {
        let window_name = self.config.title.clone().unwrap_or_default();

        let mut state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
        if let Some(ref mut s) = *state {
            s.window_callbacks.insert(window_name, Arc::new(callback));
        }

        self.has_event_callback = true;
        Ok(self)
    }

    /// Show the window.
    #[napi]
    pub fn show(&mut self) -> Result<()> {
        if self.shown {
            return Ok(());
        }

        let state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
        let state = state.as_ref().ok_or_else(|| Error::from_reason("GuiApp not initialized"))?;

        // Set widget content if we have it
        let mut config = self.config.clone();
        if let Some(ref content) = self.content {
            config.widget_content = Some(content.clone());
        }

        // Always register the event sender so events can be polled
        // This supports both callback-based (onEvent) and polling-based (pollEvents) approaches
        if let Some(ref event_sender) = state.event_sender {
            let window_name = self.config.title.clone().unwrap_or_default();
            state
                .sender
                .send(WindowCommand::RegisterEventCallback {
                    window_name,
                    event_sender: event_sender.clone(),
                })
                .map_err(|e| Error::from_reason(e.to_string()))?;
        }

        state
            .sender
            .send(WindowCommand::Create { config, effect: None })
            .map_err(|e| Error::from_reason(e.to_string()))?;

        self.shown = true;
        Ok(())
    }

    /// Close the window.
    #[napi]
    pub fn close(&self) -> Result<()> {
        let state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
        let state = state.as_ref().ok_or_else(|| Error::from_reason("GuiApp not initialized"))?;

        state
            .sender
            .send(WindowCommand::CloseByName { name: self.config.id.clone().unwrap_or_default() })
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(())
    }

    /// Update a widget's state by ID.
    #[napi]
    pub fn update_widget(&self, widget_id: String, update: JsWidgetUpdate) -> Result<()> {
        let state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
        let state = state.as_ref().ok_or_else(|| Error::from_reason("GuiApp not initialized"))?;

        let widget_update = if let Some(text) = update.text {
            WidgetUpdate::SetText(text)
        } else if let Some(checked) = update.checked {
            WidgetUpdate::SetChecked(checked)
        } else if let Some(value) = update.value {
            WidgetUpdate::SetValue(value as f32)
        } else if let Some(visible) = update.visible {
            WidgetUpdate::SetVisible(visible)
        } else if let Some(enabled) = update.enabled {
            WidgetUpdate::SetEnabled(enabled)
        } else {
            return Err(Error::from_reason("No update specified"));
        };

        state
            .sender
            .send(WindowCommand::UpdateWidget { widget_id, update: widget_update })
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(())
    }
}

/// Widget update options
#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct JsWidgetUpdate {
    /// New text value
    pub text: Option<String>,
    /// New checked state
    pub checked: Option<bool>,
    /// New numeric value
    pub value: Option<f64>,
    /// New visibility state
    pub visible: Option<bool>,
    /// New enabled state
    pub enabled: Option<bool>,
}

// ============================================================================
// Widget Style Types
// ============================================================================

/// Widget style configuration
#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct JsWidgetStyle {
    /// Margin on all sides
    pub margin: Option<f64>,
    /// Padding on all sides
    pub padding: Option<f64>,
    /// Minimum width
    pub min_width: Option<f64>,
    /// Minimum height
    pub min_height: Option<f64>,
    /// Maximum width
    pub max_width: Option<f64>,
    /// Maximum height
    pub max_height: Option<f64>,
    /// Background color as hex string (e.g., "#FF0000" or "#FF0000FF")
    pub background_color: Option<String>,
    /// Text color as hex string
    pub text_color: Option<String>,
    /// Font size in points
    pub font_size: Option<f64>,
    /// Font family name (e.g., "Helvetica", "Arial")
    pub font_family: Option<String>,
    /// Text alignment: "left", "center", "right"
    pub text_align: Option<String>,
}

impl JsWidgetStyle {
    fn to_aumate(&self) -> aumate::gui::widget::WidgetStyle {
        use aumate::gui::widget::{Spacing, TextAlign, WidgetStyle};

        let mut style = WidgetStyle::default();

        if let Some(m) = self.margin {
            style.margin = Spacing::all(m as f32);
        }
        if let Some(p) = self.padding {
            style.padding = Spacing::all(p as f32);
        }
        if let Some(w) = self.min_width {
            style.min_width = Some(w as f32);
        }
        if let Some(h) = self.min_height {
            style.min_height = Some(h as f32);
        }
        if let Some(w) = self.max_width {
            style.max_width = Some(w as f32);
        }
        if let Some(h) = self.max_height {
            style.max_height = Some(h as f32);
        }
        if let Some(ref color) = self.background_color {
            style.background_color = parse_hex_color(color);
        }
        if let Some(ref color) = self.text_color {
            style.text_color = parse_hex_color(color);
        }
        if let Some(size) = self.font_size {
            style.font_size = Some(size as f32);
        }
        if let Some(ref family) = self.font_family {
            style.font_family = Some(family.clone());
        }
        if let Some(ref align) = self.text_align {
            style.text_align = Some(match align.as_str() {
                "center" => TextAlign::Center,
                "right" => TextAlign::Right,
                _ => TextAlign::Left,
            });
        }

        style
    }
}

/// Parse hex color string to RGBA array
fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let hex = hex.trim_start_matches('#');
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some([r, g, b, a])
        }
        _ => None,
    }
}

// ============================================================================
// Widget Class
// ============================================================================

/// A widget that can be composed into UIs
#[napi]
#[derive(Debug, Clone)]
pub struct Widget {
    inner: WidgetDef,
}

#[napi]
impl Widget {
    /// Set the widget ID
    #[napi]
    pub fn with_id(&self, id: String) -> Widget {
        Widget { inner: self.inner.clone().with_id(id) }
    }

    /// Set visibility
    #[napi]
    pub fn with_visible(&self, visible: bool) -> Widget {
        Widget { inner: self.inner.clone().with_visible(visible) }
    }

    /// Set enabled state
    #[napi]
    pub fn with_enabled(&self, enabled: bool) -> Widget {
        Widget { inner: self.inner.clone().with_enabled(enabled) }
    }

    /// Set tooltip
    #[napi]
    pub fn with_tooltip(&self, tooltip: String) -> Widget {
        Widget { inner: self.inner.clone().with_tooltip(tooltip) }
    }

    /// Set style
    #[napi]
    pub fn with_style(&self, style: JsWidgetStyle) -> Widget {
        Widget { inner: self.inner.clone().with_style(style.to_aumate()) }
    }

    /// Set font family for text widgets
    #[napi]
    pub fn with_font_family(&self, family: String) -> Widget {
        Widget { inner: self.inner.clone().with_font_family(family) }
    }

    /// Set spacing (for layout widgets)
    #[napi]
    pub fn with_spacing(&self, spacing: f64) -> Widget {
        Widget { inner: self.inner.clone().with_spacing(spacing as f32) }
    }

    /// Set placeholder (for text input)
    #[napi]
    pub fn with_placeholder(&self, placeholder: String) -> Widget {
        Widget { inner: self.inner.clone().with_placeholder(placeholder) }
    }

    /// Set password mode (for text input)
    #[napi]
    pub fn with_password(&self, password: bool) -> Widget {
        Widget { inner: self.inner.clone().with_password(password) }
    }

    /// Set step (for slider)
    #[napi]
    pub fn with_step(&self, step: f64) -> Widget {
        Widget { inner: self.inner.clone().with_step(step as f32) }
    }

    /// Set max height (for scroll area)
    #[napi]
    pub fn with_max_height(&self, height: f64) -> Widget {
        Widget { inner: self.inner.clone().with_max_height(height as f32) }
    }

    /// Set collapsed state (for group)
    #[napi]
    pub fn with_collapsed(&self, collapsed: bool) -> Widget {
        Widget { inner: self.inner.clone().with_collapsed(collapsed) }
    }

    /// Set selected index (for dropdown, radio group)
    #[napi]
    pub fn with_selected(&self, index: u32) -> Widget {
        Widget { inner: self.inner.clone().with_selected(index as usize) }
    }

    /// Set horizontal layout (for radio group)
    #[napi]
    pub fn with_horizontal(&self, horizontal: bool) -> Widget {
        Widget { inner: self.inner.clone().with_horizontal(horizontal) }
    }

    /// Set number of rows (for text area)
    #[napi]
    pub fn with_rows(&self, rows: u32) -> Widget {
        Widget { inner: self.inner.clone().with_rows(rows) }
    }

    /// Set the active tab index for tabs widget
    #[napi]
    pub fn with_active(&self, index: u32) -> Widget {
        Widget { inner: self.inner.clone().with_active(index as usize) }
    }

    /// Set range for drag value widget
    #[napi]
    pub fn with_range(&self, min: f64, max: f64) -> Widget {
        Widget { inner: self.inner.clone().with_range(min, max) }
    }

    /// Set speed for drag value widget
    #[napi]
    pub fn with_speed(&self, speed: f64) -> Widget {
        Widget { inner: self.inner.clone().with_speed(speed) }
    }

    /// Set prefix for drag value widget
    #[napi]
    pub fn with_prefix(&self, prefix: String) -> Widget {
        Widget { inner: self.inner.clone().with_prefix(prefix) }
    }

    /// Set suffix for drag value widget
    #[napi]
    pub fn with_suffix(&self, suffix: String) -> Widget {
        Widget { inner: self.inner.clone().with_suffix(suffix) }
    }

    /// Set decimal places for drag value widget
    #[napi]
    pub fn with_decimals(&self, decimals: u32) -> Widget {
        Widget { inner: self.inner.clone().with_decimals(decimals as usize) }
    }

    /// Set alpha channel display for color picker
    #[napi]
    pub fn with_alpha(&self, alpha: bool) -> Widget {
        Widget { inner: self.inner.clone().with_alpha(alpha) }
    }

    /// Set new tab behavior for hyperlink
    #[napi]
    pub fn with_new_tab(&self, new_tab: bool) -> Widget {
        Widget { inner: self.inner.clone().with_new_tab(new_tab) }
    }

    /// Set frame visibility for image button
    #[napi]
    pub fn with_frame(&self, frame: bool) -> Widget {
        Widget { inner: self.inner.clone().with_frame(frame) }
    }

    /// Set selected state for image button
    #[napi]
    pub fn with_image_selected(&self, selected: bool) -> Widget {
        Widget { inner: self.inner.clone().with_image_selected(selected) }
    }

    /// Set tint color for image button [r, g, b, a]
    #[napi]
    pub fn with_tint(&self, tint: Vec<u8>) -> Widget {
        if tint.len() >= 4 {
            Widget { inner: self.inner.clone().with_tint([tint[0], tint[1], tint[2], tint[3]]) }
        } else {
            Widget { inner: self.inner.clone() }
        }
    }

    /// Get the inner WidgetDef (for internal use)
    pub(crate) fn into_inner(self) -> WidgetDef {
        self.inner
    }
}

impl From<WidgetDef> for Widget {
    fn from(def: WidgetDef) -> Self {
        Widget { inner: def }
    }
}

// ============================================================================
// Basic Widget Constructors
// ============================================================================

/// Create a text label widget
#[napi]
pub fn label(text: String) -> Widget {
    Widget::from(WidgetDef::label(text))
}

/// Create a button widget
#[napi]
pub fn button(text: String) -> Widget {
    Widget::from(WidgetDef::button(text))
}

/// Create a text input widget
#[napi]
pub fn text_input() -> Widget {
    Widget::from(WidgetDef::text_input())
}

/// Create a text input with initial value
#[napi]
pub fn text_input_with_value(value: String) -> Widget {
    Widget::from(WidgetDef::text_input_with_value(value))
}

/// Create a checkbox widget
#[napi]
pub fn checkbox(label_text: String, checked: bool) -> Widget {
    Widget::from(WidgetDef::checkbox(label_text, checked))
}

/// Create a slider widget
#[napi]
pub fn slider(value: f64, min: f64, max: f64) -> Widget {
    Widget::from(WidgetDef::slider(value as f32, min as f32, max as f32))
}

/// Create a progress bar widget
#[napi]
pub fn progress_bar(value: f64) -> Widget {
    Widget::from(WidgetDef::progress_bar(value as f32))
}

/// Create a horizontal separator
#[napi]
pub fn separator() -> Widget {
    Widget::from(WidgetDef::separator())
}

/// Create a spacer
#[napi]
pub fn spacer(size: f64) -> Widget {
    Widget::from(WidgetDef::spacer(size as f32))
}

// ============================================================================
// Layout Widget Constructors
// ============================================================================

/// Create a horizontal box layout
#[napi]
pub fn hbox(children: Vec<&Widget>) -> Widget {
    let child_defs: Vec<WidgetDef> = children.into_iter().map(|w| w.inner.clone()).collect();
    Widget::from(WidgetDef::hbox(child_defs))
}

/// Create a vertical box layout
#[napi]
pub fn vbox(children: Vec<&Widget>) -> Widget {
    let child_defs: Vec<WidgetDef> = children.into_iter().map(|w| w.inner.clone()).collect();
    Widget::from(WidgetDef::vbox(child_defs))
}

/// Create a grid layout
/// Each inner array represents a row of widgets
#[napi]
pub fn grid(rows: Vec<Vec<&Widget>>) -> Widget {
    let grid_defs: Vec<Vec<WidgetDef>> =
        rows.into_iter().map(|row| row.into_iter().map(|w| w.inner.clone()).collect()).collect();
    Widget::from(WidgetDef::grid(grid_defs))
}

// ============================================================================
// Container Widget Constructors
// ============================================================================

/// Create a panel container
#[napi]
pub fn panel(child: &Widget) -> Widget {
    Widget::from(WidgetDef::panel(child.inner.clone()))
}

/// Create a scroll area container
#[napi]
pub fn scroll_area(child: &Widget) -> Widget {
    Widget::from(WidgetDef::scroll_area(child.inner.clone()))
}

/// Create a collapsible group
#[napi]
pub fn group(title: String, child: &Widget) -> Widget {
    Widget::from(WidgetDef::group(title, child.inner.clone()))
}

// ============================================================================
// Image Widget Constructor
// ============================================================================

/// Create an image widget from RGBA data
#[napi]
pub fn image(data: Buffer, width: u32, height: u32) -> Widget {
    Widget::from(WidgetDef::image(data.to_vec(), width, height))
}

// ============================================================================
// Advanced Widget Constructors
// ============================================================================

/// Create a dropdown select widget
#[napi]
pub fn dropdown(options: Vec<String>) -> Widget {
    Widget::from(WidgetDef::dropdown(options))
}

/// Create a radio button group
#[napi]
pub fn radio_group(options: Vec<String>) -> Widget {
    Widget::from(WidgetDef::radio_group(options))
}

/// Create a multi-line text area
#[napi]
pub fn text_area() -> Widget {
    Widget::from(WidgetDef::text_area())
}

/// Create a multi-line text area with initial value
#[napi]
pub fn text_area_with_value(value: String) -> Widget {
    Widget::from(WidgetDef::text_area_with_value(value))
}

/// Create a tabbed container widget
///
/// Takes separate arrays of labels and content widgets. Each label[i] corresponds to content[i].
#[napi]
pub fn tabs(labels: Vec<String>, contents: Vec<&Widget>) -> Widget {
    let tabs: Vec<(String, WidgetDef)> = labels
        .into_iter()
        .zip(contents)
        .map(|(label, content)| (label, content.inner.clone()))
        .collect();
    Widget::from(WidgetDef::tabs(tabs))
}

/// Create a link widget (clickable text that fires an event)
#[napi]
pub fn link(text: String) -> Widget {
    Widget::from(WidgetDef::link(text))
}

/// Create a selectable label widget (toggleable selection state)
#[napi]
pub fn selectable_label(text: String, selected: bool) -> Widget {
    Widget::from(WidgetDef::selectable_label(text, selected))
}

/// Create a drag value widget for numeric input
#[napi]
pub fn drag_value(value: f64) -> Widget {
    Widget::from(WidgetDef::drag_value(value))
}

/// Create a color picker widget
///
/// @param color - Initial color as [r, g, b, a] array (0-255)
#[napi]
pub fn color_picker(color: Vec<u8>) -> Widget {
    let color_arr = if color.len() >= 4 {
        [color[0], color[1], color[2], color[3]]
    } else if color.len() == 3 {
        [color[0], color[1], color[2], 255]
    } else {
        [255, 255, 255, 255]
    };
    Widget::from(WidgetDef::color_picker(color_arr))
}

/// Create a hyperlink widget (opens URL in browser)
#[napi]
pub fn hyperlink(text: String, url: String) -> Widget {
    Widget::from(WidgetDef::hyperlink(text, url))
}

/// Create a hyperlink widget with URL as both text and link
#[napi]
pub fn hyperlink_url(url: String) -> Widget {
    Widget::from(WidgetDef::hyperlink_url(url))
}

/// Create an image button widget
#[napi]
pub fn image_button(data: Buffer, width: u32, height: u32) -> Widget {
    Widget::from(WidgetDef::image_button(data.to_vec(), width, height))
}

// ============================================================================
// File Dialog Functions
// ============================================================================

/// File dialog filter definition
#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct FileFilter {
    /// Display name for the filter (e.g., "Images")
    pub name: String,
    /// File extensions without dots (e.g., ["png", "jpg", "gif"])
    pub extensions: Vec<String>,
}

/// File dialog options
#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct FileDialogOptions {
    /// Dialog title
    pub title: Option<String>,
    /// Starting directory
    pub directory: Option<String>,
    /// Default file name (for save dialogs)
    pub default_name: Option<String>,
    /// File filters
    pub filters: Option<Vec<FileFilter>>,
    /// Allow multiple file selection (for open dialogs)
    pub multiple: Option<bool>,
}

/// File dialog result
#[napi(object)]
#[derive(Debug, Clone)]
pub struct FileDialogResult {
    /// Selected file paths, empty if cancelled
    pub paths: Vec<String>,
    /// Whether the dialog was cancelled
    pub cancelled: bool,
}

/// Show a file open dialog (non-blocking)
///
/// Opens a native file dialog. The result is delivered via a `file_dialog_completed` event
/// to the specified window's onEvent callback with the request_id as the widget_id.
///
/// @param request_id - Unique ID to identify this dialog request in events
/// @param window_name - Name of the window to receive the result event
/// @param options - Dialog options including title, directory, filters
///
/// @example
/// ```javascript
/// win.onEvent((event) => {
///   if (event.widgetId === "open-btn" && event.eventType === "button_click") {
///     showOpenFileDialog("open-request", "My Window", {
///       title: "Select an image",
///       filters: [{ name: "Images", extensions: ["png", "jpg", "gif"] }],
///     });
///   }
///   if (event.eventType === "file_dialog_completed" && event.widgetId === "open-request") {
///     if (!event.cancelled) {
///       console.log("Selected files:", event.paths);
///     }
///   }
/// });
/// ```
#[napi]
pub fn show_open_file_dialog(
    request_id: String,
    window_name: String,
    options: Option<FileDialogOptions>,
) -> Result<()> {
    use aumate::gui::window::commands::{FileDialogOptions as InternalOptions, WindowCommand};

    let opts = options.unwrap_or_default();

    // Convert to internal options
    let internal_opts = InternalOptions {
        title: opts.title,
        directory: opts.directory,
        default_name: opts.default_name,
        filters: opts
            .filters
            .map(|f| f.into_iter().map(|filter| (filter.name, filter.extensions)).collect())
            .unwrap_or_default(),
        multiple: opts.multiple.unwrap_or(false),
    };

    // Send command to GUI thread (non-blocking)
    let state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let state = state.as_ref().ok_or_else(|| {
        Error::from_reason("GuiApp not initialized. Create a GuiApp instance first.")
    })?;

    state
        .sender
        .send(WindowCommand::ShowOpenFileDialog { request_id, window_name, options: internal_opts })
        .map_err(|e| Error::from_reason(format!("Failed to send command: {}", e)))?;

    Ok(())
}

/// Show a file save dialog (non-blocking)
///
/// Opens a native save file dialog. The result is delivered via a `file_dialog_completed` event
/// to the specified window's onEvent callback with the request_id as the widget_id.
///
/// @param request_id - Unique ID to identify this dialog request in events
/// @param window_name - Name of the window to receive the result event
/// @param options - Dialog options including title, directory, default name, filters
///
/// @example
/// ```javascript
/// win.onEvent((event) => {
///   if (event.widgetId === "save-btn" && event.eventType === "button_click") {
///     showSaveFileDialog("save-request", "My Window", {
///       title: "Save file as",
///       default_name: "document.txt",
///       filters: [{ name: "Text files", extensions: ["txt"] }],
///     });
///   }
///   if (event.eventType === "file_dialog_completed" && event.widgetId === "save-request") {
///     if (!event.cancelled) {
///       console.log("Save to:", event.paths[0]);
///     }
///   }
/// });
/// ```
#[napi]
pub fn show_save_file_dialog(
    request_id: String,
    window_name: String,
    options: Option<FileDialogOptions>,
) -> Result<()> {
    use aumate::gui::window::commands::{FileDialogOptions as InternalOptions, WindowCommand};

    let opts = options.unwrap_or_default();

    // Convert to internal options
    let internal_opts = InternalOptions {
        title: opts.title,
        directory: opts.directory,
        default_name: opts.default_name,
        filters: opts
            .filters
            .map(|f| f.into_iter().map(|filter| (filter.name, filter.extensions)).collect())
            .unwrap_or_default(),
        multiple: false,
    };

    // Send command to GUI thread (non-blocking)
    let state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let state = state.as_ref().ok_or_else(|| {
        Error::from_reason("GuiApp not initialized. Create a GuiApp instance first.")
    })?;

    state
        .sender
        .send(WindowCommand::ShowSaveFileDialog { request_id, window_name, options: internal_opts })
        .map_err(|e| Error::from_reason(format!("Failed to send command: {}", e)))?;

    Ok(())
}

/// Show a folder picker dialog (non-blocking)
///
/// Opens a native folder picker dialog. The result is delivered via a `file_dialog_completed` event
/// to the specified window's onEvent callback with the request_id as the widget_id.
///
/// @param request_id - Unique ID to identify this dialog request in events
/// @param window_name - Name of the window to receive the result event
/// @param options - Dialog options including title and starting directory
///
/// @example
/// ```javascript
/// win.onEvent((event) => {
///   if (event.widgetId === "folder-btn" && event.eventType === "button_click") {
///     showFolderDialog("folder-request", "My Window", {
///       title: "Select a folder",
///     });
///   }
///   if (event.eventType === "file_dialog_completed" && event.widgetId === "folder-request") {
///     if (!event.cancelled) {
///       console.log("Selected folder:", event.paths[0]);
///     }
///   }
/// });
/// ```
#[napi]
pub fn show_folder_dialog(
    request_id: String,
    window_name: String,
    options: Option<FileDialogOptions>,
) -> Result<()> {
    use aumate::gui::window::commands::{FileDialogOptions as InternalOptions, WindowCommand};

    let opts = options.unwrap_or_default();

    // Convert to internal options
    let internal_opts = InternalOptions {
        title: opts.title,
        directory: opts.directory,
        default_name: None,
        filters: vec![],
        multiple: false,
    };

    // Send command to GUI thread (non-blocking)
    let state = GUI_STATE.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let state = state.as_ref().ok_or_else(|| {
        Error::from_reason("GuiApp not initialized. Create a GuiApp instance first.")
    })?;

    state
        .sender
        .send(WindowCommand::ShowFolderDialog { request_id, window_name, options: internal_opts })
        .map_err(|e| Error::from_reason(format!("Failed to send command: {}", e)))?;

    Ok(())
}

// ============================================================================
// Font Functions
// ============================================================================

/// Get a list of all system font family names
///
/// Returns a sorted list of unique font family names available on the system.
/// Use this with a dropdown widget to create a font picker.
///
/// @returns Promise resolving to sorted array of font family names
///
/// @example
/// ```javascript
/// import { getSystemFonts, dropdown } from "@tego/botjs";
///
/// // Get available fonts and create a font picker dropdown
/// const fonts = await getSystemFonts();
/// const fontPicker = dropdown(fonts)
///   .withId("font-family")
///   .withPlaceholder("Select a font");
/// ```
#[napi]
pub async fn get_system_fonts() -> Result<Vec<String>> {
    use font_kit::source::SystemSource;
    use std::collections::HashSet;

    // Run font enumeration in a blocking task to avoid blocking the async runtime
    let fonts = tokio::task::spawn_blocking(|| {
        let source = SystemSource::new();
        let families = source.all_families().unwrap_or_default();

        // Deduplicate and sort
        let unique: HashSet<String> = families.into_iter().collect();
        let mut sorted: Vec<String> = unique.into_iter().collect();
        sorted.sort_by_key(|a| a.to_lowercase());
        sorted
    })
    .await
    .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(fonts)
}
