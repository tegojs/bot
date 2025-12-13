// 滚动截图用例
use aumate_core_shared::UseCaseError;
use aumate_core_traits::ScrollCapturePort;
use aumate_core_traits::scroll::{ScrollCaptureOptions, ScrollCaptureRequest, ScrollDirection};
use std::sync::Arc;

use crate::dto::{ScrollCaptureResponse, StartScrollCaptureRequest};

/// 滚动截图用例
///
/// 实现滚动截图流程
pub struct ScrollScreenshotUseCase {
    scroll_capture: Arc<dyn ScrollCapturePort + Send + Sync>,
}

impl ScrollScreenshotUseCase {
    pub fn new(scroll_capture: Arc<dyn ScrollCapturePort + Send + Sync>) -> Self {
        Self { scroll_capture }
    }

    /// 执行滚动截图
    pub async fn execute(
        &self,
        request: StartScrollCaptureRequest,
    ) -> Result<ScrollCaptureResponse, UseCaseError> {
        log::info!("ScrollScreenshotUseCase: 开始滚动截图");

        // 1. 验证参数
        if request.region.width() == 0 || request.region.height() == 0 {
            return Err(UseCaseError::InvalidRequest("捕获区域宽度和高度必须大于 0".to_string()));
        }

        // 2. 解析滚动方向
        let direction = match request.direction.to_lowercase().as_str() {
            "vertical" => ScrollDirection::Vertical,
            "horizontal" => ScrollDirection::Horizontal,
            _ => {
                return Err(UseCaseError::InvalidRequest(format!(
                    "不支持的滚动方向: {}",
                    request.direction
                )));
            }
        };

        // 3. 构建捕获选项
        let options = ScrollCaptureOptions {
            sample_rate: 60.0,
            match_threshold: 95,
            max_scroll_attempts: request.max_frames.unwrap_or(100) as u32,
        };

        // 4. 构建捕获请求
        let scroll_request = ScrollCaptureRequest { direction, region: request.region, options };

        // 5. 初始化滚动截图（简化版本 - 实际需要状态管理）
        // 注意：完整实现需要多步骤流程：init -> capture_frame -> process -> finalize
        // 这里先返回占位符响应，完整流程需要在后续实现

        log::warn!("ScrollScreenshotUseCase: 完整流程待实现，需要状态机管理多步骤");

        // 临时响应
        let response = ScrollCaptureResponse {
            id: aumate_core_shared::ScreenshotId::generate(),
            data: vec![],
            width: 0,
            height: 0,
            frame_count: 0,
        };

        log::info!(
            "ScrollScreenshotUseCase: 滚动截图完成，捕获 {} 帧，尺寸 {}x{}",
            response.frame_count,
            response.width,
            response.height
        );

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_screenshot_use_case_creation() {
        // 测试需要 Mock ScrollCapturePort
    }
}
