//! Core types for event hooks - simplified from rdev
//!
//! This module provides the essential types needed for global event interception.

use std::time::SystemTime;

/// Callback type for grab function.
/// Return `None` to consume the event, `Some(event)` to let it pass through.
pub type GrabCallback = fn(event: Event) -> Option<Event>;

/// Errors that occur when trying to grab OS events.
#[derive(Debug)]
#[non_exhaustive]
pub enum GrabError {
    /// macOS: Failed to create event tap
    EventTapError,
    /// macOS: Failed to create run loop source
    LoopSourceError,
    /// Linux: Missing X11 display
    MissingDisplayError,
    /// Linux: Missing screen
    MissingScreenError,
    /// Linux: Invalid file descriptor
    InvalidFileDescriptor,
    /// Linux: Keyboard error
    KeyboardError,
    /// Windows: Key hook error
    KeyHookError(u32),
    /// Windows: Mouse hook error
    MouseHookError(u32),
    /// IO error
    IoError(std::io::Error),
    /// Exit grab error
    ExitGrabError(String),
}

/// Key names based on physical location (QWERTY layout)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    /// Alt key on Linux and Windows (Option key on macOS)
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
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
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Home,
    LeftArrow,
    /// Also known as "Windows", "Super", and "Command"
    MetaLeft,
    /// Also known as "Windows", "Super", and "Command"
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KanaMode,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    KpDecimal,
    KpEqual,
    KpComma,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Function,
    Apps,
    Cancel,
    Clear,
    Kana,
    Hangul,
    Junja,
    Final,
    Hanja,
    Hanji,
    Print,
    Select,
    Execute,
    Help,
    Sleep,
    Separator,
    /// Unknown key with platform-specific code
    Unknown(u32),
}

/// Standard mouse buttons
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
    Middle,
    Unknown(u8),
}

/// Event types for keyboard and mouse
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventType {
    /// Key pressed
    KeyPress(Key),
    /// Key released
    KeyRelease(Key),
    /// Mouse button pressed
    ButtonPress(Button),
    /// Mouse button released
    ButtonRelease(Button),
    /// Mouse moved (x, y in pixels)
    MouseMove { x: f64, y: f64 },
    /// Mouse wheel scrolled
    Wheel { delta_x: i64, delta_y: i64 },
}

/// An input event with metadata
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    /// When the event occurred
    pub time: SystemTime,
    /// The type of event
    pub event_type: EventType,
    /// Platform-specific code (keycode/scancode)
    pub platform_code: u32,
}

impl Event {
    /// Create a new event
    pub fn new(event_type: EventType) -> Self {
        Self { time: SystemTime::now(), event_type, platform_code: 0 }
    }

    /// Create a new event with platform code
    pub fn with_code(event_type: EventType, platform_code: u32) -> Self {
        Self { time: SystemTime::now(), event_type, platform_code }
    }
}
