//! Shared types and pure logic for KotoMotion.
//!
//! This crate has no internal dependency and no I/O: it defines the vocabulary every other crate
//! speaks (emotions, scores, intensity, languages) and the small pure functions around it.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The seven emotions the app animates.
///
/// Data files (prototypes, themes, benchmark) use the historical French identifiers returned by
/// [`Emotion::id`]; code uses this English enum (see ADR-0001).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Emotion {
    /// Happiness, good news, laughter, pride.
    #[serde(rename = "joie")]
    Joy,
    /// Anger, annoyance, unfairness, blame.
    #[serde(rename = "colere")]
    Anger,
    /// Motivation, hype, determination.
    #[serde(rename = "energie")]
    Energy,
    /// Curiosity, secret, rumour, mystery.
    #[serde(rename = "suspense")]
    Suspense,
    /// Sadness, grief, failure, loneliness.
    #[serde(rename = "tristesse")]
    Sadness,
    /// Love, affection, warm thanks.
    #[serde(rename = "amour")]
    Love,
    /// Neutral or practical message. Also the fallback when the engine is unsure.
    #[serde(rename = "calme")]
    Calm,
    /// Embarrassment and awkwardness. Available for explicit selection; legacy heads have no label for it.
    #[serde(rename = "genance")]
    Embarrassment,
}

impl Emotion {
    /// All emotions, in the canonical order used by models and data files.
    pub const ALL: [Emotion; 8] = [
        Emotion::Joy,
        Emotion::Anger,
        Emotion::Energy,
        Emotion::Suspense,
        Emotion::Sadness,
        Emotion::Love,
        Emotion::Calm,
        Emotion::Embarrassment,
    ];

    /// Identifier used in data files (`joie`, `colere`, …).
    pub fn id(self) -> &'static str {
        match self {
            Emotion::Joy => "joie",
            Emotion::Anger => "colere",
            Emotion::Energy => "energie",
            Emotion::Suspense => "suspense",
            Emotion::Sadness => "tristesse",
            Emotion::Love => "amour",
            Emotion::Calm => "calme",
            Emotion::Embarrassment => "genance",
        }
    }

    /// Parses a data-file identifier.
    pub fn from_id(id: &str) -> Option<Emotion> {
        Emotion::ALL.into_iter().find(|e| e.id() == id)
    }
}

impl fmt::Display for Emotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

/// Result of the emotion engine for one message ("typed choice", ADR-0003).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmotionScores {
    /// Most probable emotion.
    pub choice: Emotion,
    /// Probability of each emotion, summing to 1.
    pub probabilities: Vec<(Emotion, f32)>,
    /// 1 when all probability mass is on one emotion, 0 when it is uniform.
    pub confidence: f32,
}

impl EmotionScores {
    /// Builds scores from a probability per emotion; picks the choice and computes confidence.
    ///
    /// Returns `None` if `probabilities` is empty.
    pub fn from_probabilities(probabilities: Vec<(Emotion, f32)>) -> Option<Self> {
        let choice = probabilities
            .iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, _)| *e)?;
        let probs: Vec<f32> = probabilities.iter().map(|(_, p)| *p).collect();
        Some(Self {
            choice,
            confidence: confidence(&probs),
            probabilities,
        })
    }

    /// The emotion to animate: the choice, or [`Emotion::Calm`] below `threshold` (ADR-0008).
    pub fn effective(&self, threshold: f32) -> Emotion {
        if self.confidence < threshold {
            Emotion::Calm
        } else {
            self.choice
        }
    }
}

/// Confidence of a probability distribution: `1 - entropy / ln(n)`.
///
/// Matches the definition used by the benchmark (`research/emotion-benchmark/bench.py`).
pub fn confidence(probabilities: &[f32]) -> f32 {
    let n = probabilities.len();
    if n < 2 {
        return 1.0;
    }
    let entropy: f32 = probabilities
        .iter()
        .filter(|p| **p > 0.0)
        .map(|p| -p * p.ln())
        .sum();
    (1.0 - entropy / (n as f32).ln()).clamp(0.0, 1.0)
}

/// Numerically stable softmax.
pub fn softmax(logits: &[f32]) -> Vec<f32> {
    let max = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = logits.iter().map(|l| (l - max).exp()).collect();
    let sum: f32 = exps.iter().sum();
    exps.into_iter().map(|e| e / sum).collect()
}

/// Animation intensity, driven by writing quality: the better the message is written,
/// the "crazier" the animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Intensity {
    /// Several issues: calm animation, faulty words glitch.
    Low = 1,
    /// One error left.
    Medium = 2,
    /// No error: full animation.
    Full = 3,
}

impl Intensity {
    /// Intensity from the number of issues in a message.
    ///
    /// Initial rule from the prototypes: 0 error → full, 1 → medium, 2+ → low.
    /// Kept deliberately simple; it will be tuned with beta data (roadmap M5).
    pub fn from_issue_count(issues: usize) -> Intensity {
        match issues {
            0 => Intensity::Full,
            1 => Intensity::Medium,
            _ => Intensity::Low,
        }
    }

    /// Level as a number from 1 to 3, as used in theme files.
    pub fn level(self) -> u8 {
        self as u8
    }
}

/// Languages with a grammar provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// French (first market).
    Fr,
    /// English.
    En,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emotion_ids_roundtrip() {
        for e in Emotion::ALL {
            assert_eq!(Emotion::from_id(e.id()), Some(e));
        }
        assert_eq!(Emotion::from_id("unknown"), None);
    }

    #[test]
    fn emotion_serde_uses_data_ids() {
        assert_eq!(
            serde_json::to_string(&Emotion::Anger).unwrap(),
            "\"colere\""
        );
        let e: Emotion = serde_json::from_str("\"energie\"").unwrap();
        assert_eq!(e, Emotion::Energy);
    }

    #[test]
    fn confidence_bounds() {
        assert!((confidence(&[1.0, 0.0, 0.0]) - 1.0).abs() < 1e-6);
        assert!(confidence(&[1.0 / 3.0; 3]).abs() < 1e-6);
        let c = confidence(&[0.7, 0.2, 0.1]);
        assert!(c > 0.0 && c < 1.0);
    }

    #[test]
    fn softmax_sums_to_one_and_is_stable() {
        let p = softmax(&[1000.0, 1001.0, 999.0]);
        assert!((p.iter().sum::<f32>() - 1.0).abs() < 1e-6);
        assert!(p[1] > p[0] && p[0] > p[2]);
    }

    #[test]
    fn scores_fall_back_to_calm_when_unsure() {
        let unsure = EmotionScores::from_probabilities(
            Emotion::ALL
                .iter()
                .map(|e| (*e, 1.0 / Emotion::ALL.len() as f32))
                .collect(),
        )
        .unwrap();
        assert_eq!(unsure.effective(0.2), Emotion::Calm);

        let sure = EmotionScores::from_probabilities(vec![
            (Emotion::Joy, 0.97),
            (Emotion::Anger, 0.01),
            (Emotion::Calm, 0.02),
        ])
        .unwrap();
        assert_eq!(sure.choice, Emotion::Joy);
        assert_eq!(sure.effective(0.2), Emotion::Joy);
    }

    #[test]
    fn intensity_from_issues() {
        assert_eq!(Intensity::from_issue_count(0), Intensity::Full);
        assert_eq!(Intensity::from_issue_count(1), Intensity::Medium);
        assert_eq!(Intensity::from_issue_count(5), Intensity::Low);
        assert_eq!(Intensity::Full.level(), 3);
    }
}
