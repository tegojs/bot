//! Linux event grab implementation using X11

mod common;
mod grab;

pub use grab::{disable_grab, enable_grab, exit_grab_listen, is_grabbed, start_grab_listen};
