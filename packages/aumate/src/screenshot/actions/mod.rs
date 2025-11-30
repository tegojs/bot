//! Built-in screenshot actions
//!
//! Provides default implementations of ScreenAction for common operations.

mod annotate;
mod cancel;
mod copy;
mod save;
mod text;

pub use annotate::AnnotateAction;
pub use cancel::CancelAction;
pub use copy::CopyAction;
pub use save::SaveAction;
pub use text::TextAction;
