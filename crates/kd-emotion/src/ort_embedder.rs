//! [`Embedder`] backed by ONNX Runtime and a Hugging Face `tokenizers` tokenizer (ADR-0004).
//!
//! The graph is self-contained: it takes `input_ids` and `attention_mask` (`i64`, `[1, seq]`)
//! and returns `sentence_embedding` (`[1, dim]`, already L2-normalized). The tokenizer adds the
//! special tokens itself (BOS/EOS through its post-processor), so they are never added here.

use std::path::Path;
use std::sync::Mutex;

use ort::environment::Environment;
use ort::session::Session;
use ort::value::Tensor;
use tokenizers::Tokenizer;

use crate::{Embedder, EmotionError};

/// Maximum number of tokens fed to the model (BOS and EOS included).
pub const MAX_TOKENS: usize = 512;

fn embedder_err(context: &str, e: impl std::fmt::Display) -> EmotionError {
    EmotionError::Embedder(format!("{context}: {e}"))
}

/// Prepends the manifest prompt to the message.
pub(crate) fn assemble_input(prompt: &str, text: &str) -> String {
    let mut input = String::with_capacity(prompt.len() + text.len());
    input.push_str(prompt);
    input.push_str(text);
    input
}

/// Truncates token ids to `max` entries, keeping the last one (the EOS token) in place.
pub(crate) fn truncate_ids(ids: &[u32], max: usize) -> Vec<u32> {
    if ids.len() <= max || max < 2 {
        return ids.to_vec();
    }
    let mut out = ids[..max - 1].to_vec();
    out.push(ids[ids.len() - 1]);
    out
}

/// Scales a vector to unit L2 norm in place; a zero vector is left unchanged.
pub(crate) fn l2_normalize(v: &mut [f32]) {
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        v.iter_mut().for_each(|x| *x /= norm);
    }
}

/// ONNX Runtime sentence embedder, one message at a time (batch 1).
pub struct OrtEmbedder {
    _environment: Environment,
    session: Mutex<Session>,
    tokenizer: Tokenizer,
    prompt: String,
    dimensions: usize,
}

impl OrtEmbedder {
    /// Loads the model and the tokenizer. `prompt` is prepended to every message;
    /// `dimensions` is the expected embedding size.
    ///
    /// ONNX Runtime itself is loaded dynamically from `ORT_DYLIB_PATH`.
    pub fn new(
        model: &Path,
        tokenizer: &Path,
        prompt: &str,
        dimensions: usize,
    ) -> Result<Self, EmotionError> {
        let environment = ort::init()
            .with_name("kotomotion")
            .with_telemetry(false)
            .build()
            .map_err(|e| embedder_err("cannot initialise ONNX Runtime", e))?;
        let session = Session::builder(&environment)
            .and_then(|mut b| b.commit_from_file(model))
            .map_err(|e| embedder_err("cannot load model", e))?;
        let tokenizer = Tokenizer::from_file(tokenizer)
            .map_err(|e| embedder_err("cannot load tokenizer", e))?;
        Ok(Self {
            _environment: environment,
            session: Mutex::new(session),
            tokenizer,
            prompt: prompt.to_owned(),
            dimensions,
        })
    }

    /// Token ids the model receives for `text` (prompt and special tokens included).
    pub fn token_ids(&self, text: &str) -> Result<Vec<u32>, EmotionError> {
        let encoding = self
            .tokenizer
            .encode(assemble_input(&self.prompt, text), true)
            .map_err(|e| embedder_err("tokenization failed", e))?;
        Ok(truncate_ids(encoding.get_ids(), MAX_TOKENS))
    }
}

impl Embedder for OrtEmbedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>, EmotionError> {
        let ids: Vec<i64> = self.token_ids(text)?.into_iter().map(i64::from).collect();
        let len = ids.len();
        let mask = vec![1_i64; len];
        let shape = [1_usize, len];
        let input_ids = Tensor::from_array((shape, ids)).map_err(|e| embedder_err("input", e))?;
        let attention_mask =
            Tensor::from_array((shape, mask)).map_err(|e| embedder_err("input", e))?;

        let mut session = self
            .session
            .lock()
            .map_err(|_| EmotionError::Embedder("session lock poisoned".into()))?;
        let outputs = session
            .run(ort::inputs![
                "input_ids" => input_ids,
                "attention_mask" => attention_mask
            ])
            .map_err(|e| embedder_err("inference failed", e))?;
        let (_, data) = outputs["sentence_embedding"]
            .try_extract_tensor::<f32>()
            .map_err(|e| embedder_err("output", e))?;
        if data.len() != self.dimensions {
            return Err(EmotionError::DimensionMismatch {
                expected: self.dimensions,
                got: data.len(),
            });
        }
        let mut embedding = data.to_vec();
        l2_normalize(&mut embedding);
        Ok(embedding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_is_prepended_verbatim() {
        assert_eq!(
            assemble_input("task: x | query: ", "salut"),
            "task: x | query: salut"
        );
        assert_eq!(assemble_input("", "salut"), "salut");
    }

    #[test]
    fn truncation_keeps_the_final_special_token() {
        let ids: Vec<u32> = (0..10).collect();
        assert_eq!(truncate_ids(&ids, 20), ids);
        assert_eq!(truncate_ids(&ids, 5), vec![0, 1, 2, 3, 9]);
        assert_eq!(truncate_ids(&ids, 10), ids);
    }

    #[test]
    fn normalization_gives_unit_norm_and_tolerates_zero() {
        let mut v = vec![3.0, 4.0];
        l2_normalize(&mut v);
        assert!((v[0] - 0.6).abs() < 1e-6 && (v[1] - 0.8).abs() < 1e-6);
        let mut z = vec![0.0; 3];
        l2_normalize(&mut z);
        assert_eq!(z, vec![0.0; 3]);
    }
}
