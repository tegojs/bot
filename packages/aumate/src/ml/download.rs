//! Model downloading utilities
//!
//! Provides functionality to download models from Hugging Face Hub.

#![allow(dead_code)]

use crate::error::{AumateError, Result};
use hf_hub::api::tokio::Api;
use std::path::PathBuf;

/// Source configuration for model downloads
#[derive(Debug, Clone)]
pub struct ModelSource {
    /// Hugging Face repository ID (e.g., "openai/whisper-tiny")
    pub repo_id: String,
    /// Specific revision (branch, tag, or commit hash)
    pub revision: Option<String>,
    /// Files to download
    pub files: Vec<String>,
}

impl ModelSource {
    /// Create a new model source
    pub fn new(repo_id: impl Into<String>) -> Self {
        Self { repo_id: repo_id.into(), revision: None, files: Vec::new() }
    }

    /// Set the revision
    pub fn with_revision(mut self, revision: impl Into<String>) -> Self {
        self.revision = Some(revision.into());
        self
    }

    /// Add a file to download
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.files.push(file.into());
        self
    }

    /// Add multiple files to download
    pub fn with_files(mut self, files: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.files.extend(files.into_iter().map(|f| f.into()));
        self
    }
}

/// Model downloader using Hugging Face Hub
pub struct ModelDownloader {
    api: Api,
}

impl ModelDownloader {
    /// Create a new model downloader
    pub async fn new() -> Result<Self> {
        let api = Api::new()
            .map_err(|e| AumateError::Other(format!("Failed to create HF Hub API: {}", e)))?;
        Ok(Self { api })
    }

    /// Download a single file from a repository
    pub async fn download_file(
        &self,
        repo_id: &str,
        filename: &str,
        revision: Option<&str>,
    ) -> Result<PathBuf> {
        let repo = if let Some(rev) = revision {
            self.api.repo(hf_hub::Repo::with_revision(
                repo_id.to_string(),
                hf_hub::RepoType::Model,
                rev.to_string(),
            ))
        } else {
            self.api.model(repo_id.to_string())
        };

        log::info!("Downloading {} from {}", filename, repo_id);

        let path = repo
            .get(filename)
            .await
            .map_err(|e| AumateError::Other(format!("Failed to download {}: {}", filename, e)))?;

        log::info!("Downloaded to: {:?}", path);
        Ok(path)
    }

    /// Download all files from a model source
    pub async fn download_source(&self, source: &ModelSource) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();

        for file in &source.files {
            let path =
                self.download_file(&source.repo_id, file, source.revision.as_deref()).await?;
            paths.push(path);
        }

        Ok(paths)
    }

    /// Download model weights file
    pub async fn download_weights(&self, repo_id: &str) -> Result<PathBuf> {
        self.download_file(repo_id, "model.safetensors", None).await
    }

    /// Download tokenizer file
    pub async fn download_tokenizer(&self, repo_id: &str) -> Result<PathBuf> {
        self.download_file(repo_id, "tokenizer.json", None).await
    }

    /// Download config file
    pub async fn download_config(&self, repo_id: &str) -> Result<PathBuf> {
        self.download_file(repo_id, "config.json", None).await
    }
}

/// Convenience function to download a model
pub async fn download_model(source: &ModelSource) -> Result<Vec<PathBuf>> {
    let downloader = ModelDownloader::new().await?;
    downloader.download_source(source).await
}

/// Common Whisper model sources
pub mod whisper {
    use super::ModelSource;

    pub fn tiny() -> ModelSource {
        ModelSource::new("openai/whisper-tiny").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn tiny_en() -> ModelSource {
        ModelSource::new("openai/whisper-tiny.en").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn base() -> ModelSource {
        ModelSource::new("openai/whisper-base").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn base_en() -> ModelSource {
        ModelSource::new("openai/whisper-base.en").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn small() -> ModelSource {
        ModelSource::new("openai/whisper-small").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn small_en() -> ModelSource {
        ModelSource::new("openai/whisper-small.en").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn medium() -> ModelSource {
        ModelSource::new("openai/whisper-medium").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn medium_en() -> ModelSource {
        ModelSource::new("openai/whisper-medium.en").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn large_v3() -> ModelSource {
        ModelSource::new("openai/whisper-large-v3").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }
}

/// Common TrOCR model sources
pub mod trocr {
    use super::ModelSource;

    pub fn base_handwritten() -> ModelSource {
        ModelSource::new("microsoft/trocr-base-handwritten").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn base_printed() -> ModelSource {
        ModelSource::new("microsoft/trocr-base-printed").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn large_handwritten() -> ModelSource {
        ModelSource::new("microsoft/trocr-large-handwritten").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }

    pub fn large_printed() -> ModelSource {
        ModelSource::new("microsoft/trocr-large-printed").with_files([
            "model.safetensors",
            "tokenizer.json",
            "config.json",
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_source_builder() {
        let source = ModelSource::new("test/repo")
            .with_revision("main")
            .with_file("model.safetensors")
            .with_files(["tokenizer.json", "config.json"]);

        assert_eq!(source.repo_id, "test/repo");
        assert_eq!(source.revision, Some("main".to_string()));
        assert_eq!(source.files.len(), 3);
    }

    #[test]
    fn test_whisper_sources() {
        let tiny = whisper::tiny();
        assert_eq!(tiny.repo_id, "openai/whisper-tiny");
        assert_eq!(tiny.files.len(), 3);
    }

    #[test]
    fn test_trocr_sources() {
        let base = trocr::base_printed();
        assert_eq!(base.repo_id, "microsoft/trocr-base-printed");
        assert_eq!(base.files.len(), 3);
    }
}
