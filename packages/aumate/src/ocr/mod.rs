//! OCR (Optical Character Recognition) module
//!
//! This module provides OCR functionality using TrOCR (Transformer-based OCR):
//! - Image to text recognition
//! - Support for handwritten and printed text
//! - Model management with download support

#[cfg(feature = "gui")]
mod controller;
mod engine;

#[cfg(feature = "gui")]
pub use controller::OcrFeature;
pub use engine::{OcrEngine, OcrResult, TrOCRModel};

// Re-export shared model types from ml module
pub use crate::ml::{
    DownloadProgress, DownloadStatus, ModelInfo, ModelManager, ModelType,
    get_ml_data_dir as get_ocr_data_dir, get_models_dir, get_trocr_models_dir,
};

/// OCR model variant (maps to TROCR_MODELS in ml/model.rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OcrModelVariant {
    /// Base model for handwritten text
    BaseHandwritten,
    /// Base model for printed text
    #[default]
    BasePrinted,
    /// Large model for handwritten text
    LargeHandwritten,
    /// Large model for printed text
    LargePrinted,
}

impl OcrModelVariant {
    /// Get the model ID (used by ModelManager)
    pub fn model_id(&self) -> &'static str {
        match self {
            Self::BaseHandwritten => "trocr-base-handwritten",
            Self::BasePrinted => "trocr-base-printed",
            Self::LargeHandwritten => "trocr-large-handwritten",
            Self::LargePrinted => "trocr-large-printed",
        }
    }

    /// Get the Hugging Face repository ID for this variant
    pub fn repo_id(&self) -> &'static str {
        match self {
            Self::BaseHandwritten => "microsoft/trocr-base-handwritten",
            Self::BasePrinted => "microsoft/trocr-base-printed",
            Self::LargeHandwritten => "microsoft/trocr-large-handwritten",
            Self::LargePrinted => "microsoft/trocr-large-printed",
        }
    }

    /// Get the model name for display
    pub fn name(&self) -> &'static str {
        match self {
            Self::BaseHandwritten => "TrOCR Base Handwritten",
            Self::BasePrinted => "TrOCR Base Printed",
            Self::LargeHandwritten => "TrOCR Large Handwritten",
            Self::LargePrinted => "TrOCR Large Printed",
        }
    }

    /// Get display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::BaseHandwritten => "Base Handwritten",
            Self::BasePrinted => "Base Printed",
            Self::LargeHandwritten => "Large Handwritten",
            Self::LargePrinted => "Large Printed",
        }
    }

    /// Get description for this variant
    pub fn description(&self) -> &'static str {
        match self {
            Self::BaseHandwritten => "Optimized for handwritten text",
            Self::BasePrinted => "Optimized for printed/digital text",
            Self::LargeHandwritten => "Higher accuracy for handwritten text",
            Self::LargePrinted => "Higher accuracy for printed text",
        }
    }

    /// Get all available variants
    pub fn all() -> &'static [Self] {
        &[Self::BaseHandwritten, Self::BasePrinted, Self::LargeHandwritten, Self::LargePrinted]
    }

    /// Create from model ID
    pub fn from_model_id(id: &str) -> Option<Self> {
        match id {
            "trocr-base-handwritten" => Some(Self::BaseHandwritten),
            "trocr-base-printed" => Some(Self::BasePrinted),
            "trocr-large-handwritten" => Some(Self::LargeHandwritten),
            "trocr-large-printed" => Some(Self::LargePrinted),
            _ => None,
        }
    }
}
