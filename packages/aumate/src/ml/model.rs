//! Shared model management for ML modules
//!
//! Provides model downloading, storage, and management for Whisper (STT) and TrOCR (OCR).

use crate::error::{AumateError, Result};
use futures_util::StreamExt;
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Model type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelType {
    /// Whisper speech-to-text model
    Whisper,
    /// TrOCR optical character recognition model
    TrOCR,
    /// Silero VAD model
    Vad,
}

impl ModelType {
    /// Get the subdirectory name for this model type
    pub fn subdir(&self) -> &'static str {
        match self {
            Self::Whisper => "whisper",
            Self::TrOCR => "trocr",
            Self::Vad => "vad",
        }
    }
}

/// Information about a model
#[derive(Debug, Clone)]
pub struct ModelInfo {
    /// Model identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Model type
    pub model_type: ModelType,
    /// Size in bytes (approximate)
    pub size_bytes: u64,
    /// HuggingFace repository ID (e.g., "openai/whisper-tiny")
    pub repo_id: String,
    /// Files to download
    pub files: Vec<String>,
    /// Whether the model is downloaded
    pub is_downloaded: bool,
    /// Local directory path if downloaded
    pub local_path: Option<PathBuf>,
}

impl ModelInfo {
    /// Get human-readable size string
    pub fn size_display(&self) -> String {
        if self.size_bytes >= 1_000_000_000 {
            format!("{:.1} GB", self.size_bytes as f64 / 1_000_000_000.0)
        } else if self.size_bytes >= 1_000_000 {
            format!("{} MB", self.size_bytes / 1_000_000)
        } else {
            format!("{} KB", self.size_bytes / 1_000)
        }
    }
}

/// Download progress information
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    /// Model being downloaded
    pub model_id: String,
    /// Current file being downloaded
    pub current_file: String,
    /// Current file index (1-based)
    pub file_index: usize,
    /// Total number of files
    pub total_files: usize,
    /// Bytes downloaded for current file
    pub downloaded_bytes: u64,
    /// Total bytes for current file
    pub total_bytes: u64,
    /// Current status
    pub status: DownloadStatus,
}

impl DownloadProgress {
    /// Get progress as a percentage (0.0 - 1.0) for current file
    pub fn file_progress(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            self.downloaded_bytes as f32 / self.total_bytes as f32
        }
    }

    /// Get overall progress as a percentage (0.0 - 1.0)
    pub fn overall_progress(&self) -> f32 {
        if self.total_files == 0 {
            0.0
        } else {
            let completed = (self.file_index - 1) as f32 / self.total_files as f32;
            let current = self.file_progress() / self.total_files as f32;
            completed + current
        }
    }

    /// Get progress as a percentage string
    pub fn progress_percent(&self) -> String {
        format!("{:.1}%", self.overall_progress() * 100.0)
    }
}

/// Download status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    /// Download not started
    Pending,
    /// Currently downloading
    Downloading,
    /// Download completed
    Completed,
    /// Download failed with error message
    Failed(String),
}

// ==================== Whisper Models ====================

/// Available Whisper models (Candle format from HuggingFace)
pub const WHISPER_MODELS: &[(&str, &str, &str, u64)] = &[
    ("whisper-tiny", "Whisper Tiny", "openai/whisper-tiny", 75_000_000),
    ("whisper-tiny.en", "Whisper Tiny (English)", "openai/whisper-tiny.en", 75_000_000),
    ("whisper-base", "Whisper Base", "openai/whisper-base", 142_000_000),
    ("whisper-base.en", "Whisper Base (English)", "openai/whisper-base.en", 142_000_000),
    ("whisper-small", "Whisper Small", "openai/whisper-small", 466_000_000),
    ("whisper-small.en", "Whisper Small (English)", "openai/whisper-small.en", 466_000_000),
    ("whisper-medium", "Whisper Medium", "openai/whisper-medium", 1_500_000_000),
    ("whisper-medium.en", "Whisper Medium (English)", "openai/whisper-medium.en", 1_500_000_000),
    ("whisper-large-v3", "Whisper Large v3", "openai/whisper-large-v3", 3_000_000_000),
];

/// Standard files for Whisper models
pub const WHISPER_FILES: &[&str] = &["model.safetensors", "config.json", "tokenizer.json"];

// ==================== TrOCR Models ====================

/// Available TrOCR models
pub const TROCR_MODELS: &[(&str, &str, &str, &str, u64)] = &[
    (
        "trocr-base-handwritten",
        "TrOCR Base Handwritten",
        "Optimized for handwritten text",
        "microsoft/trocr-base-handwritten",
        350_000_000,
    ),
    (
        "trocr-base-printed",
        "TrOCR Base Printed",
        "Optimized for printed/digital text",
        "microsoft/trocr-base-printed",
        350_000_000,
    ),
    (
        "trocr-large-handwritten",
        "TrOCR Large Handwritten",
        "Higher accuracy for handwritten text",
        "microsoft/trocr-large-handwritten",
        1_400_000_000,
    ),
    (
        "trocr-large-printed",
        "TrOCR Large Printed",
        "Higher accuracy for printed text",
        "microsoft/trocr-large-printed",
        1_400_000_000,
    ),
];

/// Standard files for TrOCR models
/// TrOCR uses RoBERTa tokenizer with vocab.json + merges.txt instead of tokenizer.json
pub const TROCR_FILES: &[&str] = &[
    "model.safetensors",
    "config.json",
    "vocab.json",
    "merges.txt",
    "tokenizer_config.json",
    "preprocessor_config.json",
];

// ==================== VAD Model ====================

/// Silero VAD model info
pub const VAD_MODEL_URL: &str =
    "https://github.com/snakers4/silero-vad/raw/master/src/silero_vad/data/silero_vad.onnx";
pub const VAD_MODEL_ID: &str = "silero-vad";
pub const VAD_MODEL_SIZE: u64 = 2_000_000;

// ==================== Model Manager ====================

/// Unified model manager for downloading and managing ML models
pub struct ModelManager {
    /// Root models directory
    models_dir: PathBuf,
    /// Current downloads in progress
    downloads: Arc<Mutex<HashMap<String, DownloadProgress>>>,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new() -> Result<Self> {
        let models_dir = super::get_models_dir()?;
        Ok(Self { models_dir, downloads: Arc::new(Mutex::new(HashMap::new())) })
    }

    /// Get the root models directory
    pub fn models_dir(&self) -> &Path {
        &self.models_dir
    }

    /// Get directory for a specific model type
    pub fn type_dir(&self, model_type: ModelType) -> PathBuf {
        self.models_dir.join(model_type.subdir())
    }

    /// Get directory for a specific model
    pub fn model_dir(&self, model_type: ModelType, model_id: &str) -> PathBuf {
        self.type_dir(model_type).join(model_id)
    }

    /// Check if a model is downloaded
    pub fn is_downloaded(&self, model_type: ModelType, model_id: &str) -> bool {
        let dir = self.model_dir(model_type, model_id);
        if !dir.exists() {
            return false;
        }

        // Check if all required files exist
        let files = match model_type {
            ModelType::Whisper => WHISPER_FILES,
            ModelType::TrOCR => TROCR_FILES,
            ModelType::Vad => return dir.join("silero_vad.onnx").exists(),
        };

        files.iter().all(|f| dir.join(f).exists())
    }

    /// List all available Whisper models
    pub fn list_whisper_models(&self) -> Vec<ModelInfo> {
        WHISPER_MODELS
            .iter()
            .map(|(id, name, repo_id, size)| {
                let is_downloaded = self.is_downloaded(ModelType::Whisper, id);
                let local_path =
                    if is_downloaded { Some(self.model_dir(ModelType::Whisper, id)) } else { None };
                ModelInfo {
                    id: id.to_string(),
                    name: name.to_string(),
                    description: format!("~{}", format_size(*size)),
                    model_type: ModelType::Whisper,
                    size_bytes: *size,
                    repo_id: repo_id.to_string(),
                    files: WHISPER_FILES.iter().map(|s| s.to_string()).collect(),
                    is_downloaded,
                    local_path,
                }
            })
            .collect()
    }

    /// List all available TrOCR models
    pub fn list_trocr_models(&self) -> Vec<ModelInfo> {
        TROCR_MODELS
            .iter()
            .map(|(id, name, desc, repo_id, size)| {
                let is_downloaded = self.is_downloaded(ModelType::TrOCR, id);
                let local_path =
                    if is_downloaded { Some(self.model_dir(ModelType::TrOCR, id)) } else { None };
                ModelInfo {
                    id: id.to_string(),
                    name: name.to_string(),
                    description: desc.to_string(),
                    model_type: ModelType::TrOCR,
                    size_bytes: *size,
                    repo_id: repo_id.to_string(),
                    files: TROCR_FILES.iter().map(|s| s.to_string()).collect(),
                    is_downloaded,
                    local_path,
                }
            })
            .collect()
    }

    /// List all available models of a specific type
    pub fn list_models(&self, model_type: ModelType) -> Vec<ModelInfo> {
        match model_type {
            ModelType::Whisper => self.list_whisper_models(),
            ModelType::TrOCR => self.list_trocr_models(),
            ModelType::Vad => vec![self.get_vad_model_info()],
        }
    }

    /// Get VAD model info
    pub fn get_vad_model_info(&self) -> ModelInfo {
        let is_downloaded = self.is_downloaded(ModelType::Vad, VAD_MODEL_ID);
        let local_path =
            if is_downloaded { Some(self.model_dir(ModelType::Vad, VAD_MODEL_ID)) } else { None };
        ModelInfo {
            id: VAD_MODEL_ID.to_string(),
            name: "Silero VAD".to_string(),
            description: "Voice Activity Detection".to_string(),
            model_type: ModelType::Vad,
            size_bytes: VAD_MODEL_SIZE,
            repo_id: String::new(),
            files: vec!["silero_vad.onnx".to_string()],
            is_downloaded,
            local_path,
        }
    }

    /// Get the path to a downloaded model
    pub fn get_model_path(&self, model_type: ModelType, model_id: &str) -> Option<PathBuf> {
        let path = self.model_dir(model_type, model_id);
        if self.is_downloaded(model_type, model_id) { Some(path) } else { None }
    }

    /// Get download progress for a model
    pub fn get_download_progress(&self, model_id: &str) -> Option<DownloadProgress> {
        self.downloads.lock().unwrap().get(model_id).cloned()
    }

    /// Download a model (blocking)
    pub fn download_model_sync(
        &self,
        model_type: ModelType,
        model_id: &str,
        progress_callback: Option<Box<dyn Fn(DownloadProgress) + Send>>,
    ) -> Result<PathBuf> {
        // Special handling for VAD model (direct URL download)
        if model_type == ModelType::Vad {
            return self.download_vad_model(progress_callback);
        }

        // Find the model info
        let model_info = self
            .list_models(model_type)
            .into_iter()
            .find(|m| m.id == model_id)
            .ok_or_else(|| AumateError::Other(format!("Unknown model: {}", model_id)))?;

        // Create output directory
        let output_dir = self.model_dir(model_type, model_id);
        std::fs::create_dir_all(&output_dir)?;

        let files = &model_info.files;
        let total_files = files.len();

        // Initialize progress
        let progress = DownloadProgress {
            model_id: model_id.to_string(),
            current_file: String::new(),
            file_index: 0,
            total_files,
            downloaded_bytes: 0,
            total_bytes: 0,
            status: DownloadStatus::Pending,
        };
        self.downloads.lock().unwrap().insert(model_id.to_string(), progress);

        // Create a tokio runtime for async downloads
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| AumateError::Other(format!("Failed to create runtime: {}", e)))?;

        let repo_id = model_info.repo_id.clone();
        let downloads = self.downloads.clone();
        let model_id_owned = model_id.to_string();

        let result: Result<PathBuf> = rt.block_on(async {
            // Use hf-hub to download files
            let api = hf_hub::api::tokio::Api::new()
                .map_err(|e| AumateError::Other(format!("Failed to create HF API: {}", e)))?;

            let repo = api.model(repo_id.clone());

            for (idx, filename) in files.iter().enumerate() {
                let file_idx = idx + 1;

                // Update progress for new file
                {
                    let mut downloads = downloads.lock().unwrap();
                    if let Some(p) = downloads.get_mut(&model_id_owned) {
                        p.current_file = filename.clone();
                        p.file_index = file_idx;
                        p.downloaded_bytes = 0;
                        p.total_bytes = 0;
                        p.status = DownloadStatus::Downloading;
                    }
                }

                if let Some(ref callback) = progress_callback {
                    callback(DownloadProgress {
                        model_id: model_id_owned.clone(),
                        current_file: filename.clone(),
                        file_index: file_idx,
                        total_files,
                        downloaded_bytes: 0,
                        total_bytes: 0,
                        status: DownloadStatus::Downloading,
                    });
                }

                log::info!(
                    "Downloading {}/{}: {} from {}",
                    file_idx,
                    total_files,
                    filename,
                    repo_id
                );

                // Download file using hf-hub (it handles caching)
                let cached_path = repo.get(filename).await.map_err(|e| {
                    AumateError::Other(format!("Failed to download {}: {}", filename, e))
                })?;

                // Copy to our model directory
                let dest_path = output_dir.join(filename);
                std::fs::copy(&cached_path, &dest_path).map_err(|e| {
                    AumateError::Other(format!("Failed to copy {}: {}", filename, e))
                })?;

                log::info!("Downloaded: {} -> {:?}", filename, dest_path);

                // Update progress
                if let Some(ref callback) = progress_callback {
                    callback(DownloadProgress {
                        model_id: model_id_owned.clone(),
                        current_file: filename.clone(),
                        file_index: file_idx,
                        total_files,
                        downloaded_bytes: 1,
                        total_bytes: 1,
                        status: DownloadStatus::Downloading,
                    });
                }
            }

            // Mark as completed
            {
                let mut downloads = downloads.lock().unwrap();
                if let Some(p) = downloads.get_mut(&model_id_owned) {
                    p.status = DownloadStatus::Completed;
                }
            }

            if let Some(ref callback) = progress_callback {
                callback(DownloadProgress {
                    model_id: model_id_owned.clone(),
                    current_file: String::new(),
                    file_index: total_files,
                    total_files,
                    downloaded_bytes: 1,
                    total_bytes: 1,
                    status: DownloadStatus::Completed,
                });
            }

            Ok(output_dir.clone())
        });

        // Handle error
        if let Err(ref e) = result {
            let mut downloads = self.downloads.lock().unwrap();
            if let Some(p) = downloads.get_mut(model_id) {
                p.status = DownloadStatus::Failed(e.to_string());
            }
        }

        result
    }

    /// Download VAD model (special case - direct URL)
    fn download_vad_model(
        &self,
        progress_callback: Option<Box<dyn Fn(DownloadProgress) + Send>>,
    ) -> Result<PathBuf> {
        let output_dir = self.model_dir(ModelType::Vad, VAD_MODEL_ID);
        std::fs::create_dir_all(&output_dir)?;

        let output_path = output_dir.join("silero_vad.onnx");
        let temp_path = output_dir.join("silero_vad.onnx.tmp");

        // Initialize progress
        let progress = DownloadProgress {
            model_id: VAD_MODEL_ID.to_string(),
            current_file: "silero_vad.onnx".to_string(),
            file_index: 1,
            total_files: 1,
            downloaded_bytes: 0,
            total_bytes: VAD_MODEL_SIZE,
            status: DownloadStatus::Pending,
        };
        self.downloads.lock().unwrap().insert(VAD_MODEL_ID.to_string(), progress);

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| AumateError::Other(format!("Failed to create runtime: {}", e)))?;

        let downloads = self.downloads.clone();

        let result = rt.block_on(async {
            let client = reqwest::Client::new();
            let response = client
                .get(VAD_MODEL_URL)
                .send()
                .await
                .map_err(|e| AumateError::Other(format!("Download failed: {}", e)))?;

            if !response.status().is_success() {
                return Err(AumateError::Other(format!(
                    "Download failed with status: {}",
                    response.status()
                )));
            }

            let total_size = response.content_length().unwrap_or(VAD_MODEL_SIZE);

            {
                let mut downloads = downloads.lock().unwrap();
                if let Some(p) = downloads.get_mut(VAD_MODEL_ID) {
                    p.total_bytes = total_size;
                    p.status = DownloadStatus::Downloading;
                }
            }

            let mut file = std::fs::File::create(&temp_path)
                .map_err(|e| AumateError::Other(format!("Failed to create file: {}", e)))?;

            let mut downloaded: u64 = 0;
            let mut stream = response.bytes_stream();

            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result
                    .map_err(|e| AumateError::Other(format!("Download error: {}", e)))?;

                file.write_all(&chunk)
                    .map_err(|e| AumateError::Other(format!("Write error: {}", e)))?;

                downloaded += chunk.len() as u64;

                {
                    let mut downloads = downloads.lock().unwrap();
                    if let Some(p) = downloads.get_mut(VAD_MODEL_ID) {
                        p.downloaded_bytes = downloaded;
                    }
                }

                if let Some(ref callback) = progress_callback {
                    callback(DownloadProgress {
                        model_id: VAD_MODEL_ID.to_string(),
                        current_file: "silero_vad.onnx".to_string(),
                        file_index: 1,
                        total_files: 1,
                        downloaded_bytes: downloaded,
                        total_bytes: total_size,
                        status: DownloadStatus::Downloading,
                    });
                }
            }

            std::fs::rename(&temp_path, &output_path)
                .map_err(|e| AumateError::Other(format!("Failed to rename file: {}", e)))?;

            {
                let mut downloads = downloads.lock().unwrap();
                if let Some(p) = downloads.get_mut(VAD_MODEL_ID) {
                    p.status = DownloadStatus::Completed;
                }
            }

            Ok(output_dir.clone())
        });

        if let Err(ref e) = result {
            let mut downloads = self.downloads.lock().unwrap();
            if let Some(p) = downloads.get_mut(VAD_MODEL_ID) {
                p.status = DownloadStatus::Failed(e.to_string());
            }
        }

        result
    }

    /// Delete a downloaded model
    pub fn delete_model(&self, model_type: ModelType, model_id: &str) -> Result<()> {
        let path = self.model_dir(model_type, model_id);
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        Ok(())
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new().expect("Failed to create model manager")
    }
}

/// Format bytes as human-readable size
fn format_size(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{} MB", bytes / 1_000_000)
    } else {
        format!("{} KB", bytes / 1_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_size_display() {
        let model = ModelInfo {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test model".to_string(),
            model_type: ModelType::Whisper,
            size_bytes: 142_000_000,
            repo_id: "test/model".to_string(),
            files: vec![],
            is_downloaded: false,
            local_path: None,
        };
        assert_eq!(model.size_display(), "142 MB");
    }

    #[test]
    fn test_download_progress() {
        let progress = DownloadProgress {
            model_id: "test".to_string(),
            current_file: "model.safetensors".to_string(),
            file_index: 2,
            total_files: 3,
            downloaded_bytes: 50,
            total_bytes: 100,
            status: DownloadStatus::Downloading,
        };
        assert_eq!(progress.file_progress(), 0.5);
        // Overall: 1/3 completed + 0.5/3 current = 0.5
        assert!((progress.overall_progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_model_type_subdir() {
        assert_eq!(ModelType::Whisper.subdir(), "whisper");
        assert_eq!(ModelType::TrOCR.subdir(), "trocr");
        assert_eq!(ModelType::Vad.subdir(), "vad");
    }
}
