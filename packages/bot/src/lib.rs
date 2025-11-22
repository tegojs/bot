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

/// Global API module
///
/// Provides global functions matching robotjs API
pub mod api;
