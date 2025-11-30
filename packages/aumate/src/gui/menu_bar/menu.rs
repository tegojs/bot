//! Menu definitions for dropdown menus

/// A dropdown menu for a menu bar item
#[derive(Debug, Clone, Default)]
pub struct MenuBarMenu {
    /// Menu items
    pub items: Vec<MenuBarMenuItem>,
}

/// A menu item in a dropdown menu
#[derive(Debug, Clone)]
pub enum MenuBarMenuItem {
    /// Regular clickable item
    Item { id: String, label: String, enabled: bool },
    /// Submenu with nested items
    Submenu { label: String, items: Vec<MenuBarMenuItem> },
    /// Separator line
    Separator,
    /// Predefined items (Quit, About, etc.)
    Predefined(PredefinedMenuItemType),
}

/// Predefined menu item types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredefinedMenuItemType {
    /// Quit the application
    Quit,
    /// About dialog
    About,
    /// Separator (for convenience)
    Separator,
}

impl MenuBarMenu {
    /// Create a new empty menu
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a regular menu item
    pub fn add_item(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.items.push(MenuBarMenuItem::Item {
            id: id.into(),
            label: label.into(),
            enabled: true,
        });
        self
    }

    /// Add a disabled menu item
    pub fn add_item_disabled(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.items.push(MenuBarMenuItem::Item {
            id: id.into(),
            label: label.into(),
            enabled: false,
        });
        self
    }

    /// Add a submenu
    pub fn add_submenu(mut self, label: impl Into<String>, items: Vec<MenuBarMenuItem>) -> Self {
        self.items.push(MenuBarMenuItem::Submenu { label: label.into(), items });
        self
    }

    /// Add a separator
    pub fn add_separator(mut self) -> Self {
        self.items.push(MenuBarMenuItem::Separator);
        self
    }

    /// Add a quit item
    pub fn add_quit(mut self) -> Self {
        self.items.push(MenuBarMenuItem::Predefined(PredefinedMenuItemType::Quit));
        self
    }

    /// Add an about item
    pub fn add_about(mut self) -> Self {
        self.items.push(MenuBarMenuItem::Predefined(PredefinedMenuItemType::About));
        self
    }
}
