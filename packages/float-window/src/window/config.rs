//! Window configuration types

use crate::animation::WindowAnimation;
use crate::content::Content;
use crate::effect::{PresetEffect, PresetEffectOptions};
use crate::icon::WindowIcon;
use crate::shape::WindowShape;

/// Window position
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Window size
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 200,
            height: 200,
        }
    }
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Window level (z-order)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum WindowLevel {
    /// Normal window level
    #[default]
    Normal,
    /// Above normal windows
    Top,
    /// Always on top of all windows
    AlwaysOnTop,
}

/// Floating window configuration
#[derive(Debug, Clone, Default)]
pub struct WindowConfig {
    /// Unique window ID
    pub id: Option<String>,
    /// Window title (for debugging/accessibility)
    pub title: Option<String>,
    /// Initial position
    pub position: Position,
    /// Initial size
    pub size: Size,
    /// Window shape
    pub shape: WindowShape,
    /// Whether the window is draggable
    pub draggable: bool,
    /// Whether the window is resizable
    pub resizable: bool,
    /// Whether clicks pass through to windows below
    pub click_through: bool,
    /// Window level
    pub level: WindowLevel,
    /// Window opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Window icon
    pub icon: Option<WindowIcon>,
    /// Window content
    pub content: Option<Content>,
    /// Particle effect
    pub effect: Option<(PresetEffect, PresetEffectOptions)>,
    /// Show animation
    pub show_animation: Option<WindowAnimation>,
    /// Hide animation
    pub hide_animation: Option<WindowAnimation>,
}

impl WindowConfig {
    pub fn new() -> Self {
        Self {
            opacity: 1.0,
            ..Default::default()
        }
    }
}
