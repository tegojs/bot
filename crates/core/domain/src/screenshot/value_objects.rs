use serde::{Deserialize, Serialize};
use aumate_core_shared::{MonitorId, Rectangle, WindowId};

/// 捕获目标
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureOptions {
    pub exclude_windows: Vec<WindowId>,
    pub cursor_visible: bool,
    pub hdr_correction: Option<HdrCorrectionAlgorithm>,
}

impl CaptureOptions {
    pub fn new() -> Self {
        Self {
            exclude_windows: Vec::new(),
            cursor_visible: true,
            hdr_correction: None,
        }
    }
    
    pub fn exclude_window(mut self, window_id: WindowId) -> Self {
        self.exclude_windows.push(window_id);
        self
    }
    
    pub fn hide_cursor(mut self) -> Self {
        self.cursor_visible = false;
        self
    }
    
    pub fn with_hdr_correction(mut self, algorithm: HdrCorrectionAlgorithm) -> Self {
        self.hdr_correction = Some(algorithm);
        self
    }
}

impl Default for CaptureOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// HDR 校正算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HdrCorrectionAlgorithm {
    None,
    Gamma,
    Reinhard,
    Auto,
}

/// 图像格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
    Bmp,
}

impl ImageFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpeg" | "jpg" => Some(Self::Jpeg),
            "webp" => Some(Self::WebP),
            "bmp" => Some(Self::Bmp),
            _ => None,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// HDR 信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HdrInfo {
    pub is_hdr_enabled: bool,
    pub max_luminance: f32,
    pub min_luminance: f32,
    pub max_full_frame_luminance: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_image_format_from_str() {
        assert_eq!(ImageFormat::from_str("png"), Some(ImageFormat::Png));
        assert_eq!(ImageFormat::from_str("PNG"), Some(ImageFormat::Png));
        assert_eq!(ImageFormat::from_str("jpeg"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_str("jpg"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_str("invalid"), None);
    }
    
    #[test]
    fn test_color_format_bytes_per_pixel() {
        assert_eq!(ColorFormat::Rgb8.bytes_per_pixel(), 3);
        assert_eq!(ColorFormat::Rgba8.bytes_per_pixel(), 4);
        assert_eq!(ColorFormat::Bgra8.bytes_per_pixel(), 4);
        assert_eq!(ColorFormat::Rgba16F.bytes_per_pixel(), 8);
    }
    
    #[test]
    fn test_capture_options_builder() {
        let options = CaptureOptions::new()
            .exclude_window(WindowId::new("window1".to_string()))
            .hide_cursor()
            .with_hdr_correction(HdrCorrectionAlgorithm::Auto);
        
        assert_eq!(options.exclude_windows.len(), 1);
        assert_eq!(options.cursor_visible, false);
        assert_eq!(options.hdr_correction, Some(HdrCorrectionAlgorithm::Auto));
    }
}

