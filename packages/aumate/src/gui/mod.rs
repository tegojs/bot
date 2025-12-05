//! GUI module - Floating window system with particle effects
//!
//! This module provides a complete GUI framework for creating frameless, draggable floating
//! windows with support for shapes, icons, particle effects, and animations.
//!
//! # Example
//!
//! ```no_run
//! use aumate::gui::prelude::*;
//!
//! FloatingWindow::builder()
//!     .size(200, 200)
//!     .position(100.0, 100.0)
//!     .shape(WindowShape::Circle)
//!     .draggable(true)
//!     .always_on_top(true)
//!     .effect(PresetEffect::RotatingHalo, PresetEffectOptions::default())
//!     .build()
//!     .unwrap()
//!     .run()
//!     .unwrap();
//! ```

pub mod animation;
pub mod content;
pub mod controller;
pub mod effect;
pub mod event;
pub mod icon;
pub mod menu_bar;
pub mod render;
pub mod shape;
pub mod util;
pub mod widget;
pub mod window;

use crate::error::{AumateError, Result};

/// Prelude module for convenient imports
pub mod prelude {
    pub use super::animation::{AnimationDirection, AnimationType, Easing, WindowAnimation};
    pub use super::content::{
        Content, ContentRenderer, ImageDisplayOptions, ScaleMode, TextAlign, TextDisplayOptions,
    };
    pub use super::effect::{PresetEffect, PresetEffectOptions};
    pub use super::event::FloatingWindowEvent;
    pub use super::icon::{IconName, WindowIcon};
    pub use super::menu_bar::{
        MenuBarClickAction, MenuBarEvent, MenuBarIcon, MenuBarItem, MenuBarItemBuilder,
        MenuBarManager, MenuBarMenu, MenuBarMenuItem, MenuBarRegistry, PredefinedMenuItemType,
    };
    pub use super::shape::WindowShape;
    pub use super::widget::{
        DialogResult, FileDialogResult, Spacing, TextAlign as WidgetTextAlign, WidgetDef,
        WidgetEvent, WidgetId, WidgetProps, WidgetRenderer, WidgetState, WidgetStateUpdate,
        WidgetStyle,
    };
    pub use super::window::{
        FloatingWindow, FloatingWindowBuilder, FloatingWindowManager, Position, Size, WidgetUpdate,
        WindowConfig, WindowLevel,
    };
}

pub use prelude::*;

/// Run the GUI application with a controller window
pub fn run() -> Result<()> {
    // Initialize logging if not already done
    let _ = env_logger::try_init();

    log::info!("Starting Aumate GUI...");

    // Create controller window (rectangular, with egui UI)
    let controller = FloatingWindow::builder()
        .title("Aumate Controller")
        .size(800, 450)
        .position(100.0, 100.0)
        .shape(WindowShape::Rectangle)
        .draggable(true)
        .always_on_top(true)
        .build()
        .map_err(AumateError::Gui)?;

    // Run as controller - this enables dynamic window creation/management
    FloatingWindow::run_controller(controller).map_err(AumateError::Gui)
}
