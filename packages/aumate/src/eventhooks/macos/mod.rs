//! macOS event grab implementation using CGEventTap

mod common;
mod grab;

pub use grab::{exit_grab, grab, is_grabbed};
