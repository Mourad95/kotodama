//! The FFI result must equal the engine result. Runs only when `KD_BUNDLE` (model bundle
//! directory) and `ORT_DYLIB_PATH` (ONNX Runtime library) are set; otherwise prints SKIP and passes.

use std::path::PathBuf;

use kd_emotion::Bundle;
use kd_ffi::{EmotionEngine, EmotionResult};

#[test]
fn ffi_result_equals_engine_result() {
    let (Some(dir), Some(lib)) = (
        std::env::var_os("KD_BUNDLE"),
        std::env::var_os("ORT_DYLIB_PATH"),
    ) else {
        eprintln!("SKIP ffi engine: set KD_BUNDLE and ORT_DYLIB_PATH to run");
        return;
    };
    let (dir, lib) = (PathBuf::from(dir), PathBuf::from(lib));
    let text = "Je suis tellement content, merci pour tout !";

    // The direct engine is dropped at the end of this block, before the FFI engine is opened
    // (the ONNX Runtime environment is shared per process and reused).
    let direct = {
        let engine = Bundle::open(&dir)
            .unwrap()
            .into_engine_with_library(&lib)
            .unwrap();
        engine.analyze(text).unwrap()
    };
    let receipt = std::env::temp_dir().join(format!("kd-receipt-{}.txt", std::process::id()));
    let engine = EmotionEngine::open(
        dir.to_string_lossy().into_owned(),
        lib.to_string_lossy().into_owned(),
        Some(receipt.to_string_lossy().into_owned()),
    )
    .unwrap();
    assert!(receipt.is_file(), "the first open records a receipt");
    std::fs::remove_file(&receipt).unwrap();
    let via_ffi = engine.analyze(text.to_owned()).unwrap();

    assert_eq!(via_ffi, EmotionResult::from(&direct));
    assert_eq!(via_ffi.choice, direct.scores.choice.id());
    assert!(!via_ffi.probabilities.is_empty());
    // Failures are errors, not panics.
    assert!(EmotionEngine::open(
        "/nonexistent".into(),
        lib.to_string_lossy().into_owned(),
        None
    )
    .is_err());
}
