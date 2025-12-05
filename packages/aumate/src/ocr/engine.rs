//! TrOCR-based OCR engine
//!
//! Provides optical character recognition using TrOCR model via Candle ML framework.

use super::OcrModelVariant;
use crate::error::{AumateError, Result};
use crate::ml::{Device, DeviceConfig, get_device};
use candle_core::{DType, Tensor};
use candle_nn::Activation;
use candle_nn::VarBuilder;
use candle_transformers::models::{trocr, vit};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokenizers::{
    Tokenizer, decoders::byte_level::ByteLevel as ByteLevelDecoder, models::bpe::BPE,
    pre_tokenizers::byte_level::ByteLevel as ByteLevelPreTokenizer,
};

/// TrOCR model wrapper
pub struct TrOCRModel {
    model: trocr::TrOCRModel,
}

impl TrOCRModel {
    /// Run encoder forward pass
    pub fn encoder_forward(&mut self, image: &Tensor) -> Result<Tensor> {
        self.model
            .encoder()
            .forward(image)
            .map_err(|e| AumateError::Ml(format!("Encoder forward failed: {}", e)))
    }

    /// Run decoder forward pass
    pub fn decoder_forward(
        &mut self,
        tokens: &Tensor,
        encoder_output: &Tensor,
        past_kv_len: usize,
    ) -> Result<Tensor> {
        self.model
            .decode(tokens, encoder_output, past_kv_len)
            .map_err(|e| AumateError::Ml(format!("Decoder forward failed: {}", e)))
    }

    /// Reset KV cache
    pub fn reset_kv_cache(&mut self) {
        self.model.reset_kv_cache();
    }
}

/// Result of OCR recognition
#[derive(Debug, Clone)]
pub struct OcrResult {
    /// Recognized text
    pub text: String,
    /// Recognition duration in milliseconds
    pub duration_ms: u64,
    /// Confidence score (0.0 to 1.0) if available
    pub confidence: Option<f32>,
}

/// TrOCR-based OCR engine
pub struct OcrEngine {
    /// The loaded model
    model: Option<TrOCRModel>,
    /// Tokenizer
    tokenizer: Option<Tokenizer>,
    /// Device to run inference on
    device: Device,
    /// Path to the loaded model
    model_path: Option<PathBuf>,
    /// Current model variant
    variant: Option<OcrModelVariant>,
    /// Image size for preprocessing
    image_size: usize,
}

impl OcrEngine {
    /// Create a new OCR engine (no model loaded)
    pub fn new() -> Self {
        let device = get_device(&DeviceConfig::with_gpu()).unwrap_or(Device::Cpu);
        Self {
            model: None,
            tokenizer: None,
            device,
            model_path: None,
            variant: None,
            image_size: 384, // Default TrOCR image size
        }
    }

    /// Create a new OCR engine with specific device config
    pub fn with_device(config: DeviceConfig) -> Result<Self> {
        let device = get_device(&config)?;
        Ok(Self {
            model: None,
            tokenizer: None,
            device,
            model_path: None,
            variant: None,
            image_size: 384,
        })
    }

    /// Load a TrOCR model from directory containing model files
    ///
    /// The directory should contain:
    /// - model.safetensors
    /// - config.json
    /// - vocab.json (RoBERTa tokenizer vocabulary)
    /// - merges.txt (BPE merges file)
    pub fn load_model(&mut self, model_dir: &Path) -> Result<()> {
        log::info!("Loading TrOCR model from: {:?}", model_dir);

        if !model_dir.exists() {
            return Err(AumateError::Other(format!("Model directory not found: {:?}", model_dir)));
        }

        // Load config
        let config_path = model_dir.join("config.json");
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| AumateError::Other(format!("Failed to read config: {}", e)))?;

        let config: TrOCRConfigJson = serde_json::from_str(&config_str)
            .map_err(|e| AumateError::Other(format!("Failed to parse config: {}", e)))?;

        // Extract encoder (ViT) config
        let encoder_config = vit::Config {
            hidden_size: config.encoder.hidden_size.unwrap_or(768),
            num_hidden_layers: config.encoder.num_hidden_layers.unwrap_or(12),
            num_attention_heads: config.encoder.num_attention_heads.unwrap_or(12),
            intermediate_size: config.encoder.intermediate_size.unwrap_or(3072),
            hidden_act: Activation::Gelu,
            layer_norm_eps: config.encoder.layer_norm_eps.unwrap_or(1e-6),
            image_size: config.encoder.image_size.unwrap_or(384),
            patch_size: config.encoder.patch_size.unwrap_or(16),
            num_channels: config.encoder.num_channels.unwrap_or(3),
            qkv_bias: config.encoder.qkv_bias.unwrap_or(true),
        };

        self.image_size = encoder_config.image_size;

        // Extract decoder (TrOCR) config
        let decoder_config = trocr::TrOCRConfig {
            vocab_size: config.decoder.vocab_size.unwrap_or(50265),
            d_model: config.decoder.d_model.unwrap_or(1024),
            decoder_vocab_size: config.decoder.decoder_vocab_size,
            decoder_layers: config.decoder.decoder_layers.unwrap_or(12),
            decoder_attention_heads: config.decoder.decoder_attention_heads.unwrap_or(16),
            decoder_ffn_dim: config.decoder.decoder_ffn_dim.unwrap_or(4096),
            cross_attention_hidden_size: config.decoder.cross_attention_hidden_size.unwrap_or(768),
            activation_function: Activation::Gelu,
            max_position_embeddings: config.decoder.max_position_embeddings.unwrap_or(512),
            use_learned_position_embeddings: config
                .decoder
                .use_learned_position_embeddings
                .unwrap_or(true),
            dropout: config.decoder.dropout.unwrap_or(0.1),
            attention_dropout: config.decoder.attention_dropout.unwrap_or(0.0),
            activation_dropout: config.decoder.activation_dropout.unwrap_or(0.0),
            decoder_layerdrop: config.decoder.decoder_layerdrop.unwrap_or(0.0),
            decoder_start_token_id: config.decoder.decoder_start_token_id.unwrap_or(2),
            bos_token_id: config.decoder.bos_token_id.unwrap_or(0),
            scale_embedding: config.decoder.scale_embedding.unwrap_or(true),
            tie_word_embeddings: config.decoder.tie_word_embeddings.unwrap_or(false),
            use_cache: config.decoder.use_cache.unwrap_or(true),
            init_std: config.decoder.init_std.unwrap_or(0.02),
            eos_token_id: config.decoder.eos_token_id.unwrap_or(2),
            pad_token_id: config.decoder.pad_token_id.unwrap_or(1),
        };

        // Load tokenizer from vocab.json and merges.txt (RoBERTa BPE format)
        let vocab_path = model_dir.join("vocab.json");
        let merges_path = model_dir.join("merges.txt");

        // First try tokenizer.json if it exists (some models may have it)
        let tokenizer_json_path = model_dir.join("tokenizer.json");
        let tokenizer = if tokenizer_json_path.exists() {
            Tokenizer::from_file(&tokenizer_json_path)
                .map_err(|e| AumateError::Other(format!("Failed to load tokenizer.json: {}", e)))?
        } else if vocab_path.exists() && merges_path.exists() {
            // Build BPE tokenizer from vocab.json and merges.txt
            Self::build_roberta_tokenizer(&vocab_path, &merges_path)?
        } else {
            return Err(AumateError::Other(
                "Neither tokenizer.json nor vocab.json+merges.txt found".to_string(),
            ));
        };

        // Load model weights
        let weights_path = model_dir.join("model.safetensors");
        if !weights_path.exists() {
            return Err(AumateError::Other("model.safetensors not found".to_string()));
        }

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[&weights_path], DType::F32, &self.device)
                .map_err(|e| AumateError::Other(format!("Failed to load weights: {}", e)))?
        };

        let model = trocr::TrOCRModel::new(&encoder_config, &decoder_config, vb)
            .map_err(|e| AumateError::Other(format!("Failed to create model: {}", e)))?;

        self.model = Some(TrOCRModel { model });
        self.tokenizer = Some(tokenizer);
        self.model_path = Some(model_dir.to_path_buf());

        log::info!("TrOCR model loaded successfully on {:?}", self.device);
        Ok(())
    }

    /// Unload the current model
    pub fn unload_model(&mut self) {
        self.model = None;
        self.tokenizer = None;
        self.model_path = None;
        self.variant = None;
        log::info!("TrOCR model unloaded");
    }

    /// Build a RoBERTa-style BPE tokenizer from vocab.json and merges.txt
    fn build_roberta_tokenizer(vocab_path: &Path, merges_path: &Path) -> Result<Tokenizer> {
        log::info!("Building BPE tokenizer from vocab.json and merges.txt");

        // Build the BPE model from vocab and merges files
        let bpe = BPE::from_file(
            vocab_path
                .to_str()
                .ok_or_else(|| AumateError::Other("Invalid vocab path".to_string()))?,
            merges_path
                .to_str()
                .ok_or_else(|| AumateError::Other("Invalid merges path".to_string()))?,
        )
        .unk_token("<unk>".to_string())
        .build()
        .map_err(|e| AumateError::Other(format!("Failed to build BPE model: {}", e)))?;

        // Create tokenizer with the BPE model
        let mut tokenizer = Tokenizer::new(bpe);

        // Add byte-level pre-tokenizer (required for RoBERTa)
        tokenizer.with_pre_tokenizer(Some(ByteLevelPreTokenizer::default()));

        // Add byte-level decoder (to properly decode tokens back to text)
        tokenizer.with_decoder(Some(ByteLevelDecoder::default()));

        log::info!("BPE tokenizer built successfully");
        Ok(tokenizer)
    }

    /// Check if a model is loaded
    pub fn is_loaded(&self) -> bool {
        self.model.is_some()
    }

    /// Get the path to the loaded model
    pub fn model_path(&self) -> Option<&Path> {
        self.model_path.as_deref()
    }

    /// Get the current model variant
    pub fn variant(&self) -> Option<OcrModelVariant> {
        self.variant
    }

    /// Get device being used
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Recognize text from an image
    pub fn recognize(&mut self, image: &image::DynamicImage) -> Result<OcrResult> {
        let tokenizer = self
            .tokenizer
            .clone()
            .ok_or_else(|| AumateError::Other("No tokenizer loaded".to_string()))?;

        let start_time = Instant::now();

        // Preprocess image to tensor
        let image_tensor = self.preprocess_image(image)?;

        // Run OCR
        let text = self.decode_image(&image_tensor, &tokenizer)?;

        let duration_ms = start_time.elapsed().as_millis() as u64;

        log::info!(
            "OCR completed in {}ms: \"{}\"",
            duration_ms,
            if text.len() > 50 { format!("{}...", &text[..50]) } else { text.clone() }
        );

        Ok(OcrResult { text, duration_ms, confidence: None })
    }

    /// Recognize text from image bytes (PNG, JPEG, etc.)
    pub fn recognize_bytes(&mut self, bytes: &[u8]) -> Result<OcrResult> {
        let image = image::load_from_memory(bytes)
            .map_err(|e| AumateError::Other(format!("Failed to load image: {}", e)))?;
        self.recognize(&image)
    }

    /// Recognize text from image file path
    pub fn recognize_file(&mut self, path: &Path) -> Result<OcrResult> {
        let image = image::open(path)
            .map_err(|e| AumateError::Other(format!("Failed to open image: {}", e)))?;
        self.recognize(&image)
    }

    /// Preprocess image to tensor
    fn preprocess_image(&self, image: &image::DynamicImage) -> Result<Tensor> {
        // Resize to model's expected size
        let image = image.resize_exact(
            self.image_size as u32,
            self.image_size as u32,
            image::imageops::FilterType::Triangle,
        );

        // Convert to RGB
        let image = image.to_rgb8();

        // Convert to tensor (normalized)
        let (width, height) = (image.width() as usize, image.height() as usize);
        let data: Vec<f32> = image
            .pixels()
            .flat_map(|p| {
                // Normalize to [-1, 1] range (ImageNet normalization)
                let r = (p[0] as f32 / 255.0 - 0.5) / 0.5;
                let g = (p[1] as f32 / 255.0 - 0.5) / 0.5;
                let b = (p[2] as f32 / 255.0 - 0.5) / 0.5;
                [r, g, b]
            })
            .collect();

        // Create tensor with shape [1, 3, height, width]
        let tensor = Tensor::from_vec(data, (1, 3, height, width), &self.device)
            .map_err(|e| AumateError::Ml(format!("Failed to create image tensor: {}", e)))?;

        Ok(tensor)
    }

    /// Decode image to text using autoregressive generation
    fn decode_image(&mut self, image: &Tensor, tokenizer: &Tokenizer) -> Result<String> {
        let model =
            self.model.as_mut().ok_or_else(|| AumateError::Other("No model loaded".to_string()))?;

        // Reset cache for new sequence
        model.reset_kv_cache();

        // Encode image
        let encoder_output = model.encoder_forward(image)?;

        // Get special tokens
        let eos_token_id = tokenizer.token_to_id("</s>").unwrap_or(2);
        let bos_token_id = tokenizer.token_to_id("<s>").unwrap_or(0);

        // Start with BOS token
        let mut tokens = vec![bos_token_id];
        let max_tokens = 512;

        // Autoregressive decoding loop
        for i in 0..max_tokens {
            let tokens_tensor = Tensor::new(tokens.as_slice(), &self.device)
                .map_err(|e| AumateError::Ml(format!("Failed to create tokens tensor: {}", e)))?
                .unsqueeze(0)
                .map_err(|e| AumateError::Ml(format!("Failed to unsqueeze: {}", e)))?;

            let logits = model.decoder_forward(&tokens_tensor, &encoder_output, i)?;

            // Get logits for last position
            let seq_len =
                logits.dim(1).map_err(|e| AumateError::Ml(format!("Failed to get dim: {}", e)))?;
            let last_logits = logits
                .narrow(1, seq_len - 1, 1)
                .map_err(|e| AumateError::Ml(format!("Failed to narrow: {}", e)))?
                .squeeze(1)
                .map_err(|e| AumateError::Ml(format!("Failed to squeeze: {}", e)))?;

            // Greedy decoding: take argmax
            let next_token = last_logits
                .argmax(1)
                .map_err(|e| AumateError::Ml(format!("Failed to argmax: {}", e)))?
                .to_scalar::<u32>()
                .map_err(|e| AumateError::Ml(format!("Failed to get scalar: {}", e)))?;

            // Check for end of sequence
            if next_token == eos_token_id {
                break;
            }

            tokens.push(next_token);
        }

        // Decode tokens to text (skip BOS token)
        let text_tokens: Vec<u32> = tokens
            .into_iter()
            .skip(1) // Skip BOS
            .filter(|&t| t != eos_token_id)
            .collect();

        let text = tokenizer
            .decode(&text_tokens, true)
            .map_err(|e| AumateError::Other(format!("Failed to decode tokens: {}", e)))?;

        Ok(text.trim().to_string())
    }
}

impl Default for OcrEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// JSON config for TrOCR models (partial, for deserialization)
#[derive(Debug, serde::Deserialize)]
struct TrOCRConfigJson {
    encoder: EncoderConfig,
    decoder: DecoderConfig,
}

#[derive(Debug, serde::Deserialize)]
struct EncoderConfig {
    hidden_size: Option<usize>,
    num_hidden_layers: Option<usize>,
    num_attention_heads: Option<usize>,
    intermediate_size: Option<usize>,
    layer_norm_eps: Option<f64>,
    image_size: Option<usize>,
    patch_size: Option<usize>,
    num_channels: Option<usize>,
    qkv_bias: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
struct DecoderConfig {
    vocab_size: Option<usize>,
    d_model: Option<usize>,
    decoder_vocab_size: Option<usize>,
    decoder_layers: Option<usize>,
    decoder_attention_heads: Option<usize>,
    decoder_ffn_dim: Option<usize>,
    cross_attention_hidden_size: Option<usize>,
    max_position_embeddings: Option<usize>,
    use_learned_position_embeddings: Option<bool>,
    dropout: Option<f64>,
    attention_dropout: Option<f64>,
    activation_dropout: Option<f64>,
    decoder_layerdrop: Option<f64>,
    decoder_start_token_id: Option<u32>,
    bos_token_id: Option<usize>,
    scale_embedding: Option<bool>,
    tie_word_embeddings: Option<bool>,
    use_cache: Option<bool>,
    init_std: Option<f64>,
    eos_token_id: Option<u32>,
    pad_token_id: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = OcrEngine::new();
        assert!(!engine.is_loaded());
        assert!(engine.model_path().is_none());
    }

    #[test]
    fn test_recognize_without_model() {
        let mut engine = OcrEngine::new();
        let image = image::DynamicImage::new_rgb8(100, 100);
        let result = engine.recognize(&image);
        assert!(result.is_err());
    }
}
