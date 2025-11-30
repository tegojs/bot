//! Menu bar item definitions and builder

use super::menu::MenuBarMenu;
use crate::effect::{PresetEffect, PresetEffectOptions};
use crate::icon::WindowIcon;
use crate::window::config::WindowConfig;
use std::path::PathBuf;

/// Icon for menu bar item
#[derive(Debug, Clone)]
pub enum MenuBarIcon {
    /// Use existing WindowIcon system
    WindowIcon(WindowIcon),
    /// Raw RGBA image data
    Rgba {
        data: Vec<u8>,
        width: u32,
        height: u32,
    },
    /// Load from file path
    Path(PathBuf),
}

impl Default for MenuBarIcon {
    fn default() -> Self {
        MenuBarIcon::Rgba {
            data: vec![128, 128, 128, 255].repeat(16 * 16),
            width: 16,
            height: 16,
        }
    }
}

/// Click behavior for menu bar item
#[derive(Debug, Clone)]
pub enum MenuBarClickAction {
    /// Show a dropdown menu
    ShowMenu(MenuBarMenu),
    /// Toggle a floating window (show/hide)
    ToggleWindow {
        config: WindowConfig,
        effect: Option<(PresetEffect, PresetEffectOptions)>,
    },
    /// No action (handled externally via events)
    None,
}

impl Default for MenuBarClickAction {
    fn default() -> Self {
        MenuBarClickAction::None
    }
}

/// A menu bar item (status bar icon on macOS)
#[derive(Debug, Clone)]
pub struct MenuBarItem {
    /// Unique identifier for this menu bar item
    pub id: String,
    /// Display name
    pub name: String,
    /// Icon to display in menu bar
    pub icon: MenuBarIcon,
    /// Tooltip shown on hover
    pub tooltip: Option<String>,
    /// Action to perform on click
    pub click_action: MenuBarClickAction,
}

impl MenuBarItem {
    /// Create a new builder for MenuBarItem
    pub fn builder(name: impl Into<String>) -> MenuBarItemBuilder {
        MenuBarItemBuilder::new(name)
    }
}

/// Builder for MenuBarItem
pub struct MenuBarItemBuilder {
    name: String,
    id: Option<String>,
    icon: Option<MenuBarIcon>,
    tooltip: Option<String>,
    click_action: MenuBarClickAction,
}

impl MenuBarItemBuilder {
    /// Create a new MenuBarItemBuilder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id: None,
            icon: None,
            tooltip: None,
            click_action: MenuBarClickAction::None,
        }
    }

    /// Set the unique ID for this menu bar item
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the icon for this menu bar item
    pub fn icon(mut self, icon: MenuBarIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set the icon from a file path
    pub fn icon_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.icon = Some(MenuBarIcon::Path(path.into()));
        self
    }

    /// Set the icon from RGBA data
    pub fn icon_rgba(mut self, data: Vec<u8>, width: u32, height: u32) -> Self {
        self.icon = Some(MenuBarIcon::Rgba { data, width, height });
        self
    }

    /// Set the tooltip for this menu bar item
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set a dropdown menu to show on click
    pub fn menu(mut self, menu: MenuBarMenu) -> Self {
        self.click_action = MenuBarClickAction::ShowMenu(menu);
        self
    }

    /// Toggle a floating window on click
    pub fn toggle_window(
        mut self,
        config: WindowConfig,
        effect: Option<(PresetEffect, PresetEffectOptions)>,
    ) -> Self {
        self.click_action = MenuBarClickAction::ToggleWindow { config, effect };
        self
    }

    /// Build the MenuBarItem
    pub fn build(self) -> MenuBarItem {
        let id = self.id.unwrap_or_else(|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let duration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            format!("menu_bar_{}_{}", self.name, duration.as_nanos())
        });

        MenuBarItem {
            id,
            name: self.name,
            icon: self.icon.unwrap_or_default(),
            tooltip: self.tooltip,
            click_action: self.click_action,
        }
    }
}
