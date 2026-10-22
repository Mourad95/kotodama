//! Emotion engine ("typed choice", ADR-0003).
//!
//! An [`Embedder`] turns a message into a sentence embedding; an [`EmotionHead`] (a linear layer
//! trained on `prototypes.json` by `tools/model-export`) turns the embedding into a probability
//! per emotion. The ONNX Runtime embedder arrives in milestone M1; until then the engine is fully
//! testable with any [`Embedder`].

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
    labels: Vec<Emotion>,
    weights: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl EmotionHead {
    /// Parses and validates `emotion_head.json`.
    pub fn from_json(json: &str) -> Result<Self, EmotionError> {
        let file: HeadFile =
            serde_json::from_str(json).map_err(|e| EmotionError::InvalidHead(e.to_string()))?;
        let labels = file
            .labels
            .iter()
            .map(|l| {
                Emotion::from_id(l)
                    .ok_or_else(|| EmotionError::InvalidHead(format!("unknown label `{l}`")))
            })
            .collect::<Result<Vec<_>, _>>()?;
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
            labels,
            weights: file.weights,
            bias: file.bias,
        })
    }

    /// Embedding size expected by the head.
    pub fn dimensions(&self) -> usize {
        self.weights[0].len()
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
    /// Emotion to animate after the confidence threshold (ADR-0008).
    pub effective: Emotion,
}

/// The emotion engine: embedder + head + confidence threshold.
pub struct EmotionEngine<E: Embedder> {
    embedder: E,
    head: EmotionHead,
    threshold: f32,
}

impl<E: Embedder> EmotionEngine<E> {
    /// Default confidence threshold (ADR-0008).
    pub const DEFAULT_THRESHOLD: f32 = 0.2;

    /// Creates an engine with the default threshold.
    pub fn new(embedder: E, head: EmotionHead) -> Self {
        Self {
            embedder,
            head,
            threshold: Self::DEFAULT_THRESHOLD,
        }
    }

    /// Overrides the confidence threshold (read from the bundle manifest).
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Analyzes one message.
    pub fn analyze(&self, text: &str) -> Result<EmotionAnalysis, EmotionError> {
        let embedding = self.embedder.embed(text)?;
        let scores = self.head.score(&embedding)?;
        let effective = scores.effective(self.threshold);
        Ok(EmotionAnalysis { scores, effective })
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
        "labels": ["joie", "colere", "calme"],
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

        // Nearly uniform probabilities → falls back to calm.
        let unsure = engine.analyze("ok").unwrap();
        assert!(unsure.scores.confidence < 0.2);
        assert_eq!(unsure.effective, Emotion::Calm);
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
