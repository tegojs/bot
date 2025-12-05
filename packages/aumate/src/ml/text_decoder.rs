//! Text decoding utilities for autoregressive models
//!
//! Provides shared functionality for decoding text from transformer models
//! like Whisper and TrOCR using autoregressive generation.

use crate::error::{AumateError, Result};
use candle_core::Tensor;
use tokenizers::Tokenizer;

/// Configuration for text decoding
#[derive(Debug, Clone)]
pub struct DecodingConfig {
    /// Maximum number of tokens to generate
    pub max_tokens: usize,
    /// Temperature for sampling (1.0 = no change, lower = more deterministic)
    pub temperature: f64,
    /// Top-p (nucleus) sampling threshold
    pub top_p: Option<f64>,
    /// Token ID that signals end of sequence
    pub eos_token_id: u32,
    /// Token ID for beginning of sequence (if needed)
    pub bos_token_id: Option<u32>,
    /// Token ID for padding (if needed)
    pub pad_token_id: Option<u32>,
    /// Whether to use greedy decoding (ignores temperature/top_p)
    pub greedy: bool,
}

impl Default for DecodingConfig {
    fn default() -> Self {
        Self {
            max_tokens: 448, // Whisper default
            temperature: 1.0,
            top_p: None,
            greedy: true,
            eos_token_id: 50257, // Whisper default
            bos_token_id: None,
            pad_token_id: None,
        }
    }
}

impl DecodingConfig {
    /// Create a config for greedy decoding
    pub fn greedy(max_tokens: usize, eos_token_id: u32) -> Self {
        Self { max_tokens, eos_token_id, greedy: true, ..Default::default() }
    }

    /// Create a config with temperature sampling
    pub fn with_temperature(max_tokens: usize, eos_token_id: u32, temperature: f64) -> Self {
        Self { max_tokens, eos_token_id, temperature, greedy: false, ..Default::default() }
    }
}

/// Text decoder that handles token-to-text conversion
pub struct TextDecoder {
    tokenizer: Tokenizer,
}

impl TextDecoder {
    /// Create a new text decoder from a tokenizer file
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let tokenizer = Tokenizer::from_file(path)
            .map_err(|e| AumateError::Other(format!("Failed to load tokenizer: {}", e)))?;
        Ok(Self { tokenizer })
    }

    /// Create a new text decoder from a tokenizer
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    /// Decode a single token ID to a string
    pub fn decode_token(&self, token_id: u32) -> Result<String> {
        self.tokenizer
            .decode(&[token_id], false)
            .map_err(|e| AumateError::Other(format!("Failed to decode token: {}", e)))
    }

    /// Decode a sequence of token IDs to a string
    pub fn decode(&self, token_ids: &[u32]) -> Result<String> {
        self.tokenizer
            .decode(token_ids, true)
            .map_err(|e| AumateError::Other(format!("Failed to decode tokens: {}", e)))
    }

    /// Encode a string to token IDs
    pub fn encode(&self, text: &str) -> Result<Vec<u32>> {
        let encoding = self
            .tokenizer
            .encode(text, false)
            .map_err(|e| AumateError::Other(format!("Failed to encode text: {}", e)))?;
        Ok(encoding.get_ids().to_vec())
    }

    /// Get the vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(true)
    }

    /// Get token ID by token string
    pub fn token_to_id(&self, token: &str) -> Option<u32> {
        self.tokenizer.token_to_id(token)
    }

    /// Get token string by token ID
    pub fn id_to_token(&self, id: u32) -> Option<String> {
        self.tokenizer.id_to_token(id)
    }
}

/// Sample from logits tensor using the given configuration
#[allow(dead_code)]
pub fn sample_from_logits(logits: &Tensor, config: &DecodingConfig) -> Result<u32> {
    let logits =
        logits.squeeze(0).map_err(|e| AumateError::Ml(format!("Squeeze failed: {}", e)))?;
    let logits =
        logits.squeeze(0).map_err(|e| AumateError::Ml(format!("Squeeze failed: {}", e)))?;

    if config.greedy {
        // Greedy decoding: just take argmax
        let token_id = logits
            .argmax(0)
            .map_err(|e| AumateError::Ml(format!("Argmax failed: {}", e)))?
            .to_scalar::<u32>()
            .map_err(|e| AumateError::Ml(format!("To scalar failed: {}", e)))?;
        return Ok(token_id);
    }

    // Temperature scaling
    let logits = if config.temperature != 1.0 {
        (logits / config.temperature)
            .map_err(|e| AumateError::Ml(format!("Temperature scaling failed: {}", e)))?
    } else {
        logits
    };

    // Convert to probabilities
    let probs = candle_nn::ops::softmax(&logits, 0)
        .map_err(|e| AumateError::Ml(format!("Softmax failed: {}", e)))?;

    // Top-p (nucleus) sampling
    let probs = if let Some(top_p) = config.top_p { apply_top_p(&probs, top_p)? } else { probs };

    // Sample from distribution
    let probs_vec: Vec<f32> =
        probs.to_vec1().map_err(|e| AumateError::Ml(format!("To vec failed: {}", e)))?;
    let token_id = sample_from_distribution(&probs_vec)?;

    Ok(token_id)
}

/// Apply top-p (nucleus) sampling
#[allow(dead_code)]
fn apply_top_p(probs: &Tensor, top_p: f64) -> Result<Tensor> {
    let probs_vec: Vec<f32> =
        probs.to_vec1().map_err(|e| AumateError::Ml(format!("To vec failed: {}", e)))?;
    let mut indexed: Vec<(usize, f32)> = probs_vec.iter().copied().enumerate().collect();

    // Sort by probability descending
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Find cutoff
    let mut cumsum = 0.0;
    let mut cutoff_idx = indexed.len();
    for (i, (_, p)) in indexed.iter().enumerate() {
        cumsum += *p as f64;
        if cumsum >= top_p {
            cutoff_idx = i + 1;
            break;
        }
    }

    // Zero out probabilities below cutoff
    let mut new_probs = vec![0.0f32; probs_vec.len()];
    for (idx, prob) in indexed.iter().take(cutoff_idx) {
        new_probs[*idx] = *prob;
    }

    // Renormalize
    let sum: f32 = new_probs.iter().sum();
    if sum > 0.0 {
        for p in &mut new_probs {
            *p /= sum;
        }
    }

    Tensor::from_vec(new_probs, probs.shape(), probs.device())
        .map_err(|e| AumateError::Ml(format!("Failed to create tensor: {}", e)))
}

/// Sample from a probability distribution
#[allow(dead_code)]
fn sample_from_distribution(probs: &[f32]) -> Result<u32> {
    use rand::Rng;
    let mut rng = rand::rng();
    let r: f32 = rng.random();

    let mut cumsum = 0.0;
    for (i, &p) in probs.iter().enumerate() {
        cumsum += p;
        if r < cumsum {
            return Ok(i as u32);
        }
    }

    // Fallback to last token
    Ok((probs.len() - 1) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding_config_default() {
        let config = DecodingConfig::default();
        assert!(config.greedy);
        assert_eq!(config.max_tokens, 448);
    }

    #[test]
    fn test_decoding_config_greedy() {
        let config = DecodingConfig::greedy(100, 50256);
        assert!(config.greedy);
        assert_eq!(config.max_tokens, 100);
        assert_eq!(config.eos_token_id, 50256);
    }

    #[test]
    fn test_sample_from_distribution() {
        // Test with deterministic distribution
        let probs = vec![0.0, 0.0, 1.0, 0.0];
        let result = sample_from_distribution(&probs).unwrap();
        assert_eq!(result, 2);
    }
}
