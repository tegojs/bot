//! Event system

mod handler;

pub use handler::EventHandler;

/// Floating window event types
#[derive(Debug, Clone)]
pub enum FloatingWindowEvent {
    /// Window was shown
    Show,
    /// Window was hidden
    Hide,
    /// Window was closed
    Close,
    /// Window was moved
    Move { x: f64, y: f64 },
    /// Window was resized
    Resize { width: u32, height: u32 },
    /// Window was clicked
    Click { x: f32, y: f32 },
    /// Drag started
    DragStart { x: f32, y: f32 },
    /// Dragging
    Drag { x: f32, y: f32 },
    /// Drag ended
    DragEnd { x: f32, y: f32 },
    /// Mouse entered window
    MouseEnter,
    /// Mouse left window
    MouseLeave,
    /// Mouse moved within window
    MouseMove { x: f32, y: f32 },
}

/// Event callback type
pub type EventCallback = Box<dyn Fn(&FloatingWindowEvent) + Send + Sync>;
