//! Machine Learning module
//!
//! This module provides shared ML utilities for STT (Whisper) and OCR (TrOCR):
//! - Device management (CPU, CUDA, Metal)
//! - Model downloading from Hugging Face Hub
//! - Model management (download, storage, listing)
//! - Autoregressive text decoding

mod device;
mod download;
mod model;
mod text_decoder;

pub use device::{Device, DeviceConfig, device_name, get_device, is_gpu_available};
pub use download::{ModelDownloader, ModelSource, download_model};
pub use model::{
    DownloadProgress, DownloadStatus, ModelInfo, ModelManager, ModelType, TROCR_FILES,
    TROCR_MODELS, VAD_MODEL_ID, VAD_MODEL_SIZE, VAD_MODEL_URL, WHISPER_FILES, WHISPER_MODELS,
};
pub use text_decoder::{DecodingConfig, TextDecoder};

use crate::error::Result;
use std::path::PathBuf;

/// Get the default ML data directory (~/.aumate/)
pub fn get_ml_data_dir() -> Result<PathBuf> {
    let home = dirs_path()
        .ok_or_else(|| crate::error::AumateError::Other("Could not find home directory".into()))?;
    let data_dir = home.join(".aumate");
    std::fs::create_dir_all(&data_dir)?;
    Ok(data_dir)
}

/// Get the models directory (~/.aumate/models/)
pub fn get_models_dir() -> Result<PathBuf> {
    let data_dir = get_ml_data_dir()?;
    let models_dir = data_dir.join("models");
    std::fs::create_dir_all(&models_dir)?;
    Ok(models_dir)
}

/// Get the Whisper models directory (~/.aumate/models/whisper/)
pub fn get_whisper_models_dir() -> Result<PathBuf> {
    let models_dir = get_models_dir()?;
    let whisper_dir = models_dir.join("whisper");
    std::fs::create_dir_all(&whisper_dir)?;
    Ok(whisper_dir)
}

/// Get the TrOCR models directory (~/.aumate/models/trocr/)
pub fn get_trocr_models_dir() -> Result<PathBuf> {
    let models_dir = get_models_dir()?;
    let trocr_dir = models_dir.join("trocr");
    std::fs::create_dir_all(&trocr_dir)?;
    Ok(trocr_dir)
}

/// Get the VAD models directory (~/.aumate/models/vad/)
pub fn get_vad_models_dir() -> Result<PathBuf> {
    let models_dir = get_models_dir()?;
    let vad_dir = models_dir.join("vad");
    std::fs::create_dir_all(&vad_dir)?;
    Ok(vad_dir)
}

fn dirs_path() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        std::env::var_os("HOME").map(PathBuf::from)
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var_os("USERPROFILE").map(PathBuf::from)
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}
