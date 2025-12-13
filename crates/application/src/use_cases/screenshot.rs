// 截图用例
use aumate_core_shared::UseCaseError;
use aumate_core_traits::screenshot::{
    CaptureOptions, CaptureTarget, HdrCorrectionAlgorithm, Image, ImageFormat, Screenshot,
};
use aumate_core_traits::{ImageProcessingPort, ScreenCapturePort};
use std::sync::Arc;

use crate::dto::{CaptureRegionRequest, CaptureResponse, CaptureScreenRequest};

/// 捕获屏幕用例
///
/// 实现完整的屏幕截图流程
pub struct CaptureScreenUseCase {
    screen_capture: Arc<dyn ScreenCapturePort + Send + Sync>,
    image_processing: Arc<dyn ImageProcessingPort + Send + Sync>,
}

impl CaptureScreenUseCase {
    pub fn new(
        screen_capture: Arc<dyn ScreenCapturePort + Send + Sync>,
        image_processing: Arc<dyn ImageProcessingPort + Send + Sync>,
    ) -> Self {
        Self { screen_capture, image_processing }
    }

    /// 执行截图
    pub async fn execute(
        &self,
        request: CaptureScreenRequest,
    ) -> Result<CaptureResponse, UseCaseError> {
        log::info!("CaptureScreenUseCase: 开始捕获屏幕");

        // 1. 验证并解析参数
        let image_format = self.parse_image_format(&request.format)?;

        // 2. 构建捕获选项
        let capture_options = CaptureOptions {
            exclude_windows: vec![],
            cursor_visible: false,
            hdr_correction: if request.hdr_correction {
                Some(HdrCorrectionAlgorithm::Auto)
            } else {
                None
            },
        };

        // 3. 确定捕获目标
        let target = if let Some(monitor_id) = request.monitor_id {
            CaptureTarget::Monitor(monitor_id)
        } else {
            CaptureTarget::CurrentMonitor
        };

        // 4. 调用 Infrastructure 层捕获屏幕
        let screenshot = self
            .screen_capture
            .capture(target, capture_options)
            .await
            .map_err(|e| UseCaseError::CaptureFailed(format!("屏幕捕获失败: {:?}", e)))?;

        // 5. 编码图像（同步方法）
        let encoded_data = self
            .image_processing
            .encode(&screenshot.image, image_format)
            .map_err(|e| UseCaseError::EncodingFailed(format!("图像编码失败: {:?}", e)))?;

        // 6. 构建响应
        let response = CaptureResponse {
            id: aumate_core_shared::ScreenshotId::generate(),
            data: encoded_data.clone(),
            width: screenshot.image.width,
            height: screenshot.image.height,
            format: request.format,
        };

        log::info!(
            "CaptureScreenUseCase: 截图完成，尺寸 {}x{}，大小 {} bytes",
            response.width,
            response.height,
            response.data.len()
        );

        Ok(response)
    }

    /// 解析图像格式
    fn parse_image_format(&self, format: &str) -> Result<ImageFormat, UseCaseError> {
        match format.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
            "webp" => Ok(ImageFormat::WebP),
            _ => Err(UseCaseError::InvalidRequest(format!("不支持的图像格式: {}", format))),
        }
    }
}

/// 捕获区域用例
///
/// 实现区域截图流程
pub struct CaptureRegionUseCase {
    screen_capture: Arc<dyn ScreenCapturePort + Send + Sync>,
    image_processing: Arc<dyn ImageProcessingPort + Send + Sync>,
}

impl CaptureRegionUseCase {
    pub fn new(
        screen_capture: Arc<dyn ScreenCapturePort + Send + Sync>,
        image_processing: Arc<dyn ImageProcessingPort + Send + Sync>,
    ) -> Self {
        Self { screen_capture, image_processing }
    }

    /// 执行区域截图
    pub async fn execute(
        &self,
        request: CaptureRegionRequest,
    ) -> Result<CaptureResponse, UseCaseError> {
        log::info!(
            "CaptureRegionUseCase: 捕获区域 {}x{}",
            request.region.width(),
            request.region.height()
        );

        // 1. 验证区域参数
        if request.region.width() == 0 || request.region.height() == 0 {
            return Err(UseCaseError::InvalidRequest("区域宽度和高度必须大于 0".to_string()));
        }

        // 2. 解析图像格式
        let image_format = self.parse_image_format(&request.format)?;

        // 3. 构建捕获选项
        let capture_options =
            CaptureOptions { exclude_windows: vec![], cursor_visible: false, hdr_correction: None };

        // 4. 确定捕获目标
        let target = CaptureTarget::Region(request.region);

        // 5. 调用 Infrastructure 层捕获区域
        let screenshot = self
            .screen_capture
            .capture(target, capture_options)
            .await
            .map_err(|e| UseCaseError::CaptureFailed(format!("区域捕获失败: {:?}", e)))?;

        // 6. 编码图像（同步方法）
        let encoded_data = self
            .image_processing
            .encode(&screenshot.image, image_format)
            .map_err(|e| UseCaseError::EncodingFailed(format!("图像编码失败: {:?}", e)))?;

        // 7. 构建响应
        let response = CaptureResponse {
            id: aumate_core_shared::ScreenshotId::generate(),
            data: encoded_data,
            width: screenshot.image.width,
            height: screenshot.image.height,
            format: request.format,
        };

        log::info!(
            "CaptureRegionUseCase: 区域截图完成，尺寸 {}x{}",
            response.width,
            response.height
        );

        Ok(response)
    }

    /// 解析图像格式
    fn parse_image_format(&self, format: &str) -> Result<ImageFormat, UseCaseError> {
        match format.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
            "webp" => Ok(ImageFormat::WebP),
            _ => Err(UseCaseError::InvalidRequest(format!("不支持的图像格式: {}", format))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_image_format() {
        // 测试需要创建用例实例，这里保持简单
    }
}
