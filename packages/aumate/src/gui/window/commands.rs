//! Command system for inter-window communication

use super::config::WindowConfig;
use crate::gui::content::Content;
use crate::gui::effect::{PresetEffect, PresetEffectOptions};
use crate::gui::menu_bar::{MenuBarIcon, MenuBarItem};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, RwLock};
use winit::window::WindowId;

/// Commands sent from controller to the event loop
#[derive(Debug)]
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
