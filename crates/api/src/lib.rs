// API Layer - Tauri Commands
//
// 此层负责暴露 Tauri Commands 给前端，实现：
// 1. 命令定义和路由
// 2. 参数验证和转换
// 3. 错误处理和转换
// 4. 依赖注入

pub mod commands;
pub mod setup;
pub mod state;

// Re-export
pub use aumate_core_shared::ApiError;
pub use setup::setup_application;
pub use state::AppState;
