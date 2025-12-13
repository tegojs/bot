use async_trait::async_trait;
use aumate_core_shared::{InfrastructureError, Point, WindowId};

// Re-export domain types for convenience
// Domain 层定义了数据结构，traits 层只定义接口
pub use aumate_core_domain::hotkey::{Key, KeyEvent, MouseButton, MouseEvent};

/// 快捷键监听 Port
///
/// 负责键盘和鼠标监听
///
/// **实现者**:
/// - `KeyboardListenerAdapter`
/// - `MouseListenerAdapter`
#[async_trait]
pub trait HotkeyListenerPort: Send + Sync {
    /// 开始监听指定窗口
    async fn start_listening(&mut self, window_id: WindowId) -> Result<(), InfrastructureError>;

    /// 停止监听指定窗口
    async fn stop_listening(&mut self, window_id: WindowId) -> Result<(), InfrastructureError>;

    /// 注册事件处理器
    fn register_handler(&mut self, handler: Box<dyn InputEventHandler>);
}

/// 输入事件处理器
pub trait InputEventHandler: Send + Sync {
    fn on_key_down(&self, key: Key, window_id: WindowId);
    fn on_key_up(&self, key: Key, window_id: WindowId);
    fn on_mouse_down(&self, button: MouseButton, position: Point, window_id: WindowId);
    fn on_mouse_up(&self, button: MouseButton, position: Point, window_id: WindowId);
}

/// 输入模拟 Port
///
/// 负责输入模拟
///
/// **实现者**:
/// - `InputSimulationAdapter`
#[async_trait]
pub trait InputSimulationPort: Send + Sync {
    /// 模拟鼠标滚动
    async fn scroll(&self, delta_x: i32, delta_y: i32) -> Result<(), InfrastructureError>;

    /// 模拟鼠标点击
    async fn click(&self, button: MouseButton, position: Point) -> Result<(), InfrastructureError>;

    /// 移动鼠标
    async fn move_mouse(&self, position: Point) -> Result<(), InfrastructureError>;

    /// 获取当前鼠标位置
    async fn get_mouse_position(&self) -> Result<Point, InfrastructureError>;
}
