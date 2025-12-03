//! Click Helper configuration and settings

use crate::error::Result;
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

/// Hotkey configuration for Click Helper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// The main key (e.g., "2", "F1", etc.)
    pub key: String,
    /// Modifier keys
    pub modifiers: Vec<Modifier>,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self { key: "2".to_string(), modifiers: vec![Modifier::Ctrl] }
    }
}

impl HotkeyConfig {
    /// Get a display string for the hotkey (e.g., "Ctrl+2")
    pub fn display_string(&self) -> String {
        let mut parts = Vec::new();
        for modifier in &self.modifiers {
            parts.push(modifier.display_name());
        }
        parts.push(&self.key);
        parts.join("+")
    }
}

/// Click Helper configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickHelperConfig {
    /// Global hotkey to activate Click Helper
    pub hotkey: HotkeyConfig,

    /// Whether the global hotkey listener is enabled
    #[serde(default = "default_hotkey_enabled")]
    pub hotkey_enabled: bool,

    /// First-tier hint characters (for grouping when >26 elements)
    /// Default: ,./;'[]
    pub tier1_chars: String,

    /// Second-tier hint characters (actual element selection)
    /// Default: a-z
    pub tier2_chars: String,

    /// Hint label font size
    pub hint_font_size: f32,

    /// Hint background color (RGBA)
    pub hint_bg_color: [u8; 4],

    /// Hint text color (RGBA)
    pub hint_fg_color: [u8; 4],

    /// Overlay opacity (0-255)
    pub overlay_opacity: u8,
}

fn default_hotkey_enabled() -> bool {
    true
}

impl Default for ClickHelperConfig {
    fn default() -> Self {
        Self {
            hotkey: HotkeyConfig::default(),
            hotkey_enabled: true,
            tier1_chars: ",./;'[]".to_string(),
            tier2_chars: "abcdefghijklmnopqrstuvwxyz".to_string(),
            hint_font_size: 14.0,
            hint_bg_color: [255, 220, 0, 230], // Yellow
            hint_fg_color: [0, 0, 0, 255],     // Black
            overlay_opacity: 120,
        }
    }
}

impl ClickHelperConfig {
    /// Get the config file path
    fn config_path() -> Result<PathBuf> {
        let data_dir = super::get_click_helper_data_dir()?;
        Ok(data_dir.join("click_helper_config.json"))
    }

    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let config: Self = serde_json::from_str(&content)
                .map_err(|e| crate::error::AumateError::Other(e.to_string()))?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| crate::error::AumateError::Other(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get tier1 chars as a vector
    pub fn tier1_chars_vec(&self) -> Vec<char> {
        self.tier1_chars.chars().collect()
    }

    /// Get tier2 chars as a vector
    pub fn tier2_chars_vec(&self) -> Vec<char> {
        self.tier2_chars.chars().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClickHelperConfig::default();
        assert_eq!(config.hotkey.key, "2");
        assert_eq!(config.hotkey.modifiers, vec![Modifier::Ctrl]);
        assert_eq!(config.tier1_chars, ",./;'[]");
        assert_eq!(config.tier2_chars, "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn test_hotkey_display() {
        let hotkey = HotkeyConfig::default();
        assert_eq!(hotkey.display_string(), "Ctrl+2");
    }

    #[test]
    fn test_serialization() {
        let config = ClickHelperConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: ClickHelperConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.tier1_chars, config.tier1_chars);
    }
}
