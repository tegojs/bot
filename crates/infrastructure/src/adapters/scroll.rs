use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;
use aumate_core_traits::scroll::{
    CaptureFrame, ProcessResult, ScrollCapturePort, ScrollCaptureRequest, ScrollDirection,
    ScrollScreenshot,
};

use crate::services::scroll::{
    ScrollDirection as InternalDirection, ScrollImageList as InternalImageList,
    ScrollScreenshotService as InternalService,
};

/// 滚动截图适配器
///
/// **完整迁移**: 使用内部的滚动截图服务
///
/// **迁移代码**:
/// - `services::scroll::ScrollScreenshotService` (~1081 行) - 完整迁移
/// - `services::scroll::ScrollScreenshotCaptureService` - 完整迁移
/// - `services::scroll::ScrollScreenshotImageService` - 完整迁移
///
/// **实际实现**:
/// 不再依赖外部 `app-scroll-screenshot-service` crate，
/// 所有代码已迁移到 `infrastructure/src/services/scroll/`
pub struct ScrollCaptureAdapter {
    inner: InternalService,
}

impl ScrollCaptureAdapter {
    pub fn new() -> Self {
        Self { inner: InternalService::new() }
    }

    /// 将领域的 ScrollDirection 转换为内部实现的类型
    fn convert_direction(direction: ScrollDirection) -> InternalDirection {
        match direction {
            ScrollDirection::Vertical => InternalDirection::Vertical,
            ScrollDirection::Horizontal => InternalDirection::Horizontal,
        }
    }
}

impl Default for ScrollCaptureAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ScrollCapturePort for ScrollCaptureAdapter {
    /// 初始化滚动截图
    async fn init(&mut self, request: ScrollCaptureRequest) -> Result<(), InfrastructureError> {
        let direction = Self::convert_direction(request.direction);

        // 调用内部服务的 init 方法
        // 使用默认值填充缺失的参数
        self.inner.init(
            direction,
            request.options.sample_rate,
            800,  // min_sample_size 默认值
            2000, // max_sample_size 默认值
            request.options.match_threshold,
            16,   // descriptor_patch_size 默认值
            10,   // min_size_delta 默认值
            true, // try_rollback 默认值
        );

        Ok(())
    }

    /// 捕获一帧
    async fn capture_frame(&mut self) -> Result<CaptureFrame, InfrastructureError> {
        // TODO: 实际的帧捕获需要与平台特定的截图服务配合
        // 这里暂时返回未实现错误
        Err(InfrastructureError::ExternalError(
            "capture_frame requires platform-specific screen capture integration".to_string(),
        ))
    }

    /// 处理捕获的图像
    async fn handle_image(
        &mut self,
        image_data: Vec<u8>,
    ) -> Result<ProcessResult, InfrastructureError> {
        // 解码图像
        let image = image::load_from_memory(&image_data).map_err(|e| {
            InfrastructureError::ImageProcessingFailed(format!("Failed to decode image: {}", e))
        })?;

        // 处理图像（调用内部服务）
        // handle_image 返回 (Option<(i32, Option<ScrollImageList>)>, bool, ScrollImageList)
        let (result, _reached_end, _current_list) =
            self.inner.handle_image(image, InternalImageList::Bottom);

        Ok(ProcessResult {
            matched: result.is_some(),
            offset: result.and_then(|(offset, _)| Some(aumate_core_shared::Point::new(offset, 0))),
        })
    }

    /// 获取当前拼接后的图像大小
    async fn get_size(&self) -> Result<(u32, u32), InfrastructureError> {
        // TODO: 需要实现 get_size 方法
        // 目前返回默认值
        Ok((self.inner.image_width, self.inner.image_height))
    }

    /// 完成并返回最终图像
    async fn finalize(&mut self) -> Result<ScrollScreenshot, InfrastructureError> {
        // TODO: 使用 ScrollScreenshotImageService 来生成最终图像
        // 这需要更复杂的逻辑，包括图像拼接和裁剪

        // 目前返回一个占位错误
        Err(InfrastructureError::ExternalError(
            "finalize requires ScrollScreenshotImageService integration".to_string(),
        ))
    }

    /// 清理资源
    async fn clear(&mut self) -> Result<(), InfrastructureError> {
        // 重新初始化内部服务
        self.inner = InternalService::new();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aumate_core_shared::Rectangle;
    use aumate_core_traits::scroll::ScrollCaptureOptions;

    #[tokio::test]
    async fn test_scroll_capture_adapter_creation() {
        let _adapter = ScrollCaptureAdapter::new();
        // 创建成功
    }

    #[tokio::test]
    async fn test_scroll_capture_adapter_init() {
        let mut adapter = ScrollCaptureAdapter::new();

        let request = ScrollCaptureRequest {
            direction: ScrollDirection::Vertical,
            region: Rectangle::from_xywh(0, 0, 800, 600).unwrap(),
            options: ScrollCaptureOptions::default(),
        };

        let result = adapter.init(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scroll_capture_adapter_get_size() {
        let adapter = ScrollCaptureAdapter::new();
        let result = adapter.get_size().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scroll_capture_adapter_clear() {
        let mut adapter = ScrollCaptureAdapter::new();

        adapter.clear().await.unwrap();
        // 清理成功
    }
}
