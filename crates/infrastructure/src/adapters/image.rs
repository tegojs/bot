use aumate_core_shared::InfrastructureError;
use aumate_core_shared::Point;
use aumate_core_traits::ImageProcessingPort;
use aumate_core_traits::screenshot::{ColorFormat, HdrCorrectionAlgorithm, Image, ImageFormat};

use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::codecs::webp::WebPEncoder;
use image::{DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;

/// 图像处理适配器
///
/// 实现 `ImageProcessingPort` trait，提供图像编码、解码、叠加等功能
///
/// **复用代码**:
/// - `app-utils::encode_image` - 图像编码逻辑
/// - `app-utils::overlay_image` - 图像叠加逻辑
pub struct ImageProcessingAdapter;

impl ImageProcessingAdapter {
    pub fn new() -> Self {
        Self
    }

    /// 将领域 Image 转换为 image crate 的 DynamicImage
    fn domain_to_dynamic(image: &Image) -> Result<DynamicImage, InfrastructureError> {
        let width = image.width;
        let height = image.height;
        let data = &image.data;

        use aumate_core_domain::image::ColorFormat as DomainColorFormat;

        match image.color_format {
            DomainColorFormat::RGBA => {
                let img_buffer =
                    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(width, height, data.clone())
                        .ok_or_else(|| {
                            InfrastructureError::ImageProcessingFailed(
                                "Failed to create image buffer".to_string(),
                            )
                        })?;

                Ok(DynamicImage::ImageRgba8(img_buffer))
            }
            DomainColorFormat::RGB => {
                let img_buffer =
                    ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_vec(width, height, data.clone())
                        .ok_or_else(|| {
                        InfrastructureError::ImageProcessingFailed(
                            "Failed to create image buffer".to_string(),
                        )
                    })?;

                Ok(DynamicImage::ImageRgb8(img_buffer))
            }
            DomainColorFormat::Grayscale => {
                // 灰度图转换为 RGBA
                let mut rgba_data = Vec::with_capacity(data.len() * 4);
                for &gray in data {
                    rgba_data.push(gray);
                    rgba_data.push(gray);
                    rgba_data.push(gray);
                    rgba_data.push(255);
                }

                let img_buffer =
                    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(width, height, rgba_data)
                        .ok_or_else(|| {
                            InfrastructureError::ImageProcessingFailed(
                                "Failed to create image buffer from Grayscale".to_string(),
                            )
                        })?;

                Ok(DynamicImage::ImageRgba8(img_buffer))
            }
        }
    }

    /// 将 DynamicImage 转换为领域 Image
    fn dynamic_to_domain(dynamic: &DynamicImage) -> Image {
        use aumate_core_domain::image::{
            ColorFormat as DomainColorFormat, ImageMetadata, ImageSource,
        };

        let width = dynamic.width();
        let height = dynamic.height();
        let data = dynamic.to_rgba8().into_raw();

        Image::with_metadata(
            data,
            width,
            height,
            DomainColorFormat::RGBA,
            ImageMetadata::new(ImageSource::Other),
        )
        .expect("Failed to create image")
    }
}

impl Default for ImageProcessingAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageProcessingPort for ImageProcessingAdapter {
    /// 编码图像为指定格式
    ///
    /// **复用**: `app-utils::encode_image`
    fn encode(&self, image: &Image, format: ImageFormat) -> Result<Vec<u8>, InfrastructureError> {
        let dynamic_image = Self::domain_to_dynamic(image)?;

        // 复用 app-utils 的编码逻辑
        let mut buf = Vec::with_capacity(image.data.len() / 8);

        match format {
            ImageFormat::Jpeg => {
                dynamic_image
                    .write_with_encoder(JpegEncoder::new_with_quality(&mut buf, 80))
                    .map_err(|e| {
                        InfrastructureError::ImageProcessingFailed(format!(
                            "JPEG encoding failed: {}",
                            e
                        ))
                    })?;
            }
            ImageFormat::WebP => {
                dynamic_image.write_with_encoder(WebPEncoder::new_lossless(&mut buf)).map_err(
                    |e| {
                        InfrastructureError::ImageProcessingFailed(format!(
                            "WebP encoding failed: {}",
                            e
                        ))
                    },
                )?;
            }
            ImageFormat::Png => {
                dynamic_image
                    .write_with_encoder(PngEncoder::new_with_quality(
                        &mut buf,
                        CompressionType::Fast,
                        FilterType::Paeth,
                    ))
                    .map_err(|e| {
                        InfrastructureError::ImageProcessingFailed(format!(
                            "PNG encoding failed: {}",
                            e
                        ))
                    })?;
            }
            ImageFormat::Bmp => {
                dynamic_image
                    .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Bmp)
                    .map_err(|e| {
                        InfrastructureError::ImageProcessingFailed(format!(
                            "BMP encoding failed: {}",
                            e
                        ))
                    })?;
            }
        }

        Ok(buf)
    }

    /// 解码图像数据
    fn decode(&self, data: &[u8]) -> Result<Image, InfrastructureError> {
        let dynamic_image = image::load_from_memory(data).map_err(|e| {
            InfrastructureError::ImageProcessingFailed(format!("Image decoding failed: {}", e))
        })?;

        Ok(Self::dynamic_to_domain(&dynamic_image))
    }

    /// 叠加图像
    ///
    /// **复用**: `app-utils::overlay_image`
    fn overlay(
        &self,
        base: &Image,
        overlay: &Image,
        position: Point,
    ) -> Result<Image, InfrastructureError> {
        // 转换为 DynamicImage
        let base_dynamic = Self::domain_to_dynamic(base)?;
        let overlay_dynamic = Self::domain_to_dynamic(overlay)?;

        // 创建新的 base 图像副本用于叠加
        let mut result_pixels = base_dynamic.to_rgba8().into_raw();
        let base_width = base.width as usize;
        let channel_count = 4; // RGBA

        // 复用 app-utils 的叠加逻辑（使用 rayon 并行）
        self.overlay_image_internal(
            &mut result_pixels,
            base_width,
            &overlay_dynamic,
            position.x.max(0) as usize,
            position.y.max(0) as usize,
            channel_count,
        );

        // 转换回 Image
        use aumate_core_domain::image::{ColorFormat as DomainColorFormat, ImageMetadata};

        Image::with_metadata(
            result_pixels,
            base.width,
            base.height,
            base.color_format,
            base.metadata.clone(),
        )
        .map_err(|e| InfrastructureError::ImageProcessingFailed(e))
    }

    /// HDR 颜色校正
    fn correct_hdr(
        &self,
        image: &Image,
        algorithm: HdrCorrectionAlgorithm,
    ) -> Result<Image, InfrastructureError> {
        log::info!("ImageProcessingAdapter: correct_hdr, algorithm={:?}", algorithm);

        match algorithm {
            HdrCorrectionAlgorithm::None => Ok(image.clone()),
            HdrCorrectionAlgorithm::Gamma => self.apply_gamma_correction(image),
            HdrCorrectionAlgorithm::Reinhard => self.apply_reinhard_tonemapping(image),
            HdrCorrectionAlgorithm::Auto => self.apply_auto_correction(image),
        }
    }

    fn resize(&self, image: &Image, width: u32, height: u32) -> Result<Image, InfrastructureError> {
        let dynamic_image = Self::domain_to_dynamic(image)?;

        let resized =
            dynamic_image.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

        Ok(Self::dynamic_to_domain(&resized))
    }
}

// HDR 校正内部方法
impl ImageProcessingAdapter {
    /// Gamma 校正 (gamma = 1.0 / 2.2)
    fn apply_gamma_correction(&self, image: &Image) -> Result<Image, InfrastructureError> {
        log::info!("ImageProcessingAdapter: applying gamma correction");

        let mut corrected = image.clone();
        let gamma = 1.0 / 2.2;

        // 并行处理像素
        use rayon::prelude::*;
        corrected.data.par_chunks_mut(4).for_each(|pixel| {
            pixel[0] = ((pixel[0] as f32 / 255.0).powf(gamma) * 255.0) as u8;
            pixel[1] = ((pixel[1] as f32 / 255.0).powf(gamma) * 255.0) as u8;
            pixel[2] = ((pixel[2] as f32 / 255.0).powf(gamma) * 255.0) as u8;
            // Alpha 通道保持不变
        });

        Ok(corrected)
    }
}

// HDR 校正实现（内部方法）
impl ImageProcessingAdapter {
    /// Reinhard tone mapping
    fn apply_reinhard_tonemapping(&self, image: &Image) -> Result<Image, InfrastructureError> {
        log::info!("ImageProcessingAdapter: applying Reinhard tone mapping");

        let mut corrected = image.clone();

        // 计算平均亮度
        let luminance_sum: f32 = corrected
            .data
            .chunks(4)
            .map(|pixel| {
                // Rec. 709 luminance formula
                0.2126 * (pixel[0] as f32) + 0.7152 * (pixel[1] as f32) + 0.0722 * (pixel[2] as f32)
            })
            .sum();

        let pixel_count = corrected.data.len() / 4;
        let avg_luminance = luminance_sum / pixel_count as f32;

        // Reinhard 算法: L_d = L_w / (1 + L_w)
        // 先归一化到平均亮度，再应用 tone mapping
        use rayon::prelude::*;
        corrected.data.par_chunks_mut(4).for_each(|pixel| {
            let r = pixel[0] as f32 / avg_luminance;
            let g = pixel[1] as f32 / avg_luminance;
            let b = pixel[2] as f32 / avg_luminance;

            pixel[0] = ((r / (1.0 + r)) * 255.0).min(255.0) as u8;
            pixel[1] = ((g / (1.0 + g)) * 255.0).min(255.0) as u8;
            pixel[2] = ((b / (1.0 + b)) * 255.0).min(255.0) as u8;
            // Alpha 通道保持不变
        });

        Ok(corrected)
    }

    /// 自动校正 (检测图像特征选择最佳算法)
    fn apply_auto_correction(&self, image: &Image) -> Result<Image, InfrastructureError> {
        log::info!("ImageProcessingAdapter: applying auto correction");

        // 分析图像直方图，判断是否需要 HDR 校正
        let mut histogram = vec![0u32; 256];
        for chunk in image.data.chunks(4) {
            // 计算亮度
            let luminance = (0.2126 * chunk[0] as f32
                + 0.7152 * chunk[1] as f32
                + 0.0722 * chunk[2] as f32) as usize;
            let luminance = luminance.min(255);
            histogram[luminance] += 1;
        }

        let pixel_count = (image.data.len() / 4) as f32;

        // 计算高光和暗部像素比例
        let highlights: u32 = histogram[200..].iter().sum();
        let shadows: u32 = histogram[..50].iter().sum();

        let highlight_ratio = highlights as f32 / pixel_count;
        let shadow_ratio = shadows as f32 / pixel_count;

        // 如果高光或暗部比例较高，使用 Reinhard；否则使用 Gamma
        if highlight_ratio > 0.15 || shadow_ratio > 0.15 {
            log::info!(
                "Auto: Using Reinhard (highlight={:.2}%, shadow={:.2}%)",
                highlight_ratio * 100.0,
                shadow_ratio * 100.0
            );
            self.apply_reinhard_tonemapping(image)
        } else {
            log::info!("Auto: Using Gamma correction");
            self.apply_gamma_correction(image)
        }
    }
}

// 内部辅助方法
impl ImageProcessingAdapter {
    /// 内部叠加图像函数（使用 rayon 并行）
    ///
    /// **复用**: `app-utils::overlay_image_ptr`
    fn overlay_image_internal(
        &self,
        image_pixels: &mut [u8],
        image_width: usize,
        target_image: &DynamicImage,
        offset_x: usize,
        offset_y: usize,
        channel_count: usize,
    ) {
        let target_image_width = target_image.width() as usize;
        let target_image_height = target_image.height() as usize;
        let target_image_pixels = target_image.as_bytes();

        let image_base_index = offset_y * image_width * channel_count + offset_x * channel_count;

        // 使用 unsafe 和指针来实现并行处理（复用 app-utils 逻辑）
        let image_pixels_ptr = image_pixels.as_mut_ptr() as usize;
        let target_image_pixels_ptr = target_image_pixels.as_ptr() as usize;

        (0..target_image_height).into_par_iter().for_each(|y| unsafe {
            let image_row_ptr = (image_pixels_ptr as *mut u8)
                .add(image_base_index + y * image_width * channel_count);
            let target_image_row_ptr =
                (target_image_pixels_ptr as *const u8).add(y * target_image_width * channel_count);

            std::ptr::copy_nonoverlapping(
                target_image_row_ptr,
                image_row_ptr,
                target_image_width * channel_count,
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_processing_adapter_creation() {
        let _adapter = ImageProcessingAdapter::new();
        // Adapter 创建成功，无需额外验证
    }

    #[test]
    fn test_encode_png() {
        let adapter = ImageProcessingAdapter::new();

        // 创建一个简单的 2x2 RGBA 图像
        let data = vec![255u8; 2 * 2 * 4]; // 白色
        let image = Image { width: 2, height: 2, data, color_format: ColorFormat::Rgba8 };

        let result = adapter.encode(&image, ImageFormat::Png);
        assert!(result.is_ok());

        let encoded = result.unwrap();
        assert!(!encoded.is_empty());

        // PNG 文件头检查
        assert_eq!(&encoded[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn test_decode_and_encode_roundtrip() {
        let adapter = ImageProcessingAdapter::new();

        // 创建测试图像
        let data = vec![128u8; 10 * 10 * 4];
        let original_image =
            Image { width: 10, height: 10, data, color_format: ColorFormat::Rgba8 };

        // 编码
        let encoded = adapter.encode(&original_image, ImageFormat::Png).unwrap();

        // 解码
        let decoded = adapter.decode(&encoded).unwrap();

        // 验证尺寸
        assert_eq!(decoded.width, 10);
        assert_eq!(decoded.height, 10);
    }

    #[test]
    fn test_resize() {
        let adapter = ImageProcessingAdapter::new();

        // 创建 10x10 图像
        let data = vec![255u8; 10 * 10 * 4];
        let image = Image { width: 10, height: 10, data, color_format: ColorFormat::Rgba8 };

        // 调整为 5x5
        let resized = adapter.resize(&image, 5, 5).unwrap();

        assert_eq!(resized.width, 5);
        assert_eq!(resized.height, 5);
    }
}

/// 图像叠加辅助函数 (从 app-utils 迁移)
///
/// 将目标图像叠加到指定位置的图像缓冲区上
pub fn overlay_image(
    image_pixels: &mut Vec<u8>,
    image_width: usize,
    target_image: &image::DynamicImage,
    offset_x: usize,
    offset_y: usize,
    channel_count: usize,
) {
    overlay_image_ptr(
        image_pixels.as_mut_ptr(),
        image_width,
        target_image,
        offset_x,
        offset_y,
        channel_count,
    );
}

/// 图像叠加辅助函数 (指针版本，从 app-utils 迁移)
pub fn overlay_image_ptr(
    image_pixels: *mut u8,
    image_width: usize,
    target_image: &image::DynamicImage,
    offset_x: usize,
    offset_y: usize,
    channel_count: usize,
) {
    let image_pixels_ptr = image_pixels as usize;

    let target_image_width = target_image.width() as usize;
    let target_image_height = target_image.height() as usize;
    let target_image_pixels = target_image.as_bytes();
    let target_image_pixels_ptr = target_image_pixels.as_ptr() as usize;

    let image_base_index = offset_y * image_width * channel_count + offset_x * channel_count;

    // 并行处理每一行
    (0..target_image_height).into_par_iter().for_each(|y| {
        let image_index = image_base_index + y * image_width * channel_count;
        let target_image_index = y * target_image_width * channel_count;

        unsafe {
            std::ptr::copy_nonoverlapping(
                (target_image_pixels_ptr + target_image_index) as *const u8,
                (image_pixels_ptr + image_index) as *mut u8,
                target_image_width * channel_count,
            );
        }
    });
}
