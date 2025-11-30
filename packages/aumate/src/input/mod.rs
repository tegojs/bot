//! Input control module
//!
//! Provides mouse and keyboard input simulation for desktop automation.

mod keyboard;
mod mouse;

pub use keyboard::Keyboard;
pub use mouse::{Mouse, MouseButton, MousePosition};
