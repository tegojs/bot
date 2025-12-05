//! Menu Bar module for macOS status bar (tray icon) support
//!
//! This module provides the ability to create and manage menu bar items
//! that can trigger window creation, show dropdown menus, and more.

mod controller;
mod item;
mod manager;
mod menu;

pub use controller::MenuBarFeature;
pub use item::{MenuBarClickAction, MenuBarIcon, MenuBarItem, MenuBarItemBuilder};
pub use manager::{MenuBarEvent, MenuBarManager, MenuBarRegistry};
pub use menu::{MenuBarMenu, MenuBarMenuItem, PredefinedMenuItemType};
