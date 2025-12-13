// 领域模型定义

pub mod clipboard;
pub mod hotkey;
pub mod image;
pub mod page;
pub mod screenshot;
pub mod settings;
pub mod storage;
pub mod window;

// Re-export for convenience
pub use clipboard::*;
pub use hotkey::*;
pub use image::*;
pub use page::*;
pub use screenshot::*;
pub use settings::*;
pub use storage::*;
pub use window::*;
