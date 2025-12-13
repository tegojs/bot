/// 监视器信息管理 Use Case
use crate::dto::monitor::{MonitorInfo, MonitorsResponse};
use aumate_core_shared::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<UseCaseError> for aumate_core_shared::DomainError {
    fn from(e: UseCaseError) -> Self {
        aumate_core_shared::DomainError::InvalidRectangle(e.to_string())
    }
}
use aumate_core_traits::screenshot::ScreenCapturePort;
use std::sync::Arc;

/// 获取所有监视器信息 Use Case
pub struct GetMonitorsUseCase<P: ScreenCapturePort> {
    screen_capture: Arc<P>,
}

impl<P: ScreenCapturePort> GetMonitorsUseCase<P> {
    pub fn new(screen_capture: Arc<P>) -> Self {
        Self { screen_capture }
    }

    pub async fn execute(&self) -> Result<MonitorsResponse> {
        log::info!("GetMonitorsUseCase: Fetching all monitors");

        let monitors = self
            .screen_capture
            .get_monitors()
            .await
            .map_err(|e| UseCaseError::Infrastructure(e.to_string()))?;

        let monitor_infos: Vec<MonitorInfo> = monitors
            .iter()
            .map(|m| MonitorInfo {
                id: m.id.value().to_string(),
                name: m.name.clone(),
                x: m.rect.min_x(),
                y: m.rect.min_y(),
                width: m.rect.width(),
                height: m.rect.height(),
                scale_factor: m.scale_factor,
                is_primary: m.is_primary,
            })
            .collect();

        let primary_monitor_id =
            monitors.iter().find(|m| m.is_primary).map(|m| m.id.value().to_string());

        Ok(MonitorsResponse { monitors: monitor_infos, primary_monitor_id })
    }
}

/// 获取当前监视器信息 Use Case
pub struct GetCurrentMonitorUseCase<P: ScreenCapturePort> {
    screen_capture: Arc<P>,
}

impl<P: ScreenCapturePort> GetCurrentMonitorUseCase<P> {
    pub fn new(screen_capture: Arc<P>) -> Self {
        Self { screen_capture }
    }

    pub async fn execute(&self) -> Result<MonitorInfo> {
        log::info!("GetCurrentMonitorUseCase: Fetching current monitor");

        // 获取所有监视器
        let monitors = self
            .screen_capture
            .get_monitors()
            .await
            .map_err(|e| UseCaseError::Infrastructure(e.to_string()))?;

        // 查找主监视器
        let current = monitors
            .iter()
            .find(|m| m.is_primary)
            .or_else(|| monitors.first())
            .ok_or_else(|| UseCaseError::NotFound("No monitor found".to_string()))?;

        Ok(MonitorInfo {
            id: current.id.value().to_string(),
            name: current.name.clone(),
            x: current.rect.min_x(),
            y: current.rect.min_y(),
            width: current.rect.width(),
            height: current.rect.height(),
            scale_factor: current.scale_factor,
            is_primary: current.is_primary,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use aumate_core_shared::{InfrastructureError, MonitorId, Rectangle};
    use aumate_core_traits::screenshot::Monitor;

    struct MockScreenCapture {
        monitors: Vec<Monitor>,
    }

    #[async_trait]
    impl ScreenCapturePort for MockScreenCapture {
        async fn capture(
            &self,
            _: aumate_core_traits::screenshot::CaptureTarget,
            _: aumate_core_traits::screenshot::CaptureOptions,
        ) -> std::result::Result<aumate_core_traits::screenshot::Screenshot, InfrastructureError>
        {
            unimplemented!()
        }

        async fn get_monitors(&self) -> std::result::Result<Vec<Monitor>, InfrastructureError> {
            Ok(self.monitors.clone())
        }

        async fn get_current_monitor(&self) -> std::result::Result<Monitor, InfrastructureError> {
            self.monitors
                .iter()
                .find(|m| m.is_primary)
                .cloned()
                .ok_or_else(|| InfrastructureError::CaptureFailed("No primary monitor".to_string()))
        }

        async fn get_focused_window(&self) -> std::result::Result<WindowId, InfrastructureError> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_get_monitors() {
        let monitors = vec![Monitor {
            id: MonitorId::new(0),
            name: "Monitor 1".to_string(),
            rect: Rectangle::from_bounds(0, 0, 1920, 1080),
            scale_factor: 1.0,
            is_primary: true,
        }];

        let mock = Arc::new(MockScreenCapture { monitors: monitors.clone() });
        let use_case = GetMonitorsUseCase::new(mock);

        let result = use_case.execute().await.unwrap();
        assert_eq!(result.monitors.len(), 1);
        assert_eq!(result.monitors[0].id, "0");
        assert!(result.monitors[0].is_primary);
    }
}
