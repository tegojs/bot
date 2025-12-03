//! Windows event grab implementation using low-level hooks

mod common;
mod grab;

pub use grab::{exit_grab, grab, is_grabbed};
