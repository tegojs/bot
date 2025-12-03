//! Global event hooks module
//!
//! This module provides cross-platform global event interception (grab) functionality.
//! It allows intercepting keyboard and mouse events before they reach other applications.
//!
//! # Usage
//!
//! ```no_run
//! use aumate::eventhooks::{grab, Event, EventType, Key};
//!
//! // Callback receives events and can block them by returning None
//! let callback = |event: Event| -> Option<Event> {
//!     if let EventType::KeyPress(Key::Escape) = event.event_type {
//!         println!("Escape pressed - blocking it");
//!         return None; // Block the event
//!     }
//!     Some(event) // Let other events pass through
//! };
//!
//! // This blocks until exit_grab() is called
//! if let Err(e) = grab(callback) {
//!     eprintln!("Failed to grab: {:?}", e);
//! }
//! ```
//!
//! # Platform Notes
//!
//! ## macOS
//! - Requires Accessibility permissions (System Preferences > Security & Privacy > Privacy > Accessibility)
//! - Uses CGEventTap for event interception
//!
//! ## Linux
//! - Uses X11 XGrabKeyboard (keyboard only)
//! - Requires user to be in `input` group for full functionality
//!
//! ## Windows
//! - Uses low-level hooks (SetWindowsHookEx with WH_KEYBOARD_LL and WH_MOUSE_LL)
//! - Works without special permissions

mod keycodes;
mod types;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

// Re-export types
pub use types::{Button, Event, EventType, GrabCallback, GrabError, Key};

// Platform-specific re-exports

#[cfg(target_os = "macos")]
pub use macos::{exit_grab, grab, is_grabbed};

#[cfg(target_os = "linux")]
pub use linux::{disable_grab, enable_grab, exit_grab_listen, is_grabbed, start_grab_listen};

// Convenience aliases for Linux to match other platforms
#[cfg(target_os = "linux")]
pub use linux::exit_grab_listen as exit_grab;
#[cfg(target_os = "linux")]
pub use linux::start_grab_listen as grab;

#[cfg(target_os = "windows")]
pub use windows::{exit_grab, grab, is_grabbed};
