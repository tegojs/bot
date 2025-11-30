//! Float Window - A floating window library for Rust
//!
//! This library provides a simple API for creating frameless, draggable floating windows
//! with support for shapes, icons, particle effects, and animations.
//!
//! # Example
//!
//! ```no_run
//! use float_window::prelude::*;
//!
//! fn main() {
//!     FloatingWindow::builder()
//!         .size(200, 200)
//!         .position(100.0, 100.0)
//!         .shape(WindowShape::Circle)
//!         .draggable(true)
//!         .always_on_top(true)
//!         .effect(PresetEffect::RotatingHalo, PresetEffectOptions::default())
//!         .build()
//!         .unwrap()
//!         .run()
//!         .unwrap();
//! }
//! ```

pub mod animation;
pub mod content;
pub mod effect;
pub mod event;
pub mod icon;
pub mod menu_bar;
pub mod render;
pub mod shape;
pub mod util;
pub mod window;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::animation::{AnimationDirection, AnimationType, Easing, WindowAnimation};
    pub use crate::content::{Content, ContentRenderer, ImageDisplayOptions, ScaleMode, TextAlign, TextDisplayOptions};
    pub use crate::effect::{PresetEffect, PresetEffectOptions};
    pub use crate::event::FloatingWindowEvent;
    pub use crate::icon::{IconName, WindowIcon};
    pub use crate::menu_bar::{
        MenuBarClickAction, MenuBarEvent, MenuBarIcon, MenuBarItem, MenuBarItemBuilder,
        MenuBarManager, MenuBarMenu, MenuBarMenuItem, MenuBarRegistry, PredefinedMenuItemType,
    };
    pub use crate::shape::WindowShape;
    pub use crate::window::{
        FloatingWindow, FloatingWindowBuilder, FloatingWindowManager, Position, Size, WindowConfig,
        WindowLevel,
    };
}

pub use prelude::*;
