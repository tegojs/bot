// Tauri Commands
//
// 每个模块对应一个功能域

pub mod clipboard;
pub mod hotkey;
pub mod monitor;
pub mod page;
pub mod screenshot;
pub mod scroll;
pub mod ui;
pub mod window;

// Re-export all commands
pub use clipboard::*;
pub use hotkey::*;
pub use monitor::*;
pub use page::*;
pub use screenshot::*;
pub use scroll::*;
pub use ui::*;
pub use window::*;
