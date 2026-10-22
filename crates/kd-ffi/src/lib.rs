//! Public facade of the Rust core, exposed to Swift and Kotlin through UniFFI (ADR-0002).
//!
//! This is the **only** surface the native shells see. Keep it tiny: a handful of functions with
//! plain data types. Everything behind it can change without touching Swift or Kotlin.
//!
//! Exposed so far: version, intensity and theme validation (M0) and the emotion engine handle
//! [`EmotionEngine`] (M2). Grammar and rendering arrive with their milestones.
//!
//! Privacy (ADR-0007, rules L1/L18): message text is only ever passed to the engine; it is never
//! logged, stored, or put in an error message.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::sync::{Arc, Once};

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
    /// Emotion id to animate: `choice`, or `calme` below the confidence threshold (ADR-0008).
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
}

#[uniffi::export]
impl EmotionEngine {
    /// Opens the model bundle in `bundle_dir`, verifying its checksums, and loads ONNX Runtime
    /// from `ort_library_path` (the `libonnxruntime.so` of the onnxruntime-android package).
    #[uniffi::constructor]
    pub fn open(bundle_dir: String, ort_library_path: String) -> Result<Arc<Self>, EngineError> {
        install_panic_hook();
        guarded(|| {
            let bundle = Bundle::open(&bundle_dir).map_err(open_error)?;
            let inner = bundle
                .into_engine_with_library(Path::new(&ort_library_path))
                .map_err(open_error)?;
            Ok(Arc::new(Self { inner }))
        })
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
            r#"{"labels":["joie","calme"],"weights":[[8,0],[0,8]],"bias":[0,0]}"#,
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
    fn low_confidence_falls_back_to_calm() {
        let scores =
            EmotionScores::from_probabilities(vec![(Emotion::Joy, 0.5), (Emotion::Calm, 0.5)])
                .unwrap();
        let analysis = EmotionAnalysis {
            effective: scores.effective(0.2),
            scores,
        };
        let r = EmotionResult::from(&analysis);
        assert_eq!(r.effective, "calme");
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
        );
        assert!(matches!(e, Err(EngineError::Bundle(_))));
    }

    #[test]
    fn error_messages_are_readable() {
        assert_eq!(EngineError::Internal.to_string(), "internal error");
        assert!(EngineError::Runtime("x".into()).to_string().contains('x'));
    }
}
