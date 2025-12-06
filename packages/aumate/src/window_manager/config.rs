//! Window Manager configuration and settings

use crate::error::{AumateError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Modifier keys for hotkey combinations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
    Meta,
}

impl Modifier {
    /// Get display name for the modifier
    pub fn display_name(&self) -> &'static str {
        match self {
            Modifier::Ctrl => "Ctrl",
            Modifier::Alt => "Alt",
            Modifier::Shift => "Shift",
            Modifier::Meta => {
                #[cfg(target_os = "macos")]
                {
                    "Cmd"
                }
                #[cfg(not(target_os = "macos"))]
                {
                    "Win"
                }
            }
        }
    }
}

/// Hotkey configuration for Window Manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// The main key (e.g., "Space", "K", etc.)
    pub key: String,
    /// Modifier keys
    pub modifiers: Vec<Modifier>,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        // Default: Cmd+Space
        Self { key: "Space".to_string(), modifiers: vec![Modifier::Meta] }
    }
}

impl HotkeyConfig {
    /// Get a display string for the hotkey (e.g., "Cmd+Shift+Space")
    pub fn display_string(&self) -> String {
        let mut parts = Vec::new();
        for modifier in &self.modifiers {
            parts.push(modifier.display_name());
        }
        parts.push(&self.key);
        parts.join("+")
    }
}

/// Window Manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowManagerConfig {
    /// Global hotkey to open the command palette
    pub hotkey: HotkeyConfig,

    /// Whether the global hotkey listener is enabled
    #[serde(default = "default_hotkey_enabled")]
    pub hotkey_enabled: bool,
}

fn default_hotkey_enabled() -> bool {
    true
}

impl Default for WindowManagerConfig {
    fn default() -> Self {
        Self { hotkey: HotkeyConfig::default(), hotkey_enabled: true }
    }
}

/// Get the data directory for window manager
pub fn get_window_manager_data_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").map_err(|e| AumateError::Other(e.to_string()))?;
    let data_dir = PathBuf::from(home).join(".aumate");
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
    }
    Ok(data_dir)
}

impl WindowManagerConfig {
    /// Get the config file path
    fn config_path() -> Result<PathBuf> {
        let data_dir = get_window_manager_data_dir()?;
        Ok(data_dir.join("window_manager_config.json"))
    }

    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let config: Self =
                serde_json::from_str(&content).map_err(|e| AumateError::Other(e.to_string()))?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content =
            serde_json::to_string_pretty(self).map_err(|e| AumateError::Other(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = WindowManagerConfig::default();
        assert_eq!(config.hotkey.key, "Space");
        assert!(config.hotkey.modifiers.contains(&Modifier::Meta));
        assert!(config.hotkey.modifiers.contains(&Modifier::Shift));
        assert!(config.hotkey_enabled);
    }

    #[test]
    fn test_hotkey_display() {
        let hotkey = HotkeyConfig::default();
        let display = hotkey.display_string();
        // On macOS: "Cmd+Shift+Space", on others: "Win+Shift+Space"
        assert!(display.contains("Shift"));
        assert!(display.contains("Space"));
    }

    #[test]
    fn test_serialization() {
        let config = WindowManagerConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: WindowManagerConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.hotkey.key, config.hotkey.key);
        assert_eq!(parsed.hotkey_enabled, config.hotkey_enabled);
    }
}
