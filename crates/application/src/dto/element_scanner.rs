use aumate_core_traits::{ElementType, ScannableElement};
use serde::{Deserialize, Serialize};

/// 矩形边界 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectangleDto {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// 可扫描元素 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannableElementDto {
    /// 元素唯一标识符
    pub id: String,
    /// 元素类型 ("InputField" 或 "TaskbarIcon")
    pub element_type: String,
    /// 元素边界
    pub bounds: RectangleDto,
    /// 元素标题（如果有）
    pub title: Option<String>,
    /// 分配的字母标签 (A-Z)
    pub label: char,
}

impl From<ScannableElement> for ScannableElementDto {
    fn from(element: ScannableElement) -> Self {
        Self {
            id: element.id,
            element_type: match element.element_type {
                ElementType::InputField => "InputField".to_string(),
                ElementType::TaskbarIcon => "TaskbarIcon".to_string(),
            },
            bounds: RectangleDto {
                x: element.bounds.min_x(),
                y: element.bounds.min_y(),
                width: element.bounds.width(),
                height: element.bounds.height(),
            },
            title: element.title,
            label: element.label,
        }
    }
}

