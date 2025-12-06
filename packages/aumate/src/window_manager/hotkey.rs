//! Global hotkey management for Window Manager
//!
//! Uses eventhooks::grab for truly global hotkey interception across platforms.
//!
//! Note: On macOS, this requires Accessibility permissions.
//! On Linux, the process needs to run as root or be in the 'input' group.

use super::config::{HotkeyConfig, Modifier};
use crate::error::{AumateError, Result};
use crate::eventhooks::{Event, EventType, Key, grab};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Callback type for Window Manager hotkey events
pub type HotkeyCallback = Arc<dyn Fn() + Send + Sync>;

/// Global hotkey manager for Window Manager
pub struct WindowManagerHotkeyManager {
    /// Whether the listener is running
    is_running: Arc<AtomicBool>,
    /// Listener thread handle
    listener_handle: Option<JoinHandle<()>>,
    /// Hotkey configuration
    config: Arc<Mutex<HotkeyConfig>>,
    /// Event callback
    callback: Option<HotkeyCallback>,
}

impl WindowManagerHotkeyManager {
    /// Create a new hotkey manager
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            listener_handle: None,
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

    /// Set the event callback (called when hotkey is triggered)
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(callback));
    }

    /// Check if the listener is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    /// Start listening for hotkeys using eventhooks::grab
    pub fn start(&mut self) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        let callback = self
            .callback
            .clone()
            .ok_or_else(|| AumateError::Other("No callback set".to_string()))?;

        let is_running = self.is_running.clone();
        let config = self.config.clone();

        is_running.store(true, Ordering::Relaxed);

        let handle = thread::spawn(move || {
            run_grab_loop(is_running.clone(), config, callback);
        });

        self.listener_handle = Some(handle);
        log::info!("Window Manager hotkey listener started");

        Ok(())
    }

    /// Stop listening for hotkeys
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        // Note: eventhooks::grab is blocking, so we can't cleanly stop it.
        // The thread will exit on the next event or when the process exits.
        self.listener_handle = None;
        log::info!("Window Manager hotkey listener stopped");
    }
}

impl Default for WindowManagerHotkeyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WindowManagerHotkeyManager {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Run the eventhooks::grab event loop
fn run_grab_loop(
    is_running: Arc<AtomicBool>,
    config: Arc<Mutex<HotkeyConfig>>,
    callback: HotkeyCallback,
) {
    log::info!("[WM Hotkey] Starting grab loop...");

    // Track pressed modifiers
    let pressed_modifiers = Arc::new(Mutex::new(std::collections::HashSet::<Modifier>::new()));
    let main_key_pressed = Arc::new(AtomicBool::new(false));

    let pressed_modifiers_clone = pressed_modifiers.clone();
    let main_key_pressed_clone = main_key_pressed.clone();

    // Log the target configuration
    {
        let cfg = config.lock().unwrap();
        log::info!(
            "[WM Hotkey] Listening for: {} (key={:?}, modifiers={:?})",
            cfg.display_string(),
            cfg.key,
            cfg.modifiers
        );
    }

    let grab_callback = move |event: Event| -> Option<Event> {
        // Check if we should stop
        if !is_running.load(Ordering::Relaxed) {
            return Some(event);
        }

        let config = config.lock().unwrap();

        match event.event_type {
            EventType::KeyPress(key) => {
                // Debug: log all key presses
                log::debug!("[WM Hotkey] KeyPress: {:?}", key);

                // Track modifier key presses
                if let Some(modifier) = key_to_modifier(&key) {
                    pressed_modifiers_clone.lock().unwrap().insert(modifier);
                    log::debug!(
                        "[WM Hotkey] Modifier pressed: {:?}, current modifiers: {:?}",
                        modifier,
                        *pressed_modifiers_clone.lock().unwrap()
                    );
                }

                // Check if this is our target key
                if let Some(target_key) = parse_key_string(&config.key) {
                    if key == target_key {
                        log::debug!("[WM Hotkey] Target key pressed: {:?}", key);

                        // Check if all required modifiers are pressed
                        // We use subset matching instead of exact match because key release events
                        // for modifiers like Ctrl may not be received when the palette window has focus
                        let pressed = pressed_modifiers_clone.lock().unwrap();
                        let required_set: std::collections::HashSet<_> =
                            config.modifiers.iter().cloned().collect();
                        let has_all_required = required_set.iter().all(|m| pressed.contains(m));

                        log::debug!(
                            "[WM Hotkey] Required modifiers: {:?}, Pressed: {:?}, Has all required: {}",
                            config.modifiers,
                            *pressed,
                            has_all_required
                        );

                        if has_all_required && !main_key_pressed_clone.load(Ordering::Relaxed) {
                            main_key_pressed_clone.store(true, Ordering::Relaxed);
                            log::info!(
                                "[WM Hotkey] >>> HOTKEY TRIGGERED: {} <<<",
                                config.display_string()
                            );
                            // Clear modifiers before callback to prevent stale state
                            drop(pressed);
                            pressed_modifiers_clone.lock().unwrap().clear();
                            (callback)();
                            // Don't consume - let Space key through so it can be released properly
                            return Some(event);
                        }
                    }
                }
            }
            EventType::KeyRelease(key) => {
                // Track modifier key releases
                if let Some(modifier) = key_to_modifier(&key) {
                    pressed_modifiers_clone.lock().unwrap().remove(&modifier);
                }

                // Reset main key pressed state and ALWAYS clear all modifiers when main key is released
                // This ensures we don't get stuck with stale modifier state after palette closes
                // The palette window may intercept KeyRelease events for other modifiers (like Ctrl)
                // so we can't rely on tracking them individually
                if let Some(target_key) = parse_key_string(&config.key) {
                    if key == target_key {
                        // Always clear all modifiers when Space is released
                        // This handles the case where Ctrl release wasn't detected while palette was open
                        pressed_modifiers_clone.lock().unwrap().clear();
                        log::debug!("[WM Hotkey] Main key released, clearing ALL modifier state");
                        main_key_pressed_clone.store(false, Ordering::Relaxed);
                    }
                }
            }
            _ => {}
        }

        // Pass through all other events
        Some(event)
    };

    log::info!("[WM Hotkey] Calling eventhooks::grab()...");

    // This will block until an error occurs or the process exits
    if let Err(error) = grab(grab_callback) {
        log::error!("[WM Hotkey] grab() error: {:?}", error);
    }

    log::info!("[WM Hotkey] grab() returned, loop ending");
}

/// Convert eventhooks Key to our Modifier enum
fn key_to_modifier(key: &Key) -> Option<Modifier> {
    match key {
        Key::ControlLeft | Key::ControlRight => Some(Modifier::Ctrl),
        Key::Alt | Key::AltGr => Some(Modifier::Alt),
        Key::ShiftLeft | Key::ShiftRight => Some(Modifier::Shift),
        Key::MetaLeft | Key::MetaRight => Some(Modifier::Meta),
        _ => None,
    }
}

/// Parse a key string to eventhooks::Key
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
        let manager = WindowManagerHotkeyManager::new();
        assert!(!manager.is_running());
    }

    #[test]
    fn test_parse_key_string() {
        assert_eq!(parse_key_string("Space"), Some(Key::Space));
        assert_eq!(parse_key_string("k"), Some(Key::KeyK));
        assert_eq!(parse_key_string("F1"), Some(Key::F1));
        assert_eq!(parse_key_string("unknown"), None);
    }

    #[test]
    fn test_key_to_modifier() {
        assert_eq!(key_to_modifier(&Key::ControlLeft), Some(Modifier::Ctrl));
        assert_eq!(key_to_modifier(&Key::Alt), Some(Modifier::Alt));
        assert_eq!(key_to_modifier(&Key::ShiftLeft), Some(Modifier::Shift));
        assert_eq!(key_to_modifier(&Key::MetaLeft), Some(Modifier::Meta));
        assert_eq!(key_to_modifier(&Key::KeyA), None);
    }
}
