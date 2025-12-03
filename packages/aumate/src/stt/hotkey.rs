//! Global hotkey management for STT
//!
//! Uses eventhooks::grab for truly global hotkey interception across platforms.
//!
//! Note: On macOS, this requires Accessibility permissions.
//! On Linux, the process needs to run as root or be in the 'input' group.

use super::config::{HotkeyConfig, HotkeyMode, Modifier};
use crate::error::{AumateError, Result};
use crate::eventhooks::{Event, EventType, Key, grab};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Hotkey event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyEvent {
    /// Start recording
    RecordStart,
    /// Stop recording
    RecordStop,
}

/// Callback type for hotkey events
pub type HotkeyCallback = Arc<dyn Fn(HotkeyEvent) + Send + Sync>;

/// Global hotkey manager
pub struct HotkeyManager {
    /// Whether the listener is running
    is_running: Arc<AtomicBool>,
    /// Listener thread handle
    listener_handle: Option<JoinHandle<()>>,
    /// Whether we're in recording state (for toggle mode)
    is_recording: Arc<AtomicBool>,
    /// Hotkey configuration
    config: Arc<Mutex<HotkeyConfig>>,
    /// Event callback
    callback: Option<HotkeyCallback>,
}

impl HotkeyManager {
    /// Create a new hotkey manager
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            listener_handle: None,
            is_recording: Arc::new(AtomicBool::new(false)),
            config: Arc::new(Mutex::new(HotkeyConfig::default())),
            callback: None,
        }
    }

    /// Set the hotkey configuration
    pub fn set_config(&mut self, config: HotkeyConfig) {
        *self.config.lock().unwrap() = config;
    }

    /// Get the current hotkey configuration
    pub fn config(&self) -> HotkeyConfig {
        self.config.lock().unwrap().clone()
    }

    /// Set the event callback
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(HotkeyEvent) + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(callback));
    }

    /// Check if the listener is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    /// Start listening for hotkeys using rdev::grab
    pub fn start(&mut self) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        let callback = self
            .callback
            .clone()
            .ok_or_else(|| AumateError::Other("No callback set".to_string()))?;

        let is_running = self.is_running.clone();
        let is_recording = self.is_recording.clone();
        let config = self.config.clone();

        is_running.store(true, Ordering::Relaxed);

        let handle = thread::spawn(move || {
            run_grab_loop(is_running.clone(), config, callback, is_recording);
        });

        self.listener_handle = Some(handle);
        log::info!("STT hotkey listener started (rdev::grab)");

        Ok(())
    }

    /// Stop listening for hotkeys
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        self.is_recording.store(false, Ordering::Relaxed);
        // Note: rdev::grab is blocking, so we can't cleanly stop it.
        // The thread will exit on the next event or when the process exits.
        self.listener_handle = None;
        log::info!("STT hotkey listener stopped");
    }

    /// Reset recording state (for toggle mode)
    pub fn reset_recording_state(&self) {
        self.is_recording.store(false, Ordering::Relaxed);
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Run the rdev::grab event loop
fn run_grab_loop(
    is_running: Arc<AtomicBool>,
    config: Arc<Mutex<HotkeyConfig>>,
    callback: HotkeyCallback,
    is_recording: Arc<AtomicBool>,
) {
    // Track pressed modifiers
    let pressed_modifiers = Arc::new(Mutex::new(std::collections::HashSet::<Modifier>::new()));
    let main_key_pressed = Arc::new(AtomicBool::new(false));

    let pressed_modifiers_clone = pressed_modifiers.clone();
    let main_key_pressed_clone = main_key_pressed.clone();

    let grab_callback = move |event: Event| -> Option<Event> {
        // Check if we should stop
        if !is_running.load(Ordering::Relaxed) {
            return Some(event);
        }

        let config = config.lock().unwrap();

        match event.event_type {
            EventType::KeyPress(key) => {
                // Track modifier key presses
                if let Some(modifier) = key_to_modifier(&key) {
                    pressed_modifiers_clone.lock().unwrap().insert(modifier);
                }

                // Check if this is our target key
                if let Some(target_key) = parse_key_string(&config.key) {
                    if key == target_key {
                        // Check if all required modifiers are pressed
                        let pressed = pressed_modifiers_clone.lock().unwrap();
                        let all_modifiers_match =
                            config.modifiers.iter().all(|m| pressed.contains(m));

                        if all_modifiers_match && !main_key_pressed_clone.load(Ordering::Relaxed) {
                            main_key_pressed_clone.store(true, Ordering::Relaxed);

                            match config.mode {
                                HotkeyMode::PushToTalk => {
                                    // Start recording on key press
                                    log::info!("STT hotkey pressed - start recording");
                                    (callback)(HotkeyEvent::RecordStart);
                                }
                                HotkeyMode::Toggle => {
                                    // Toggle recording state
                                    let was_recording =
                                        is_recording.fetch_xor(true, Ordering::Relaxed);
                                    if was_recording {
                                        log::info!("STT hotkey pressed - stop recording (toggle)");
                                        (callback)(HotkeyEvent::RecordStop);
                                    } else {
                                        log::info!("STT hotkey pressed - start recording (toggle)");
                                        (callback)(HotkeyEvent::RecordStart);
                                    }
                                }
                            }
                            // Consume the event (don't pass to other apps)
                            return None;
                        }
                    }
                }
            }
            EventType::KeyRelease(key) => {
                // Track modifier key releases
                if let Some(modifier) = key_to_modifier(&key) {
                    pressed_modifiers_clone.lock().unwrap().remove(&modifier);
                }

                // Handle key release for push-to-talk mode
                if let Some(target_key) = parse_key_string(&config.key) {
                    if key == target_key && main_key_pressed_clone.load(Ordering::Relaxed) {
                        main_key_pressed_clone.store(false, Ordering::Relaxed);

                        if config.mode == HotkeyMode::PushToTalk {
                            // Stop recording on key release
                            log::info!("STT hotkey released - stop recording");
                            (callback)(HotkeyEvent::RecordStop);
                        }
                    }
                }
            }
            _ => {}
        }

        // Pass through all other events
        Some(event)
    };

    // This will block until an error occurs or the process exits
    if let Err(error) = grab(grab_callback) {
        log::error!("STT hotkey grab error: {:?}", error);
    }
}

/// Convert rdev Key to our Modifier enum
fn key_to_modifier(key: &Key) -> Option<Modifier> {
    match key {
        Key::ControlLeft | Key::ControlRight => Some(Modifier::Ctrl),
        Key::Alt | Key::AltGr => Some(Modifier::Alt),
        Key::ShiftLeft | Key::ShiftRight => Some(Modifier::Shift),
        Key::MetaLeft | Key::MetaRight => Some(Modifier::Meta),
        _ => None,
    }
}

/// Parse a key string to rdev::Key
fn parse_key_string(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        "0" => Some(Key::Num0),
        "1" => Some(Key::Num1),
        "2" => Some(Key::Num2),
        "3" => Some(Key::Num3),
        "4" => Some(Key::Num4),
        "5" => Some(Key::Num5),
        "6" => Some(Key::Num6),
        "7" => Some(Key::Num7),
        "8" => Some(Key::Num8),
        "9" => Some(Key::Num9),
        "a" => Some(Key::KeyA),
        "b" => Some(Key::KeyB),
        "c" => Some(Key::KeyC),
        "d" => Some(Key::KeyD),
        "e" => Some(Key::KeyE),
        "f" => Some(Key::KeyF),
        "g" => Some(Key::KeyG),
        "h" => Some(Key::KeyH),
        "i" => Some(Key::KeyI),
        "j" => Some(Key::KeyJ),
        "k" => Some(Key::KeyK),
        "l" => Some(Key::KeyL),
        "m" => Some(Key::KeyM),
        "n" => Some(Key::KeyN),
        "o" => Some(Key::KeyO),
        "p" => Some(Key::KeyP),
        "q" => Some(Key::KeyQ),
        "r" => Some(Key::KeyR),
        "s" => Some(Key::KeyS),
        "t" => Some(Key::KeyT),
        "u" => Some(Key::KeyU),
        "v" => Some(Key::KeyV),
        "w" => Some(Key::KeyW),
        "x" => Some(Key::KeyX),
        "y" => Some(Key::KeyY),
        "z" => Some(Key::KeyZ),
        "space" => Some(Key::Space),
        "enter" | "return" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        "escape" | "esc" => Some(Key::Escape),
        "backspace" => Some(Key::Backspace),
        "delete" => Some(Key::Delete),
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_manager_creation() {
        let manager = HotkeyManager::new();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_config_update() {
        let mut manager = HotkeyManager::new();
        let config = HotkeyConfig {
            key: "F1".to_string(),
            modifiers: vec![Modifier::Ctrl],
            mode: HotkeyMode::Toggle,
        };
        manager.set_config(config.clone());
        assert_eq!(manager.config().key, "F1");
    }
}
