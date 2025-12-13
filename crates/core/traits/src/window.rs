use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, Point, Rectangle, WindowId};

/// 窗口信息
#[derive(Debug, Clone)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub rect: Rectangle,
    pub state: WindowState,
}

/// 窗口状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    AlwaysOnTop,
}

/// 拖拽操作
#[derive(Debug, Clone)]
pub struct DragOperation {
    pub window_id: WindowId,
    pub start_pos: Point,
}

/// 调整大小操作
#[derive(Debug, Clone)]
pub struct ResizeOperation {
    pub window_id: WindowId,
    pub side: ResizeSide,
    pub constraints: ResizeConstraints,
}

/// 调整大小的边
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeSide {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// 调整大小约束
#[derive(Debug, Clone)]
pub struct ResizeConstraints {
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
}

/// 绘制窗口样式 (macOS 特定)
#[derive(Debug, Clone, Copy)]
pub enum DrawWindowStyle {
    Default,
    Transparent,
}

/// 窗口配置
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub resizable: bool,
    pub decorations: bool,
}

/// 窗口管理 Port
///
/// 负责窗口管理操作
///
/// **实现者**:
/// - `WindowManagementAdapter`
#[async_trait]
pub trait WindowManagementPort: Send + Sync {
    /// 获取窗口信息
    async fn get_window(&self, id: WindowId) -> Result<Window, InfrastructureError>;

    /// 设置窗口置顶
    async fn set_always_on_top(
        &self,
        id: WindowId,
        enable: bool,
    ) -> Result<(), InfrastructureError>;

    /// 开始拖拽窗口
    async fn start_drag(&self, operation: DragOperation) -> Result<(), InfrastructureError>;

    /// 开始调整窗口大小
    async fn start_resize(&self, operation: ResizeOperation) -> Result<(), InfrastructureError>;

    /// 设置窗口样式（macOS draw window）
    async fn set_draw_style(
        &self,
        id: WindowId,
        style: DrawWindowStyle,
    ) -> Result<(), InfrastructureError>;

    /// 创建固定内容窗口
    async fn create_fixed_content_window(
        &self,
        config: WindowConfig,
    ) -> Result<WindowId, InfrastructureError>;

    /// 关闭窗口
    async fn close_window(&self, id: WindowId) -> Result<(), InfrastructureError>;
}

/// UI 元素
#[derive(Debug, Clone)]
pub struct UIElement {
    pub bounds: Rectangle,
    pub role: Option<String>,
    pub title: Option<String>,
    pub value: Option<String>,
}

/// UI 自动化 Port
///
/// 负责 UI 自动化操作
///
/// **实现者**:
/// - `WindowsUIAutomationAdapter`
/// - `MacOSUIAutomationAdapter`
/// - `LinuxUIAutomationAdapter`
#[async_trait]
pub trait UIAutomationPort: Send + Sync {
    /// 初始化 UI 元素
    async fn init_ui_elements(&mut self) -> Result<(), InfrastructureError>;

    /// 初始化 UI 元素缓存
    async fn init_ui_elements_cache(&mut self) -> Result<(), InfrastructureError>;

    /// 获取窗口元素
    async fn get_window_elements(&self) -> Result<Vec<UIElement>, InfrastructureError>;

    /// 获取指定位置的元素
    async fn get_element_from_position(
        &self,
        position: Point,
    ) -> Result<Vec<UIElement>, InfrastructureError>;
}
