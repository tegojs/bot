// 窗口列表相关 DTO
use aumate_core_traits::window::WindowInfo;
use serde::{Deserialize, Serialize};

/// 窗口元素（前端使用的格式）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowElementDto {
    pub id: String,
    pub window_id: u32,
    pub title: String,
    pub app_name: String,
    pub process_name: String,
    pub process_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub rect: WindowRectDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowRectDto {
    #[serde(rename = "minX")]
    pub min_x: f64,
    #[serde(rename = "minY")]
    pub min_y: f64,
    #[serde(rename = "maxX")]
    pub max_x: f64,
    #[serde(rename = "maxY")]
    pub max_y: f64,
}

impl From<WindowInfo> for WindowElementDto {
    fn from(info: WindowInfo) -> Self {
        Self {
            id: info.id,
            window_id: info.window_id,
            title: info.title,
            app_name: info.app_name,
            process_name: info.process_name,
            process_path: info.process_path,
            icon: info.icon,
            rect: WindowRectDto {
                min_x: info.bounds.min_x() as f64,
                min_y: info.bounds.min_y() as f64,
                max_x: info.bounds.max_x() as f64,
                max_y: info.bounds.max_y() as f64,
            },
        }
    }
}

/// 窗口列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowListResponse {
    pub windows: Vec<WindowElementDto>,
}
