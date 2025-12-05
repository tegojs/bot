//! Unified error types for aumate

use thiserror::Error;

/// Main error type for aumate operations
#[derive(Error, Debug)]
pub enum AumateError {
    /// Input-related errors (mouse, keyboard)
    #[error("Input error: {0}")]
    Input(String),

    /// Screen capture errors
    #[error("Screen error: {0}")]
    Screen(String),

    /// Clipboard errors
    #[error("Clipboard error: {0}")]
    Clipboard(String),

    /// Window management errors
    #[error("Window error: {0}")]
    Window(String),

    /// Screenshot errors
    #[error("Screenshot error: {0}")]
    Screenshot(String),

    /// GUI errors
    #[error("GUI error: {0}")]
    Gui(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Image processing errors
    #[cfg(feature = "screen")]
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    /// ML/Candle errors
    #[cfg(feature = "ml")]
    #[error("ML error: {0}")]
    Ml(String),

    /// Generic errors
    #[error("{0}")]
    Other(String),
}

#[cfg(feature = "ml")]
impl From<candle_core::Error> for AumateError {
    fn from(e: candle_core::Error) -> Self {
        AumateError::Ml(e.to_string())
    }
}

/// Result type alias for aumate operations
pub type Result<T> = std::result::Result<T, AumateError>;
