//! Click Helper - Keyboard-driven UI element clicking (EasyMotion/Vimium-style)
//!
//! Provides quick keyboard navigation to clickable UI elements using accessibility APIs.
//! When triggered by a global hotkey, it:
//! 1. Shows a dimmed overlay over the entire screen
//! 2. Scans for clickable UI elements using platform accessibility APIs
//! 3. Labels each element with hint characters (two-tier: symbols first, then letters)
//! 4. User types hint characters to click the corresponding element
//! 5. ESC cancels the mode

pub mod accessibility;
pub mod config;
mod hints;
mod hotkey;
mod mode;
mod overlay;

pub use accessibility::{AccessibilityProvider, ClickableElement};
pub use config::{ClickHelperConfig, Modifier};
pub use hints::{HintGenerator, HintLabel};
pub use hotkey::ClickHelperHotkeyManager;
pub use mode::{ClickHelperAction, ClickHelperMode, ClickHelperState};
pub use overlay::ClickHelperOverlay;

/// Get the click helper data directory
pub fn get_click_helper_data_dir() -> crate::error::Result<std::path::PathBuf> {
    let home = std::env::var("HOME").map_err(|_| {
        crate::error::AumateError::Other("HOME environment variable not set".into())
    })?;
    let data_dir = std::path::PathBuf::from(home).join(".aumate");
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
    }
    Ok(data_dir)
}
