//! Window module

mod builder;
pub mod commands;
pub mod config;
pub mod controller;
mod floating;
mod manager;

pub use builder::FloatingWindowBuilder;
pub use commands::{
    CommandReceiver, CommandSender, WindowCommand, WindowRegistry, create_command_channel,
};
pub use config::{Position, Size, WindowConfig, WindowLevel};
pub use controller::ControllerState;
pub use floating::FloatingWindow;
pub use manager::FloatingWindowManager;
