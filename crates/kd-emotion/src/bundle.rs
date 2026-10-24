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

/// The only manifest format version this engine understands.
pub const SUPPORTED_SCHEMA: u32 = 1;
/// Upper bound on `embedding_dim`, so a hostile manifest cannot request huge allocations.
pub const MAX_EMBEDDING_DIM: usize = 8192;

/// Whitelist for bundle file names: non-empty, `[A-Za-z0-9._-]` only, no leading dot.
fn is_plain_file_name(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with('.')
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

/// `manifest.json`. Unknown fields (versions, quality metrics, …) are ignored so the exporter can
/// extend the manifest without breaking older engines.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    /// Manifest format version.
    pub schema: u32,
    /// Text prepended to every message before tokenization.
    pub prompt: String,
    /// Confidence threshold below which `neutre` is used (ADR-0008, ADR-0021).
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
        if m.schema != SUPPORTED_SCHEMA {
            return Err(EmotionError::Bundle(format!(
                "unsupported manifest schema {} (expected {SUPPORTED_SCHEMA})",
                m.schema
            )));
        }
        if m.embedding_dim == 0 || m.embedding_dim > MAX_EMBEDDING_DIM {
            return Err(EmotionError::Bundle(format!(
                "embedding_dim must be in 1..={MAX_EMBEDDING_DIM}"
            )));
        }
        for required in [HEAD_FILE, TOKENIZER_FILE, MODEL_FILE] {
            if !m.files.contains_key(required) {
                return Err(EmotionError::Bundle(format!(
                    "manifest does not list `{required}`"
                )));
            }
        }
        // File names are plain names: nothing may point outside the bundle directory.
        if let Some(bad) = m.files.keys().find(|n| !is_plain_file_name(n)) {
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

/// What a verification receipt records: the manifest digest and the size of every listed file.
/// `None` if a listed file cannot be stat-ed.
fn fingerprint(dir: &Path, manifest_json: &str, manifest: &Manifest) -> Option<String> {
    let mut out = format!(
        "kd-receipt-1\n{:x}\n",
        Sha256::digest(manifest_json.as_bytes())
    );
    for name in manifest.files.keys() {
        let len = std::fs::metadata(dir.join(name)).ok()?.len();
        out.push_str(&format!("{name}:{len}\n"));
    }
    Some(out)
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
        Self::open_inner(dir.as_ref(), None)
    }

    /// Like [`Bundle::open`], but hashes the files only once: after a full verification a
    /// receipt is written to `receipt` (ADR-0014). While the manifest and the size of every file
    /// still match the receipt, later calls skip the hashing (about a second on a phone).
    ///
    /// The receipt must live where only the app can write (its private storage): whoever can
    /// write there can already replace the app itself. A missing, unreadable or stale receipt
    /// only means a full verification; failing to write it is not an error.
    pub fn open_with_receipt(
        dir: impl AsRef<Path>,
        receipt: impl AsRef<Path>,
    ) -> Result<Self, EmotionError> {
        Self::open_inner(dir.as_ref(), Some(receipt.as_ref()))
    }

    fn open_inner(dir: &Path, receipt: Option<&Path>) -> Result<Self, EmotionError> {
        let dir = dir.to_path_buf();
        let manifest_path = dir.join(MANIFEST_FILE);
        let json = std::fs::read_to_string(&manifest_path)
            .map_err(|e| EmotionError::Bundle(format!("{}: {e}", manifest_path.display())))?;
        let manifest = Manifest::from_json(&json)?;
        let fingerprint = receipt.and_then(|_| fingerprint(&dir, &json, &manifest));
        let already_verified = match (receipt, &fingerprint) {
            (Some(path), Some(now)) => {
                std::fs::read_to_string(path).is_ok_and(|saved| saved == *now)
            }
            _ => false,
        };
        if !already_verified {
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
            if let (Some(path), Some(now)) = (receipt, &fingerprint) {
                let _ = std::fs::write(path, now);
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
    /// ONNX Runtime is loaded from `ORT_DYLIB_PATH`.
    #[cfg(feature = "ort")]
    pub fn into_engine(
        self,
    ) -> Result<crate::EmotionEngine<crate::ort_embedder::OrtEmbedder>, EmotionError> {
        self.build_engine(None)
    }

    /// Like [`Bundle::into_engine`], but loads ONNX Runtime from the given library path.
    #[cfg(feature = "ort")]
    pub fn into_engine_with_library(
        self,
        library: &Path,
    ) -> Result<crate::EmotionEngine<crate::ort_embedder::OrtEmbedder>, EmotionError> {
        self.build_engine(Some(library))
    }

    #[cfg(feature = "ort")]
    fn build_engine(
        self,
        library: Option<&Path>,
    ) -> Result<crate::EmotionEngine<crate::ort_embedder::OrtEmbedder>, EmotionError> {
        let (model, tokenizer) = (self.path(MODEL_FILE), self.path(TOKENIZER_FILE));
        let (prompt, dim) = (&self.manifest.prompt, self.manifest.embedding_dim);
        let embedder = match library {
            Some(lib) => {
                crate::ort_embedder::OrtEmbedder::with_library(lib, &model, &tokenizer, prompt, dim)
            }
            None => crate::ort_embedder::OrtEmbedder::new(&model, &tokenizer, prompt, dim),
        }?;
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
        for bad in [".hidden", "a b", "a:b", "é.json", "a\\u0000b", "a/b"] {
            let json = manifest_json("abc", "").replace("tokenizer.json", bad);
            assert!(Manifest::from_json(&json).is_err(), "{bad:?} accepted");
        }
        let empty = manifest_json("abc", "").replace("\"tokenizer.json\"", "\"\"");
        assert!(Manifest::from_json(&empty).is_err());
        let schema2 = manifest_json("abc", "").replace("\"schema\":1", "\"schema\":2");
        assert!(Manifest::from_json(&schema2).is_err());
        for dim in ["0", "8193"] {
            let json = manifest_json("abc", "")
                .replace("\"embedding_dim\":2", &format!("\"embedding_dim\":{dim}"));
            assert!(Manifest::from_json(&json).is_err(), "dim {dim} accepted");
        }
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
    fn receipt_skips_hashing_until_something_changes() {
        let dir = write_bundle("receipt", &sha(HEAD.as_bytes()));
        let receipt = dir.join("receipt.txt");
        Bundle::open_with_receipt(&dir, &receipt).unwrap();
        assert!(receipt.is_file(), "a verified bundle leaves a receipt");
        // Same size, different content: the receipt hides it (documented trust model)...
        std::fs::write(dir.join(MODEL_FILE), "MODEL").unwrap();
        Bundle::open_with_receipt(&dir, &receipt).unwrap();
        // ...but plain `open` and any size change still catch it.
        assert!(matches!(
            Bundle::open(&dir),
            Err(EmotionError::ChecksumMismatch { .. })
        ));
        std::fs::write(dir.join(MODEL_FILE), "model2").unwrap();
        assert!(Bundle::open_with_receipt(&dir, &receipt).is_err());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn corrupt_receipt_means_a_full_verification() {
        let dir = write_bundle("receipt-bad", &sha(b"something else"));
        let receipt = dir.join("receipt.txt");
        std::fs::write(&receipt, "kd-receipt-1\nnonsense\n").unwrap();
        assert!(matches!(
            Bundle::open_with_receipt(&dir, &receipt),
            Err(EmotionError::ChecksumMismatch { .. })
        ));
        assert_eq!(
            std::fs::read_to_string(&receipt).unwrap(),
            "kd-receipt-1\nnonsense\n",
            "a failed verification never writes a receipt"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn missing_bundle_directory_is_an_error() {
        assert!(Bundle::open("/nonexistent/kd-bundle").is_err());
    }
}
