//! Effects engine (ADR-0005).
//!
//! A theme is data: for each emotion and each intensity level it describes how letters enter,
//! which particles and screen effects play, which onomatopoeias appear. This crate parses and
//! validates themes; the `wgpu` renderer that interprets them arrives in milestone M3.

use lt_core::{Emotion, Intensity};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

/// Current version of the theme format. Bump on breaking changes, with a migration.
pub const SCHEMA_VERSION: u32 = 1;

/// Errors raised while loading a theme.
#[derive(Debug, Error)]
pub enum ThemeError {
    /// Not valid JSON, or does not match the format.
    #[error("invalid theme JSON: {0}")]
    Parse(String),
    /// Valid JSON but inconsistent content.
    #[error("invalid theme `{theme}`: {reason}")]
    Invalid {
        /// Theme identifier.
        theme: String,
        /// What is wrong.
        reason: String,
    },
}

/// A complete theme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    /// Format version, must equal [`SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable identifier (`anime`, `minimal`…).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Theme version, bumped on every content change.
    pub version: String,
    /// Motion per emotion. `calme` is mandatory: it is the fallback for any missing emotion.
    pub emotions: BTreeMap<Emotion, EmotionMotion>,
}

/// Motion of one emotion: exactly three levels, from [`Intensity::Low`] to [`Intensity::Full`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmotionMotion {
    /// Levels 1, 2 and 3, in this order.
    pub levels: Vec<Level>,
}

/// Everything that plays for one emotion at one intensity level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Level {
    /// 1, 2 or 3.
    pub level: u8,
    /// Text colour, `#RRGGBB`.
    pub text_color: String,
    /// How letters appear.
    pub letters: LetterEntrance,
    /// Particle emitters.
    #[serde(default)]
    pub particles: Vec<ParticleEmitter>,
    /// Full-screen effects.
    #[serde(default)]
    pub screen: Vec<ScreenEffect>,
    /// Onomatopoeias drawn around the message.
    #[serde(default)]
    pub onomatopoeias: Vec<Onomatopoeia>,
}

/// Letter entrance animation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LetterEntrance {
    /// Animation kind.
    pub kind: LetterKind,
    /// Duration of one letter's entrance, in milliseconds.
    pub duration_ms: u32,
    /// Delay between two consecutive letters, in milliseconds.
    pub stagger_ms: u32,
}

/// Built-in letter entrances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LetterKind {
    /// Fade and rise.
    Rise,
    /// Scale up with overshoot.
    Pop,
    /// Arrive huge and slam into place.
    Impact,
    /// Charge with flickering glow.
    Charge,
    /// Appear one by one like typing.
    Type,
    /// Fall slowly from above.
    Drip,
    /// Bloom from a rotated small scale.
    Bloom,
    /// Faulty word glitch (used for words with errors).
    Glitch,
}

/// A particle emitter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParticleEmitter {
    /// Particle shape and behaviour.
    pub kind: ParticleKind,
    /// Number of particles.
    pub count: u32,
    /// Colours, `#RRGGBB`, cycled over particles.
    pub colors: Vec<String>,
    /// Particle lifetime in milliseconds.
    pub lifetime_ms: u32,
}

/// Built-in particle kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticleKind {
    /// Rectangular confetti with gravity.
    Confetti,
    /// Four-point twinkling stars.
    Stars,
    /// Rising flame-like aura around the message.
    Aura,
    /// Hearts rising from the bottom.
    Hearts,
    /// Falling petals.
    Petals,
    /// Rain streaks.
    Rain,
    /// Embers rising from the message.
    Embers,
}

/// A full-screen effect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScreenEffect {
    /// Effect kind.
    pub kind: ScreenKind,
    /// Start offset from the message entrance, in milliseconds.
    #[serde(default)]
    pub at_ms: u32,
    /// Duration in milliseconds.
    pub duration_ms: u32,
    /// Strength between 0 and 1.
    pub strength: f32,
}

/// Built-in screen effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenKind {
    /// Manga focus lines converging on the message.
    SpeedLines,
    /// Black/white impact frame.
    ImpactFlash,
    /// Expanding rings.
    Shockwave,
    /// Darkened screen with a spotlight on the message.
    Spotlight,
    /// Tinted overlay.
    Tint,
    /// Screen shake.
    Shake,
}

/// Onomatopoeia text drawn on screen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Onomatopoeia {
    /// Text to draw.
    pub text: String,
    /// Start offset in milliseconds.
    #[serde(default)]
    pub at_ms: u32,
    /// Colour, `#RRGGBB`.
    pub color: String,
    /// Size relative to the message text (1.0 = same size).
    pub scale: f32,
}

impl Theme {
    /// Parses and validates a theme.
    pub fn from_json(json: &str) -> Result<Self, ThemeError> {
        let theme: Theme =
            serde_json::from_str(json).map_err(|e| ThemeError::Parse(e.to_string()))?;
        theme.validate()?;
        Ok(theme)
    }

    /// Motion for an emotion at an intensity, falling back to `calme` if the emotion is absent.
    pub fn level(&self, emotion: Emotion, intensity: Intensity) -> &Level {
        let motion = self
            .emotions
            .get(&emotion)
            .or_else(|| self.emotions.get(&Emotion::Calm))
            .expect("validated themes always define calme");
        &motion.levels[usize::from(intensity.level() - 1)]
    }

    fn validate(&self) -> Result<(), ThemeError> {
        let fail = |reason: String| ThemeError::Invalid {
            theme: self.id.clone(),
            reason,
        };
        if self.schema_version != SCHEMA_VERSION {
            return Err(fail(format!(
                "schema_version {} unsupported (expected {SCHEMA_VERSION})",
                self.schema_version
            )));
        }
        if !self.emotions.contains_key(&Emotion::Calm) {
            return Err(fail("`calme` must be defined (fallback emotion)".into()));
        }
        for (emotion, motion) in &self.emotions {
            let numbers: Vec<u8> = motion.levels.iter().map(|l| l.level).collect();
            if numbers != [1, 2, 3] {
                return Err(fail(format!("{emotion}: levels must be 1, 2, 3 in order")));
            }
            for level in &motion.levels {
                let colors = std::iter::once(&level.text_color)
                    .chain(level.particles.iter().flat_map(|p| &p.colors))
                    .chain(level.onomatopoeias.iter().map(|o| &o.color));
                for color in colors {
                    if !is_hex_color(color) {
                        return Err(fail(format!("{emotion}: `{color}` is not #RRGGBB")));
                    }
                }
                if level.letters.duration_ms == 0 {
                    return Err(fail(format!("{emotion}: letter duration must be > 0")));
                }
                if level
                    .screen
                    .iter()
                    .any(|s| !(0.0..=1.0).contains(&s.strength) || s.duration_ms == 0)
                {
                    return Err(fail(format!(
                        "{emotion}: screen effect strength must be in [0, 1] and duration > 0"
                    )));
                }
            }
        }
        Ok(())
    }
}

fn is_hex_color(s: &str) -> bool {
    s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANIME: &str = include_str!("../../../themes/anime/theme.json");

    #[test]
    fn anime_theme_is_valid() {
        let theme = Theme::from_json(ANIME).unwrap();
        assert_eq!(theme.id, "anime");
        let full = theme.level(Emotion::Energy, Intensity::Full);
        assert_eq!(full.level, 3);
        assert!(full
            .screen
            .iter()
            .any(|s| s.kind == ScreenKind::ImpactFlash));
    }

    #[test]
    fn missing_emotion_falls_back_to_calm() {
        let theme = Theme::from_json(ANIME).unwrap();
        let calm = theme.level(Emotion::Calm, Intensity::Low);
        let missing = theme.level(Emotion::Sadness, Intensity::Low);
        assert_eq!(calm, missing);
    }

    #[test]
    fn invalid_themes_are_rejected() {
        let no_calm = ANIME.replace("\"calme\"", "\"amour\"");
        assert!(matches!(
            Theme::from_json(&no_calm),
            Err(ThemeError::Invalid { .. })
        ));

        let bad_color = ANIME.replacen("#0C447C", "blue", 1);
        assert!(Theme::from_json(&bad_color).is_err());

        let unknown_field = ANIME.replacen("\"name\"", "\"nmae\"", 1);
        assert!(matches!(
            Theme::from_json(&unknown_field),
            Err(ThemeError::Parse(_))
        ));
    }
}
