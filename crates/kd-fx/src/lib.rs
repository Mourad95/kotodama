//! Effects engine (ADR-0005).
//!
//! A theme is data: for each emotion and each intensity level it describes how letters enter,
//! which particles and screen effects play, which onomatopoeias appear. This crate parses and
//! validates themes, and [`scene::sample`] turns a level into what to draw at a given time. The
//! renderers that draw it (a CPU reference renderer, then `wgpu`) come in milestone M3 (ADR-0015).

pub mod scene;

#[cfg(feature = "cpu-render")]
pub mod render;

#[cfg(feature = "cpu-render")]
mod manga;
#[cfg(feature = "cpu-render")]
mod scatter;

use kd_core::{Emotion, Intensity};
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
    /// Optional ink illustration, shared by the animated overlay and the settled bubble.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manga: Option<MangaStyle>,
}

/// Data-driven illustration language; motifs describe drawing primitives, not theme identifiers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MangaStyle {
    /// Ink composition around the message.
    pub motif: MangaMotif,
    /// Highlight colour, distinct from the darker readable text colour.
    pub accent_color: String,
    /// Amount of offset lettering and perspective, between zero and one.
    pub depth: f32,
    /// Colours of the motif's scattered drawings (stars, flames, hearts…), `#RRGGBB`, used in turn;
    /// empty uses the accent colour.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub palette: Vec<String>,
    /// Full-screen colour panel behind the effects, around the message (ADR-0020).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backdrop: Option<Backdrop>,
}

/// A full-screen colour panel: `inner` around the message fading to `outer` at the screen edges. The
/// message's own rect is always left clear.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Backdrop {
    /// Colour around the message, `#RRGGBB`.
    pub inner: String,
    /// Colour at the screen edges, `#RRGGBB`.
    pub outer: String,
    /// Peak opacity, between zero and one.
    pub opacity: f32,
    /// Manga speed lines bursting from the message to the screen edges.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rays: Option<Rays>,
}

/// Radial speed lines over the backdrop: thin wedges from around the message out to the screen edges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rays {
    /// Number of lines around the message, between 8 and 128.
    pub count: u32,
    /// Colour of the lines, `#RRGGBB`.
    pub color: String,
    /// Opacity of the lines, between zero and one.
    pub opacity: f32,
}

/// Original manga illustration primitives understood by the renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MangaMotif {
    /// Warm rays and four-point glints.
    Radiance,
    /// Jagged ink pressure and an anger mark.
    Pressure,
    /// Perspective energy ribbon and angular shards.
    Orbit,
    /// Converging hatching and a dark local aura.
    Tension,
    /// Descending gloom hatching and ink drops.
    Rainfall,
    /// Soft floral contours and blush hatching.
    Blossom,
    /// Sparse flowing curves and leaves.
    Breeze,
    /// Blush, hesitation marks and a sweat drop.
    Blush,
    /// Electric neon: a glowing tube around the message and crackling lightning arcs.
    Neon,
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
    /// Faulty word glitch (used for words with issues).
    Glitch,
    /// Hesitate, tilt inward and gently shrink into place.
    Shy,
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
    /// Electric sparks crackling out of the message.
    Sparks,
}

impl ParticleKind {
    /// True for particles born at the message (aura, embers, sparks): they follow it wherever the
    /// shell shows it (ADR-0020). The others fill the whole screen.
    pub fn is_anchored(self) -> bool {
        matches!(self, Self::Aura | Self::Embers | Self::Sparks)
    }
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
    /// Manga screentone: a grid of dots, bigger towards the edges, in the level's ink colour.
    Halftone,
    /// Manga explosion: a pale spiky shape with an ink outline drawn behind the message.
    Burst,
}

impl ScreenKind {
    /// True for an effect without hard edges (flash, tint, spotlight): it survives being drawn small and
    /// scaled up, so it goes in the back layer (ADR-0019). Lines, dots, rings and the burst outline
    /// stay sharp in the front layer.
    pub fn is_soft(self) -> bool {
        matches!(self, Self::ImpactFlash | Self::Tint | Self::Spotlight)
    }
}

/// Slant of manga lettering at extrusion `depth` (0 to 1), as a horizontal skew to the right.
pub fn letter_slant(depth: f32) -> f32 {
    0.16 * depth
}

/// Number of offset copies drawn to extrude manga lettering at `depth` (0 to 1).
pub fn extrusion_steps(depth: f32) -> u32 {
    (depth.clamp(0.0, 1.0) * 5.0).ceil() as u32
}

/// Offset of each extrusion copy, down and to the right, as fractions of the font size.
pub const EXTRUSION_STEP: (f32, f32) = (0.014 * 0.65, 0.014);

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
                    .chain(level.manga.iter().map(|m| &m.accent_color))
                    .chain(level.manga.iter().flat_map(|m| &m.palette))
                    .chain(level.manga.iter().flat_map(|m| &m.backdrop).flat_map(|b| {
                        [&b.inner, &b.outer]
                            .into_iter()
                            .chain(b.rays.iter().map(|r| &r.color))
                    }))
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
                    .manga
                    .as_ref()
                    .is_some_and(|m| !(0.0..=1.0).contains(&m.depth))
                {
                    return Err(fail(format!("{emotion}: manga depth must be in [0, 1]")));
                }
                if level
                    .manga
                    .iter()
                    .flat_map(|m| &m.backdrop)
                    .any(|b| !(0.0..=1.0).contains(&b.opacity))
                {
                    return Err(fail(format!(
                        "{emotion}: backdrop opacity must be in [0, 1]"
                    )));
                }
                if level
                    .manga
                    .iter()
                    .flat_map(|m| &m.backdrop)
                    .flat_map(|b| &b.rays)
                    .any(|r| !(8..=128).contains(&r.count) || !(0.0..=1.0).contains(&r.opacity))
                {
                    return Err(fail(format!(
                        "{emotion}: rays need 8 to 128 lines and an opacity in [0, 1]"
                    )));
                }
                if level
                    .particles
                    .iter()
                    .any(|p| p.colors.is_empty() || p.lifetime_ms == 0)
                {
                    return Err(fail(format!(
                        "{emotion}: emitters need colours and a positive lifetime"
                    )));
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
                if let Some(reason) = photosensitivity_violation(level) {
                    return Err(fail(format!("{emotion} level {}: {reason}", level.level)));
                }
            }
        }
        Ok(())
    }
}

/// Most flashes allowed in any one second (WCAG 2.3.1: no more than three).
pub const MAX_FLASHES_PER_SECOND: usize = 3;
/// Strongest impact flash a theme may ask for: the flash never reaches full white, which keeps
/// the luminance jump below the general flash threshold.
pub const MAX_FLASH_STRENGTH: f32 = 0.8;
/// Strongest shake a theme may ask for (the frame moves at most 3 % of its size at 1.0).
pub const MAX_SHAKE_STRENGTH: f32 = 0.7;
/// Longest a single shake may last, in milliseconds.
pub const MAX_SHAKE_MS: u32 = 600;
/// Most particles one emitter may have, so a hostile or careless theme cannot stall a frame.
pub const MAX_PARTICLES_PER_EMITTER: u32 = 200;

/// Photosensitivity and budget rules for the effects of one level (docs/legal.md L26, gpu-effects
/// skill). Returns what is wrong, or `None`.
fn photosensitivity_violation(level: &Level) -> Option<String> {
    let effects = &level.screen;
    let flashes: Vec<u64> = effects
        .iter()
        .filter(|e| e.kind == ScreenKind::ImpactFlash)
        .map(|e| u64::from(e.at_ms))
        .collect();
    for &start in &flashes {
        let in_window = flashes
            .iter()
            .filter(|&&other| other >= start && other - start < 1000)
            .count();
        if in_window > MAX_FLASHES_PER_SECOND {
            return Some(format!(
                "more than {MAX_FLASHES_PER_SECOND} impact flashes within one second"
            ));
        }
    }
    for e in effects {
        match e.kind {
            ScreenKind::ImpactFlash if e.strength > MAX_FLASH_STRENGTH => {
                return Some(format!("impact flash strength above {MAX_FLASH_STRENGTH}"));
            }
            ScreenKind::Shake if e.strength > MAX_SHAKE_STRENGTH => {
                return Some(format!("shake strength above {MAX_SHAKE_STRENGTH}"));
            }
            ScreenKind::Shake if e.duration_ms > MAX_SHAKE_MS => {
                return Some(format!("shake longer than {MAX_SHAKE_MS} ms"));
            }
            _ => {}
        }
    }
    if level
        .particles
        .iter()
        .any(|p| p.count > MAX_PARTICLES_PER_EMITTER)
    {
        return Some(format!(
            "more than {MAX_PARTICLES_PER_EMITTER} particles in one emitter"
        ));
    }
    None
}

fn is_hex_color(s: &str) -> bool {
    s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANIME: &str = include_str!("../../../themes/anime/theme.json");

    #[test]
    fn lettering_grows_with_depth_and_is_flat_at_zero() {
        assert_eq!(letter_slant(0.0), 0.0);
        assert_eq!(extrusion_steps(0.0), 0);
        assert_eq!(extrusion_steps(0.5), 3);
        assert_eq!(extrusion_steps(1.0), 5);
        assert_eq!(extrusion_steps(7.0), 5, "depth is clamped");
        assert!(letter_slant(1.0) > letter_slant(0.5));
    }

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
        let mut theme = Theme::from_json(ANIME).unwrap();
        theme.emotions.remove(&Emotion::Sadness);
        let calm = theme.level(Emotion::Calm, Intensity::Low);
        let missing = theme.level(Emotion::Sadness, Intensity::Low);
        assert_eq!(calm, missing);
    }

    #[test]
    fn more_than_three_flashes_per_second_are_rejected() {
        let flash = |at| {
            format!(
                r#"{{ "kind": "impact_flash", "at_ms": {at}, "duration_ms": 80, "strength": 0.8 }}"#
            )
        };
        let with = |at: &[u64]| {
            let list: Vec<String> = at.iter().map(|&a| flash(a)).collect();
            replace_once(
                ANIME,
                r#"{ "kind": "impact_flash", "at_ms": 1000, "duration_ms": 120, "strength": 0.8 }"#,
                &list.join(","),
            )
        };
        assert!(Theme::from_json(&with(&[1000, 1300, 1600])).is_ok());
        assert!(Theme::from_json(&with(&[1000, 1200, 1400, 1600])).is_err());
        assert!(Theme::from_json(&with(&[1000, 2100, 3200, 4300, 5400])).is_ok());
        // Start times near `u32::MAX` must not overflow the window arithmetic.
        let late = [4_294_967_000, 4_294_967_100, 4_294_967_200, 4_294_967_295];
        assert!(Theme::from_json(&with(&late)).is_err());
    }

    /// `text.replacen(from, to, 1)` that fails loudly when `from` is not there.
    fn replace_once(text: &str, from: &str, to: &str) -> String {
        assert!(text.contains(from), "test fixture out of date: {from}");
        text.replacen(from, to, 1)
    }

    #[test]
    fn strong_flashes_long_shakes_and_huge_emitters_are_rejected() {
        let flash =
            r#"{ "kind": "impact_flash", "at_ms": 1000, "duration_ms": 120, "strength": 0.8 }"#;
        let shake = r#"{ "kind": "shake", "at_ms": 1050, "duration_ms": 400, "strength": 0.5 }"#;
        let emitter = r#""count": 40"#;
        assert!(Theme::from_json(ANIME).is_ok());
        for (from, to) in [
            (flash, &flash.replace("0.8", "0.9")),
            (shake, &shake.replace("0.5", "0.8")),
            (shake, &shake.replace("400", "700")),
            (emitter, &emitter.replace("40", "201")),
        ] {
            assert!(
                Theme::from_json(&replace_once(ANIME, from, to)).is_err(),
                "{to}"
            );
        }
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
