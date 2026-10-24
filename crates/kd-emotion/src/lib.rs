//! Emotion engine ("typed choice", ADR-0003).
//!
//! An [`Embedder`] turns a message into a sentence embedding; an [`EmotionHead`] (a linear layer
//! trained on `prototypes.json` by `tools/model-export`) turns the embedding into a probability
//! per emotion. The ONNX Runtime embedder ([`OrtEmbedder`], feature `ort`) is loaded from a verified
//! [`Bundle`]; the engine is fully testable with any [`Embedder`].

pub mod bundle;
pub mod emoji;
#[cfg(feature = "ort")]
pub mod ort_embedder;

pub use bundle::{Bundle, Manifest};
pub use emoji::EmojiMap;
#[cfg(feature = "ort")]
pub use ort_embedder::OrtEmbedder;

use kd_core::{softmax, Emotion, EmotionScores};
use serde::Deserialize;
use thiserror::Error;

/// Errors of the emotion engine.
#[derive(Debug, Error)]
pub enum EmotionError {
    /// The head file could not be parsed.
    #[error("invalid emotion head: {0}")]
    InvalidHead(String),
    /// Embedding size does not match the head.
    #[error("embedding has {got} dimensions, head expects {expected}")]
    DimensionMismatch {
        /// Dimensions expected by the head.
        expected: usize,
        /// Dimensions produced by the embedder.
        got: usize,
    },
    /// The model bundle is missing, malformed or inconsistent.
    #[error("invalid bundle: {0}")]
    Bundle(String),
    /// A bundle file does not match the SHA-256 recorded in the manifest.
    #[error("checksum mismatch for `{file}`: expected {expected}, got {got}")]
    ChecksumMismatch {
        /// File name inside the bundle.
        file: String,
        /// Checksum recorded in the manifest.
        expected: String,
        /// Checksum of the file on disk.
        got: String,
    },
    /// The emoji map could not be parsed.
    #[error("invalid emoji map: {0}")]
    InvalidEmoji(String),
    /// The embedder failed.
    #[error("embedder failed: {0}")]
    Embedder(String),
}

/// Produces a sentence embedding for a message.
pub trait Embedder: Send + Sync {
    /// Embeds one message. The vector must be L2-normalized.
    fn embed(&self, text: &str) -> Result<Vec<f32>, EmotionError>;
}

/// Raw `emotion_head.json` as written by `tools/model-export`.
#[derive(Debug, Deserialize)]
struct HeadFile {
    labels: Vec<String>,
    weights: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

/// Linear few-shot head: `softmax(W · embedding + b)`.
#[derive(Debug, Clone)]
pub struct EmotionHead {
    ids: Vec<String>,
    labels: Vec<Emotion>,
    weights: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl EmotionHead {
    /// Parses and validates `emotion_head.json`.
    pub fn from_json(json: &str) -> Result<Self, EmotionError> {
        let file: HeadFile =
            serde_json::from_str(json).map_err(|e| EmotionError::InvalidHead(e.to_string()))?;
        // Heads trained before ADR-0021 have no `neutre`: their `calme` holds neutral messages.
        let legacy = !file.labels.iter().any(|l| l == Emotion::Neutral.id());
        let labels = file
            .labels
            .iter()
            .map(|l| match Emotion::from_id(l) {
                Some(Emotion::Calm) if legacy => Ok(Emotion::Neutral),
                Some(e) => Ok(e),
                None => Err(EmotionError::InvalidHead(format!("unknown label `{l}`"))),
            })
            .collect::<Result<Vec<_>, _>>()?;
        for (i, label) in labels.iter().enumerate() {
            if labels[..i].contains(label) {
                return Err(EmotionError::InvalidHead(format!(
                    "duplicate label `{}`",
                    label.id()
                )));
            }
        }
        if labels.is_empty()
            || file.weights.len() != labels.len()
            || file.bias.len() != labels.len()
        {
            return Err(EmotionError::InvalidHead(
                "labels, weights and bias must have the same non-zero length".into(),
            ));
        }
        let dim = file.weights[0].len();
        if dim == 0 || file.weights.iter().any(|row| row.len() != dim) {
            return Err(EmotionError::InvalidHead(
                "all weight rows must have the same non-zero length".into(),
            ));
        }
        Ok(Self {
            ids: file.labels,
            labels,
            weights: file.weights,
            bias: file.bias,
        })
    }

    /// Embedding size expected by the head.
    pub fn dimensions(&self) -> usize {
        self.weights[0].len()
    }

    /// Label ids in head order, as written in the head file.
    pub fn label_ids(&self) -> Vec<String> {
        self.ids.clone()
    }

    /// Scores one embedding.
    pub fn score(&self, embedding: &[f32]) -> Result<EmotionScores, EmotionError> {
        if embedding.len() != self.dimensions() {
            return Err(EmotionError::DimensionMismatch {
                expected: self.dimensions(),
                got: embedding.len(),
            });
        }
        let logits: Vec<f32> = self
            .weights
            .iter()
            .zip(&self.bias)
            .map(|(row, b)| row.iter().zip(embedding).map(|(w, x)| w * x).sum::<f32>() + b)
            .collect();
        let probabilities = self.labels.iter().copied().zip(softmax(&logits)).collect();
        EmotionScores::from_probabilities(probabilities)
            .ok_or_else(|| EmotionError::InvalidHead("empty head".into()))
    }
}

/// Analysis of one message.
#[derive(Debug, Clone, PartialEq)]
pub struct EmotionAnalysis {
    /// Raw scores from the engine.
    pub scores: EmotionScores,
    /// Emotion to animate: the emoji's emotion when the message has a known emoji (ADR-0028),
    /// otherwise the model's choice after the confidence threshold (ADR-0008).
    pub effective: Emotion,
    /// `effective` was revealed by the message's emoji rather than by the model.
    pub from_emoji: bool,
}

/// The emotion engine: embedder + head + confidence threshold, with emoji first (ADR-0028).
pub struct EmotionEngine<E: Embedder> {
    embedder: E,
    head: EmotionHead,
    threshold: f32,
    emoji: EmojiMap,
}

impl<E: Embedder> EmotionEngine<E> {
    /// Default confidence threshold (ADR-0008, raised by ADR-0026).
    pub const DEFAULT_THRESHOLD: f32 = 0.5;

    /// Creates an engine with the default threshold.
    pub fn new(embedder: E, head: EmotionHead) -> Self {
        Self {
            embedder,
            head,
            threshold: Self::DEFAULT_THRESHOLD,
            emoji: EmojiMap::builtin(),
        }
    }

    /// Overrides the confidence threshold (read from the bundle manifest).
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Analyzes one message.
    ///
    /// The model always scores the message; a known emoji then decides the emotion to animate,
    /// whatever the model's confidence (ADR-0028).
    pub fn analyze(&self, text: &str) -> Result<EmotionAnalysis, EmotionError> {
        let embedding = self.embedder.embed(text)?;
        let scores = self.head.score(&embedding)?;
        let analysis = match self.emoji.emotion(text) {
            Some(effective) => EmotionAnalysis {
                scores,
                effective,
                from_emoji: true,
            },
            None => EmotionAnalysis {
                effective: scores.effective(self.threshold),
                scores,
                from_emoji: false,
            },
        };
        Ok(analysis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test embedder: one-hot vector chosen by a keyword.
    struct KeywordEmbedder;

    impl Embedder for KeywordEmbedder {
        fn embed(&self, text: &str) -> Result<Vec<f32>, EmotionError> {
            let mut v = vec![0.0; 3];
            if text.contains("bravo") {
                v[0] = 1.0;
            } else if text.contains("marre") {
                v[1] = 1.0;
            } else {
                v[2] = 1.0;
            }
            Ok(v)
        }
    }

    const HEAD: &str = r#"{
        "labels": ["joie", "colere", "neutre"],
        "weights": [[8.0, 0.0, 0.0], [0.0, 8.0, 0.0], [0.0, 0.0, 0.1]],
        "bias": [0.0, 0.0, 0.0]
    }"#;

    #[test]
    fn head_scores_and_engine_applies_threshold() {
        let engine = EmotionEngine::new(KeywordEmbedder, EmotionHead::from_json(HEAD).unwrap());

        let joy = engine.analyze("bravo !").unwrap();
        assert_eq!(joy.scores.choice, Emotion::Joy);
        assert_eq!(joy.effective, Emotion::Joy);
        let sum: f32 = joy.scores.probabilities.iter().map(|(_, p)| p).sum();
        assert!((sum - 1.0).abs() < 1e-5);

        let anger = engine.analyze("j'en ai marre").unwrap();
        assert_eq!(anger.effective, Emotion::Anger);

        // Nearly uniform probabilities → falls back to neutral.
        let unsure = engine.analyze("ok").unwrap();
        assert!(unsure.scores.confidence < 0.2);
        assert_eq!(unsure.effective, Emotion::Neutral);
    }

    #[test]
    fn emoji_decide_the_emotion_and_the_model_stays_the_fallback() {
        let engine = EmotionEngine::new(KeywordEmbedder, EmotionHead::from_json(HEAD).unwrap());

        // The model reads joy, the emoji reveal energy.
        let fire = engine.analyze("bravo 🔥💪").unwrap();
        assert_eq!(fire.scores.choice, Emotion::Joy);
        assert_eq!(fire.effective, Emotion::Energy);
        assert!(fire.from_emoji);

        // Below the threshold the emoji still decide.
        let unsure = engine.analyze("ok 😭").unwrap();
        assert!(unsure.scores.confidence < 0.2);
        assert_eq!(unsure.effective, Emotion::Sadness);

        // No known emoji: the model decides.
        let plain = engine.analyze("bravo 👍").unwrap();
        assert_eq!(plain.effective, Emotion::Joy);
        assert!(!plain.from_emoji);
    }

    #[test]
    fn legacy_heads_read_calme_as_neutral() {
        let legacy = HEAD.replace("neutre", "calme");
        let head = EmotionHead::from_json(&legacy).unwrap();
        assert_eq!(head.label_ids(), ["joie", "colere", "calme"]);
        let engine = EmotionEngine::new(KeywordEmbedder, head);
        let scores = engine.analyze("ok").unwrap().scores;
        assert!(scores
            .probabilities
            .iter()
            .any(|(e, _)| *e == Emotion::Neutral));
        assert!(scores
            .probabilities
            .iter()
            .all(|(e, _)| *e != Emotion::Calm));

        // With `neutre` present, `calme` is the calm emotion.
        let both = r#"{"labels":["calme","neutre"],"weights":[[1],[0]],"bias":[0,0]}"#;
        let scores = EmotionHead::from_json(both).unwrap().score(&[9.0]).unwrap();
        assert_eq!(scores.choice, Emotion::Calm);
    }

    #[test]
    fn full_heads_can_select_embarrassment() {
        let mut bias = vec![0.0; Emotion::ALL.len()];
        let at = Emotion::ALL
            .iter()
            .position(|e| *e == Emotion::Embarrassment);
        bias[at.unwrap()] = 9.0;
        let json = serde_json::json!({
            "labels": Emotion::ALL.map(Emotion::id),
            "weights": vec![vec![0.0; 3]; Emotion::ALL.len()],
            "bias": bias,
        });
        let engine = EmotionEngine::new(
            KeywordEmbedder,
            EmotionHead::from_json(&json.to_string()).unwrap(),
        );
        let result = engine.analyze("A test message").unwrap();
        assert_eq!(result.effective, Emotion::Embarrassment);
        assert_eq!(result.scores.probabilities.len(), Emotion::ALL.len());
    }

    #[test]
    fn head_rejects_bad_files() {
        assert!(EmotionHead::from_json(r#"{"labels":["joie"],"weights":[],"bias":[0]}"#).is_err());
        assert!(
            EmotionHead::from_json(r#"{"labels":["happy"],"weights":[[1]],"bias":[0]}"#).is_err()
        );
        assert!(EmotionHead::from_json(
            r#"{"labels":["joie","calme"],"weights":[[1,2],[1]],"bias":[0,0]}"#
        )
        .is_err());
        assert!(EmotionHead::from_json(
            r#"{"labels":["joie","joie"],"weights":[[1],[1]],"bias":[0,0]}"#
        )
        .is_err());
    }

    #[test]
    fn head_checks_embedding_size() {
        let head = EmotionHead::from_json(HEAD).unwrap();
        assert!(matches!(
            head.score(&[1.0]),
            Err(EmotionError::DimensionMismatch {
                expected: 3,
                got: 1
            })
        ));
    }
}
