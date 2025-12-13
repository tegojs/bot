use thiserror::Error;

/// 领域层错误
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid rectangle: {0}")]
    InvalidRectangle(String),

    #[error("Invalid dimensions: width={0}, height={1}")]
    InvalidDimensions(u32, u32),

    #[error("Invalid image data")]
    InvalidImageData,

    #[error("Unsupported image format: {0}")]
    UnsupportedImageFormat(String),

    #[error("Region out of bounds")]
    RegionOutOfBounds,

    #[error("No monitors available")]
    NoMonitorsAvailable,

    #[error("Window not found: {0}")]
    WindowNotFound(String),

    #[error("Invalid window state")]
    InvalidWindowState,

    #[error("Domain validation failed: {0}")]
    ValidationFailed(String),
}

/// 基础设施层错误
#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Capture failed: {0}")]
    CaptureFailed(String),

    #[error("Platform operation failed: {0}")]
    PlatformOperationFailed(String),

    #[error("File operation failed: {0}")]
    FileOperationFailed(String),

    #[error("Clipboard operation failed: {0}")]
    ClipboardFailed(String),

    #[error("Image processing failed: {0}")]
    ImageProcessingFailed(String),

    #[error("Platform not supported")]
    PlatformNotSupported,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("IO operation failed: {0}")]
    IoFailed(String),

    #[error("Serialization failed: {0}")]
    SerializationFailed(String),

    #[error("External library error: {0}")]
    ExternalError(String),
}

/// 应用层错误 (Application Error)
pub type ApplicationError = UseCaseError;

/// 用例层错误
#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("Capture failed: {0}")]
    CaptureFailed(String),

    #[error("Processing failed: {0}")]
    ProcessingFailed(String),

    #[error("Encoding failed: {0}")]
    EncodingFailed(String),

    #[error("Save failed: {0}")]
    SaveFailed(String),

    #[error("Clipboard operation failed: {0}")]
    ClipboardFailed(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] InfrastructureError),
}

/// API 层错误 (用于 Tauri Commands)
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),

    #[error("Use case error: {0}")]
    UseCase(#[from] UseCaseError),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] InfrastructureError),
}

// 为 Tauri 实现 Serialize
impl serde::Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
