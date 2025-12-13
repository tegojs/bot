use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, MonitorId, Point, Rectangle, WindowId};

// 从 domain 导入核心类型
pub use aumate_core_domain::image::Image;
pub use aumate_core_domain::screenshot::{CaptureMetadata, CaptureRegion, Screenshot};

/// 捕获目标
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureTarget {
    /// 捕获鼠标所在的监视器
    CurrentMonitor,
    /// 捕获所有监视器
    AllMonitors,
    /// 捕获当前焦点窗口
    FocusedWindow,
    /// 捕获指定区域
    Region(Rectangle),
    /// 捕获指定监视器
    Monitor(MonitorId),
}

/// 捕获选项
#[derive(Debug, Clone)]
pub struct CaptureOptions {
    pub exclude_windows: Vec<WindowId>,
    pub cursor_visible: bool,
    pub hdr_correction: Option<HdrCorrectionAlgorithm>,
}

impl Default for CaptureOptions {
    fn default() -> Self {
        Self { exclude_windows: Vec::new(), cursor_visible: true, hdr_correction: None }
    }
}

/// HDR 校正算法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdrCorrectionAlgorithm {
    None,
    Gamma,
    Reinhard,
    Auto,
}

/// 图像格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
    Bmp,
}

impl ImageFormat {
    pub fn from_str(s: &str) -> Result<Self, InfrastructureError> {
        match s.to_lowercase().as_str() {
            "png" => Ok(Self::Png),
            "jpeg" | "jpg" => Ok(Self::Jpeg),
            "webp" => Ok(Self::WebP),
            "bmp" => Ok(Self::Bmp),
            _ => {
                Err(InfrastructureError::ExternalError(format!("Unsupported image format: {}", s)))
            }
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::WebP => "webp",
            Self::Bmp => "bmp",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::WebP => "image/webp",
            Self::Bmp => "image/bmp",
        }
    }
}

/// 颜色格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    Rgb8,
    Rgba8,
    Bgra8,
    Rgba16F,
}

impl ColorFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            Self::Rgb8 => 3,
            Self::Rgba8 | Self::Bgra8 => 4,
            Self::Rgba16F => 8,
        }
    }
}

// Image, Screenshot, CaptureMetadata 现在从 domain 导入

/// 监视器信息
#[derive(Debug, Clone)]
pub struct Monitor {
    pub id: MonitorId,
    pub name: String,
    pub rect: Rectangle,
    pub scale_factor: f64,
    pub is_primary: bool,
}

/// 屏幕捕获 Port
///
/// 负责屏幕捕获操作
///
/// **实现者**:
/// - `WindowsScreenCaptureAdapter`
/// - `MacOSScreenCaptureAdapter`
/// - `LinuxScreenCaptureAdapter`
#[async_trait]
pub trait ScreenCapturePort: Send + Sync {
    /// 捕获指定目标
    async fn capture(
        &self,
        target: CaptureTarget,
        options: CaptureOptions,
    ) -> Result<Screenshot, InfrastructureError>;

    /// 获取所有可用监视器
    async fn get_monitors(&self) -> Result<Vec<Monitor>, InfrastructureError>;

    /// 获取当前鼠标所在的监视器
    async fn get_current_monitor(&self) -> Result<Monitor, InfrastructureError>;

    /// 获取焦点窗口
    async fn get_focused_window(&self) -> Result<WindowId, InfrastructureError>;
}

/// 图像处理 Port
///
/// 负责图像处理操作
///
/// **实现者**:
/// - `ImageProcessingAdapter`
pub trait ImageProcessingPort: Send + Sync {
    /// 编码图像为指定格式
    fn encode(&self, image: &Image, format: ImageFormat) -> Result<Vec<u8>, InfrastructureError>;

    /// 解码图像数据
    fn decode(&self, data: &[u8]) -> Result<Image, InfrastructureError>;

    /// 叠加图像
    fn overlay(
        &self,
        base: &Image,
        overlay: &Image,
        position: Point,
    ) -> Result<Image, InfrastructureError>;

    /// HDR 颜色校正
    fn correct_hdr(
        &self,
        image: &Image,
        algorithm: HdrCorrectionAlgorithm,
    ) -> Result<Image, InfrastructureError>;

    /// 调整图像大小
    fn resize(&self, image: &Image, width: u32, height: u32) -> Result<Image, InfrastructureError>;
}
