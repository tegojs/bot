use aumate_core_shared::{Point, WindowId};
use serde::{Deserialize, Serialize};

/// 按键枚举（完整版本）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    // 功能键
    Escape,
    Return,
    Space,
    Backspace,
    Delete,
    Tab,

    // 方向键
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,

    // 字母键
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // 数字键
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // F键
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // 修饰键
    LeftControl,
    RightControl,
    LeftShift,
    RightShift,
    LeftAlt,
    RightAlt,
    LeftMeta,
    RightMeta,

    // 其他
    Other(u32),
}

/// 鼠标按钮
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// 键盘事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key: Key,
    pub window_id: WindowId,
    pub is_down: bool,
    pub timestamp: u64,
}

/// 鼠标事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub position: Point,
    pub window_id: WindowId,
    pub is_down: bool,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_enum() {
        let key = Key::A;
        assert_eq!(key, Key::A);
    }

    #[test]
    fn test_mouse_button() {
        let button = MouseButton::Left;
        assert_eq!(button, MouseButton::Left);
    }
}
