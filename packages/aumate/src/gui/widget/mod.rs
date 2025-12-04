//! Widget system for building UI components
//!
//! This module provides a declarative widget system that can be rendered via egui.
//! Widgets are defined as data structures (`WidgetDef`) that describe the UI,
//! which are then rendered each frame.
//!
//! # Example
//!
//! ```ignore
//! use aumate::gui::widget::*;
//!
//! let ui = WidgetDef::vbox(vec![
//!     WidgetDef::label("Hello, World!"),
//!     WidgetDef::button("Click me").with_id("btn1"),
//! ]).with_spacing(8.0);
//! ```

mod definition;
mod events;
mod fonts;
mod renderer;
mod style;

pub use definition::*;
pub use events::*;
pub use fonts::*;
pub use renderer::*;
pub use style::*;
