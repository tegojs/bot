//! Command system for inter-window communication

use super::config::WindowConfig;
use crate::gui::content::Content;
use crate::gui::effect::{PresetEffect, PresetEffectOptions};
use crate::gui::menu_bar::{MenuBarIcon, MenuBarItem};
use crate::gui::widget::{WidgetDef, WidgetEvent};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, RwLock};
use winit::window::WindowId;

/// Options for file dialogs
#[derive(Debug, Clone, Default)]
pub struct FileDialogOptions {
    /// Dialog title
    pub title: Option<String>,
    /// Starting directory
    pub directory: Option<String>,
    /// Default file name (for save dialogs)
    pub default_name: Option<String>,
    /// File filters: Vec<(filter_name, extensions)>
    pub filters: Vec<(String, Vec<String>)>,
    /// Allow multiple file selection (for open dialogs)
    pub multiple: bool,
}

/// Result from file dialogs
#[derive(Debug, Clone)]
pub struct FileDialogResult {
    /// Selected file/folder paths
    pub paths: Vec<String>,
    /// Whether the dialog was cancelled
    pub cancelled: bool,
}

/// Event sender for widget events from a window
pub type WidgetEventSender = Sender<(String, WidgetEvent)>;

/// Commands sent from controller to the event loop
pub enum WindowCommand {
    /// Create a new window with the given configuration
    Create { config: WindowConfig, effect: Option<(PresetEffect, PresetEffectOptions)> },
    /// Close a specific window by its ID
    Close { id: WindowId },
    /// Close a window by its name
    CloseByName { name: String },
    /// Update effect options for a specific window
    UpdateEffectOptions { id: WindowId, options: PresetEffectOptions },
    /// Close all managed windows (not the controller)
    CloseAll,
    /// Add a new menu bar item
    AddMenuBarItem { item: MenuBarItem },
    /// Remove a menu bar item by ID
    RemoveMenuBarItem { id: String },
    /// Update the icon of a menu bar item
    UpdateMenuBarIcon { id: String, icon: MenuBarIcon },
    /// Update the tooltip of a menu bar item
    UpdateMenuBarTooltip { id: String, tooltip: String },
    /// Update the content (image) of a window
    UpdateContent { id: WindowId, content: Option<Content> },
    /// Start screenshot mode with specified enabled actions
    StartScreenshotMode { enabled_actions: Vec<String> },
    /// Exit screenshot mode
    ExitScreenshotMode,
    /// Exit the entire application (close all windows and quit)
    ExitApplication,
    /// Start Click Helper mode
    StartClickHelperMode,
    /// Exit Click Helper mode
    ExitClickHelperMode,
    /// Set widget-based content for a window (replaces the entire UI)
    SetWidgetContent { id: WindowId, content: WidgetDef },
    /// Update a specific widget's state by its ID
    UpdateWidget { widget_id: String, update: WidgetUpdate },
    /// Register an event callback sender for a window by name
    RegisterEventCallback { window_name: String, event_sender: WidgetEventSender },
    /// Show an open file dialog (sync, runs on main thread)
    /// Result is emitted as FileDialogCompleted event to the specified window
    ShowOpenFileDialog { request_id: String, window_name: String, options: FileDialogOptions },
    /// Show a save file dialog (sync, runs on main thread)
    /// Result is emitted as FileDialogCompleted event to the specified window
    ShowSaveFileDialog { request_id: String, window_name: String, options: FileDialogOptions },
    /// Show a folder picker dialog (sync, runs on main thread)
    /// Result is emitted as FileDialogCompleted event to the specified window
    ShowFolderDialog { request_id: String, window_name: String, options: FileDialogOptions },
}

impl std::fmt::Debug for WindowCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create { config, effect } => {
                f.debug_struct("Create").field("config", config).field("effect", effect).finish()
            }
            Self::Close { id } => f.debug_struct("Close").field("id", id).finish(),
            Self::CloseByName { name } => {
                f.debug_struct("CloseByName").field("name", name).finish()
            }
            Self::UpdateEffectOptions { id, options } => f
                .debug_struct("UpdateEffectOptions")
                .field("id", id)
                .field("options", options)
                .finish(),
            Self::CloseAll => write!(f, "CloseAll"),
            Self::AddMenuBarItem { item } => {
                f.debug_struct("AddMenuBarItem").field("item", item).finish()
            }
            Self::RemoveMenuBarItem { id } => {
                f.debug_struct("RemoveMenuBarItem").field("id", id).finish()
            }
            Self::UpdateMenuBarIcon { id, icon } => {
                f.debug_struct("UpdateMenuBarIcon").field("id", id).field("icon", icon).finish()
            }
            Self::UpdateMenuBarTooltip { id, tooltip } => f
                .debug_struct("UpdateMenuBarTooltip")
                .field("id", id)
                .field("tooltip", tooltip)
                .finish(),
            Self::UpdateContent { id, content } => {
                f.debug_struct("UpdateContent").field("id", id).field("content", content).finish()
            }
            Self::StartScreenshotMode { enabled_actions } => f
                .debug_struct("StartScreenshotMode")
                .field("enabled_actions", enabled_actions)
                .finish(),
            Self::ExitScreenshotMode => write!(f, "ExitScreenshotMode"),
            Self::ExitApplication => write!(f, "ExitApplication"),
            Self::StartClickHelperMode => write!(f, "StartClickHelperMode"),
            Self::ExitClickHelperMode => write!(f, "ExitClickHelperMode"),
            Self::SetWidgetContent { id, content } => f
                .debug_struct("SetWidgetContent")
                .field("id", id)
                .field("content", content)
                .finish(),
            Self::UpdateWidget { widget_id, update } => f
                .debug_struct("UpdateWidget")
                .field("widget_id", widget_id)
                .field("update", update)
                .finish(),
            Self::RegisterEventCallback { window_name, .. } => f
                .debug_struct("RegisterEventCallback")
                .field("window_name", window_name)
                .field("event_sender", &"<Sender>")
                .finish(),
            Self::ShowOpenFileDialog { request_id, window_name, options } => f
                .debug_struct("ShowOpenFileDialog")
                .field("request_id", request_id)
                .field("window_name", window_name)
                .field("options", options)
                .finish(),
            Self::ShowSaveFileDialog { request_id, window_name, options } => f
                .debug_struct("ShowSaveFileDialog")
                .field("request_id", request_id)
                .field("window_name", window_name)
                .field("options", options)
                .finish(),
            Self::ShowFolderDialog { request_id, window_name, options } => f
                .debug_struct("ShowFolderDialog")
                .field("request_id", request_id)
                .field("window_name", window_name)
                .field("options", options)
                .finish(),
        }
    }
}

/// Sender end of the command channel
pub type CommandSender = Sender<WindowCommand>;

/// Receiver end of the command channel
pub type CommandReceiver = Receiver<WindowCommand>;

/// Create a new command channel
pub fn create_command_channel() -> (CommandSender, CommandReceiver) {
    channel()
}

/// Information about a managed window
#[derive(Debug, Clone)]
pub struct WindowInfo {
    /// Window ID
    pub id: WindowId,
    /// User-friendly name
    pub name: String,
    /// Window size (width, height)
    pub size: (u32, u32),
    /// Current effect type
    pub effect: Option<PresetEffect>,
    /// Current effect options
    pub options: Option<PresetEffectOptions>,
}

/// Shared registry of managed windows
#[derive(Debug, Clone, Default)]
pub struct WindowRegistry {
    windows: Arc<RwLock<Vec<WindowInfo>>>,
    next_id: Arc<RwLock<usize>>,
}

impl WindowRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new window
    pub fn register(
        &self,
        id: WindowId,
        name: String,
        size: (u32, u32),
        effect: Option<PresetEffect>,
        options: Option<PresetEffectOptions>,
    ) {
        let mut windows = self.windows.write().unwrap();
        windows.push(WindowInfo { id, name, size, effect, options });
    }

    /// Unregister a window by ID
    pub fn unregister(&self, id: WindowId) {
        let mut windows = self.windows.write().unwrap();
        windows.retain(|w| w.id != id);
    }

    /// Get all registered windows
    pub fn list(&self) -> Vec<WindowInfo> {
        self.windows.read().unwrap().clone()
    }

    /// Generate a unique window name
    pub fn generate_name(&self) -> String {
        let mut id = self.next_id.write().unwrap();
        let name = format!("Window {}", *id + 1);
        *id += 1;
        name
    }

    /// Find window ID by name
    pub fn find_by_name(&self, name: &str) -> Option<WindowId> {
        let windows = self.windows.read().unwrap();
        windows.iter().find(|w| w.name == name).map(|w| w.id)
    }

    /// Update effect options for a window
    pub fn update_options(&self, id: WindowId, options: PresetEffectOptions) {
        let mut windows = self.windows.write().unwrap();
        if let Some(window) = windows.iter_mut().find(|w| w.id == id) {
            window.options = Some(options);
        }
    }

    /// Clear all windows
    pub fn clear(&self) {
        self.windows.write().unwrap().clear();
    }
}

/// Updates that can be applied to widget state
#[derive(Debug, Clone)]
pub enum WidgetUpdate {
    /// Set text value (for TextInput)
    SetText(String),
    /// Set checked state (for Checkbox)
    SetChecked(bool),
    /// Set numeric value (for Slider, ProgressBar)
    SetValue(f32),
    /// Set visibility
    SetVisible(bool),
    /// Set enabled state
    SetEnabled(bool),
}
