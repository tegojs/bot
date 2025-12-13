// 窗口管理用例
use aumate_core_shared::{Point, UseCaseError, WindowId};
use aumate_core_traits::WindowManagementPort;
use aumate_core_traits::window::{
    DragOperation, ResizeConstraints, ResizeOperation, ResizeSide, WindowConfig,
};
use std::sync::Arc;

use crate::dto::{
    CreateWindowRequest, CreateWindowResponse, DragWindowRequest, ResizeWindowRequest,
};

/// 窗口管理用例
///
/// 实现窗口管理相关功能
pub struct WindowManagementUseCase {
    window_management: Arc<dyn WindowManagementPort + Send + Sync>,
}

impl WindowManagementUseCase {
    pub fn new(window_management: Arc<dyn WindowManagementPort + Send + Sync>) -> Self {
        Self { window_management }
    }

    /// 创建窗口
    pub async fn create_window(
        &self,
        request: CreateWindowRequest,
    ) -> Result<CreateWindowResponse, UseCaseError> {
        log::info!("WindowManagementUseCase: 创建窗口 {}", request.label);

        // 1. 构建窗口配置
        let config = WindowConfig {
            title: request.title.clone(),
            width: request.width.unwrap_or(800.0) as u32,
            height: request.height.unwrap_or(600.0) as u32,
            x: 0,
            y: 0,
            resizable: true,
            decorations: true,
        };

        // 2. 调用 Infrastructure 层创建窗口
        let window = self
            .window_management
            .create_fixed_content_window(config)
            .await
            .map_err(|e| UseCaseError::Infrastructure(e))?;

        // 3. 构建响应
        let response = CreateWindowResponse {
            window_id: WindowId::new(request.label.clone()),
            label: request.label,
        };

        log::info!("WindowManagementUseCase: 窗口创建成功");
        Ok(response)
    }

    /// 拖动窗口
    pub async fn drag_window(&self, request: DragWindowRequest) -> Result<(), UseCaseError> {
        log::info!("WindowManagementUseCase: 拖动窗口 {}", request.window_id.as_str());

        // 1. 构建拖动操作
        let operation = DragOperation {
            window_id: request.window_id.clone(),
            start_pos: request.delta, // 使用 delta 作为起始位置
        };

        // 2. 调用 Infrastructure 层开始拖动窗口
        self.window_management
            .start_drag(operation)
            .await
            .map_err(|e| UseCaseError::Infrastructure(e))?;

        log::info!("WindowManagementUseCase: 窗口拖动完成");
        Ok(())
    }

    /// 调整窗口大小
    pub async fn resize_window(&self, request: ResizeWindowRequest) -> Result<(), UseCaseError> {
        log::info!("WindowManagementUseCase: 调整窗口大小 {}", request.window_id.as_str());

        // 1. 解析调整边
        let side = match request.side.to_lowercase().as_str() {
            "top" => ResizeSide::Top,
            "bottom" => ResizeSide::Bottom,
            "left" => ResizeSide::Left,
            "right" => ResizeSide::Right,
            _ => {
                return Err(UseCaseError::InvalidRequest(format!(
                    "不支持的调整边: {}",
                    request.side
                )));
            }
        };

        // 2. 构建调整操作
        let operation = ResizeOperation {
            window_id: request.window_id.clone(),
            side,
            constraints: ResizeConstraints {
                min_width: None,
                min_height: None,
                max_width: None,
                max_height: None,
            },
        };

        // 3. 调用 Infrastructure 层开始调整窗口
        self.window_management
            .start_resize(operation)
            .await
            .map_err(|e| UseCaseError::Infrastructure(e))?;

        log::info!("WindowManagementUseCase: 窗口调整完成");
        Ok(())
    }

    /// 设置窗口置顶
    pub async fn set_always_on_top(
        &self,
        window_id: WindowId,
        enable: bool,
    ) -> Result<(), UseCaseError> {
        log::info!("WindowManagementUseCase: 设置窗口置顶 {} = {}", window_id.as_str(), enable);

        self.window_management
            .set_always_on_top(window_id, enable)
            .await
            .map_err(|e| UseCaseError::Infrastructure(e))?;

        log::info!("WindowManagementUseCase: 窗口置顶设置完成");
        Ok(())
    }

    /// 关闭窗口
    pub async fn close_window(&self, window_id: WindowId) -> Result<(), UseCaseError> {
        log::info!("WindowManagementUseCase: 关闭窗口 {}", window_id.as_str());

        self.window_management
            .close_window(window_id)
            .await
            .map_err(|e| UseCaseError::Infrastructure(e))?;

        log::info!("WindowManagementUseCase: 窗口关闭完成");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_management_use_case_creation() {
        // 测试需要 Mock WindowManagementPort
    }
}
