//! Public facade of the Rust core, exposed to Swift and Kotlin through UniFFI (ADR-0002).
//!
//! This is the **only** surface the native shells see. Keep it tiny: a handful of functions with
//! plain data types. Everything behind it can change without touching Swift or Kotlin.
//!
//! Exposed so far: version, intensity and theme validation (M0), the emotion engine handle
//! [`EmotionEngine`] (M2) and the animation player [`Animator`] (M3). Grammar and rendering arrive with their milestones.
//!
//! Privacy (ADR-0007, rules L1/L18): message text is only ever passed to the engine; it is never
//! logged, stored, or put in an error message.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::sync::{Arc, Once};
use std::time::Instant;

use kd_emotion::{Bundle, Embedder, EmotionAnalysis, EmotionError, OrtEmbedder};

uniffi::setup_scaffolding!();

/// Version of the Rust core, shown in the app's debug screen.
#[uniffi::export]
pub fn core_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Animation intensity level (1 to 3) for a number of writing issues.
#[uniffi::export]
pub fn intensity_level(issue_count: u32) -> u8 {
    kd_core::Intensity::from_issue_count(issue_count as usize).level()
}

/// Validates a theme file. Returns `None` if valid, or the error message.
#[uniffi::export]
pub fn validate_theme(json: String) -> Option<String> {
    kd_fx::Theme::from_json(&json).err().map(|e| e.to_string())
}

/// Failure of the emotion engine. The message is safe to show in a debug screen: it never
/// contains the analysed text.
#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum EngineError {
    /// The model bundle is missing, corrupted or inconsistent.
    #[error("invalid model bundle: {0}")]
    Bundle(String),
    /// ONNX Runtime could not be loaded or the model could not be created.
    #[error("runtime unavailable: {0}")]
    Runtime(String),
    /// The analysis of one message failed.
    #[error("analysis failed: {0}")]
    Analysis(String),
    /// A theme could not be loaded or an animation could not be drawn.
    #[error("animation unavailable: {0}")]
    Animation(String),
    /// An internal error (a panic was caught at the boundary).
    #[error("internal error")]
    Internal,
}

/// Probability of one emotion.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct EmotionProbability {
    /// Emotion id (`joie`, `colere`, ...).
    pub emotion: String,
    /// Probability in [0, 1].
    pub probability: f32,
}

/// Result of analysing one message.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct EmotionResult {
    /// Most probable emotion id.
    pub choice: String,
    /// Emotion id to animate: `choice`, or `neutre` below the confidence threshold (ADR-0008, ADR-0021).
    pub effective: String,
    /// Confidence in [0, 1]: 1 when all probability mass is on one emotion.
    pub confidence: f32,
    /// Probability of every emotion.
    pub probabilities: Vec<EmotionProbability>,
}

impl From<&EmotionAnalysis> for EmotionResult {
    fn from(a: &EmotionAnalysis) -> Self {
        Self {
            choice: a.scores.choice.id().to_owned(),
            effective: a.effective.id().to_owned(),
            confidence: a.scores.confidence,
            probabilities: a
                .scores
                .probabilities
                .iter()
                .map(|(e, p)| EmotionProbability {
                    emotion: e.id().to_owned(),
                    probability: *p,
                })
                .collect(),
        }
    }
}

fn analysis_error(e: EmotionError) -> EngineError {
    EngineError::Analysis(e.to_string())
}

fn open_error(e: EmotionError) -> EngineError {
    match e {
        EmotionError::Embedder(m) => EngineError::Runtime(m),
        other => EngineError::Bundle(other.to_string()),
    }
}

/// How much motion the viewer accepts: the user's intensity setting (full, gentle, minimal).
/// The shell must force [`MotionMode::Minimal`] when the OS asks to reduce motion (docs/legal.md L26).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MotionMode {
    /// The theme as designed.
    Full,
    /// Fewer, fainter particles; no flash, shockwave or shake.
    Gentle,
    /// The message only fades in.
    Minimal,
}

impl From<MotionMode> for kd_fx::scene::Motion {
    fn from(m: MotionMode) -> Self {
        match m {
            MotionMode::Full => Self::Full,
            MotionMode::Gentle => Self::Gentle,
            MotionMode::Minimal => Self::Reduced,
        }
    }
}

/// Which part of a frame [`Animator::render`] draws (ADR-0017).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum FrameLayer {
    /// Everything, on a light opaque background (export).
    Opaque,
    /// Everything, opaque, as the sticker a friend receives (ADR-0023): like [`FrameLayer::Opaque`],
    /// but the manga panel stays at the end, so the last frame is the whole settled scene. With
    /// reduced motion every instant is that scene, still. Draw it square.
    Sticker,
    /// Every effect, on a transparent background, to play over a conversation. The message itself is
    /// not drawn: the shell shows it in its bubble (ADR-0019).
    Over,
    /// Only the soft screen effects (flash, tint, spotlight), transparent: draw it small and scale it up.
    Back,
    /// The hard-edged screen effects, the particles, the manga ink and the onomatopoeias, transparent:
    /// draw it at screen resolution, over [`FrameLayer::Back`].
    Front,
}

impl From<FrameLayer> for kd_fx::render::Layer {
    fn from(l: FrameLayer) -> Self {
        match l {
            FrameLayer::Opaque | FrameLayer::Sticker => Self::Opaque,
            FrameLayer::Over => Self::Over,
            FrameLayer::Back => Self::Back,
            FrameLayer::Front => Self::Front,
        }
    }
}

/// Where the message bubble sits in a frame, as fractions of the frame's width and height from its
/// top left (ADR-0020). The overlay layers anchor their effects on it and keep it clear.
#[derive(Debug, Clone, Copy, PartialEq, uniffi::Record)]
pub struct FrameStage {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Width.
    pub width: f32,
    /// Height.
    pub height: f32,
}

/// How the shell letters a message in its bubble (ADR-0019): colours `#RRGGBB` from the theme and the
/// same slant and extrusion as the renderer's letters, so the rule lives in one place.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct BubbleLook {
    /// Colour of the letters.
    pub ink: String,
    /// Colour of the letters' extruded side.
    pub accent: String,
    /// Horizontal skew of the letters to the right (0 is upright).
    pub slant: f32,
    /// Number of offset copies that extrude the letters (0 is flat).
    pub extrusion_steps: u32,
    /// Offset of each copy to the right, as a fraction of the font size.
    pub extrusion_dx: f32,
    /// Offset of each copy downwards, as a fraction of the font size.
    pub extrusion_dy: f32,
}

/// Largest frame an [`Animator`] draws, per side, in pixels.
const MAX_FRAME_SIDE: u32 = 2048;
/// Longest message an [`Animator`] draws, in characters (same cap as the emotion engine).
const MAX_ANIMATED_CHARS: usize = 2000;

/// Plays a theme: draws the frames of the animation of a message. Build it once from the theme
/// file, then call [`Animator::render`] for each instant from a background thread.
#[derive(uniffi::Object)]
pub struct Animator {
    theme: kd_fx::Theme,
    renderer: kd_fx::render::Renderer,
}

fn animation_level(theme: &kd_fx::Theme, emotion_id: &str, writing_level: u8) -> kd_fx::Level {
    let emotion = kd_core::Emotion::from_id(emotion_id).unwrap_or(kd_core::Emotion::Neutral);
    let intensity = match writing_level {
        0 | 1 => kd_core::Intensity::Low,
        2 => kd_core::Intensity::Medium,
        _ => kd_core::Intensity::Full,
    };
    theme.level(emotion, intensity).clone()
}

#[uniffi::export]
impl Animator {
    /// Parses and validates a theme file (a `theme.json` file under `themes/`).
    #[uniffi::constructor]
    pub fn new(theme_json: String) -> Result<Arc<Self>, EngineError> {
        install_panic_hook();
        guarded(|| {
            let theme = kd_fx::Theme::from_json(&theme_json)
                .map_err(|e| EngineError::Animation(e.to_string()))?;
            Ok(Arc::new(Self {
                theme,
                renderer: kd_fx::render::Renderer::new(),
            }))
        })
    }

    /// Duration of the animation in milliseconds: after it, the frame no longer changes.
    /// `emotion_id` is an id such as `joie` (unknown ids play `neutre`); `writing_level` is 1 to 3
    /// (3 when the message has no mistake). Only the length of the message is used, not its text.
    pub fn duration_ms(
        &self,
        emotion_id: String,
        writing_level: u8,
        mode: MotionMode,
        message_length: u32,
    ) -> u32 {
        let level = animation_level(&self.theme, &emotion_id, writing_level);
        kd_fx::scene::duration_ms(&level, message_length as usize, mode.into())
    }

    /// How to letter a message of this emotion and writing level in its bubble.
    pub fn bubble_look(&self, emotion_id: String, writing_level: u8) -> BubbleLook {
        let level = animation_level(&self.theme, &emotion_id, writing_level);
        let (accent, depth) = level.manga.as_ref().map_or_else(
            || (level.text_color.clone(), 0.0),
            |m| (m.accent_color.clone(), m.depth.clamp(0.0, 1.0)),
        );
        let (extrusion_dx, extrusion_dy) = kd_fx::EXTRUSION_STEP;
        BubbleLook {
            ink: level.text_color,
            accent,
            slant: kd_fx::letter_slant(depth),
            extrusion_steps: kd_fx::extrusion_steps(depth),
            extrusion_dx,
            extrusion_dy,
        }
    }

    /// Draws a permanent manga bubble decoration in premultiplied RGBA: the inked panel and the
    /// level's onomatopoeias.
    /// The shell lays out native, accessible text above it; no message text crosses this call.
    pub fn render_bubble(
        &self,
        emotion_id: String,
        writing_level: u8,
        width: u32,
        height: u32,
        dark: bool,
    ) -> Result<Vec<u8>, EngineError> {
        guarded(|| {
            let level = animation_level(&self.theme, &emotion_id, writing_level);
            self.renderer
                .render_bubble(&level, width, height, dark)
                .map(|image| image.take())
                .ok_or_else(|| EngineError::Animation("unsupported bubble size".into()))
        })
    }

    /// Draws the animation of `message` at `t_ms` after its start, as `width` x `height` pixels in
    /// RGBA, premultiplied alpha, row by row from the top left (`width * height * 4` bytes).
    /// `layer` picks what is drawn: two layers of the same instant line up when stretched to the same
    /// screen area, whatever their pixel sizes. With a `stage`, the overlay layers frame the bubble
    /// there and fill the rest of the screen (ADR-0020); the opaque and sticker layers ignore it.
    /// The text is only drawn: it is never logged, stored or put in an error.
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        emotion_id: String,
        writing_level: u8,
        mode: MotionMode,
        message: String,
        t_ms: u32,
        width: u32,
        height: u32,
        seed: u64,
        layer: FrameLayer,
        stage: Option<FrameStage>,
    ) -> Result<Vec<u8>, EngineError> {
        guarded(|| {
            if width == 0 || height == 0 || width > MAX_FRAME_SIDE || height > MAX_FRAME_SIDE {
                return Err(EngineError::Animation("unsupported frame size".into()));
            }
            let message: String = message.chars().take(MAX_ANIMATED_CHARS).collect();
            let level = animation_level(&self.theme, &emotion_id, writing_level);
            let glyphs = message.chars().count();
            let (frame, opaque) = if layer == FrameLayer::Sticker {
                let frame = kd_fx::scene::sample_sticker(&level, glyphs, t_ms, seed, mode.into());
                (frame, true)
            } else {
                let frame = kd_fx::scene::sample(&level, glyphs, t_ms, seed, mode.into());
                (frame, layer == FrameLayer::Opaque)
            };
            let pixmap = match stage.filter(|_| !opaque) {
                Some(s) => self.renderer.render_layer_on(
                    &frame,
                    width,
                    height,
                    layer.into(),
                    kd_fx::render::Stage {
                        x: s.x,
                        y: s.y,
                        width: s.width,
                        height: s.height,
                    },
                ),
                None => self
                    .renderer
                    .render_layer(&frame, &message, width, height, layer.into()),
            }
            .ok_or_else(|| EngineError::Animation("cannot allocate the frame".into()))?;
            Ok(pixmap.take())
        })
    }
}

static PANIC_HOOK: Once = Once::new();

/// Replaces, once per process, the default panic hook (which prints the panic message, and that
/// message can embed message text, to stderr) by one printing a fixed line: no payload, no
/// location. Called by [`EmotionEngine::open`]; not part of the shell-facing API.
#[doc(hidden)]
pub fn install_panic_hook() {
    PANIC_HOOK.call_once(|| {
        std::panic::set_hook(Box::new(|_| {
            eprintln!("kd-ffi: internal error (details withheld for privacy)");
        }));
    });
}

/// Runs `f`, turning a panic into [`EngineError::Internal`] (the panic payload may hold text and
/// is dropped without being read).
fn guarded<T>(f: impl FnOnce() -> Result<T, EngineError>) -> Result<T, EngineError> {
    catch_unwind(AssertUnwindSafe(f)).unwrap_or(Err(EngineError::Internal))
}

fn analyze_with<E: Embedder>(
    engine: &kd_emotion::EmotionEngine<E>,
    text: &str,
) -> Result<EmotionResult, EngineError> {
    guarded(|| {
        engine
            .analyze(text)
            .map(|a| EmotionResult::from(&a))
            .map_err(analysis_error)
    })
}

/// The emotion engine: model bundle + ONNX Runtime. Create it once, off the UI thread (it
/// verifies the bundle checksums and loads the model), then call [`EmotionEngine::analyze`]
/// from a background thread.
#[derive(uniffi::Object)]
pub struct EmotionEngine {
    inner: kd_emotion::EmotionEngine<OrtEmbedder>,
    timings: OpenTimings,
}

/// How long [`EmotionEngine::open`] spent in each phase, in milliseconds (for the M2 measurements).
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct OpenTimings {
    /// Reading the manifest, verifying the SHA-256 of every file, parsing the head.
    pub verify_ms: u64,
    /// Loading ONNX Runtime, the model and the tokenizer.
    pub load_ms: u64,
}

#[uniffi::export]
impl EmotionEngine {
    /// Opens the model bundle in `bundle_dir`, verifying its checksums, and loads ONNX Runtime
    /// from `ort_library_path` (the `libonnxruntime.so` of the onnxruntime-android package).
    ///
    /// `receipt_path` is a file in the app's private storage: the checksums are computed once and
    /// recorded there, and skipped while the bundle is unchanged (ADR-0014). Pass `None` to verify
    /// on every open.
    #[uniffi::constructor]
    pub fn open(
        bundle_dir: String,
        ort_library_path: String,
        receipt_path: Option<String>,
    ) -> Result<Arc<Self>, EngineError> {
        install_panic_hook();
        guarded(|| {
            let t0 = Instant::now();
            let bundle = match &receipt_path {
                Some(receipt) => Bundle::open_with_receipt(&bundle_dir, receipt),
                None => Bundle::open(&bundle_dir),
            }
            .map_err(open_error)?;
            let t1 = Instant::now();
            let inner = bundle
                .into_engine_with_library(Path::new(&ort_library_path))
                .map_err(open_error)?;
            let timings = OpenTimings {
                verify_ms: (t1 - t0).as_millis() as u64,
                load_ms: t1.elapsed().as_millis() as u64,
            };
            Ok(Arc::new(Self { inner, timings }))
        })
    }

    /// Time spent in each phase of [`EmotionEngine::open`].
    pub fn open_timings(&self) -> OpenTimings {
        self.timings
    }

    /// Analyses one message. Blocking (tens of milliseconds): call it from a background thread.
    pub fn analyze(&self, text: String) -> Result<EmotionResult, EngineError> {
        analyze_with(&self.inner, &text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_works() {
        assert!(!core_version().is_empty());
        assert_eq!(intensity_level(0), 3);
        assert_eq!(intensity_level(2), 1);
        assert!(validate_theme(include_str!("../../../themes/anime/theme.json").into()).is_none());
        assert!(validate_theme("{}".into()).is_some());
    }

    const ANIME: &str = include_str!("../../../themes/anime/theme.json");

    #[test]
    fn bubble_decoration_is_separate_from_text_and_rejects_invalid_sizes() {
        let animator = Animator::new(ANIME.into()).unwrap();
        let light = animator
            .render_bubble("genance".into(), 3, 180, 80, false)
            .unwrap();
        let dark = animator
            .render_bubble("genance".into(), 3, 180, 80, true)
            .unwrap();
        assert_eq!(light.len(), 180 * 80 * 4);
        assert_ne!(light, dark);
        assert_eq!(light[3], 0);
        let look = animator.bubble_look("colere".into(), 3);
        assert_eq!(look.accent, "#D7263D");
        assert!(look.slant > 0.0 && look.slant <= 0.16);
        assert!((3..=5).contains(&look.extrusion_steps));
        assert_eq!(
            animator.bubble_look("inconnue".into(), 3),
            animator.bubble_look("neutre".into(), 3)
        );
        assert!(animator
            .render_bubble("genance".into(), 3, 0, 80, false)
            .is_err());
        assert!(animator
            .render_bubble("genance".into(), 3, 4096, 80, false)
            .is_err());
    }

    #[test]
    fn animator_draws_frames_and_reports_durations() {
        let a = Animator::new(ANIME.into()).unwrap();
        let (w, h) = (90, 160);
        let draw = |emotion: &str, mode, t| {
            a.render(
                emotion.into(),
                3,
                mode,
                "Bravo !".into(),
                t,
                w,
                h,
                1,
                FrameLayer::Opaque,
                None,
            )
            .unwrap()
        };
        let frame = draw("energie", MotionMode::Full, 1100);
        assert_eq!(frame.len(), (w * h * 4) as usize);
        assert!(
            frame.chunks(4).all(|px| px[3] == 255),
            "the background is opaque"
        );
        assert_ne!(
            frame,
            draw("energie", MotionMode::Minimal, 1100),
            "modes differ"
        );
        assert_eq!(
            frame,
            draw("energie", MotionMode::Full, 1100),
            "deterministic"
        );
        // Unknown emotions play `neutre`, which differs from `calme`.
        assert_eq!(
            draw("inconnue", MotionMode::Full, 500),
            draw("neutre", MotionMode::Full, 500)
        );
        assert_ne!(
            draw("neutre", MotionMode::Full, 500),
            draw("calme", MotionMode::Full, 500)
        );
        // The frame stops changing at the announced duration.
        let end = a.duration_ms("energie".into(), 3, MotionMode::Full, 7);
        assert!(end > 1000 && end < 10_000, "{end}");
        assert_eq!(
            draw("energie", MotionMode::Full, end),
            draw("energie", MotionMode::Full, end + 5000)
        );
        assert!(a.duration_ms("energie".into(), 3, MotionMode::Minimal, 7) < end);
    }

    #[test]
    fn sticker_frames_end_on_the_whole_scene() {
        let a = Animator::new(ANIME.into()).unwrap();
        let side = 96;
        let draw = |layer, mode, t| {
            a.render(
                "amour".into(),
                3,
                mode,
                "Bravo !".into(),
                t,
                side,
                side,
                1,
                layer,
                None,
            )
            .unwrap()
        };
        let end = a.duration_ms("amour".into(), 3, MotionMode::Full, 7);
        let sticker = draw(FrameLayer::Sticker, MotionMode::Full, end);
        assert!(sticker.chunks(4).all(|px| px[3] == 255), "opaque");
        // The export fades its panel out at the end; the sticker keeps it.
        assert_ne!(sticker, draw(FrameLayer::Opaque, MotionMode::Full, end));
        // With reduced motion, the sticker is that settled scene from the first instant.
        assert_eq!(sticker, draw(FrameLayer::Sticker, MotionMode::Minimal, 0));
    }

    #[test]
    fn overlay_frames_are_transparent_where_nothing_is_drawn() {
        let a = Animator::new(ANIME.into()).unwrap();
        let (w, h) = (90, 160);
        let over = |message: &str, t| {
            a.render(
                "joie".into(),
                3,
                MotionMode::Full,
                message.into(),
                t,
                w,
                h,
                1,
                FrameLayer::Over,
                None,
            )
            .unwrap()
        };
        let settled = over("Bravo", 5000);
        let alpha = |x: u32, y: u32| settled[((y * w + x) * 4 + 3) as usize];
        assert_eq!(alpha(0, 0), 0, "the corner is transparent");
        assert_eq!(alpha(w - 1, h - 1), 0);
        // The message is left to the bubble (ADR-0019): its text changes nothing in the overlay.
        assert!(over("Bravo", 900).chunks(4).any(|px| px[3] > 0));
        assert_eq!(over("Bravo", 900), over("Salut", 900));
        assert_eq!(a.bubble_look("joie".into(), 3).ink, "#7A3E00");
    }

    #[test]
    fn overlay_frames_anchor_on_the_stage() {
        let a = Animator::new(ANIME.into()).unwrap();
        let (w, h) = (90, 160);
        let stage = FrameStage {
            x: 0.2,
            y: 0.6,
            width: 0.7,
            height: 0.12,
        };
        let over = |stage| {
            a.render(
                "joie".into(),
                3,
                MotionMode::Full,
                "Bravo".into(),
                900,
                w,
                h,
                1,
                FrameLayer::Over,
                stage,
            )
            .unwrap()
        };
        let anchored = over(Some(stage));
        let alpha = |x: u32, y: u32| anchored[((y * w + x) * 4 + 3) as usize];
        assert!(alpha(1, 1) > 0, "the backdrop fills the screen");
        assert!(
            alpha(w / 2 + 5, (0.66 * h as f32) as u32) < 64,
            "the bubble stays clear"
        );
        assert_ne!(anchored, over(None));
        // The opaque export ignores the stage.
        let opaque = |stage| {
            a.render(
                "joie".into(),
                3,
                MotionMode::Full,
                "Bravo".into(),
                900,
                w,
                h,
                1,
                FrameLayer::Opaque,
                stage,
            )
            .unwrap()
        };
        assert_eq!(opaque(Some(stage)), opaque(None));
    }

    #[test]
    fn a_broken_stage_falls_back_to_the_centred_panel() {
        let a = Animator::new(ANIME.into()).unwrap();
        let over = |stage| {
            a.render(
                "joie".into(),
                3,
                MotionMode::Full,
                "Bravo".into(),
                900,
                90,
                160,
                1,
                FrameLayer::Over,
                stage,
            )
            .unwrap()
        };
        let centred = over(None);
        for (x, y, width, height) in [
            (f32::NAN, 0.6, 0.7, 0.12),
            (0.2, f32::INFINITY, 0.7, 0.12),
            (0.2, 0.6, -0.7, 0.12),
            (0.2, 0.6, 0.7, 0.0),
        ] {
            let stage = FrameStage {
                x,
                y,
                width,
                height,
            };
            assert_eq!(over(Some(stage)), centred, "{stage:?}");
        }
    }

    #[test]
    fn animator_rejects_bad_input_without_panicking() {
        assert!(matches!(
            Animator::new("{}".into()),
            Err(EngineError::Animation(_))
        ));
        let a = Animator::new(ANIME.into()).unwrap();
        for (w, h) in [(0, 10), (10, 0), (4096, 10), (10, 4096)] {
            assert!(a
                .render(
                    "joie".into(),
                    3,
                    MotionMode::Full,
                    "x".into(),
                    0,
                    w,
                    h,
                    1,
                    FrameLayer::Opaque,
                    None,
                )
                .is_err());
        }
        let long = "a".repeat(100_000);
        assert!(a
            .render(
                "joie".into(),
                3,
                MotionMode::Full,
                long,
                500,
                90,
                160,
                1,
                FrameLayer::Opaque,
                None,
            )
            .is_ok());
        assert!(a
            .render(
                "joie".into(),
                3,
                MotionMode::Full,
                String::new(),
                500,
                90,
                160,
                1,
                FrameLayer::Opaque,
                None,
            )
            .is_ok());
    }

    use kd_core::{Emotion, EmotionScores};
    use kd_emotion::{EmotionEngine as Engine, EmotionHead};

    /// Embedder choosing a one-hot vector by keyword; panics on "boom".
    struct Fake;

    impl Embedder for Fake {
        fn embed(&self, text: &str) -> Result<Vec<f32>, EmotionError> {
            assert!(!text.contains("boom"), "simulated panic");
            if text.contains("fail") {
                return Err(EmotionError::Embedder("simulated".into()));
            }
            Ok(if text.contains("bravo") {
                vec![1.0, 0.0]
            } else if text.contains("nul") {
                vec![0.0, 0.0]
            } else {
                vec![0.0, 1.0]
            })
        }
    }

    fn engine() -> Engine<Fake> {
        let head = EmotionHead::from_json(
            r#"{"labels":["joie","neutre"],"weights":[[8,0],[0,8]],"bias":[0,0]}"#,
        )
        .unwrap();
        Engine::new(Fake, head)
    }

    #[test]
    fn result_mirrors_the_analysis() {
        let analysis = engine().analyze("bravo").unwrap();
        let r = analyze_with(&engine(), "bravo").unwrap();
        assert_eq!(r, EmotionResult::from(&analysis));
        assert_eq!((r.choice.as_str(), r.effective.as_str()), ("joie", "joie"));
        assert_eq!(r.probabilities.len(), 2);
        assert_eq!(r.probabilities[0].emotion, "joie");
        assert!((r.probabilities.iter().map(|p| p.probability).sum::<f32>() - 1.0).abs() < 1e-5);
        assert!(r.confidence > 0.9);
    }

    #[test]
    fn low_confidence_falls_back_to_neutral() {
        let scores =
            EmotionScores::from_probabilities(vec![(Emotion::Joy, 0.5), (Emotion::Calm, 0.5)])
                .unwrap();
        let analysis = EmotionAnalysis {
            effective: scores.effective(0.2),
            scores,
        };
        let r = EmotionResult::from(&analysis);
        assert_eq!(r.effective, "neutre");
    }

    #[test]
    fn failures_become_errors_never_panics() {
        match analyze_with(&engine(), "please fail") {
            Err(EngineError::Analysis(m)) => assert!(!m.contains("please")),
            other => panic!("unexpected {other:?}"),
        }
        assert!(matches!(
            analyze_with(&engine(), "boom"),
            Err(EngineError::Internal)
        ));
    }

    #[test]
    fn opening_a_missing_bundle_is_an_error() {
        let e = EmotionEngine::open(
            "/nonexistent/bundle".into(),
            "/nonexistent/libort.so".into(),
            None,
        );
        assert!(matches!(e, Err(EngineError::Bundle(_))));
    }

    #[test]
    fn error_messages_are_readable() {
        assert_eq!(EngineError::Internal.to_string(), "internal error");
        assert!(EngineError::Runtime("x".into()).to_string().contains('x'));
    }
}
