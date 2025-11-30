//! Menu bar manager for handling tray icons

use super::item::{MenuBarClickAction, MenuBarIcon, MenuBarItem};
use super::menu::{MenuBarMenu, MenuBarMenuItem, PredefinedMenuItemType};
use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};

/// Events from menu bar items
#[derive(Debug, Clone)]
pub enum MenuBarEvent {
    /// Menu bar icon was clicked
    Click { id: String },
    /// Menu bar icon was double-clicked
    DoubleClick { id: String },
    /// A menu item was clicked
    MenuItemClick { tray_id: String, menu_item_id: String },
    /// Quit was requested
    QuitRequested,
}

/// Information about a registered menu bar item
#[derive(Debug, Clone)]
pub struct MenuBarInfo {
    pub id: String,
    pub name: String,
    pub click_action: MenuBarClickAction,
}

/// Registry for tracking menu bar items
#[derive(Debug, Clone, Default)]
pub struct MenuBarRegistry {
    items: Arc<RwLock<HashMap<String, MenuBarInfo>>>,
}

impl MenuBarRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, item: &MenuBarItem) {
        let mut items = self.items.write().unwrap();
        items.insert(
            item.id.clone(),
            MenuBarInfo {
                id: item.id.clone(),
                name: item.name.clone(),
                click_action: item.click_action.clone(),
            },
        );
    }

    pub fn unregister(&self, id: &str) {
        let mut items = self.items.write().unwrap();
        items.remove(id);
    }

    pub fn list(&self) -> Vec<MenuBarInfo> {
        let items = self.items.read().unwrap();
        items.values().cloned().collect()
    }

    pub fn get(&self, id: &str) -> Option<MenuBarInfo> {
        let items = self.items.read().unwrap();
        items.get(id).cloned()
    }
}

/// Manager for menu bar items
pub struct MenuBarManager {
    /// Active tray icons (kept alive to prevent dropping)
    tray_icons: HashMap<String, TrayIcon>,
    /// Map from muda menu item ID (String) to (tray_id, menu_item_id)
    menu_item_map: HashMap<String, (String, String)>,
    /// Registry of menu bar items
    registry: MenuBarRegistry,
}

impl MenuBarManager {
    /// Create a new MenuBarManager
    pub fn new() -> Self {
        Self {
            tray_icons: HashMap::new(),
            menu_item_map: HashMap::new(),
            registry: MenuBarRegistry::new(),
        }
    }

    /// Get the registry
    pub fn registry(&self) -> &MenuBarRegistry {
        &self.registry
    }

    /// Add a new menu bar item
    pub fn add_item(&mut self, item: MenuBarItem) -> Result<String, String> {
        let icon = self.create_icon(&item.icon)?;

        let mut builder = TrayIconBuilder::new().with_icon(icon);

        if let Some(tooltip) = &item.tooltip {
            builder = builder.with_tooltip(tooltip);
        }

        // Create menu if click action is ShowMenu
        if let MenuBarClickAction::ShowMenu(ref menu) = item.click_action {
            let muda_menu = self.create_muda_menu(&item.id, menu)?;
            builder = builder.with_menu(Box::new(muda_menu));
        }

        let tray_icon = builder.build().map_err(|e| e.to_string())?;

        let id = item.id.clone();
        self.registry.register(&item);
        self.tray_icons.insert(id.clone(), tray_icon);

        log::info!("Added menu bar item: {}", id);
        Ok(id)
    }

    /// Remove a menu bar item
    pub fn remove_item(&mut self, id: &str) -> Result<(), String> {
        if self.tray_icons.remove(id).is_some() {
            self.registry.unregister(id);
            // Remove menu item mappings for this tray
            self.menu_item_map.retain(|_, (tray_id, _)| tray_id != id);
            log::info!("Removed menu bar item: {}", id);
            Ok(())
        } else {
            Err(format!("Menu bar item not found: {}", id))
        }
    }

    /// Update the icon for a menu bar item
    pub fn update_icon(&mut self, id: &str, icon: &MenuBarIcon) -> Result<(), String> {
        let tray_icon = self
            .tray_icons
            .get(id)
            .ok_or_else(|| format!("Menu bar item not found: {}", id))?;

        let new_icon = self.create_icon(icon)?;
        tray_icon.set_icon(Some(new_icon)).map_err(|e| e.to_string())
    }

    /// Update the tooltip for a menu bar item
    pub fn update_tooltip(&mut self, id: &str, tooltip: &str) -> Result<(), String> {
        let tray_icon = self
            .tray_icons
            .get(id)
            .ok_or_else(|| format!("Menu bar item not found: {}", id))?;

        tray_icon
            .set_tooltip(Some(tooltip))
            .map_err(|e| e.to_string())
    }

    /// Process pending events (call this in your event loop)
    pub fn process_events(&mut self) -> Vec<MenuBarEvent> {
        let mut events = Vec::new();

        // Process tray icon events
        while let Ok(event) = TrayIconEvent::receiver().try_recv() {
            match event {
                TrayIconEvent::Click {
                    id,
                    position: _,
                    rect: _,
                    button: _,
                    button_state: _,
                } => {
                    // Find the tray ID from the internal tray icon ID
                    for (tray_id, tray_icon) in &self.tray_icons {
                        if tray_icon.id() == &id {
                            events.push(MenuBarEvent::Click {
                                id: tray_id.clone(),
                            });
                            break;
                        }
                    }
                }
                TrayIconEvent::DoubleClick {
                    id,
                    position: _,
                    rect: _,
                    button: _,
                } => {
                    for (tray_id, tray_icon) in &self.tray_icons {
                        if tray_icon.id() == &id {
                            events.push(MenuBarEvent::DoubleClick {
                                id: tray_id.clone(),
                            });
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        // Process menu events
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            let menu_id = event.id.0.clone();
            if let Some((tray_id, menu_item_id)) = self.menu_item_map.get(&menu_id) {
                // Check for quit
                if menu_item_id == "__quit__" {
                    events.push(MenuBarEvent::QuitRequested);
                } else {
                    events.push(MenuBarEvent::MenuItemClick {
                        tray_id: tray_id.clone(),
                        menu_item_id: menu_item_id.clone(),
                    });
                }
            }
        }

        events
    }

    /// Get list of all menu bar item IDs
    pub fn list_items(&self) -> Vec<String> {
        self.tray_icons.keys().cloned().collect()
    }

    /// Create a tray icon from MenuBarIcon
    fn create_icon(&self, icon: &MenuBarIcon) -> Result<Icon, String> {
        match icon {
            MenuBarIcon::Rgba { data, width, height } => {
                Icon::from_rgba(data.clone(), *width, *height).map_err(|e| e.to_string())
            }
            MenuBarIcon::Path(path) => {
                // Use new image loader that supports SVG
                let loaded = crate::util::load_image(path, Some(22), Some(22))?;
                Icon::from_rgba(loaded.data, loaded.width, loaded.height).map_err(|e| e.to_string())
            }
            MenuBarIcon::WindowIcon(window_icon) => {
                // Convert WindowIcon to RGBA
                self.window_icon_to_rgba(window_icon)
            }
        }
    }

    /// Convert WindowIcon to tray Icon
    fn window_icon_to_rgba(&self, _window_icon: &crate::icon::WindowIcon) -> Result<Icon, String> {
        // For now, create a simple colored icon
        // TODO: Properly render WindowIcon (emoji/preset) to RGBA
        let size = 22u32; // Standard macOS menu bar icon size
        let mut data = Vec::with_capacity((size * size * 4) as usize);

        // Create a simple circular icon
        let center = size as f32 / 2.0;
        let radius = center - 2.0;

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius {
                    // Inside circle - cyan color
                    data.extend_from_slice(&[100, 200, 255, 255]);
                } else {
                    // Outside - transparent
                    data.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }

        Icon::from_rgba(data, size, size).map_err(|e| e.to_string())
    }

    /// Create a muda Menu from MenuBarMenu
    fn create_muda_menu(&mut self, tray_id: &str, menu: &MenuBarMenu) -> Result<Menu, String> {
        let muda_menu = Menu::new();

        for item in &menu.items {
            self.add_menu_item_to_menu(&muda_menu, tray_id, item)?;
        }

        Ok(muda_menu)
    }

    /// Add a MenuBarMenuItem to a muda Menu
    fn add_menu_item_to_menu(
        &mut self,
        menu: &Menu,
        tray_id: &str,
        item: &MenuBarMenuItem,
    ) -> Result<(), String> {
        match item {
            MenuBarMenuItem::Item { id, label, enabled } => {
                let menu_item = MenuItem::new(label, *enabled, None);
                let muda_id = menu_item.id().0.clone();
                self.menu_item_map
                    .insert(muda_id, (tray_id.to_string(), id.clone()));
                menu.append(&menu_item).map_err(|e| e.to_string())?;
            }
            MenuBarMenuItem::Submenu { label, items } => {
                let submenu = Submenu::new(label, true);
                for sub_item in items {
                    self.add_menu_item_to_submenu(&submenu, tray_id, sub_item)?;
                }
                menu.append(&submenu).map_err(|e| e.to_string())?;
            }
            MenuBarMenuItem::Separator => {
                menu.append(&PredefinedMenuItem::separator())
                    .map_err(|e| e.to_string())?;
            }
            MenuBarMenuItem::Predefined(predefined) => match predefined {
                PredefinedMenuItemType::Quit => {
                    let quit_item = MenuItem::new("Quit", true, None);
                    let muda_id = quit_item.id().0.clone();
                    self.menu_item_map
                        .insert(muda_id, (tray_id.to_string(), "__quit__".to_string()));
                    menu.append(&quit_item).map_err(|e| e.to_string())?;
                }
                PredefinedMenuItemType::About => {
                    menu.append(&PredefinedMenuItem::about(None, None))
                        .map_err(|e| e.to_string())?;
                }
                PredefinedMenuItemType::Separator => {
                    menu.append(&PredefinedMenuItem::separator())
                        .map_err(|e| e.to_string())?;
                }
            },
        }
        Ok(())
    }

    /// Add a MenuBarMenuItem to a muda Submenu
    fn add_menu_item_to_submenu(
        &mut self,
        submenu: &Submenu,
        tray_id: &str,
        item: &MenuBarMenuItem,
    ) -> Result<(), String> {
        match item {
            MenuBarMenuItem::Item { id, label, enabled } => {
                let menu_item = MenuItem::new(label, *enabled, None);
                let muda_id = menu_item.id().0.clone();
                self.menu_item_map
                    .insert(muda_id, (tray_id.to_string(), id.clone()));
                submenu.append(&menu_item).map_err(|e| e.to_string())?;
            }
            MenuBarMenuItem::Submenu { label, items } => {
                let nested_submenu = Submenu::new(label, true);
                for sub_item in items {
                    self.add_menu_item_to_submenu(&nested_submenu, tray_id, sub_item)?;
                }
                submenu.append(&nested_submenu).map_err(|e| e.to_string())?;
            }
            MenuBarMenuItem::Separator => {
                submenu
                    .append(&PredefinedMenuItem::separator())
                    .map_err(|e| e.to_string())?;
            }
            MenuBarMenuItem::Predefined(predefined) => match predefined {
                PredefinedMenuItemType::Quit => {
                    let quit_item = MenuItem::new("Quit", true, None);
                    let muda_id = quit_item.id().0.clone();
                    self.menu_item_map
                        .insert(muda_id, (tray_id.to_string(), "__quit__".to_string()));
                    submenu.append(&quit_item).map_err(|e| e.to_string())?;
                }
                PredefinedMenuItemType::About => {
                    submenu
                        .append(&PredefinedMenuItem::about(None, None))
                        .map_err(|e| e.to_string())?;
                }
                PredefinedMenuItemType::Separator => {
                    submenu
                        .append(&PredefinedMenuItem::separator())
                        .map_err(|e| e.to_string())?;
                }
            },
        }
        Ok(())
    }
}

impl Default for MenuBarManager {
    fn default() -> Self {
        Self::new()
    }
}
