//! Window module

mod builder;
pub mod config;
mod floating;
mod manager;

pub use builder::FloatingWindowBuilder;
pub use config::{Position, Size, WindowConfig, WindowLevel};
pub use floating::FloatingWindow;
pub use manager::FloatingWindowManager;
