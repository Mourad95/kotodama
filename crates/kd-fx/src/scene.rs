//! Samples a theme level at a point in time: a pure function of `(level, glyph count, t, seed)`
//! that returns what to draw, with no GPU, font or clock involved (ADR-0015).
//!
//! Coordinates are fractions of the frame: `x` from 0 (left) to 1 (right), `y` from 0 (top) to 1
//! (bottom), the message is centred on (0.5, 0.5). Sizes are fractions of the frame height and
//! letter offsets are in em (the message font size), so the same frame renders at any resolution.

use crate::{LetterKind, Level, MangaStyle, Onomatopoeia, ParticleKind, ScreenKind};

/// An 8-bit RGB colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// The light background every renderer draws behind the message; text and onomatopoeia colours are
/// checked against it (contrast tests).
pub const LIGHT_BACKGROUND: Rgb = Rgb(0xF7, 0xF7, 0xFA);

impl Rgb {
    /// Parses `#RRGGBB` (already validated for themes). Never panics: anything else gives black.
    pub fn from_hex(s: &str) -> Rgb {
        let byte = |i: usize| s.get(i..i + 2).and_then(|h| u8::from_str_radix(h, 16).ok());
        match (byte(1), byte(3), byte(5)) {
            (Some(r), Some(g), Some(b)) => Rgb(r, g, b),
            _ => Rgb(0, 0, 0),
        }
    }
}

/// State of one letter of the message.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphState {
    /// Index of the letter in the message (counting characters, spaces included).
    pub index: usize,
    /// Horizontal offset from its resting place, in em.
    pub dx: f32,
    /// Vertical offset from its resting place, in em (negative is up).
    pub dy: f32,
    /// Scale around the letter's centre (1 = resting size).
    pub scale: f32,
    /// Rotation around the letter's centre, in radians.
    pub rotation: f32,
    /// Opacity from 0 to 1.
    pub alpha: f32,
    /// Glow strength from 0 to 1.
    pub glow: f32,
}

/// State of one living particle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParticleState {
    /// Shape and behaviour.
    pub kind: ParticleKind,
    /// Centre, as a fraction of the frame width.
    pub x: f32,
    /// Centre, as a fraction of the frame height.
    pub y: f32,
    /// Size as a fraction of the frame's size unit (its height for a 9:16 portrait frame, see `render::size_unit`).
    pub size: f32,
    /// Rotation in radians.
    pub rotation: f32,
    /// Opacity from 0 to 1.
    pub alpha: f32,
    /// Colour.
    pub color: Rgb,
    /// Perspective plane, from distant (zero) to foreground (one).
    pub depth: f32,
}

/// What a running full-screen effect asks the renderer to draw this frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenState {
    /// Effect kind. `Shake` never appears here: it moves the whole frame ([`Frame::shake`]).
    pub kind: ScreenKind,
    /// Progress through the effect, from 0 to 1.
    pub progress: f32,
    /// Strength declared by the theme, from 0 to 1.
    pub strength: f32,
    /// Opacity of the effect right now (strength with its fade applied), from 0 to 1.
    pub alpha: f32,
}

/// State of one onomatopoeia.
#[derive(Debug, Clone, PartialEq)]
pub struct OnomatopoeiaState {
    /// Text to draw.
    pub text: String,
    /// Colour.
    pub color: Rgb,
    /// Size relative to the message text, with the entrance bounce applied.
    pub scale: f32,
    /// The entrance bounce alone (1 once settled), already included in `scale`.
    pub entrance: f32,
    /// Rotation in radians.
    pub rotation: f32,
    /// Opacity from 0 to 1.
    pub alpha: f32,
    /// Position of its centre, as a fraction of the frame.
    pub x: f32,
    /// Position of its centre, as a fraction of the frame height.
    pub y: f32,
}

/// Time-sampled ink illustration, independent of any rendering API.
#[derive(Debug, Clone, PartialEq)]
pub struct MangaState {
    /// Drawing primitives and highlight colour from the theme.
    pub style: MangaStyle,
    /// Normalised time within the illustration.
    pub progress: f32,
    /// Fade applied to the complete illustration.
    pub alpha: f32,
    /// Writing intensity, softened by gentle mode.
    pub strength: f32,
}

/// Everything to draw at one instant, back to front: screen effects, particles, letters,
/// onomatopoeias (the message stays readable above the background effects).
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    /// Colour of the message text.
    pub text_color: Rgb,
    /// Optional manga illustration; absent in reduced motion.
    pub manga: Option<MangaState>,
    /// Letters of the message, in order.
    pub glyphs: Vec<GlyphState>,
    /// Living particles.
    pub particles: Vec<ParticleState>,
    /// Running full-screen effects (except shake).
    pub screen: Vec<ScreenState>,
    /// Visible onomatopoeias.
    pub onomatopoeias: Vec<OnomatopoeiaState>,
    /// Offset applied to the whole frame, as a fraction of its size.
    pub shake: (f32, f32),
}

/// How much motion the viewer accepts (docs/legal.md L26).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    /// The theme as designed.
    Full,
    /// A calmer version for people who find effects tiring: fewer, fainter particles, softer
    /// screen effects, and no impact flash, shockwave or shake. Letters and onomatopoeias play as designed.
    Gentle,
    /// The OS asked to reduce motion (or the user lowered effects): the message only fades in,
    /// with no letter movement, particles, screen effects, shake or onomatopoeias.
    Reduced,
}

/// In gentle mode, screen effects keep this share of their strength.
const GENTLE_SCREEN: f32 = 0.5;
/// In gentle mode, particles keep this share of their opacity.
const GENTLE_PARTICLE_ALPHA: f32 = 0.7;
/// In gentle mode, this many particles out of every five are drawn.
const GENTLE_KEEP_PER_FIVE: u32 = 2;
/// How long the message fades in when motion is reduced, in milliseconds.
const REDUCED_FADE_MS: f32 = 300.0;

/// How long the animation of `level` lasts for a message of `glyph_count` letters, in milliseconds:
/// after this instant nothing moves any more and the frame is the settled message. The whole
/// timeline is covered: letters (with their stagger), screen effects, particles and onomatopoeias.
pub fn duration_ms(level: &Level, glyph_count: usize, motion: Motion) -> u32 {
    if motion == Motion::Reduced {
        return REDUCED_FADE_MS as u32;
    }
    let letters = if glyph_count == 0 {
        0
    } else {
        (letter_span_ms(level, glyph_count) + level.letters.duration_ms.min(1800) as f32) as u64
    };
    let screen = level
        .screen
        .iter()
        .map(|e| u64::from(e.at_ms) + u64::from(e.duration_ms))
        .max()
        .unwrap_or(0);
    let particles = level
        .particles
        .iter()
        .map(|p| (EMIT_WINDOW_LIFETIMES as u64 + 1) * u64::from(p.lifetime_ms))
        .max()
        .unwrap_or(0);
    let onomatopoeias = level
        .onomatopoeias
        .iter()
        .map(|o| u64::from(o.at_ms) + ONOMATOPOEIA_ENTRANCE_MS as u64)
        .max()
        .unwrap_or(0);
    letters
        .max(screen)
        .max(particles)
        .max(onomatopoeias)
        .max(if level.manga.is_some() { 3000 } else { 0 })
        .min(4500) as u32
}

/// Samples `level` at `t_ms` after the start of the message, for a message of `glyph_count`
/// letters. The same arguments always give the same frame.
pub fn sample(level: &Level, glyph_count: usize, t_ms: u32, seed: u64, motion: Motion) -> Frame {
    let t = t_ms as f32;
    if motion == Motion::Reduced {
        let alpha = (t / REDUCED_FADE_MS).clamp(0.0, 1.0);
        return Frame {
            text_color: Rgb::from_hex(&level.text_color),
            manga: None,
            glyphs: (0..glyph_count)
                .map(|index| GlyphState {
                    index,
                    dx: 0.0,
                    dy: 0.0,
                    scale: 1.0,
                    rotation: 0.0,
                    alpha,
                    glow: 0.0,
                })
                .collect(),
            particles: Vec::new(),
            screen: Vec::new(),
            onomatopoeias: Vec::new(),
            shake: (0.0, 0.0),
        };
    }
    let end = duration_ms(level, glyph_count, motion) as f32;
    let t = t.min(end);
    let glyphs = (0..glyph_count)
        .map(|i| glyph(level, i, glyph_count, t, seed))
        .collect();
    let gentle = motion == Motion::Gentle;
    let mut shake = (0.0, 0.0);
    let mut screen = Vec::new();
    for (n, effect) in level.screen.iter().enumerate() {
        let jarring = matches!(
            effect.kind,
            ScreenKind::ImpactFlash | ScreenKind::Shockwave | ScreenKind::Shake
        );
        if gentle && jarring {
            continue;
        }
        let (start, duration) = (effect.at_ms as f32, effect.duration_ms as f32);
        if t >= end || t < start || t >= start + duration {
            continue;
        }
        let progress = (t - start) / duration;
        if effect.kind == ScreenKind::Shake {
            let decay = (1.0 - progress) * effect.strength * SHAKE_AMPLITUDE;
            let step = (t / SHAKE_STEP_MS) as u64;
            let (rx, ry) = (
                hash01(seed, n as u64, step, 1),
                hash01(seed, n as u64, step, 2),
            );
            shake.0 += (rx * 2.0 - 1.0) * decay;
            shake.1 += (ry * 2.0 - 1.0) * decay;
        } else {
            screen.push(ScreenState {
                kind: effect.kind,
                progress,
                strength: effect.strength,
                alpha: effect.strength
                    * screen_fade(effect.kind, progress)
                    * if gentle { GENTLE_SCREEN } else { 1.0 },
            });
        }
    }
    let mut particles = Vec::new();
    for (e, emitter) in level.particles.iter().enumerate() {
        let lifetime = emitter.lifetime_ms as f32;
        let window = lifetime * EMIT_WINDOW_LIFETIMES;
        for k in 0..emitter.count {
            // Gentle mode keeps two particles out of every five.
            if gentle && k % 5 >= GENTLE_KEEP_PER_FIVE {
                continue;
            }
            let r = |salt| hash01(seed, 100 + e as u64, u64::from(k), salt);
            let age = t - r(0) * window;
            if t >= end || age < 0.0 || age >= lifetime || emitter.colors.is_empty() {
                continue;
            }
            let color = Rgb::from_hex(&emitter.colors[k as usize % emitter.colors.len()]);
            let mut state = particle(emitter.kind, age / lifetime, lifetime / 1000.0, color, r);
            if gentle {
                state.alpha *= GENTLE_PARTICLE_ALPHA;
            }
            particles.push(state);
        }
    }
    let onomatopoeias = level
        .onomatopoeias
        .iter()
        .enumerate()
        .filter_map(|(n, o)| onomatopoeia(o, n, t, seed))
        .collect();
    Frame {
        text_color: Rgb::from_hex(&level.text_color),
        manga: level.manga.as_ref().map(|style| MangaState {
            style: style.clone(),
            progress: t / end.max(1.0),
            alpha: ((t / 250.0).min(1.0) * ((end - t) / 450.0).clamp(0.0, 1.0)),
            strength: (0.35 + 0.65 * level.level as f32 / 3.0) * if gentle { 0.55 } else { 1.0 },
        }),
        glyphs,
        particles,
        screen,
        onomatopoeias,
        shake,
    }
}

/// Peak shake displacement for `strength = 1`, as a fraction of the frame.
const SHAKE_AMPLITUDE: f32 = 0.03;
/// The shake jumps to a new offset every this many milliseconds (about 30 Hz).
const SHAKE_STEP_MS: f32 = 33.0;
/// Particles are born over this many lifetimes, so an emitter fades in rather than bursting.
const EMIT_WINDOW_LIFETIMES: f32 = 2.0;

fn letter_span_ms(level: &Level, count: usize) -> f32 {
    (count.saturating_sub(1) as f32 * level.letters.stagger_ms as f32).min(1200.0)
}

fn glyph(level: &Level, index: usize, count: usize, t: f32, seed: u64) -> GlyphState {
    let letters = &level.letters;
    let start = index as f32 / count.saturating_sub(1).max(1) as f32 * letter_span_ms(level, count);
    let p = ((t - start) / letters.duration_ms.min(1800) as f32).clamp(0.0, 1.0);
    let mut g = GlyphState {
        index,
        dx: 0.0,
        dy: 0.0,
        scale: 1.0,
        rotation: 0.0,
        alpha: p,
        glow: 0.0,
    };
    let eased = ease_out(p);
    match letters.kind {
        LetterKind::Rise => g.dy = (1.0 - eased) * 0.5,
        LetterKind::Pop => {
            g.scale = ease_out_back(p);
            g.alpha = (p * 4.0).min(1.0);
        }
        LetterKind::Impact => {
            g.scale = 1.0 + 1.0 * (1.0 - ease_in(p));
            g.alpha = if p > 0.0 { 1.0 } else { 0.0 };
        }
        LetterKind::Charge => {
            let flicker = 0.5 + 0.5 * (t * 0.045 + index as f32 * 1.7).sin();
            g.scale = 0.9 + 0.1 * eased;
            g.glow = (1.0 - p) * (0.4 + 0.6 * flicker);
        }
        LetterKind::Type => g.alpha = if p > 0.0 { 1.0 } else { 0.0 },
        LetterKind::Drip => g.dy = -(1.0 - eased) * 1.2,
        LetterKind::Bloom => {
            g.scale = 0.2 + 0.8 * ease_out_back(p);
            g.rotation = (1.0 - eased) * 0.8;
        }
        LetterKind::Shy => {
            g.scale = 0.78 + 0.22 * eased;
            g.dy = -0.15 * (1.0 - eased);
            g.rotation = (index as f32 * 1.7).sin() * 0.10 * (1.0 - eased);
            g.dx = (t * 0.012 + index as f32).sin() * 0.045 * (1.0 - p);
        }
        LetterKind::Glitch => {
            if p > 0.0 && p < 1.0 {
                let step = (t / 50.0) as u64;
                g.dx = (hash01(seed, 7, index as u64 * 1000 + step, 1) - 0.5) * 0.4;
                g.dy = (hash01(seed, 7, index as u64 * 1000 + step, 2) - 0.5) * 0.2;
            }
            g.alpha = if p > 0.0 { 1.0 } else { 0.0 };
        }
    }
    g
}

fn screen_fade(kind: ScreenKind, p: f32) -> f32 {
    match kind {
        ScreenKind::ImpactFlash => 1.0 - p,
        ScreenKind::Shockwave => 1.0 - p,
        ScreenKind::Spotlight
        | ScreenKind::Tint
        | ScreenKind::SpeedLines
        | ScreenKind::Halftone
        | ScreenKind::Burst => {
            // Fade in over the first 15 %, out over the last 25 %.
            (p / 0.15).min(1.0).min((1.0 - p) / 0.25).clamp(0.0, 1.0)
        }
        ScreenKind::Shake => 1.0,
    }
}

fn particle(
    kind: ParticleKind,
    p: f32,
    lifetime_s: f32,
    color: Rgb,
    r: impl Fn(u64) -> f32,
) -> ParticleState {
    let fade = (p / 0.1).min(1.0).min((1.0 - p) / 0.3).clamp(0.0, 1.0);
    let (x0, y0, spin) = (r(1), r(2), (r(3) - 0.5) * 6.0);
    let (x, y, size, rotation, alpha) = match kind {
        ParticleKind::Confetti => {
            let age = p * lifetime_s;
            let y = -0.05 + 0.25 * age + 0.5 * 0.9 * age * age;
            (
                x0 + 0.05 * (age * 4.0 + y0 * 6.0).sin(),
                y,
                0.012,
                spin * age * 3.0,
                1.0,
            )
        }
        ParticleKind::Stars => {
            let twinkle = 0.5 + 0.5 * (p * 14.0 + y0 * 6.0).sin();
            (x0, 0.1 + 0.8 * y0, 0.008 + 0.014 * r(4), p * spin, twinkle)
        }
        ParticleKind::Aura => (
            0.2 + 0.6 * x0,
            0.58 - 0.3 * p * (0.6 + 0.4 * y0),
            0.02 * (1.0 - 0.5 * p),
            0.0,
            0.8,
        ),
        ParticleKind::Hearts => (
            0.1 + 0.8 * x0 + 0.03 * (p * 8.0 + y0 * 6.0).sin(),
            1.05 - 1.1 * p,
            0.018 + 0.01 * r(4),
            0.3 * (p * 6.0 + y0 * 6.0).sin(),
            1.0,
        ),
        ParticleKind::Petals => (
            x0 + 0.06 * (p * 6.0 + y0 * 6.0).sin(),
            -0.05 + 1.1 * p,
            0.012,
            spin * p * 2.0,
            1.0,
        ),
        ParticleKind::Rain => (x0 - 0.05 * p, -0.1 + 1.2 * p * p.sqrt(), 0.03, 0.15, 0.7),
        ParticleKind::Embers => (
            0.3 + 0.4 * x0 + 0.03 * (p * 9.0 + y0 * 6.0).sin(),
            0.55 - 0.45 * p,
            0.007 * (1.0 - 0.6 * p),
            0.0,
            1.0,
        ),
        ParticleKind::Sparks => {
            // Thrown out of the message on all sides, flickering like an electric discharge.
            let a = x0 * std::f32::consts::TAU;
            let reach = 0.18 + 0.22 * p.sqrt() * (0.6 + 0.4 * r(4));
            (
                0.5 + reach * a.cos(),
                0.5 + 0.45 * reach * a.sin(),
                0.011 * (1.0 - 0.4 * p),
                a + (r(5) - 0.5) * 1.2,
                0.55 + 0.45 * (p * 37.0 + y0 * 11.0).sin().abs(),
            )
        }
    };
    let depth = r(7);
    let perspective = 0.55 + depth * 1.15;
    // Large foreground fragments stay outside the central reading area.
    let x = if depth > 0.68 && (0.18..0.82).contains(&x) {
        if x < 0.5 {
            0.08 + x * 0.12
        } else {
            0.86 + (x - 0.5) * 0.12
        }
    } else {
        x
    };
    ParticleState {
        kind,
        x,
        y,
        size: size * perspective,
        rotation,
        alpha: alpha * fade,
        color,
        depth,
    }
}

fn onomatopoeia(o: &Onomatopoeia, n: usize, t: f32, seed: u64) -> Option<OnomatopoeiaState> {
    let age = t - o.at_ms as f32;
    if age < 0.0 {
        return None;
    }
    let p = (age / ONOMATOPOEIA_ENTRANCE_MS).clamp(0.0, 1.0);
    let entrance = ease_out_back(p);
    // Placed above the message, alternating sides and heights so several of them do not overlap.
    let side = if n.is_multiple_of(2) { -1.0 } else { 1.0 };
    let jitter = hash01(seed, 200 + n as u64, 0, 0) - 0.5;
    Some(OnomatopoeiaState {
        text: o.text.clone(),
        color: Rgb::from_hex(&o.color),
        scale: o.scale * entrance,
        entrance,
        rotation: side * (0.12 + 0.1 * jitter),
        alpha: (p * 3.0).min(1.0),
        x: 0.5 + side * 0.1 + 0.04 * jitter,
        y: if n.is_multiple_of(2) { 0.30 } else { 0.72 } + 0.02 * jitter,
    })
}

/// How long an onomatopoeia takes to pop in, in milliseconds.
const ONOMATOPOEIA_ENTRANCE_MS: f32 = 180.0;

fn ease_out(p: f32) -> f32 {
    1.0 - (1.0 - p).powi(3)
}

fn ease_in(p: f32) -> f32 {
    p * p * p
}

pub(crate) fn ease_out_back(p: f32) -> f32 {
    const C1: f32 = 1.701_58;
    const C3: f32 = C1 + 1.0;
    1.0 + C3 * (p - 1.0).powi(3) + C1 * (p - 1.0).powi(2)
}

/// A deterministic number in [0, 1) derived from the four inputs (SplitMix64 finaliser).
fn hash01(seed: u64, a: u64, b: u64, c: u64) -> f32 {
    let mut z = seed
        .wrapping_add(a.wrapping_mul(0x9E37_79B9_7F4A_7C15))
        .wrapping_add(b.wrapping_mul(0xBF58_476D_1CE4_E5B9))
        .wrapping_add(c.wrapping_mul(0x94D0_49BB_1331_11EB));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    ((z >> 40) as f32) / (1u64 << 24) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LetterEntrance, ParticleEmitter, Theme};
    use kd_core::{Emotion, Intensity};

    fn anime(level: Intensity) -> Level {
        let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
        theme.level(Emotion::Energy, level).clone()
    }

    #[test]
    fn sampling_is_deterministic() {
        let level = anime(Intensity::Full);
        assert_eq!(
            sample(&level, 12, 1100, 7, Motion::Full),
            sample(&level, 12, 1100, 7, Motion::Full)
        );
        assert_ne!(
            sample(&level, 12, 1100, 7, Motion::Full),
            sample(&level, 12, 1100, 8, Motion::Full)
        );
    }

    #[test]
    fn letters_enter_in_order_and_settle() {
        let level = anime(Intensity::Low);
        let start = sample(&level, 5, 0, 1, Motion::Full);
        assert!(start.glyphs.iter().all(|g| g.alpha == 0.0));
        let mid = sample(&level, 5, 100, 1, Motion::Full);
        assert!(
            mid.glyphs[0].alpha > mid.glyphs[4].alpha,
            "earlier letters appear first"
        );
        let end = sample(&level, 5, 5000, 1, Motion::Full);
        for g in &end.glyphs {
            assert_eq!(
                (g.alpha, g.scale, g.dx, g.dy, g.rotation),
                (1.0, 1.0, 0.0, 0.0, 0.0)
            );
        }
    }

    #[test]
    fn effects_follow_their_timeline() {
        let level = anime(Intensity::Full);
        let kinds = |t| {
            sample(&level, 8, t, 3, Motion::Full)
                .screen
                .iter()
                .map(|s| s.kind)
                .collect::<Vec<_>>()
        };
        assert!(kinds(500).contains(&ScreenKind::SpeedLines));
        assert!(
            !kinds(500).contains(&ScreenKind::ImpactFlash),
            "the flash comes later"
        );
        assert!(kinds(1060).contains(&ScreenKind::ImpactFlash));
        assert!(kinds(1060).contains(&ScreenKind::Shockwave));
        assert!(kinds(9000).is_empty());
        assert_eq!(sample(&level, 8, 500, 3, Motion::Full).shake, (0.0, 0.0));
        assert_ne!(sample(&level, 8, 1100, 3, Motion::Full).shake, (0.0, 0.0));
        assert_eq!(sample(&level, 8, 9000, 3, Motion::Full).shake, (0.0, 0.0));
    }

    #[test]
    fn onomatopoeias_appear_at_their_time() {
        let level = anime(Intensity::Full);
        assert!(sample(&level, 8, 200, 1, Motion::Full)
            .onomatopoeias
            .is_empty());
        assert_eq!(
            sample(&level, 8, 400, 1, Motion::Full).onomatopoeias.len(),
            1
        );
        assert_eq!(
            sample(&level, 8, 1500, 1, Motion::Full).onomatopoeias.len(),
            2
        );
    }

    #[test]
    fn particles_stay_within_their_budget_and_lifetime() {
        let level = anime(Intensity::Full);
        let budget: u32 = level.particles.iter().map(|p| p.count).sum();
        for t in (0..4000).step_by(100) {
            let frame = sample(&level, 8, t, 5, Motion::Full);
            assert!(frame.particles.len() as u32 <= budget);
            assert!(frame
                .particles
                .iter()
                .all(|p| (0.0..=1.0).contains(&p.alpha)));
        }
        assert!(sample(&level, 8, 1000, 5, Motion::Full).particles.len() > 10);
        assert!(
            sample(&level, 8, 6000, 5, Motion::Full)
                .particles
                .is_empty(),
            "nothing outlives its lifetime"
        );
    }

    #[test]
    fn higher_levels_are_busier() {
        let count = |i| sample(&anime(i), 8, 1100, 2, Motion::Full);
        let (low, mid, full) = (
            count(Intensity::Low),
            count(Intensity::Medium),
            count(Intensity::Full),
        );
        assert!(
            low.particles.len() < mid.particles.len() && mid.particles.len() < full.particles.len()
        );
        assert!(low.screen.len() < full.screen.len());
    }

    #[test]
    fn reduced_motion_is_a_plain_fade() {
        let level = anime(Intensity::Full);
        for t in [0, 150, 300, 1100, 60_000] {
            let f = sample(&level, 8, t, 3, Motion::Reduced);
            assert!(f.particles.is_empty() && f.screen.is_empty() && f.onomatopoeias.is_empty());
            assert_eq!(f.shake, (0.0, 0.0));
            assert!(f.glyphs.iter().all(|g| (g.dx, g.dy, g.rotation, g.glow)
                == (0.0, 0.0, 0.0, 0.0)
                && g.scale == 1.0));
        }
        assert_eq!(
            sample(&level, 3, 0, 3, Motion::Reduced).glyphs[0].alpha,
            0.0
        );
        assert_eq!(
            sample(&level, 3, 300, 3, Motion::Reduced).glyphs[0].alpha,
            1.0
        );
    }

    #[test]
    fn nothing_moves_after_the_duration() {
        let level = anime(Intensity::Full);
        for motion in [Motion::Full, Motion::Gentle, Motion::Reduced] {
            let end = duration_ms(&level, 20, motion);
            assert!(end > 0 && end < 10_000, "{motion:?}: {end} ms");
            let (a, b) = (
                sample(&level, 20, end, 5, motion),
                sample(&level, 20, end + 777, 5, motion),
            );
            assert_eq!(a, b, "{motion:?}: frame still changes at the end");
            assert!(
                a.particles.is_empty()
                    && a.screen.is_empty()
                    && a.onomatopoeias.len() <= level.onomatopoeias.len()
            );
            let before = sample(&level, 20, end.saturating_sub(300), 5, motion);
            assert!(
                motion == Motion::Reduced || before != a,
                "{motion:?}: nothing happens before the end"
            );
        }
        assert!(
            duration_ms(&level, 200, Motion::Full) >= duration_ms(&level, 5, Motion::Full)
                || level.letters.stagger_ms == 0
        );
        assert_eq!(duration_ms(&level, 0, Motion::Reduced), 300);
    }

    #[test]
    fn gentle_mode_is_calmer_but_still_animated() {
        let level = anime(Intensity::Full);
        let full = sample(&level, 8, 1100, 3, Motion::Full);
        let gentle = sample(&level, 8, 1100, 3, Motion::Gentle);
        assert!(gentle.particles.len() < full.particles.len() && !gentle.particles.is_empty());
        assert!(gentle.screen.iter().all(|s| !matches!(
            s.kind,
            ScreenKind::ImpactFlash | ScreenKind::Shockwave | ScreenKind::Shake
        )));
        assert_eq!(gentle.shake, (0.0, 0.0));
        assert!(gentle.glyphs == full.glyphs, "letters play as designed");
        let strength = |f: &Frame| {
            f.screen
                .iter()
                .find(|s| s.kind == ScreenKind::SpeedLines)
                .map(|s| s.alpha)
        };
        let (a, b) = (
            sample(&level, 8, 500, 3, Motion::Full),
            sample(&level, 8, 500, 3, Motion::Gentle),
        );
        assert!(strength(&b).unwrap() < strength(&a).unwrap());
        assert_eq!(gentle.onomatopoeias.len(), full.onomatopoeias.len());
    }

    #[test]
    fn empty_messages_and_late_instants_are_fine() {
        let level = anime(Intensity::Full);
        assert!(sample(&level, 0, 1100, 1, Motion::Full).glyphs.is_empty());
        let late = sample(&level, 4, u32::MAX, 1, Motion::Full);
        assert!(late.particles.is_empty() && late.screen.is_empty());
    }

    #[test]
    fn every_letter_kind_and_particle_kind_stays_in_range() {
        let letter = |kind| Level {
            level: 3,
            text_color: "#000000".into(),
            letters: LetterEntrance {
                kind,
                duration_ms: 300,
                stagger_ms: 40,
            },
            particles: Vec::new(),
            screen: Vec::new(),
            onomatopoeias: Vec::new(),
            manga: None,
        };
        for kind in [
            LetterKind::Rise,
            LetterKind::Pop,
            LetterKind::Impact,
            LetterKind::Charge,
            LetterKind::Type,
            LetterKind::Drip,
            LetterKind::Bloom,
            LetterKind::Glitch,
            LetterKind::Shy,
        ] {
            let level = letter(kind);
            for t in (0..1200).step_by(25) {
                for g in sample(&level, 6, t, 9, Motion::Full).glyphs {
                    assert!((0.0..=1.0).contains(&g.alpha), "{kind:?} alpha {}", g.alpha);
                    assert!(
                        g.scale.is_finite()
                            && g.scale >= 0.0
                            && g.dx.is_finite()
                            && g.dy.is_finite(),
                        "{kind:?} at {t}"
                    );
                }
            }
            let settled = sample(&level, 6, 5000, 9, Motion::Full);
            assert!(
                settled
                    .glyphs
                    .iter()
                    .all(|g| g.alpha == 1.0 && g.scale == 1.0 && g.dx == 0.0 && g.dy == 0.0),
                "{kind:?} must settle"
            );
        }
        for kind in [
            ParticleKind::Confetti,
            ParticleKind::Stars,
            ParticleKind::Aura,
            ParticleKind::Hearts,
            ParticleKind::Petals,
            ParticleKind::Rain,
            ParticleKind::Embers,
            ParticleKind::Sparks,
        ] {
            let mut level = letter(LetterKind::Rise);
            level.particles = vec![ParticleEmitter {
                kind,
                count: 50,
                colors: vec!["#FFFFFF".into()],
                lifetime_ms: 1000,
            }];
            let mut seen = 0;
            for t in (0..3000).step_by(50) {
                for p in sample(&level, 3, t, 4, Motion::Full).particles {
                    seen += 1;
                    assert!((0.0..=1.0).contains(&p.alpha) && p.size > 0.0, "{kind:?}");
                    assert!(p.x.is_finite() && p.y.is_finite());
                }
            }
            assert!(seen > 0, "{kind:?} never shows");
        }
    }

    #[test]
    fn hash_is_in_range_and_spread() {
        let values: Vec<f32> = (0..1000).map(|i| hash01(1, 2, i, 3)).collect();
        assert!(values.iter().all(|v| (0.0..1.0).contains(v)));
        let mean = values.iter().sum::<f32>() / 1000.0;
        assert!((mean - 0.5).abs() < 0.05, "mean {mean}");
    }

    #[test]
    fn colours_parse() {
        assert_eq!(Rgb::from_hex("#0C447C"), Rgb(12, 68, 124));
        assert_eq!(Rgb::from_hex("nope"), Rgb(0, 0, 0));
    }
}
