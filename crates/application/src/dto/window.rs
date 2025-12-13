// 窗口管理相关 DTOs
use aumate_core_shared::{Point, WindowId};
use serde::{Deserialize, Serialize};

/// 创建窗口请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWindowRequest {
    /// 窗口标签
    pub label: String,
    /// 窗口标题
    pub title: String,
    /// URL
    pub url: String,
    /// 宽度
    pub width: Option<f64>,
    /// 高度
    pub height: Option<f64>,
}

/// 创建窗口响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWindowResponse {
    /// 窗口 ID
    pub window_id: WindowId,
    /// 窗口标签
    pub label: String,
}

/// 拖动窗口请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragWindowRequest {
    /// 窗口 ID
    pub window_id: WindowId,
    /// 拖动增量
    pub delta: Point,
}

/// 调整窗口大小请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeWindowRequest {
    /// 窗口 ID
    pub window_id: WindowId,
    /// 调整边
    pub side: String, // "top", "bottom", "left", "right"
    /// 增量
    pub delta: i32,
}
