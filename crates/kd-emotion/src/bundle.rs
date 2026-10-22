//! The model bundle: `manifest.json`, `emotion_head.json`, `tokenizer.json`, `model_q4.onnx`
//! (produced by `tools/model-export`, never stored in git). Every file listed in the manifest is
//! checked against its SHA-256 before use.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::{EmotionError, EmotionHead};

/// Manifest file name inside a bundle directory.
pub const MANIFEST_FILE: &str = "manifest.json";
/// Few-shot head file name.
pub const HEAD_FILE: &str = "emotion_head.json";
/// Tokenizer file name.
pub const TOKENIZER_FILE: &str = "tokenizer.json";
/// Quantized model file name.
pub const MODEL_FILE: &str = "model_q4.onnx";

/// `manifest.json`. Unknown fields (versions, quality metrics, …) are ignored so the exporter can
/// extend the manifest without breaking older engines.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    /// Manifest format version.
    pub schema: u32,
    /// Text prepended to every message before tokenization.
    pub prompt: String,
    /// Confidence threshold below which `calme` is used (ADR-0008).
    pub threshold: f32,
    /// Embedding size produced by the model.
    pub embedding_dim: usize,
    /// Emotion ids in head order.
    pub labels: Vec<String>,
    /// SHA-256 (lowercase hex) of every file of the bundle, by file name.
    pub files: BTreeMap<String, String>,
}

impl Manifest {
    /// Parses and validates a manifest.
    pub fn from_json(json: &str) -> Result<Self, EmotionError> {
        let m: Manifest = serde_json::from_str(json)
            .map_err(|e| EmotionError::Bundle(format!("invalid manifest: {e}")))?;
        if !(0.0..=1.0).contains(&m.threshold) {
            return Err(EmotionError::Bundle("threshold must be in [0, 1]".into()));
        }
        if m.embedding_dim == 0 {
            return Err(EmotionError::Bundle(
                "embedding_dim must be positive".into(),
            ));
        }
        for required in [HEAD_FILE, TOKENIZER_FILE, MODEL_FILE] {
            if !m.files.contains_key(required) {
                return Err(EmotionError::Bundle(format!(
                    "manifest does not list `{required}`"
                )));
            }
        }
        // File names are plain names: nothing may point outside the bundle directory.
        if let Some(bad) = m
            .files
            .keys()
            .find(|n| n.is_empty() || n.contains(['/', '\\']) || n.starts_with('.'))
        {
            return Err(EmotionError::Bundle(format!("illegal file name `{bad}`")));
        }
        Ok(m)
    }
}

/// Lowercase hex SHA-256 of a file, streamed (the model is over 100 MB).
pub fn sha256_file(path: &Path) -> Result<String, EmotionError> {
    let io = |e: std::io::Error| EmotionError::Bundle(format!("{}: {e}", path.display()));
    let mut file = File::open(path).map_err(io)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0_u8; 1 << 20];
    loop {
        let n = file.read(&mut buf).map_err(io)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

/// A bundle directory whose files all passed their checksum.
#[derive(Debug, Clone)]
pub struct Bundle {
    dir: PathBuf,
    /// The verified manifest.
    pub manifest: Manifest,
    /// The head parsed from `emotion_head.json`.
    pub head: EmotionHead,
}

impl Bundle {
    /// Reads the manifest, verifies the SHA-256 of every listed file and parses the head.
    pub fn open(dir: impl AsRef<Path>) -> Result<Self, EmotionError> {
        let dir = dir.as_ref().to_path_buf();
        let manifest_path = dir.join(MANIFEST_FILE);
        let json = std::fs::read_to_string(&manifest_path)
            .map_err(|e| EmotionError::Bundle(format!("{}: {e}", manifest_path.display())))?;
        let manifest = Manifest::from_json(&json)?;
        for (name, expected) in &manifest.files {
            let got = sha256_file(&dir.join(name))?;
            if !got.eq_ignore_ascii_case(expected) {
                return Err(EmotionError::ChecksumMismatch {
                    file: name.clone(),
                    expected: expected.clone(),
                    got,
                });
            }
        }
        let head_path = dir.join(HEAD_FILE);
        let head_json = std::fs::read_to_string(&head_path)
            .map_err(|e| EmotionError::Bundle(format!("{}: {e}", head_path.display())))?;
        let head = EmotionHead::from_json(&head_json)?;
        if head.dimensions() != manifest.embedding_dim {
            return Err(EmotionError::DimensionMismatch {
                expected: manifest.embedding_dim,
                got: head.dimensions(),
            });
        }
        if head.label_ids() != manifest.labels {
            return Err(EmotionError::Bundle(
                "manifest labels differ from the head labels".into(),
            ));
        }
        Ok(Self {
            dir,
            manifest,
            head,
        })
    }

    /// Path of a file of the bundle.
    pub fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    /// Builds the engine: loads the model and tokenizer and applies the manifest threshold.
    #[cfg(feature = "ort")]
    pub fn into_engine(
        self,
    ) -> Result<crate::EmotionEngine<crate::ort_embedder::OrtEmbedder>, EmotionError> {
        let embedder = crate::ort_embedder::OrtEmbedder::new(
            &self.path(MODEL_FILE),
            &self.path(TOKENIZER_FILE),
            &self.manifest.prompt,
            self.manifest.embedding_dim,
        )?;
        Ok(crate::EmotionEngine::new(embedder, self.head).with_threshold(self.manifest.threshold))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &str = r#"{"labels":["joie","calme"],"weights":[[1,0],[0,1]],"bias":[0,0]}"#;

    fn sha(bytes: &[u8]) -> String {
        Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    fn manifest_json(head_sha: &str, extra: &str) -> String {
        format!(
            r#"{{"schema":1,"prompt":"p: ","threshold":0.2,"embedding_dim":2,
            "labels":["joie","calme"],{extra}
            "files":{{"emotion_head.json":"{head_sha}","tokenizer.json":"{t}","model_q4.onnx":"{m}"}}}}"#,
            t = sha(b"tok"),
            m = sha(b"model")
        )
    }

    fn write_bundle(name: &str, head_sha: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("kd-bundle-test-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(HEAD_FILE), HEAD).unwrap();
        std::fs::write(dir.join(TOKENIZER_FILE), "tok").unwrap();
        std::fs::write(dir.join(MODEL_FILE), "model").unwrap();
        std::fs::write(
            dir.join(MANIFEST_FILE),
            manifest_json(head_sha, r#""extra":1,"#),
        )
        .unwrap();
        dir
    }

    #[test]
    fn manifest_parses_and_ignores_unknown_fields() {
        let m = Manifest::from_json(&manifest_json("abc", r#""extra":{"a":1},"#)).unwrap();
        assert_eq!(m.prompt, "p: ");
        assert_eq!(m.embedding_dim, 2);
        assert_eq!(m.files.len(), 3);
    }

    #[test]
    fn manifest_rejects_bad_content() {
        assert!(Manifest::from_json("{}").is_err());
        let bad_threshold = manifest_json("abc", "").replace("0.2", "1.5");
        assert!(Manifest::from_json(&bad_threshold).is_err());
        let missing = manifest_json("abc", "").replace("model_q4.onnx", "other.onnx");
        assert!(Manifest::from_json(&missing).is_err());
        let traversal = manifest_json("abc", "").replace("tokenizer.json", "../tokenizer.json");
        assert!(Manifest::from_json(&traversal).is_err());
    }

    #[test]
    fn bundle_with_valid_checksums_opens() {
        let dir = write_bundle("ok", &sha(HEAD.as_bytes()));
        let bundle = Bundle::open(&dir).unwrap();
        assert_eq!(bundle.head.dimensions(), 2);
        assert_eq!(bundle.manifest.threshold, 0.2);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn bundle_with_wrong_checksum_is_rejected() {
        let dir = write_bundle("bad", &sha(b"something else"));
        match Bundle::open(&dir) {
            Err(EmotionError::ChecksumMismatch { file, .. }) => assert_eq!(file, HEAD_FILE),
            other => panic!("expected a checksum mismatch, got {other:?}"),
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn missing_bundle_directory_is_an_error() {
        assert!(Bundle::open("/nonexistent/kd-bundle").is_err());
    }
}
