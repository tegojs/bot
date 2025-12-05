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
    /// Whether to use KV cache for decoding
    use_cache: bool,
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
            image_size: 384,  // Default TrOCR image size
            use_cache: false, // Default to no cache (TrOCR base models don't use it)
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
            use_cache: false,
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

        // Store use_cache setting from config (default false for TrOCR)
        self.use_cache = config.decoder.use_cache.unwrap_or(false);

        self.model = Some(TrOCRModel { model });
        self.tokenizer = Some(tokenizer);
        self.model_path = Some(model_dir.to_path_buf());

        log::info!(
            "TrOCR model loaded successfully on {:?} (use_cache={})",
            self.device,
            self.use_cache
        );
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

    /// Recognize text from an image (supports multi-line text)
    pub fn recognize(&mut self, image: &image::DynamicImage) -> Result<OcrResult> {
        let tokenizer = self
            .tokenizer
            .clone()
            .ok_or_else(|| AumateError::Other("No tokenizer loaded".to_string()))?;

        let start_time = Instant::now();

        // Detect text lines in the image
        let line_images = self.detect_text_lines(image);

        let text = if line_images.len() > 1 {
            // Multi-line: process each line separately and join with newlines
            log::info!("Detected {} text lines, processing each separately", line_images.len());
            let mut line_results = Vec::new();
            for (i, line_image) in line_images.iter().enumerate() {
                let image_tensor = self.preprocess_image(line_image)?;
                let line_text = self.decode_image(&image_tensor, &tokenizer)?;
                if !line_text.is_empty() {
                    log::info!("Line {}: \"{}\"", i + 1, line_text);
                    line_results.push(line_text);
                }
            }
            line_results.join("\n")
        } else {
            // Single line or no clear line detection: process whole image
            let image_tensor = self.preprocess_image(image)?;
            self.decode_image(&image_tensor, &tokenizer)?
        };

        let duration_ms = start_time.elapsed().as_millis() as u64;

        log::info!(
            "OCR completed in {}ms: \"{}\"",
            duration_ms,
            if text.len() > 50 { format!("{}...", &text[..50]) } else { text.clone() }
        );

        Ok(OcrResult { text, duration_ms, confidence: None })
    }

    /// Detect text lines in an image using horizontal projection analysis
    ///
    /// Returns a vector of cropped line images. If only one line is detected
    /// or detection fails, returns the original image.
    fn detect_text_lines(&self, image: &image::DynamicImage) -> Vec<image::DynamicImage> {
        let gray = image.to_luma8();
        let (width, height) = gray.dimensions();

        if height < 20 || width < 20 {
            return vec![image.clone()];
        }

        // Apply adaptive binarization to handle complex backgrounds
        // This helps detect text lines regardless of background color
        let binary = self.adaptive_binarize(&gray);

        // Calculate horizontal projection (count of "text" pixels per row)
        let mut projection: Vec<u32> = Vec::with_capacity(height as usize);
        for y in 0..height {
            let mut row_sum: u32 = 0;
            for x in 0..width {
                // In binary image, 0 = text (foreground), 255 = background
                if binary.get_pixel(x, y).0[0] == 0 {
                    row_sum += 1;
                }
            }
            projection.push(row_sum);
        }

        // Find threshold for detecting gaps between lines
        // A row is considered a "gap" if it has very few text pixels
        let max_projection = *projection.iter().max().unwrap_or(&1);
        let gap_threshold = max_projection / 10; // Rows with <10% of max are gaps

        // Find line boundaries (runs of non-gap rows)
        let mut lines: Vec<(u32, u32)> = Vec::new(); // (start_y, end_y)
        let mut in_line = false;
        let mut line_start = 0u32;
        let min_line_height = (height / 30).max(3); // Minimum line height to avoid noise
        let min_gap_height = (height / 50).max(2); // Minimum gap height

        let mut gap_start = 0u32;
        let mut in_gap = false;

        for (y, &proj) in projection.iter().enumerate() {
            let y = y as u32;
            if proj > gap_threshold {
                // This row has text
                if in_gap && in_line {
                    // Check if gap was significant enough
                    let gap_height = y - gap_start;
                    if gap_height >= min_gap_height {
                        // End previous line at gap start
                        let line_height = gap_start - line_start;
                        if line_height >= min_line_height {
                            lines.push((line_start, gap_start));
                        }
                        line_start = y;
                    }
                }
                if !in_line {
                    line_start = y;
                    in_line = true;
                }
                in_gap = false;
            } else {
                // This row is a potential gap
                if in_line && !in_gap {
                    gap_start = y;
                    in_gap = true;
                }
            }
        }

        // Don't forget the last line
        if in_line {
            let end_y = if in_gap { gap_start } else { height };
            let line_height = end_y - line_start;
            if line_height >= min_line_height {
                lines.push((line_start, end_y));
            }
        }

        // If we detected less than 2 lines, return original image
        if lines.len() < 2 {
            return vec![image.clone()];
        }

        // Crop each detected line with some vertical padding
        let padding = (height / 40).max(2);
        let mut line_images = Vec::new();

        for (start_y, end_y) in lines {
            let padded_start = start_y.saturating_sub(padding);
            let padded_end = (end_y + padding).min(height);
            let crop_height = padded_end - padded_start;

            if crop_height > 0 {
                let cropped = image.crop_imm(0, padded_start, width, crop_height);
                line_images.push(cropped);
            }
        }

        if line_images.is_empty() {
            vec![image.clone()]
        } else {
            log::info!(
                "Text line detection: found {} lines in {}x{} image",
                line_images.len(),
                width,
                height
            );
            line_images
        }
    }

    /// Apply adaptive binarization to handle various backgrounds
    ///
    /// Uses local mean thresholding to separate text from background,
    /// works for both dark-on-light and light-on-dark text.
    fn adaptive_binarize(&self, gray: &image::GrayImage) -> image::GrayImage {
        let (width, height) = gray.dimensions();

        // Window size for local thresholding (adapt to image size)
        let window_size = ((width.min(height) / 15) as usize).clamp(15, 51);
        let half_window = window_size / 2;

        // Create integral image for fast mean calculation
        let mut integral: Vec<Vec<u64>> =
            vec![vec![0; (width + 1) as usize]; (height + 1) as usize];
        for y in 0..height {
            for x in 0..width {
                let pixel = gray.get_pixel(x, y).0[0] as u64;
                integral[(y + 1) as usize][(x + 1) as usize] = pixel
                    + integral[y as usize][(x + 1) as usize]
                    + integral[(y + 1) as usize][x as usize]
                    - integral[y as usize][x as usize];
            }
        }

        let mut binary = image::GrayImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                // Calculate local mean using integral image
                let x1 = (x as i32 - half_window as i32).max(0) as usize;
                let y1 = (y as i32 - half_window as i32).max(0) as usize;
                let x2 = ((x as usize + half_window + 1).min(width as usize)).min(width as usize);
                let y2 = ((y as usize + half_window + 1).min(height as usize)).min(height as usize);

                let area = ((x2 - x1) * (y2 - y1)) as u64;
                // Reorder to avoid underflow: (a + d) - b - c instead of a - b - c + d
                let sum = (integral[y2][x2] + integral[y1][x1])
                    .saturating_sub(integral[y1][x2])
                    .saturating_sub(integral[y2][x1]);
                let local_mean = (sum / area.max(1)) as u8;

                let pixel = gray.get_pixel(x, y).0[0];

                // Adaptive threshold with bias towards detecting text
                // Text is darker than local mean (dark text) or lighter (light text)
                let threshold_bias = 15u8;
                let lower_bound = local_mean.saturating_sub(threshold_bias);
                let upper_bound = local_mean.saturating_add(threshold_bias);
                // Text pixels deviate significantly from local mean
                let is_text = pixel < lower_bound || pixel > upper_bound;

                binary.put_pixel(x, y, image::Luma([if is_text { 0 } else { 255 }]));
            }
        }

        binary
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

        // Get dimensions
        let height = self.image_size;
        let width = self.image_size;

        // Get raw pixel data (HWC format: height * width * 3)
        let data = image.into_raw();

        // Create tensor in HWC format then convert to CHW
        // Following official Candle TrOCR example pattern:
        // 1. Create tensor as (height, width, channels)
        // 2. Permute to (channels, height, width)
        // 3. Normalize: (pixel / 255.0 - mean) / std where mean=std=0.5
        let tensor = Tensor::from_vec(data, (height, width, 3), &self.device)
            .map_err(|e| AumateError::Ml(format!("Failed to create image tensor: {}", e)))?
            .permute((2, 0, 1))
            .map_err(|e| AumateError::Ml(format!("Failed to permute tensor: {}", e)))?;

        // Normalize: (pixel / 255.0 - 0.5) / 0.5 = pixel / 127.5 - 1.0
        let mean = Tensor::new(&[0.5f32, 0.5, 0.5], &self.device)
            .map_err(|e| AumateError::Ml(format!("Failed to create mean tensor: {}", e)))?
            .reshape((3, 1, 1))
            .map_err(|e| AumateError::Ml(format!("Failed to reshape mean: {}", e)))?;

        let std = Tensor::new(&[0.5f32, 0.5, 0.5], &self.device)
            .map_err(|e| AumateError::Ml(format!("Failed to create std tensor: {}", e)))?
            .reshape((3, 1, 1))
            .map_err(|e| AumateError::Ml(format!("Failed to reshape std: {}", e)))?;

        let tensor = tensor
            .to_dtype(DType::F32)
            .map_err(|e| AumateError::Ml(format!("Failed to convert dtype: {}", e)))?;

        let tensor = (tensor / 255.0)
            .map_err(|e| AumateError::Ml(format!("Failed to scale: {}", e)))?
            .broadcast_sub(&mean)
            .map_err(|e| AumateError::Ml(format!("Failed to subtract mean: {}", e)))?
            .broadcast_div(&std)
            .map_err(|e| AumateError::Ml(format!("Failed to divide by std: {}", e)))?;

        // Add batch dimension: (3, H, W) -> (1, 3, H, W)
        let tensor = tensor
            .unsqueeze(0)
            .map_err(|e| AumateError::Ml(format!("Failed to add batch dim: {}", e)))?;

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
        // Use decoder_start_token_id (usually 2 for TrOCR)
        let decoder_start_token_id = 2u32;

        // Start with decoder start token
        let mut token_ids: Vec<u32> = vec![decoder_start_token_id];
        let max_tokens = 512;

        // Autoregressive decoding loop (following official Candle example pattern)
        for index in 0..max_tokens {
            // On first iteration: pass all tokens, start_pos=0
            // On subsequent iterations: pass only the last token, start_pos=previous length
            let context_size = if index >= 1 { 1 } else { token_ids.len() };
            let start_pos = token_ids.len().saturating_sub(context_size);
            let input_ids = &token_ids[start_pos..];

            let tokens_tensor = Tensor::new(input_ids, &self.device)
                .map_err(|e| AumateError::Ml(format!("Failed to create tokens tensor: {}", e)))?
                .unsqueeze(0)
                .map_err(|e| AumateError::Ml(format!("Failed to unsqueeze: {}", e)))?;

            let logits = model.decoder_forward(&tokens_tensor, &encoder_output, start_pos)?;

            // Get logits for last position
            let seq_len =
                logits.dim(1).map_err(|e| AumateError::Ml(format!("Failed to get dim: {}", e)))?;
            let last_logits = logits
                .narrow(1, seq_len - 1, 1)
                .map_err(|e| AumateError::Ml(format!("Failed to narrow: {}", e)))?
                .squeeze(1)
                .map_err(|e| AumateError::Ml(format!("Failed to squeeze: {}", e)))?;

            // Greedy decoding: take argmax
            // last_logits has shape [1, vocab_size], argmax on dim 1 gives [1]
            // We need to squeeze or index to get a scalar
            let next_token = last_logits
                .argmax(candle_core::D::Minus1)
                .map_err(|e| AumateError::Ml(format!("Failed to argmax: {}", e)))?
                .squeeze(0)
                .map_err(|e| AumateError::Ml(format!("Failed to squeeze argmax result: {}", e)))?
                .to_scalar::<u32>()
                .map_err(|e| AumateError::Ml(format!("Failed to get scalar: {}", e)))?;

            // Check for end of sequence
            if next_token == eos_token_id {
                break;
            }

            token_ids.push(next_token);
        }

        // Decode tokens to text (skip decoder start token)
        let text_tokens: Vec<u32> = token_ids
            .into_iter()
            .skip(1) // Skip decoder start token
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

    #[test]
    fn test_detect_text_lines_single_line() {
        use image::{DynamicImage, Rgb, RgbImage};

        let engine = OcrEngine::new();

        // Create a simple white image with a single black line of "text" in the middle
        let mut img = RgbImage::new(200, 50);
        // Fill with white
        for pixel in img.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }
        // Add a "text line" (dark pixels) in the middle
        for x in 20..180 {
            for y in 20..30 {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }

        let dynamic_img = DynamicImage::ImageRgb8(img);
        let lines = engine.detect_text_lines(&dynamic_img);

        // Should return original image for single line
        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_detect_text_lines_multi_line() {
        use image::{DynamicImage, Rgb, RgbImage};

        let engine = OcrEngine::new();

        // Create an image with two distinct text lines
        let mut img = RgbImage::new(200, 100);
        // Fill with white
        for pixel in img.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }
        // First text line (y: 15-25)
        for x in 20..180 {
            for y in 15..25 {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }
        // Second text line (y: 60-70)
        for x in 20..180 {
            for y in 60..70 {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }

        let dynamic_img = DynamicImage::ImageRgb8(img);
        let lines = engine.detect_text_lines(&dynamic_img);

        // Should detect 2 lines
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_detect_text_lines_small_image() {
        use image::{DynamicImage, RgbImage};

        let engine = OcrEngine::new();

        // Very small image should return original
        let img = RgbImage::new(10, 10);
        let dynamic_img = DynamicImage::ImageRgb8(img);
        let lines = engine.detect_text_lines(&dynamic_img);

        assert_eq!(lines.len(), 1);
    }
}
