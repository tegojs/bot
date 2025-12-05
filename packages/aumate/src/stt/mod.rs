//! Speech-to-Text (STT) module
//!
//! This module provides speech-to-text functionality with:
//! - Global hotkey support for push-to-talk and toggle modes
//! - Audio capture via cpal
//! - Voice Activity Detection (VAD) for auto-stop on silence
//! - Whisper engine for transcription (Candle-based)
//! - Output to keystrokes or clipboard
//! - Model management with download support

mod audio;
mod config;
mod controller;
mod engine;
mod hotkey;
mod model;
mod output;
mod vad;

pub use audio::{AudioData, AudioRecorder};
pub use config::{HotkeyConfig, HotkeyMode, OutputMode, SttConfig};
pub use controller::SttFeature;
pub use engine::{TranscriptionResult, WhisperEngine};
pub use hotkey::{HotkeyEvent, HotkeyManager};
pub use output::OutputHandler;
pub use vad::VoiceActivityDetector;

// Re-export model types - use local model.rs for backward compatibility,
// but also expose shared types from ml module
pub use model::{DownloadProgress, DownloadStatus, ModelInfo, ModelManager};

// Re-export shared model types from ml module for new code
pub use crate::ml::{
    ModelManager as SharedModelManager, ModelType, get_ml_data_dir as get_stt_data_dir,
    get_models_dir, get_whisper_models_dir,
};

use crate::error::Result;
use std::sync::{Arc, Mutex};

/// Main STT controller that orchestrates all components
#[allow(dead_code)]
pub struct SttController {
    config: SttConfig,
    audio_recorder: Option<AudioRecorder>,
    engine: Option<WhisperEngine>,
    model_manager: ModelManager,
    hotkey_manager: Option<HotkeyManager>,
    vad: Option<VoiceActivityDetector>,
    output_handler: Option<OutputHandler>,
    is_recording: Arc<Mutex<bool>>,
    last_transcription: Arc<Mutex<Option<String>>>,
}

impl SttController {
    /// Create a new STT controller
    pub fn new() -> Result<Self> {
        let config = SttConfig::load().unwrap_or_default();
        let model_manager = ModelManager::new()?;

        Ok(Self {
            config,
            audio_recorder: None,
            engine: None,
            model_manager,
            hotkey_manager: None,
            vad: None,
            output_handler: None,
            is_recording: Arc::new(Mutex::new(false)),
            last_transcription: Arc::new(Mutex::new(None)),
        })
    }

    /// Get the current configuration
    pub fn config(&self) -> &SttConfig {
        &self.config
    }

    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut SttConfig {
        &mut self.config
    }

    /// Get the model manager
    pub fn model_manager(&self) -> &ModelManager {
        &self.model_manager
    }

    /// Get mutable model manager
    pub fn model_manager_mut(&mut self) -> &mut ModelManager {
        &mut self.model_manager
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        *self.is_recording.lock().unwrap()
    }

    /// Get the last transcription result
    pub fn last_transcription(&self) -> Option<String> {
        self.last_transcription.lock().unwrap().clone()
    }

    /// Initialize the STT system with the current configuration
    pub fn initialize(&mut self) -> Result<()> {
        // Initialize audio recorder
        self.audio_recorder = Some(AudioRecorder::new()?);

        // Initialize output handler
        self.output_handler = Some(OutputHandler::new(self.config.output_mode)?);

        // Try to load the selected model if available
        if let Some(model_path) = self.model_manager.get_model_path(&self.config.model_id) {
            let mut engine = WhisperEngine::new();
            if engine.load_model(&model_path).is_ok() {
                self.engine = Some(engine);
            }
        }

        // Initialize VAD if enabled
        if self.config.vad_enabled {
            if let Some(vad_path) = self.model_manager.get_vad_model_path() {
                if let Ok(vad) = VoiceActivityDetector::new(&vad_path) {
                    self.vad = Some(vad);
                }
            }
        }

        Ok(())
    }

    /// Start recording audio
    pub fn start_recording(&mut self) -> Result<()> {
        if let Some(ref mut recorder) = self.audio_recorder {
            recorder.start_recording()?;
            *self.is_recording.lock().unwrap() = true;
        }
        Ok(())
    }

    /// Stop recording and transcribe
    pub fn stop_recording(&mut self) -> Result<Option<String>> {
        *self.is_recording.lock().unwrap() = false;

        let audio_data = if let Some(ref mut recorder) = self.audio_recorder {
            recorder.stop_recording()?
        } else {
            return Ok(None);
        };

        // Transcribe the audio
        let transcription = if let Some(ref mut engine) = self.engine {
            let result = engine.transcribe(&audio_data)?;
            Some(result.text)
        } else {
            None
        };

        // Store the transcription
        if let Some(ref text) = transcription {
            *self.last_transcription.lock().unwrap() = Some(text.clone());

            // Output the transcription
            if let Some(ref handler) = self.output_handler {
                handler.output(text)?;
            }
        }

        Ok(transcription)
    }

    /// Save the current configuration
    pub fn save_config(&self) -> Result<()> {
        self.config.save()
    }
}

impl Default for SttController {
    fn default() -> Self {
        Self::new().expect("Failed to create STT controller")
    }
}
