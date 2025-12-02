//! Built-in screenshot actions
//!
//! Provides default implementations of ScreenAction for common operations.

mod annotate;
mod arrow;
mod blur;
mod cancel;
mod copy;
mod ellipse;
mod eraser;
mod highlighter;
mod mosaic;
mod polyline;
mod rectangle;
mod redo;
mod save;
mod sequence;
mod text;
mod undo;

pub use annotate::AnnotateAction;
pub use arrow::ArrowAction;
pub use blur::BlurAction;
pub use cancel::CancelAction;
pub use copy::CopyAction;
pub use ellipse::EllipseAction;
pub use eraser::EraserAction;
pub use highlighter::HighlighterAction;
pub use mosaic::MosaicAction;
pub use polyline::PolylineAction;
pub use rectangle::RectangleAction;
pub use redo::RedoAction;
pub use save::SaveAction;
pub use sequence::SequenceAction;
pub use text::TextAction;
pub use undo::UndoAction;
