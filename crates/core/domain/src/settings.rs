// Settings Domain Models
use serde::{Deserialize, Serialize};

/// General settings for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub follow_system_appearance: bool,
    pub open_at_login: bool,
    pub show_in_system_tray: bool,
    pub hotkey: String,
    pub window_mode: String, // "compact" or "expanded"
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            follow_system_appearance: true,
            open_at_login: false,
            show_in_system_tray: true,
            hotkey: "F3".to_string(),
            window_mode: "compact".to_string(),
        }
    }
}

/// Shortcut settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutSettings {
    pub toggle_palette: String,
    pub open_settings: String,
    #[serde(default = "default_screenshot_hotkey")]
    pub screenshot: String,
}

fn default_screenshot_hotkey() -> String {
    "Ctrl+4".to_string()
}

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self { 
            toggle_palette: "F3".to_string(), 
            open_settings: "Ctrl+,".to_string(),
            screenshot: default_screenshot_hotkey(),
        }
    }
}

/// Advanced settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub debug_mode: bool,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self { debug_mode: false }
    }
}

/// Expression polishing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionPolishingSettings {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
    pub system_prompt: String,
}

impl Default for ExpressionPolishingSettings {
    fn default() -> Self {
        Self {
            api_url: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model: "gpt-4".to_string(),
            system_prompt: "You are an expression polishing assistant. When given text:\n1. Provide a polished, improved version of the expression\n2. Explain the key adjustments you made\n\nFormat your response as:\n**Polished:**\n[improved text]\n\n**Adjustments:**\n[bullet points explaining changes]".to_string(),
        }
    }
}

/// AI dialogue settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDialogueSettings {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
    pub system_prompt: String,
    pub max_history_messages: i32,
}

impl Default for AIDialogueSettings {
    fn default() -> Self {
        Self {
            api_url: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model: "gpt-4".to_string(),
            system_prompt: "You are a helpful assistant.".to_string(),
            max_history_messages: 20,
        }
    }
}

/// Enabled modes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledModes {
    pub search: bool,
    pub polish: bool,
    pub dialogue: bool,
    #[serde(default = "default_true")]
    pub switcher: bool,
}

fn default_true() -> bool {
    true
}

impl Default for EnabledModes {
    fn default() -> Self {
        Self { search: true, polish: true, dialogue: true, switcher: true }
    }
}

/// Screenshot settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotSettings {
    pub save_folder: String,
    pub filename_pattern: String,
    pub image_format: String,
    pub auto_copy_clipboard: bool,
}

impl Default for ScreenshotSettings {
    fn default() -> Self {
        Self {
            save_folder: String::new(),
            filename_pattern: "screenshot_%Y%m%d_%H%M%S".to_string(),
            image_format: "png".to_string(),
            auto_copy_clipboard: true,
        }
    }
}

/// Complete application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub shortcuts: ShortcutSettings,
    pub advanced: AdvancedSettings,
    pub expression_polishing: ExpressionPolishingSettings,
    pub screenshot: ScreenshotSettings,
    #[serde(default)]
    pub ai_dialogue: AIDialogueSettings,
    #[serde(default)]
    pub enabled_modes: EnabledModes,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            shortcuts: ShortcutSettings::default(),
            advanced: AdvancedSettings::default(),
            expression_polishing: ExpressionPolishingSettings::default(),
            screenshot: ScreenshotSettings::default(),
            ai_dialogue: AIDialogueSettings::default(),
            enabled_modes: EnabledModes::default(),
        }
    }
}

