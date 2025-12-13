// 截图相关 DTOs
use aumate_core_shared::{MonitorId, Point, Rectangle, ScreenshotId};
use serde::{Deserialize, Serialize};

/// 捕获屏幕请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureScreenRequest {
    /// 监视器 ID (None 表示当前监视器)
    pub monitor_id: Option<MonitorId>,
    /// 图像格式 ("png", "jpeg", "webp")
    pub format: String,
    /// JPEG 质量 (1-100)
    pub quality: Option<u8>,
    /// 是否启用 HDR 校正
    pub hdr_correction: bool,
}

/// 捕获区域请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRegionRequest {
    /// 捕获区域
    pub region: Rectangle,
    /// 监视器 ID
    pub monitor_id: Option<MonitorId>,
    /// 图像格式
    pub format: String,
    /// JPEG 质量
    pub quality: Option<u8>,
}

/// 截图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureResponse {
    /// 截图 ID
    pub id: ScreenshotId,
    /// 图像数据 (编码后的字节)
    pub data: Vec<u8>,
    /// 图像宽度
    pub width: u32,
    /// 图像高度
    pub height: u32,
    /// 图像格式
    pub format: String,
}

/// 保存截图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveScreenshotRequest {
    /// 截图 ID
    pub id: ScreenshotId,
    /// 保存路径
    pub path: String,
}

/// 保存截图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveScreenshotResponse {
    /// 保存的文件路径
    pub path: String,
    /// 文件大小（字节）
    pub size: u64,
}
