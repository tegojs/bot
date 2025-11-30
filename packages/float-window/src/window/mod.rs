//! Window module

mod builder;
pub mod commands;
pub mod config;
pub mod controller;
mod floating;
mod manager;

pub use builder::FloatingWindowBuilder;
pub use commands::{create_command_channel, CommandReceiver, CommandSender, WindowCommand, WindowRegistry};
pub use config::{Position, Size, WindowConfig, WindowLevel};
pub use controller::ControllerState;
pub use floating::FloatingWindow;
pub use manager::FloatingWindowManager;
