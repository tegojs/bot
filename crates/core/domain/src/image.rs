use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// 颜色格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorFormat {
    /// RGBA (32-bit, 8-bit per channel)
    RGBA,
    /// RGB (24-bit, 8-bit per channel)
    RGB,
    /// Grayscale (8-bit)
    Grayscale,
}

impl ColorFormat {
    /// 获取每像素字节数
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            ColorFormat::RGBA => 4,
            ColorFormat::RGB => 3,
            ColorFormat::Grayscale => 1,
        }
    }
}

/// 图像来源
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageSource {
    /// 截图
    Screenshot,
    /// 剪贴板
    Clipboard,
    /// 文件
    File(PathBuf),
    /// 其他来源
    Other,
}

/// 图像元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    /// DPI (横向, 纵向)
    pub dpi: Option<(u32, u32)>,
    /// 创建时间
    pub creation_time: Option<SystemTime>,
    /// 图像来源
    pub source: ImageSource,
}

impl Default for ImageMetadata {
    fn default() -> Self {
        Self { dpi: None, creation_time: Some(SystemTime::now()), source: ImageSource::Other }
    }
}

impl ImageMetadata {
    pub fn new(source: ImageSource) -> Self {
        Self { dpi: None, creation_time: Some(SystemTime::now()), source }
    }

    pub fn with_dpi(mut self, dpi: (u32, u32)) -> Self {
        self.dpi = Some(dpi);
        self
    }

    pub fn with_creation_time(mut self, time: SystemTime) -> Self {
        self.creation_time = Some(time);
        self
    }
}

/// 图像数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// 原始像素数据
    pub data: Vec<u8>,
    /// 宽度 (像素)
    pub width: u32,
    /// 高度 (像素)
    pub height: u32,
    /// 颜色格式
    pub color_format: ColorFormat,
    /// 元数据
    pub metadata: ImageMetadata,
}

impl Image {
    /// 创建新图像
    pub fn new(
        data: Vec<u8>,
        width: u32,
        height: u32,
        color_format: ColorFormat,
    ) -> Result<Self, String> {
        let expected_size = (width as usize) * (height as usize) * color_format.bytes_per_pixel();
        if data.len() != expected_size {
            return Err(format!(
                "Invalid image data size: expected {}, got {}",
                expected_size,
                data.len()
            ));
        }

        Ok(Self { data, width, height, color_format, metadata: ImageMetadata::default() })
    }

    /// 创建带元数据的图像
    pub fn with_metadata(
        data: Vec<u8>,
        width: u32,
        height: u32,
        color_format: ColorFormat,
        metadata: ImageMetadata,
    ) -> Result<Self, String> {
        let expected_size = (width as usize) * (height as usize) * color_format.bytes_per_pixel();
        if data.len() != expected_size {
            return Err(format!(
                "Invalid image data size: expected {}, got {}",
                expected_size,
                data.len()
            ));
        }

        Ok(Self { data, width, height, color_format, metadata })
    }

    /// 获取图像大小 (字节)
    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }

    /// 获取像素总数
    pub fn pixel_count(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// 转换为 RGBA 格式
    pub fn to_rgba(&self) -> Result<Image, String> {
        if self.color_format == ColorFormat::RGBA {
            return Ok(self.clone());
        }

        let pixel_count = self.pixel_count();
        let mut rgba_data = Vec::with_capacity(pixel_count * 4);

        match self.color_format {
            ColorFormat::RGB => {
                for chunk in self.data.chunks_exact(3) {
                    rgba_data.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
                }
            }
            ColorFormat::Grayscale => {
                for &gray in &self.data {
                    rgba_data.extend_from_slice(&[gray, gray, gray, 255]);
                }
            }
            ColorFormat::RGBA => unreachable!(),
        }

        Ok(Image {
            data: rgba_data,
            width: self.width,
            height: self.height,
            color_format: ColorFormat::RGBA,
            metadata: self.metadata.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let data = vec![0u8; 100 * 100 * 4]; // 100x100 RGBA
        let image = Image::new(data, 100, 100, ColorFormat::RGBA);
        assert!(image.is_ok());

        let image = image.unwrap();
        assert_eq!(image.width, 100);
        assert_eq!(image.height, 100);
        assert_eq!(image.pixel_count(), 10000);
    }

    #[test]
    fn test_invalid_image_size() {
        let data = vec![0u8; 100]; // Too small for 100x100 RGBA
        let image = Image::new(data, 100, 100, ColorFormat::RGBA);
        assert!(image.is_err());
    }

    #[test]
    fn test_rgb_to_rgba_conversion() {
        let data = vec![255, 0, 0, 0, 255, 0, 0, 0, 255]; // 3 RGB pixels
        let image = Image::new(data, 3, 1, ColorFormat::RGB).unwrap();

        let rgba_image = image.to_rgba().unwrap();
        assert_eq!(rgba_image.color_format, ColorFormat::RGBA);
        assert_eq!(rgba_image.data.len(), 12); // 3 pixels * 4 bytes
        assert_eq!(&rgba_image.data[0..4], &[255, 0, 0, 255]); // Red
        assert_eq!(&rgba_image.data[4..8], &[0, 255, 0, 255]); // Green
        assert_eq!(&rgba_image.data[8..12], &[0, 0, 255, 255]); // Blue
    }
}
