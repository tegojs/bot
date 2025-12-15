use async_trait::async_trait;
use aumate_core_shared::InfrastructureError;
use aumate_core_traits::{ElementScannerPort, ScannableElement};

/// 元素扫描器适配器
///
/// 调用平台特定代码实现元素扫描功能
pub struct ElementScannerAdapter;

impl ElementScannerAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ElementScannerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ElementScannerPort for ElementScannerAdapter {
    async fn scan_elements(&self) -> Result<Vec<ScannableElement>, InfrastructureError> {
        #[cfg(target_os = "windows")]
        {
            crate::platform::windows::element_scanner::scan_elements()
                .await
                .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
        }

        #[cfg(target_os = "macos")]
        {
            // macOS 实现待补充
            Err(InfrastructureError::PlatformOperationFailed(
                "Element scanner not implemented for macOS".to_string(),
            ))
        }

        #[cfg(target_os = "linux")]
        {
            // Linux 实现待补充
            Err(InfrastructureError::PlatformOperationFailed(
                "Element scanner not implemented for Linux".to_string(),
            ))
        }
    }

    async fn click_element(&self, element_id: &str) -> Result<(), InfrastructureError> {
        #[cfg(target_os = "windows")]
        {
            crate::platform::windows::element_scanner::click_element(element_id)
                .await
                .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
        }

        #[cfg(target_os = "macos")]
        {
            let _ = element_id;
            Err(InfrastructureError::PlatformOperationFailed(
                "Element scanner not implemented for macOS".to_string(),
            ))
        }

        #[cfg(target_os = "linux")]
        {
            let _ = element_id;
            Err(InfrastructureError::PlatformOperationFailed(
                "Element scanner not implemented for Linux".to_string(),
            ))
        }
    }

    async fn focus_element(&self, element_id: &str) -> Result<(), InfrastructureError> {
        #[cfg(target_os = "windows")]
        {
            crate::platform::windows::element_scanner::focus_element(element_id)
                .await
                .map_err(|e| InfrastructureError::PlatformOperationFailed(e))
        }

        #[cfg(target_os = "macos")]
        {
            let _ = element_id;
            Err(InfrastructureError::PlatformOperationFailed(
                "Element scanner not implemented for macOS".to_string(),
            ))
        }

        #[cfg(target_os = "linux")]
        {
            let _ = element_id;
            Err(InfrastructureError::PlatformOperationFailed(
                "Element scanner not implemented for Linux".to_string(),
            ))
        }
    }
}

