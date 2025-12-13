use crate::image::Image;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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
    Region { x: i32, y: i32, width: u32, height: u32 },
    /// 捕获指定监视器
    Monitor { id: String },
}

/// 捕获元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureMetadata {
    /// 捕获时间
    pub capture_time: SystemTime,
    /// 捕获目标
    pub capture_target: CaptureTarget,
    /// 监视器 ID
    pub monitor_id: Option<String>,
    /// 捕获区域
    pub region: Option<CaptureRegion>,
    /// 光标是否可见
    pub cursor_visible: bool,
    /// 是否进行了 HDR 校正
    pub hdr_corrected: bool,
}

/// 捕获区域
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for CaptureMetadata {
    fn default() -> Self {
        Self {
            capture_time: SystemTime::now(),
            capture_target: CaptureTarget::CurrentMonitor,
            monitor_id: None,
            region: None,
            cursor_visible: true,
            hdr_corrected: false,
        }
    }
}

impl CaptureMetadata {
    pub fn new(capture_target: CaptureTarget) -> Self {
        Self {
            capture_time: SystemTime::now(),
            capture_target,
            monitor_id: None,
            region: None,
            cursor_visible: true,
            hdr_corrected: false,
        }
    }

    pub fn with_monitor_id(mut self, monitor_id: String) -> Self {
        self.monitor_id = Some(monitor_id);
        self
    }

    pub fn with_region(mut self, x: i32, y: i32, width: u32, height: u32) -> Self {
        self.region = Some(CaptureRegion { x, y, width, height });
        self
    }

    pub fn with_cursor_visible(mut self, visible: bool) -> Self {
        self.cursor_visible = visible;
        self
    }

    pub fn with_hdr_corrected(mut self, corrected: bool) -> Self {
        self.hdr_corrected = corrected;
        self
    }
}

/// 截图 (包装 Image + 捕获元数据)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screenshot {
    /// 图像数据
    pub image: Image,
    /// 捕获元数据
    pub capture_metadata: CaptureMetadata,
}

impl Screenshot {
    /// 创建新截图
    pub fn new(image: Image, capture_metadata: CaptureMetadata) -> Self {
        Self { image, capture_metadata }
    }

    /// 创建简单截图 (默认元数据)
    pub fn from_image(image: Image) -> Self {
        Self { image, capture_metadata: CaptureMetadata::default() }
    }

    /// 获取图像宽度
    pub fn width(&self) -> u32 {
        self.image.width
    }

    /// 获取图像高度
    pub fn height(&self) -> u32 {
        self.image.height
    }

    /// 获取捕获时间
    pub fn capture_time(&self) -> SystemTime {
        self.capture_metadata.capture_time
    }

    /// 是否已进行 HDR 校正
    pub fn is_hdr_corrected(&self) -> bool {
        self.capture_metadata.hdr_corrected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::{ColorFormat, ImageMetadata, ImageSource};

    #[test]
    fn test_screenshot_creation() {
        let data = vec![0u8; 100 * 100 * 4];
        let image = Image::new(data, 100, 100, ColorFormat::RGBA).unwrap();

        let metadata = CaptureMetadata::new(CaptureTarget::CurrentMonitor)
            .with_monitor_id("0".to_string())
            .with_cursor_visible(false);

        let screenshot = Screenshot::new(image, metadata);

        assert_eq!(screenshot.width(), 100);
        assert_eq!(screenshot.height(), 100);
        assert_eq!(screenshot.capture_metadata.monitor_id, Some("0".to_string()));
        assert!(!screenshot.capture_metadata.cursor_visible);
    }

    #[test]
    fn test_screenshot_from_image() {
        let data = vec![0u8; 50 * 50 * 4];
        let image = Image::new(data, 50, 50, ColorFormat::RGBA).unwrap();

        let screenshot = Screenshot::from_image(image);

        assert_eq!(screenshot.width(), 50);
        assert_eq!(screenshot.height(), 50);
        assert!(!screenshot.is_hdr_corrected());
    }
}
