//! Keyboard control module
//!
//! Provides keyboard input simulation including key presses, typing, and modifier support.

use crate::error::{AumateError, Result};
use enigo::{Direction, Enigo, Key, Keyboard as KeyboardTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Keyboard controller
pub struct Keyboard {
    enigo: Arc<Mutex<Enigo>>,
    delay_ms: Arc<Mutex<u32>>,
}

impl Keyboard {
    /// Create a new keyboard controller
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&enigo::Settings::default())
            .map_err(|e| AumateError::Input(format!("Failed to create Enigo: {}", e)))?;
        Ok(Self { enigo: Arc::new(Mutex::new(enigo)), delay_ms: Arc::new(Mutex::new(10)) })
    }

    /// Tap a key (press and release)
    pub fn key_tap(&self, key: &str, modifier: Option<&[String]>) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;

        // Handle modifiers
        if let Some(mods) = modifier {
            for mod_key in mods {
                let key_code = self.parse_key(mod_key)?;
                let _ = enigo.key(key_code, Direction::Press);
            }
        }

        // Press the main key
        let key_code = self.parse_key(key)?;
        let _ = enigo.key(key_code, Direction::Click);

        // Release modifiers in reverse order
        if let Some(mods) = modifier {
            for mod_key in mods.iter().rev() {
                let key_code = self.parse_key(mod_key)?;
                let _ = enigo.key(key_code, Direction::Release);
            }
        }

        self.apply_delay();
        Ok(())
    }

    /// Toggle a key (press or release)
    pub fn key_toggle(&self, key: &str, down: &str, modifier: Option<&[String]>) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;

        let direction = match down {
            "down" => Direction::Press,
            "up" => Direction::Release,
            _ => return Err(AumateError::Input(format!("Invalid direction: {}", down))),
        };

        // Handle modifiers
        if let Some(mods) = modifier {
            for mod_key in mods {
                let key_code = self.parse_key(mod_key)?;
                let _ = enigo.key(key_code, direction);
            }
        }

        // Press/release the main key
        let key_code = self.parse_key(key)?;
        let _ = enigo.key(key_code, direction);

        self.apply_delay();
        Ok(())
    }

    /// Type a string
    pub fn type_string(&self, string: &str) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        let _ = enigo.text(string);
        self.apply_delay();
        Ok(())
    }

    /// Type a string with delay between characters
    pub fn type_string_delayed(&self, string: &str, cpm: u32) -> Result<()> {
        let delay_ms = if cpm > 0 { (60000.0 / cpm as f64) as u64 } else { 0 };

        for ch in string.chars() {
            let mut enigo =
                self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
            let _ = enigo.text(&ch.to_string());
            drop(enigo);

            if delay_ms > 0 {
                thread::sleep(Duration::from_millis(delay_ms));
            }
        }

        self.apply_delay();
        Ok(())
    }

    /// Tap a Unicode character
    pub fn unicode_tap(&self, ch: char) -> Result<()> {
        let mut enigo =
            self.enigo.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        let _ = enigo.key(Key::Unicode(ch), Direction::Click);
        self.apply_delay();
        Ok(())
    }

    /// Set the keyboard delay in milliseconds
    pub fn set_delay(&self, delay_ms: u32) -> Result<()> {
        let mut delay =
            self.delay_ms.lock().map_err(|e| AumateError::Input(format!("Lock error: {}", e)))?;
        *delay = delay_ms;
        Ok(())
    }

    /// Set the keyboard delay in milliseconds (robotjs compatible alias)
    pub fn set_keyboard_delay(&self, delay_ms: u32) -> Result<()> {
        self.set_delay(delay_ms)
    }

    fn parse_key(&self, key: &str) -> Result<Key> {
        match key.to_lowercase().as_str() {
            // Modifiers
            "command" | "cmd" | "meta" => Ok(Key::Meta),
            "alt" => Ok(Key::Alt),
            "control" | "ctrl" => Ok(Key::Control),
            "shift" => Ok(Key::Shift),

            // Function keys
            "f1" => Ok(Key::F1),
            "f2" => Ok(Key::F2),
            "f3" => Ok(Key::F3),
            "f4" => Ok(Key::F4),
            "f5" => Ok(Key::F5),
            "f6" => Ok(Key::F6),
            "f7" => Ok(Key::F7),
            "f8" => Ok(Key::F8),
            "f9" => Ok(Key::F9),
            "f10" => Ok(Key::F10),
            "f11" => Ok(Key::F11),
            "f12" => Ok(Key::F12),

            // Special keys
            "enter" | "return" => Ok(Key::Return),
            "escape" | "esc" => Ok(Key::Escape),
            "backspace" => Ok(Key::Backspace),
            "tab" => Ok(Key::Tab),
            "space" => Ok(Key::Space),
            "delete" | "del" => Ok(Key::Delete),
            "up" => Ok(Key::UpArrow),
            "down" => Ok(Key::DownArrow),
            "left" => Ok(Key::LeftArrow),
            "right" => Ok(Key::RightArrow),
            "home" => Ok(Key::Home),
            "end" => Ok(Key::End),
            "pageup" | "page_up" => Ok(Key::PageUp),
            "pagedown" | "page_down" => Ok(Key::PageDown),

            // Single character keys
            _ => {
                if key.len() == 1 {
                    let ch = key.chars().next().expect("key has at least one char");
                    Ok(Key::Unicode(ch))
                } else {
                    Err(AumateError::Input(format!("Invalid key: {}", key)))
                }
            }
        }
    }

    fn apply_delay(&self) {
        let delay = self.delay_ms.lock().ok().map(|d| *d).unwrap_or(10);
        if delay > 0 {
            thread::sleep(Duration::from_millis(delay as u64));
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new().expect("Failed to create Keyboard")
    }
}
