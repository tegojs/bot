// 页面管理服务
//
// **完整迁移**: 从 app-services/src/hot_load_page_service.rs (~188 行)
//
// 提供热加载页面池管理功能，用于快速创建和重用窗口

pub mod hot_load_page_service;

pub use hot_load_page_service::{HotLoadPage, HotLoadPageRoutePushEvent, HotLoadPageService};
