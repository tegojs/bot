use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, Point, Rectangle};

/// 滚动方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
}

/// 滚动捕获请求
#[derive(Debug, Clone)]
pub struct ScrollCaptureRequest {
    pub direction: ScrollDirection,
    pub region: Rectangle,
    pub options: ScrollCaptureOptions,
}

/// 滚动捕获选项
#[derive(Debug, Clone)]
pub struct ScrollCaptureOptions {
    pub sample_rate: f32,
    pub match_threshold: u8,
    pub max_scroll_attempts: u32,
}

impl Default for ScrollCaptureOptions {
    fn default() -> Self {
        Self { sample_rate: 1.0, match_threshold: 10, max_scroll_attempts: 100 }
    }
}

/// 捕获帧
#[derive(Debug)]
pub struct CaptureFrame {
    pub image: Vec<u8>,
    pub position: Point,
}

/// 处理结果
#[derive(Debug)]
pub struct ProcessResult {
    pub matched: bool,
    pub offset: Option<Point>,
}

/// 滚动截图
#[derive(Debug)]
pub struct ScrollScreenshot {
    pub composite_image: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// 滚动捕获 Port
///
/// 负责滚动截图操作
///
/// **实现者**:
/// - `ScrollCaptureAdapter` (封装 `app-scroll-screenshot-service`)
#[async_trait]
pub trait ScrollCapturePort: Send + Sync {
    /// 初始化滚动截图
    async fn init(&mut self, request: ScrollCaptureRequest) -> Result<(), InfrastructureError>;

    /// 捕获一帧
    async fn capture_frame(&mut self) -> Result<CaptureFrame, InfrastructureError>;

    /// 处理捕获的图像
    async fn handle_image(&mut self, image: Vec<u8>) -> Result<ProcessResult, InfrastructureError>;

    /// 获取当前拼接后的图像大小
    async fn get_size(&self) -> Result<(u32, u32), InfrastructureError>;

    /// 完成并返回最终图像
    async fn finalize(&mut self) -> Result<ScrollScreenshot, InfrastructureError>;

    /// 清理资源
    async fn clear(&mut self) -> Result<(), InfrastructureError>;
}
