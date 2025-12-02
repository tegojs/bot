//! Aumate - Cross-platform desktop automation library
//!
//! This library provides core functionality for desktop automation:
//! - Mouse and keyboard input control
//! - Screen capture and pixel operations
//! - Clipboard text and image operations
//! - Window management
//! - Screenshot with interactive UI
//! - Floating window system with particle effects
//!
//! # Features
//!
//! - `input` - Mouse and keyboard control (enabled by default)
//! - `screen` - Screen capture operations (enabled by default)
//! - `clipboard` - Clipboard operations (enabled by default)
//! - `window` - Window management (enabled by default)
//! - `gui` - Full GUI support including screenshot UI and floating windows (enabled by default)
//!
//! # Example
//!
//! ```no_run
//! use aumate::prelude::*;
//!
//! // Mouse operations
//! let mouse = Mouse::new().unwrap();
//! mouse.move_mouse(100, 100).unwrap();
//!
//! // Keyboard operations
//! let keyboard = Keyboard::new().unwrap();
//! keyboard.type_string("Hello, World!").unwrap();
//! ```

pub mod error;

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "screen")]
pub mod screen;

#[cfg(feature = "clipboard")]
pub mod clipboard;

#[cfg(feature = "window")]
pub mod window;

#[cfg(feature = "gui")]
pub mod screenshot;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "gui")]
pub mod clipboard_manager;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::error::{AumateError, Result};

    #[cfg(feature = "input")]
    pub use crate::input::{Keyboard, Mouse, MouseButton, MousePosition};

    #[cfg(feature = "screen")]
    pub use crate::screen::{
        MonitorInfo, PixelColor, ScreenCapture, ScreenSize, capture_screen, capture_screen_region,
        get_monitors, get_pixel_color, get_screen_size,
    };

    #[cfg(feature = "clipboard")]
    pub use crate::clipboard;

    #[cfg(feature = "window")]
    pub use crate::window::{
        WindowInfo, find_windows_by_process, find_windows_by_title, get_active_window_info,
        get_all_windows,
    };

    #[cfg(feature = "gui")]
    pub use crate::gui::prelude::*;

    #[cfg(feature = "gui")]
    pub use crate::screenshot::{ScreenRegion, ScreenshotResult};
}
