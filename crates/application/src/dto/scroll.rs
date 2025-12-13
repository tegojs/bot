// 滚动截图相关 DTOs
use aumate_core_shared::{Rectangle, ScreenshotId};
use serde::{Deserialize, Serialize};

/// 开始滚动截图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartScrollCaptureRequest {
    /// 监视器 ID
    pub monitor_id: Option<String>,
    /// 捕获区域
    pub region: Rectangle,
    /// 滚动方向 ("vertical" 或 "horizontal")
    pub direction: String,
    /// 最大帧数
    pub max_frames: Option<usize>,
}

/// 滚动截图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollCaptureResponse {
    /// 截图 ID
    pub id: ScreenshotId,
    /// 拼接后的图像数据
    pub data: Vec<u8>,
    /// 图像宽度
    pub width: u32,
    /// 图像高度
    pub height: u32,
    /// 捕获的帧数
    pub frame_count: usize,
}
