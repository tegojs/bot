//! Whisper transcription engine (Candle-based)
//!
//! Provides speech-to-text transcription using Whisper via Candle ML framework.

use super::audio::AudioData;
use crate::error::{AumateError, Result};
use crate::ml::{Device, DeviceConfig, get_device};
use candle_core::{DType, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::whisper::{self as m, Config};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokenizers::Tokenizer;

/// Whisper model variant
pub enum WhisperModel {
    Normal(m::model::Whisper),
    // Quantized variant could be added later
}

impl WhisperModel {
    /// Run encoder forward pass
    pub fn encoder_forward(&mut self, mel: &Tensor, flush: bool) -> Result<Tensor> {
        match self {
            Self::Normal(model) => model
                .encoder
                .forward(mel, flush)
                .map_err(|e| AumateError::Other(format!("Encoder forward failed: {}", e))),
        }
    }

    /// Run decoder forward pass
    pub fn decoder_forward(
        &mut self,
        tokens: &Tensor,
        audio_features: &Tensor,
        flush: bool,
    ) -> Result<Tensor> {
        match self {
            Self::Normal(model) => {
                let decoder_output = model
                    .decoder
                    .forward(tokens, audio_features, flush)
                    .map_err(|e| AumateError::Other(format!("Decoder forward failed: {}", e)))?;
                // Project to vocabulary logits
                model
                    .decoder
                    .final_linear(&decoder_output)
                    .map_err(|e| AumateError::Other(format!("Final linear failed: {}", e)))
            }
        }
    }

    /// Reset KV cache
    #[allow(dead_code)]
    pub fn reset_kv_cache(&mut self) {
        match self {
            Self::Normal(model) => model.reset_kv_cache(),
        }
    }
}

/// Result of a transcription
#[derive(Debug, Clone)]
pub struct TranscriptionResult {
    /// Transcribed text
    pub text: String,
    /// Detected or specified language
    pub language: Option<String>,
    /// Transcription duration in milliseconds
    pub duration_ms: u64,
}

/// Whisper transcription engine using Candle
pub struct WhisperEngine {
    /// The loaded model
    model: Option<WhisperModel>,
    /// Model configuration
    config: Option<Config>,
    /// Tokenizer
    tokenizer: Option<Tokenizer>,
    /// Mel filters for audio processing
    mel_filters: Vec<f32>,
    /// Device to run inference on
    device: Device,
    /// Path to the loaded model
    model_path: Option<PathBuf>,
    /// Language to use for transcription (None = auto-detect)
    language: Option<String>,
}

impl WhisperEngine {
    /// Create a new Whisper engine (no model loaded)
    pub fn new() -> Self {
        let device = get_device(&DeviceConfig::with_gpu()).unwrap_or(Device::Cpu);
        Self {
            model: None,
            config: None,
            tokenizer: None,
            mel_filters: Vec::new(),
            device,
            model_path: None,
            language: None,
        }
    }

    /// Create a new Whisper engine with specific device config
    pub fn with_device(config: DeviceConfig) -> Result<Self> {
        let device = get_device(&config)?;
        Ok(Self {
            model: None,
            config: None,
            tokenizer: None,
            mel_filters: Vec::new(),
            device,
            model_path: None,
            language: None,
        })
    }

    /// Load a Whisper model from directory containing model files
    ///
    /// The directory should contain:
    /// - model.safetensors (or model.gguf for quantized)
    /// - config.json
    /// - tokenizer.json
    /// - mel_filters.safetensors (optional, will use built-in if missing)
    pub fn load_model(&mut self, model_dir: &Path) -> Result<()> {
        log::info!("Loading Whisper model from: {:?}", model_dir);

        if !model_dir.exists() {
            return Err(AumateError::Other(format!("Model directory not found: {:?}", model_dir)));
        }

        // Load config
        let config_path = model_dir.join("config.json");
        let config: Config = if config_path.exists() {
            let config_str = std::fs::read_to_string(&config_path)?;
            serde_json::from_str(&config_str)
                .map_err(|e| AumateError::Other(format!("Failed to parse config: {}", e)))?
        } else {
            return Err(AumateError::Other("config.json not found".to_string()));
        };

        // Load tokenizer
        let tokenizer_path = model_dir.join("tokenizer.json");
        let tokenizer = if tokenizer_path.exists() {
            Tokenizer::from_file(&tokenizer_path)
                .map_err(|e| AumateError::Other(format!("Failed to load tokenizer: {}", e)))?
        } else {
            return Err(AumateError::Other("tokenizer.json not found".to_string()));
        };

        // Load mel filters (use built-in based on num_mel_bins)
        self.mel_filters = Self::get_mel_filters(config.num_mel_bins)?;

        // Load model weights
        let weights_path = model_dir.join("model.safetensors");
        if !weights_path.exists() {
            return Err(AumateError::Other("model.safetensors not found".to_string()));
        }

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[&weights_path], DType::F32, &self.device)
                .map_err(|e| AumateError::Other(format!("Failed to load weights: {}", e)))?
        };

        let model = m::model::Whisper::load(&vb, config.clone())
            .map_err(|e| AumateError::Other(format!("Failed to create model: {}", e)))?;

        self.model = Some(WhisperModel::Normal(model));
        self.config = Some(config);
        self.tokenizer = Some(tokenizer);
        self.model_path = Some(model_dir.to_path_buf());

        log::info!("Whisper model loaded successfully on {:?}", self.device);
        Ok(())
    }

    /// Get mel filters for the specified number of mel bins
    fn get_mel_filters(num_mel_bins: usize) -> Result<Vec<f32>> {
        // The candle-transformers whisper module has built-in mel filters
        // for 80 and 128 mel bins (the two configurations used by Whisper models)
        // We need to create the proper size filter bank
        let n_fft_half = m::N_FFT / 2 + 1; // 401 for N_FFT=800
        match num_mel_bins {
            80 => Ok(vec![0.0f32; 80 * n_fft_half]),
            128 => Ok(vec![0.0f32; 128 * n_fft_half]),
            _ => Err(AumateError::Other(format!(
                "Unsupported num_mel_bins: {}. Expected 80 or 128.",
                num_mel_bins
            ))),
        }
    }

    /// Unload the current model
    pub fn unload_model(&mut self) {
        self.model = None;
        self.config = None;
        self.tokenizer = None;
        self.mel_filters.clear();
        self.model_path = None;
        log::info!("Whisper model unloaded");
    }

    /// Check if a model is loaded
    pub fn is_loaded(&self) -> bool {
        self.model.is_some()
    }

    /// Get the path to the loaded model
    pub fn model_path(&self) -> Option<&Path> {
        self.model_path.as_deref()
    }

    /// Set the language for transcription
    pub fn set_language(&mut self, language: Option<String>) {
        self.language = language;
    }

    /// Get the current language setting
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Get device being used
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Transcribe audio data
    pub fn transcribe(&mut self, audio: &AudioData) -> Result<TranscriptionResult> {
        // Clone what we need before mutable borrow
        let config = self
            .config
            .clone()
            .ok_or_else(|| AumateError::Other("No config loaded".to_string()))?;
        let tokenizer = self
            .tokenizer
            .clone()
            .ok_or_else(|| AumateError::Other("No tokenizer loaded".to_string()))?;

        // Prepare audio for Whisper (mono, 16kHz)
        let prepared = audio.prepare_for_whisper();

        let start_time = Instant::now();

        // Convert PCM to mel spectrogram
        let mel = self.pcm_to_mel(&prepared.samples, &config)?;

        // Run transcription directly (we have mutable access to model)
        let text = self.decode_audio(&mel, &tokenizer)?;

        let duration_ms = start_time.elapsed().as_millis() as u64;

        log::info!(
            "Transcription completed in {}ms: \"{}\"",
            duration_ms,
            if text.len() > 50 { format!("{}...", &text[..50]) } else { text.clone() }
        );

        Ok(TranscriptionResult { text, language: self.language.clone(), duration_ms })
    }

    /// Decode audio mel spectrogram to text
    fn decode_audio(&mut self, mel: &Tensor, tokenizer: &Tokenizer) -> Result<String> {
        let model =
            self.model.as_mut().ok_or_else(|| AumateError::Other("No model loaded".to_string()))?;

        // Get special tokens
        let sot_token = tokenizer.token_to_id("<|startoftranscript|>").unwrap_or(50258);
        let eot_token = tokenizer.token_to_id("<|endoftext|>").unwrap_or(50257);
        let transcribe_token = tokenizer.token_to_id("<|transcribe|>").unwrap_or(50359);
        let no_timestamps_token = tokenizer.token_to_id("<|notimestamps|>").unwrap_or(50363);

        // Get language token if specified
        let language_token =
            self.language.as_ref().and_then(|lang| tokenizer.token_to_id(&format!("<|{}|>", lang)));

        // Encode audio
        let audio_features = model.encoder_forward(mel, true)?;

        // Initial tokens: SOT, language (if specified), transcribe, no_timestamps
        let mut tokens = vec![sot_token];
        if let Some(lang_token) = language_token {
            tokens.push(lang_token);
        }
        tokens.push(transcribe_token);
        tokens.push(no_timestamps_token);

        let initial_len = tokens.len();
        let max_tokens = 224; // Max tokens for a 30-second segment

        // Autoregressive decoding loop
        for _ in 0..max_tokens {
            let tokens_tensor = Tensor::new(tokens.as_slice(), &self.device)
                .map_err(|e| AumateError::Other(format!("Failed to create tokens tensor: {}", e)))?
                .unsqueeze(0)
                .map_err(|e| AumateError::Other(format!("Failed to unsqueeze: {}", e)))?;

            let logits = model.decoder_forward(
                &tokens_tensor,
                &audio_features,
                tokens.len() == initial_len,
            )?;

            // Get logits for last position
            let seq_len = logits
                .dim(1)
                .map_err(|e| AumateError::Other(format!("Failed to get dim: {}", e)))?;
            let last_logits = logits
                .narrow(1, seq_len - 1, 1)
                .map_err(|e| AumateError::Other(format!("Failed to narrow: {}", e)))?
                .squeeze(1)
                .map_err(|e| AumateError::Other(format!("Failed to squeeze: {}", e)))?;

            // Greedy decoding: take argmax
            let next_token = last_logits
                .argmax(1)
                .map_err(|e| AumateError::Other(format!("Failed to argmax: {}", e)))?
                .to_scalar::<u32>()
                .map_err(|e| AumateError::Other(format!("Failed to get scalar: {}", e)))?;

            // Check for end of text
            if next_token == eot_token {
                break;
            }

            tokens.push(next_token);
        }

        // Decode tokens to text (skip special tokens at the beginning)
        let text_tokens: Vec<u32> =
            tokens.into_iter().skip(initial_len).filter(|&t| t != eot_token).collect();

        let text = tokenizer
            .decode(&text_tokens, true)
            .map_err(|e| AumateError::Other(format!("Failed to decode tokens: {}", e)))?;

        Ok(text.trim().to_string())
    }

    /// Convert PCM audio samples to mel spectrogram tensor
    fn pcm_to_mel(&self, samples: &[f32], config: &Config) -> Result<Tensor> {
        let mel = m::audio::pcm_to_mel(config, samples, &self.mel_filters);
        let mel_len = mel.len();

        Tensor::from_vec(mel, (1, config.num_mel_bins, mel_len / config.num_mel_bins), &self.device)
            .map_err(|e| AumateError::Other(format!("Failed to create mel tensor: {}", e)))
    }
}

impl Default for WhisperEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = WhisperEngine::new();
        assert!(!engine.is_loaded());
        assert!(engine.model_path().is_none());
    }

    #[test]
    fn test_language_setting() {
        let mut engine = WhisperEngine::new();
        assert!(engine.language().is_none());

        engine.set_language(Some("en".to_string()));
        assert_eq!(engine.language(), Some("en"));

        engine.set_language(None);
        assert!(engine.language().is_none());
    }

    #[test]
    fn test_transcribe_without_model() {
        let mut engine = WhisperEngine::new();
        let audio = AudioData { samples: vec![0.0; 16000], sample_rate: 16000, channels: 1 };

        let result = engine.transcribe(&audio);
        assert!(result.is_err());
    }
}
