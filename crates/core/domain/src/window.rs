use aumate_core_shared::{Point, Rectangle, WindowId};
use serde::{Deserialize, Serialize};

/// 窗口实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub rect: Rectangle,
    pub state: WindowState,
}

impl Window {
    pub fn new(id: WindowId, title: String, rect: Rectangle) -> Self {
        Self { id, title, rect, state: WindowState::Normal }
    }

    pub fn with_state(mut self, state: WindowState) -> Self {
        self.state = state;
        self
    }

    pub fn set_state(&mut self, state: WindowState) {
        self.state = state;
    }

    pub fn is_always_on_top(&self) -> bool {
        matches!(self.state, WindowState::AlwaysOnTop)
    }
}

/// 窗口状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    AlwaysOnTop,
}

/// 拖拽操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragOperation {
    pub window_id: WindowId,
    pub start_pos: Point,
}

impl DragOperation {
    pub fn new(window_id: WindowId, start_pos: Point) -> Self {
        Self { window_id, start_pos }
    }
}

/// 调整大小操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeOperation {
    pub window_id: WindowId,
    pub side: ResizeSide,
    pub constraints: ResizeConstraints,
}

impl ResizeOperation {
    pub fn new(window_id: WindowId, side: ResizeSide) -> Self {
        Self { window_id, side, constraints: ResizeConstraints::default() }
    }

    pub fn with_constraints(mut self, constraints: ResizeConstraints) -> Self {
        self.constraints = constraints;
        self
    }
}

/// 调整大小的边
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeConstraints {
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
}

impl ResizeConstraints {
    pub fn new() -> Self {
        Self { min_width: None, min_height: None, max_width: None, max_height: None }
    }

    pub fn with_min_size(mut self, width: u32, height: u32) -> Self {
        self.min_width = Some(width);
        self.min_height = Some(height);
        self
    }

    pub fn with_max_size(mut self, width: u32, height: u32) -> Self {
        self.max_width = Some(width);
        self.max_height = Some(height);
        self
    }

    pub fn validate_size(&self, width: u32, height: u32) -> bool {
        if let Some(min_width) = self.min_width {
            if width < min_width {
                return false;
            }
        }
        if let Some(min_height) = self.min_height {
            if height < min_height {
                return false;
            }
        }
        if let Some(max_width) = self.max_width {
            if width > max_width {
                return false;
            }
        }
        if let Some(max_height) = self.max_height {
            if height > max_height {
                return false;
            }
        }
        true
    }
}

impl Default for ResizeConstraints {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let rect = Rectangle::from_xywh(100, 100, 800, 600).unwrap();
        let window =
            Window::new(WindowId::new("test_window".to_string()), "Test Window".to_string(), rect);

        assert_eq!(window.title, "Test Window");
        assert_eq!(window.state, WindowState::Normal);
        assert!(!window.is_always_on_top());
    }

    #[test]
    fn test_window_state_change() {
        let rect = Rectangle::from_xywh(100, 100, 800, 600).unwrap();
        let mut window =
            Window::new(WindowId::new("test_window".to_string()), "Test Window".to_string(), rect);

        window.set_state(WindowState::AlwaysOnTop);
        assert!(window.is_always_on_top());
    }

    #[test]
    fn test_resize_constraints_validation() {
        let constraints =
            ResizeConstraints::new().with_min_size(200, 150).with_max_size(1920, 1080);

        assert!(constraints.validate_size(800, 600));
        assert!(!constraints.validate_size(100, 100)); // Too small
        assert!(!constraints.validate_size(2000, 2000)); // Too large
    }

    #[test]
    fn test_drag_operation() {
        let op = DragOperation::new(WindowId::new("test".to_string()), Point::new(100, 200));
        assert_eq!(op.start_pos.x, 100);
        assert_eq!(op.start_pos.y, 200);
    }
}
