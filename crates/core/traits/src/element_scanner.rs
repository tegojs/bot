use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, Rectangle};

/// 可扫描元素的类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementType {
    /// 输入框
    InputField,
    /// 任务栏图标
    TaskbarIcon,
}

/// 可扫描的元素
#[derive(Debug, Clone)]
pub struct ScannableElement {
    /// 元素唯一标识符
    pub id: String,
    /// 元素类型
    pub element_type: ElementType,
    /// 元素边界
    pub bounds: Rectangle,
    /// 元素标题（如果有）
    pub title: Option<String>,
    /// 分配的字母标签 (A-Z)
    pub label: char,
}

/// 元素扫描器 Port
///
/// 负责扫描屏幕上的可交互元素并提供操作接口
///
/// **实现者**:
/// - `ElementScannerAdapter` (infrastructure/adapters/element_scanner.rs)
#[async_trait]
pub trait ElementScannerPort: Send + Sync {
    /// 扫描屏幕上的可交互元素（输入框、任务栏图标等）
    ///
    /// 返回最多 26 个元素，按照从上到下、从左到右排序，
    /// 并分配字母标签 A-Z
    async fn scan_elements(&self) -> Result<Vec<ScannableElement>, InfrastructureError>;
    
    /// 点击指定元素
    ///
    /// # 参数
    /// - `element_id`: 元素的唯一标识符
    async fn click_element(&self, element_id: &str) -> Result<(), InfrastructureError>;
    
    /// 聚焦到指定元素（主要用于输入框）
    ///
    /// # 参数
    /// - `element_id`: 元素的唯一标识符
    async fn focus_element(&self, element_id: &str) -> Result<(), InfrastructureError>;
}



