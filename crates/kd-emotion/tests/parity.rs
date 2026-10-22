//! Parity with the Python reference: every probability of `parity.json` must match within 1e-3.
//!
//! Reads the bundle from `$KD_BUNDLE` (default `<repo>/bundle`). Weights are not in git, so the
//! test skips (and passes) when the bundle directory does not exist or ONNX Runtime is not given. ONNX Runtime is loaded from
//! `$ORT_DYLIB_PATH`.

#![cfg(feature = "ort")]

use std::collections::BTreeMap;
use std::path::PathBuf;

use kd_emotion::Bundle;
use serde::Deserialize;

#[derive(Deserialize)]
struct Parity {
    prompt: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Message {
    text: String,
    probabilities: BTreeMap<String, f32>,
}

#[test]
fn probabilities_match_the_python_reference() {
    let dir = std::env::var_os("KD_BUNDLE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../bundle"));
    if !dir.is_dir() {
        eprintln!(
            "SKIP parity: no bundle at {} (weights are not in git)",
            dir.display()
        );
        return;
    }
    if std::env::var_os("ORT_DYLIB_PATH").is_none() {
        eprintln!("SKIP parity: ORT_DYLIB_PATH does not point at libonnxruntime");
        return;
    }
    let parity: Parity =
        serde_json::from_str(&std::fs::read_to_string(dir.join("parity.json")).unwrap()).unwrap();
    let bundle = Bundle::open(&dir).unwrap();
    assert_eq!(
        bundle.manifest.prompt, parity.prompt,
        "prompt differs from the reference"
    );
    let engine = bundle.into_engine().unwrap();

    let mut worst = 0.0_f32;
    for m in &parity.messages {
        let scores = engine.analyze(&m.text).unwrap().scores;
        assert_eq!(scores.probabilities.len(), m.probabilities.len());
        for (emotion, p) in &scores.probabilities {
            let expected = m.probabilities[emotion.id()];
            let diff = (p - expected).abs();
            worst = worst.max(diff);
            assert!(
                diff < 1e-3,
                "{:?}: {} is {p}, expected {expected}",
                m.text,
                emotion.id()
            );
        }
    }
    eprintln!(
        "parity ok on {} messages, worst diff {worst:.2e}",
        parity.messages.len()
    );
}
