// 基础设施层 - 适配器实现

pub mod adapters;
pub mod platform;
pub mod services;
pub mod utils;

// Re-export for convenience
pub use adapters::*;
pub use services::*;
