// Data Transfer Objects
//
// 定义用例的请求和响应数据结构

pub mod clipboard;
pub mod element_scanner;
pub mod monitor;
pub mod screenshot;
pub mod scroll;
pub mod storage;
pub mod window;
pub mod window_list;

pub use clipboard::*;
pub use element_scanner::*;
pub use monitor::*;
pub use screenshot::*;
pub use scroll::*;
pub use storage::*;
pub use window::*;
pub use window_list::*;
