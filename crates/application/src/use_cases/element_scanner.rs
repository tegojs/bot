use aumate_core_shared::UseCaseError;
use aumate_core_traits::ElementScannerPort;
use std::sync::Arc;

use crate::dto::ScannableElementDto;

/// 扫描元素用例
pub struct ScanElementsUseCase {
    scanner: Arc<dyn ElementScannerPort>,
}

impl ScanElementsUseCase {
    pub fn new(scanner: Arc<dyn ElementScannerPort>) -> Self {
        Self { scanner }
    }

    pub async fn execute(&self) -> Result<Vec<ScannableElementDto>, UseCaseError> {
        log::info!("[ScanElementsUseCase] Executing element scan");

        let elements = self
            .scanner
            .scan_elements()
            .await
            .map_err(|e| {
                log::error!("[ScanElementsUseCase] Failed to scan elements: {}", e);
                UseCaseError::from(e)
            })?;

        log::info!(
            "[ScanElementsUseCase] Successfully scanned {} elements",
            elements.len()
        );

        Ok(elements.into_iter().map(Into::into).collect())
    }
}

/// 点击元素用例
pub struct ClickElementUseCase {
    scanner: Arc<dyn ElementScannerPort>,
}

impl ClickElementUseCase {
    pub fn new(scanner: Arc<dyn ElementScannerPort>) -> Self {
        Self { scanner }
    }

    pub async fn execute(&self, element_id: &str) -> Result<(), UseCaseError> {
        log::info!("[ClickElementUseCase] Clicking element: {}", element_id);

        self.scanner
            .click_element(element_id)
            .await
            .map_err(|e| {
                log::error!(
                    "[ClickElementUseCase] Failed to click element '{}': {}",
                    element_id,
                    e
                );
                UseCaseError::from(e)
            })?;

        log::info!("[ClickElementUseCase] Successfully clicked element: {}", element_id);
        Ok(())
    }
}

/// 聚焦元素用例
pub struct FocusElementUseCase {
    scanner: Arc<dyn ElementScannerPort>,
}

impl FocusElementUseCase {
    pub fn new(scanner: Arc<dyn ElementScannerPort>) -> Self {
        Self { scanner }
    }

    pub async fn execute(&self, element_id: &str) -> Result<(), UseCaseError> {
        log::info!("[FocusElementUseCase] Focusing element: {}", element_id);

        self.scanner
            .focus_element(element_id)
            .await
            .map_err(|e| {
                log::error!(
                    "[FocusElementUseCase] Failed to focus element '{}': {}",
                    element_id,
                    e
                );
                UseCaseError::from(e)
            })?;

        log::info!("[FocusElementUseCase] Successfully focused element: {}", element_id);
        Ok(())
    }
}

/// 元素操作类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementActionType {
    Click,
    Focus,
}

impl ElementActionType {
    pub fn from_str(s: &str) -> Result<Self, UseCaseError> {
        match s.to_lowercase().as_str() {
            "click" => Ok(Self::Click),
            "focus" => Ok(Self::Focus),
            _ => Err(UseCaseError::InvalidRequest(format!(
                "Unknown action type: '{}'. Expected 'click' or 'focus'",
                s
            ))),
        }
    }
}

/// 触发元素操作用例（统一处理点击和聚焦）
///
/// 这个用例封装了业务逻辑判断，根据操作类型调用相应的底层操作
pub struct TriggerElementActionUseCase {
    scanner: Arc<dyn ElementScannerPort>,
}

impl TriggerElementActionUseCase {
    pub fn new(scanner: Arc<dyn ElementScannerPort>) -> Self {
        Self { scanner }
    }

    pub async fn execute(
        &self,
        element_id: &str,
        action_type: ElementActionType,
    ) -> Result<(), UseCaseError> {
        log::info!(
            "[TriggerElementActionUseCase] Triggering {:?} on element: {}",
            action_type,
            element_id
        );

        match action_type {
            ElementActionType::Click => {
                self.scanner
                    .click_element(element_id)
                    .await
                    .map_err(|e| {
                        log::error!(
                            "[TriggerElementActionUseCase] Failed to click element '{}': {}",
                            element_id,
                            e
                        );
                        UseCaseError::from(e)
                    })?;
            }
            ElementActionType::Focus => {
                self.scanner
                    .focus_element(element_id)
                    .await
                    .map_err(|e| {
                        log::error!(
                            "[TriggerElementActionUseCase] Failed to focus element '{}': {}",
                            element_id,
                            e
                        );
                        UseCaseError::from(e)
                    })?;
            }
        }

        log::info!(
            "[TriggerElementActionUseCase] Successfully triggered {:?} on element: {}",
            action_type,
            element_id
        );
        Ok(())
    }
}

