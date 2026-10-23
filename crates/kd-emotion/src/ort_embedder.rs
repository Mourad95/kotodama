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

/// Maximum number of characters of a message that are tokenized; the rest is dropped (on a
/// character boundary) before tokenization, to bound the cost of a hostile input. The Python
/// reference does not truncate characters, so parity messages must stay below this limit (the
/// 300-character edge case of `parity.json` does).
pub const MAX_INPUT_CHARS: usize = 2000;

/// Returns `text` cut to at most `max` characters, on a character boundary.
pub(crate) fn truncate_chars(text: &str, max: usize) -> &str {
    match text.char_indices().nth(max) {
        Some((end, _)) => &text[..end],
        None => text,
    }
}

/// Fails with a precise error when `ORT_DYLIB_PATH` is set to a file that does not exist.
/// (With the variable unset, the dynamic loader searches its default path.)
pub(crate) fn check_library_path(value: Option<&std::ffi::OsStr>) -> Result<(), EmotionError> {
    match value {
        Some(path) if !Path::new(path).is_file() => Err(EmotionError::Embedder(format!(
            "ONNX Runtime library not found at ORT_DYLIB_PATH={}",
            Path::new(path).display()
        ))),
        _ => Ok(()),
    }
}

/// Fails with a precise error when an explicitly given ONNX Runtime library does not exist.
pub(crate) fn check_explicit_library(path: &Path) -> Result<(), EmotionError> {
    if path.is_file() {
        Ok(())
    } else {
        Err(EmotionError::Embedder(format!(
            "ONNX Runtime library not found at {}",
            path.display()
        )))
    }
}

/// ONNX Runtime allows one environment per process, ever: it is created on first use and then
/// shared by every embedder (also after one is dropped and a new one opened).
static ENVIRONMENT: Mutex<Option<Environment>> = Mutex::new(None);

fn shared_environment(library: Option<&Path>) -> Result<Environment, EmotionError> {
    let mut slot = ENVIRONMENT
        .lock()
        .map_err(|_| EmotionError::Embedder("environment lock poisoned".into()))?;
    if let Some(env) = slot.as_ref() {
        return Ok(env.clone());
    }
    let env = match library {
        Some(path) => {
            check_explicit_library(path)?;
            guard_init(|| {
                ort::init_from(path)
                    .map_err(|e| embedder_err("cannot load ONNX Runtime", e))?
                    .with_name("kotomotion")
                    .with_telemetry(false)
                    .build()
                    .map_err(|e| embedder_err("cannot initialise ONNX Runtime", e))
            })?
        }
        None => {
            check_library_path(std::env::var_os("ORT_DYLIB_PATH").as_deref())?;
            guard_init(|| {
                ort::init()
                    .with_name("kotomotion")
                    .with_telemetry(false)
                    .build()
                    .map_err(|e| embedder_err("cannot initialise ONNX Runtime", e))
            })?
        }
    };
    *slot = Some(env.clone());
    Ok(env)
}

/// Runs `init`, turning a panic (ort panics when it cannot load its library) into an error.
pub(crate) fn guard_init<T>(
    init: impl FnOnce() -> Result<T, EmotionError>,
) -> Result<T, EmotionError> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(init)).unwrap_or_else(|_| {
        Err(EmotionError::Embedder(
            "cannot initialise ONNX Runtime: the library could not be loaded (check the library path)"
                .into(),
        ))
    })
}

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
    /// ONNX Runtime itself is loaded dynamically from `ORT_DYLIB_PATH` (or the loader's default
    /// search path when unset). Use [`OrtEmbedder::with_library`] to give the path explicitly.
    pub fn new(
        model: &Path,
        tokenizer: &Path,
        prompt: &str,
        dimensions: usize,
    ) -> Result<Self, EmotionError> {
        Self::build(None, model, tokenizer, prompt, dimensions)
    }

    /// Like [`OrtEmbedder::new`], but loads ONNX Runtime from `library` (for example the
    /// `libonnxruntime.so` an Android shell extracts from its package) instead of reading
    /// `ORT_DYLIB_PATH`. The first library loaded in a process is kept for the whole process
    /// (later calls reuse it).
    pub fn with_library(
        library: &Path,
        model: &Path,
        tokenizer: &Path,
        prompt: &str,
        dimensions: usize,
    ) -> Result<Self, EmotionError> {
        Self::build(Some(library), model, tokenizer, prompt, dimensions)
    }

    fn build(
        library: Option<&Path>,
        model: &Path,
        tokenizer: &Path,
        prompt: &str,
        dimensions: usize,
    ) -> Result<Self, EmotionError> {
        let environment = shared_environment(library)?;
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
            .encode(
                assemble_input(&self.prompt, truncate_chars(text, MAX_INPUT_CHARS)),
                true,
            )
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

        // A poisoned lock means an earlier call panicked while holding the session. The session is
        // only a handle to the loaded model and `run` keeps no state between calls, so we recover
        // the guard instead of failing every later message with a permanent error.
        let mut session = self.session.lock().unwrap_or_else(|e| e.into_inner());
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
    fn characters_are_truncated_on_a_boundary() {
        assert_eq!(truncate_chars("abcdef", 3), "abc");
        assert_eq!(truncate_chars("abc", 3), "abc");
        assert_eq!(truncate_chars("éàü", 2), "éà");
        assert_eq!(truncate_chars("", 0), "");
    }

    #[test]
    fn missing_library_is_an_error_not_a_panic() {
        let missing = std::ffi::OsString::from("/nonexistent/libonnxruntime.so");
        assert!(matches!(
            check_library_path(Some(&missing)),
            Err(EmotionError::Embedder(_))
        ));
        assert!(check_library_path(None).is_ok());
        assert!(check_explicit_library(Path::new("/nonexistent/libonnxruntime.so")).is_err());
        let panicked: Result<(), _> = guard_init(|| panic!("simulated ort panic"));
        assert!(matches!(panicked, Err(EmotionError::Embedder(_))));
        assert_eq!(guard_init(|| Ok(7)).unwrap(), 7);
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
