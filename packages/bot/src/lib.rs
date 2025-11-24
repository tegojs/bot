extern crate napi_derive;

/// Mouse automation module
///
/// Provides functionality to control mouse movements, clicks, and scrolling
pub mod mouse;

/// Keyboard automation module
///
/// Provides functionality to simulate keyboard input and key presses
pub mod keyboard;

/// Screen capture module
///
/// Provides functionality to capture screenshots and read pixel colors
pub mod screen;

/// Clipboard operations module
///
/// Provides functionality to read and write clipboard contents
pub mod clipboard;

/// Window management module
///
/// Provides functionality to get and manipulate windows
pub mod window;

/// Screenshot tool module
///
/// Provides advanced screenshot functionality with interactive selection,
/// annotations, and multi-format export
pub mod screenshot;

/// Global API module
///
/// Provides global functions matching robotjs API
pub mod api;
