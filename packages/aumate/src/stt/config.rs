//! STT configuration and settings

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Output mode for transcribed text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    /// Type text as keystrokes
    #[default]
    Keystrokes,
    /// Copy text to clipboard
    Clipboard,
    /// Copy to clipboard and paste
    Both,
}

impl OutputMode {
    /// Get all available output modes
    pub fn all() -> &'static [OutputMode] {
        &[OutputMode::Keystrokes, OutputMode::Clipboard, OutputMode::Both]
    }

    /// Get display name for the output mode
    pub fn display_name(&self) -> &'static str {
        match self {
            OutputMode::Keystrokes => "Keystrokes",
            OutputMode::Clipboard => "Clipboard",
            OutputMode::Both => "Both",
        }
    }
}

/// Hotkey activation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum HotkeyMode {
    /// Hold to record, release to transcribe
    #[default]
    PushToTalk,
    /// Press to start, press again to stop
    Toggle,
}

impl HotkeyMode {
    /// Get display name for the hotkey mode
    pub fn display_name(&self) -> &'static str {
        match self {
            HotkeyMode::PushToTalk => "Push to Talk",
            HotkeyMode::Toggle => "Toggle",
        }
    }
}

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

/// Hotkey configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    /// The main key (e.g., "Space", "F1", etc.)
    pub key: String,
    /// Modifier keys
    pub modifiers: Vec<Modifier>,
    /// Activation mode
    pub mode: HotkeyMode,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            key: "Space".to_string(),
            modifiers: vec![Modifier::Ctrl, Modifier::Shift],
            mode: HotkeyMode::PushToTalk,
        }
    }
}

impl HotkeyConfig {
    /// Get a display string for the hotkey (e.g., "Ctrl+Shift+Space")
    pub fn display_string(&self) -> String {
        let mut parts = Vec::new();
        for modifier in &self.modifiers {
            parts.push(modifier.display_name());
        }
        parts.push(&self.key);
        parts.join("+")
    }
}

fn default_hotkey_enabled() -> bool {
    true
}

/// STT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttConfig {
    /// Hotkey configuration
    pub hotkey: HotkeyConfig,
    /// Whether the global hotkey listener is enabled
    #[serde(default = "default_hotkey_enabled")]
    pub hotkey_enabled: bool,
    /// Output mode for transcribed text
    pub output_mode: OutputMode,
    /// Selected model ID
    pub model_id: String,
    /// Language for transcription (None = auto-detect)
    pub language: Option<String>,
    /// Enable Voice Activity Detection
    pub vad_enabled: bool,
    /// Silence duration in ms to auto-stop recording
    pub vad_silence_duration_ms: u32,
    /// Input device name (None = default)
    pub input_device: Option<String>,
}

impl Default for SttConfig {
    fn default() -> Self {
        Self {
            hotkey: HotkeyConfig::default(),
            hotkey_enabled: true,
            output_mode: OutputMode::default(),
            model_id: "whisper-base".to_string(),
            language: None,
            vad_enabled: true,
            vad_silence_duration_ms: 1500,
            input_device: None,
        }
    }
}

impl SttConfig {
    /// Get the config file path
    fn config_path() -> Result<PathBuf> {
        let data_dir = super::get_stt_data_dir()?;
        Ok(data_dir.join("stt_config.json"))
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

    /// Get available languages for transcription
    pub fn available_languages() -> &'static [(&'static str, &'static str)] {
        &[
            ("auto", "Auto-detect"),
            ("en", "English"),
            ("zh", "Chinese"),
            ("de", "German"),
            ("es", "Spanish"),
            ("ru", "Russian"),
            ("ko", "Korean"),
            ("fr", "French"),
            ("ja", "Japanese"),
            ("pt", "Portuguese"),
            ("tr", "Turkish"),
            ("pl", "Polish"),
            ("ca", "Catalan"),
            ("nl", "Dutch"),
            ("ar", "Arabic"),
            ("sv", "Swedish"),
            ("it", "Italian"),
            ("id", "Indonesian"),
            ("hi", "Hindi"),
            ("fi", "Finnish"),
            ("vi", "Vietnamese"),
            ("he", "Hebrew"),
            ("uk", "Ukrainian"),
            ("el", "Greek"),
            ("ms", "Malay"),
            ("cs", "Czech"),
            ("ro", "Romanian"),
            ("da", "Danish"),
            ("hu", "Hungarian"),
            ("ta", "Tamil"),
            ("no", "Norwegian"),
            ("th", "Thai"),
            ("ur", "Urdu"),
            ("hr", "Croatian"),
            ("bg", "Bulgarian"),
            ("lt", "Lithuanian"),
            ("la", "Latin"),
            ("mi", "Maori"),
            ("ml", "Malayalam"),
            ("cy", "Welsh"),
            ("sk", "Slovak"),
            ("te", "Telugu"),
            ("fa", "Persian"),
            ("lv", "Latvian"),
            ("bn", "Bengali"),
            ("sr", "Serbian"),
            ("az", "Azerbaijani"),
            ("sl", "Slovenian"),
            ("kn", "Kannada"),
            ("et", "Estonian"),
            ("mk", "Macedonian"),
            ("br", "Breton"),
            ("eu", "Basque"),
            ("is", "Icelandic"),
            ("hy", "Armenian"),
            ("ne", "Nepali"),
            ("mn", "Mongolian"),
            ("bs", "Bosnian"),
            ("kk", "Kazakh"),
            ("sq", "Albanian"),
            ("sw", "Swahili"),
            ("gl", "Galician"),
            ("mr", "Marathi"),
            ("pa", "Punjabi"),
            ("si", "Sinhala"),
            ("km", "Khmer"),
            ("sn", "Shona"),
            ("yo", "Yoruba"),
            ("so", "Somali"),
            ("af", "Afrikaans"),
            ("oc", "Occitan"),
            ("ka", "Georgian"),
            ("be", "Belarusian"),
            ("tg", "Tajik"),
            ("sd", "Sindhi"),
            ("gu", "Gujarati"),
            ("am", "Amharic"),
            ("yi", "Yiddish"),
            ("lo", "Lao"),
            ("uz", "Uzbek"),
            ("fo", "Faroese"),
            ("ht", "Haitian creole"),
            ("ps", "Pashto"),
            ("tk", "Turkmen"),
            ("nn", "Nynorsk"),
            ("mt", "Maltese"),
            ("sa", "Sanskrit"),
            ("lb", "Luxembourgish"),
            ("my", "Myanmar"),
            ("bo", "Tibetan"),
            ("tl", "Tagalog"),
            ("mg", "Malagasy"),
            ("as", "Assamese"),
            ("tt", "Tatar"),
            ("haw", "Hawaiian"),
            ("ln", "Lingala"),
            ("ha", "Hausa"),
            ("ba", "Bashkir"),
            ("jw", "Javanese"),
            ("su", "Sundanese"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SttConfig::default();
        assert_eq!(config.model_id, "whisper-base");
        assert!(config.vad_enabled);
        assert_eq!(config.vad_silence_duration_ms, 1500);
    }

    #[test]
    fn test_hotkey_display() {
        let hotkey = HotkeyConfig::default();
        assert_eq!(hotkey.display_string(), "Ctrl+Shift+Space");
    }

    #[test]
    fn test_serialization() {
        let config = SttConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: SttConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.model_id, config.model_id);
    }
}
