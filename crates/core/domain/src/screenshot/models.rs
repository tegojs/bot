use serde::{Deserialize, Serialize};
use aumate_core_shared::{DomainError, MonitorId, Point, Rectangle, ScreenshotId, Timestamp, WindowId};
use std::sync::Arc;

use super::value_objects::{ColorFormat, HdrInfo};

/// 图像实体
#[derive(Debug, Clone)]
pub struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
    format: ColorFormat,
}

impl Image {
    /// 创建新图像
    pub fn new(width: u32, height: u32, data: Vec<u8>, format: ColorFormat) -> Result<Self, DomainError> {
        // 验证数据大小
        let expected_size = (width * height * format.bytes_per_pixel()) as usize;
        if data.len() != expected_size {
            return Err(DomainError::InvalidImageData);
        }
        
        // 验证尺寸限制 (最大 16384x16384)
        if width > 16384 || height > 16384 {
            return Err(DomainError::InvalidDimensions(width, height));
        }
        
        Ok(Self {
            width,
            height,
            data,
            format,
        })
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    
    pub fn format(&self) -> ColorFormat {
        self.format
    }
    
    pub fn into_data(self) -> Vec<u8> {
        self.data
    }
}

/// 监视器实体
#[derive(Debug, Clone, PartialEq)]
pub struct Monitor {
    id: MonitorId,
    name: String,
    rect: Rectangle,
    scale_factor: f64,
    is_primary: bool,
    hdr_info: Option<HdrInfo>,
}

impl Monitor {
    pub fn new(
        id: MonitorId,
        name: String,
        rect: Rectangle,
        scale_factor: f64,
        is_primary: bool,
    ) -> Self {
        Self {
            id,
            name,
            rect,
            scale_factor,
            is_primary,
            hdr_info: None,
        }
    }
    
    pub fn with_hdr_info(mut self, hdr_info: HdrInfo) -> Self {
        self.hdr_info = Some(hdr_info);
        self
    }
    
    pub fn id(&self) -> &MonitorId {
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn rect(&self) -> &Rectangle {
        &self.rect
    }
    
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
    
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
    
    pub fn hdr_info(&self) -> Option<&HdrInfo> {
        self.hdr_info.as_ref()
    }
    
    /// 获取物理尺寸 (考虑缩放因子)
    pub fn physical_size(&self) -> (u32, u32) {
        (
            (self.rect.width() as f64 * self.scale_factor) as u32,
            (self.rect.height() as f64 * self.scale_factor) as u32,
        )
    }
}

/// 捕获元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureMetadata {
    pub monitor_id: Option<MonitorId>,
    pub window_id: Option<WindowId>,
    pub capture_region: Option<Rectangle>,
    pub cursor_position: Option<Point>,
    pub captured_at: Timestamp,
}

impl CaptureMetadata {
    pub fn new() -> Self {
        Self {
            monitor_id: None,
            window_id: None,
            capture_region: None,
            cursor_position: None,
            captured_at: Timestamp::now(),
        }
    }
    
    pub fn with_monitor(mut self, monitor_id: MonitorId) -> Self {
        self.monitor_id = Some(monitor_id);
        self
    }
    
    pub fn with_window(mut self, window_id: WindowId) -> Self {
        self.window_id = Some(window_id);
        self
    }
    
    pub fn with_region(mut self, region: Rectangle) -> Self {
        self.capture_region = Some(region);
        self
    }
    
    pub fn with_cursor_position(mut self, position: Point) -> Self {
        self.cursor_position = Some(position);
        self
    }
}

impl Default for CaptureMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// 截图聚合根
#[derive(Debug)]
pub struct Screenshot {
    id: ScreenshotId,
    image: Arc<Image>,
    metadata: CaptureMetadata,
    created_at: Timestamp,
}

impl Screenshot {
    /// 创建新截图
    pub fn new(image: Image, metadata: CaptureMetadata) -> Self {
        Self {
            id: ScreenshotId::generate(),
            image: Arc::new(image),
            metadata,
            created_at: Timestamp::now(),
        }
    }
    
    pub fn id(&self) -> &ScreenshotId {
        &self.id
    }
    
    pub fn image(&self) -> &Arc<Image> {
        &self.image
    }
    
    pub fn metadata(&self) -> &CaptureMetadata {
        &self.metadata
    }
    
    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }
    
    pub fn width(&self) -> u32 {
        self.image.width()
    }
    
    pub fn height(&self) -> u32 {
        self.image.height()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_image_creation_valid() {
        let data = vec![0u8; 100 * 100 * 4]; // RGBA8
        let image = Image::new(100, 100, data, ColorFormat::Rgba8);
        assert!(image.is_ok());
        
        let image = image.unwrap();
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);
    }
    
    #[test]
    fn test_image_creation_invalid_data_size() {
        let data = vec![0u8; 100]; // Too small
        let image = Image::new(100, 100, data, ColorFormat::Rgba8);
        assert!(image.is_err());
    }
    
    #[test]
    fn test_image_creation_invalid_dimensions() {
        let data = vec![0u8; 20000 * 20000 * 4]; // Too large
        let image = Image::new(20000, 20000, data, ColorFormat::Rgba8);
        assert!(image.is_err());
    }
    
    #[test]
    fn test_monitor_physical_size() {
        let rect = Rectangle::from_xywh(0, 0, 1920, 1080).unwrap();
        let monitor = Monitor::new(
            MonitorId::new(0),
            "Monitor 1".to_string(),
            rect,
            2.0,
            true,
        );
        
        let (width, height) = monitor.physical_size();
        assert_eq!(width, 3840);
        assert_eq!(height, 2160);
    }
}



