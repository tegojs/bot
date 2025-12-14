// Tauri Commands
//
// 每个模块对应一个功能域

pub mod clipboard;
pub mod draw;
pub mod frontend_log;
pub mod hotkey;
pub mod monitor;
pub mod page;
pub mod permissions;
pub mod screenshot;
pub mod scroll;
pub mod settings;
pub mod ui;
pub mod ui_control;
pub mod window;

// Re-export all commands
pub use clipboard::*;
pub use draw::*;
pub use frontend_log::*;
pub use hotkey::*;
pub use monitor::*;
pub use page::*;
pub use screenshot::*;
pub use scroll::*;
pub use settings::*;
pub use ui::*;
pub use ui_control::*;
pub use window::*;
