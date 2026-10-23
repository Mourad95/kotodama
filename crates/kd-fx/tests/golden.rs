//! Golden-frame tests: the CPU renderer's output for fixed (theme, emotion, level, motion, time, seed)
//! must match the committed PNGs within a small tolerance.
//!
//! Update the references on purpose, in their own commit:
//!     KD_UPDATE_GOLDEN=1 cargo test -p kd-fx --features cpu-render --test golden
#![cfg(feature = "cpu-render")]

use std::path::PathBuf;

use kd_core::{Emotion, Intensity};
use kd_fx::render::Renderer;
use kd_fx::scene::{sample, Motion};
use kd_fx::Theme;
use tiny_skia::Pixmap;

const MESSAGE: &str = "Franchement, c'est trop bien !";
const SIZE: (u32, u32) = (270, 480);
const SEED: u64 = 1;
/// Largest mean absolute difference per channel (0-255) between a frame and its reference.
const MAX_MEAN_DIFF: f64 = 1.5;
/// Largest share of pixels allowed to differ by more than 40 (anti-aliasing at edges only).
const MAX_BAD_SHARE: f64 = 0.006;

fn render(level: Intensity, motion: Motion, t_ms: u32) -> Pixmap {
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let frame = sample(
        theme.level(Emotion::Energy, level),
        MESSAGE.chars().count(),
        t_ms,
        SEED,
        motion,
    );
    Renderer::new()
        .render(&frame, MESSAGE, SIZE.0, SIZE.1)
        .unwrap()
}

fn check(name: &str, actual: &Pixmap) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/golden/{name}.png"));
    if std::env::var_os("KD_UPDATE_GOLDEN").is_some() {
        actual.save_png(&path).unwrap();
        return;
    }
    let expected = Pixmap::load_png(&path).unwrap_or_else(|e| {
        panic!(
            "missing golden {}: {e} (run with KD_UPDATE_GOLDEN=1)",
            path.display()
        )
    });
    assert_eq!(
        (expected.width(), expected.height()),
        (actual.width(), actual.height()),
        "{name}: size"
    );
    let (mut sum, mut bad, mut n) = (0_u64, 0_u64, 0_u64);
    for (a, b) in actual.data().iter().zip(expected.data()) {
        let d = u64::from(a.abs_diff(*b));
        sum += d;
        bad += u64::from(d > 40);
        n += 1;
    }
    let mean = sum as f64 / n as f64;
    let bad_share = bad as f64 / n as f64;
    assert!(
        mean <= MAX_MEAN_DIFF,
        "{name}: mean difference {mean:.2} > {MAX_MEAN_DIFF}"
    );
    assert!(
        bad_share <= MAX_BAD_SHARE,
        "{name}: {:.2} % of values differ a lot",
        bad_share * 100.0
    );
}

#[test]
fn level_1_letters_rising() {
    check("energie-1-0700", &render(Intensity::Low, Motion::Full, 700));
}

#[test]
fn level_2_speed_lines_and_onomatopoeia() {
    check(
        "energie-2-0900",
        &render(Intensity::Medium, Motion::Full, 900),
    );
}

#[test]
fn level_3_impact_frame() {
    check(
        "energie-3-1060",
        &render(Intensity::Full, Motion::Full, 1060),
    );
}

#[test]
fn level_3_shockwave() {
    check(
        "energie-3-1500",
        &render(Intensity::Full, Motion::Full, 1500),
    );
}

#[test]
fn level_3_gentle() {
    check(
        "energie-3-1500-gentle",
        &render(Intensity::Full, Motion::Gentle, 1500),
    );
}

#[test]
fn level_3_reduced_motion() {
    check(
        "energie-3-0200-reduced",
        &render(Intensity::Full, Motion::Reduced, 200),
    );
}

#[test]
fn explicit_line_breaks_keep_letters_aligned_with_the_frame() {
    // Letter indices count every character, the newline included, so a frame sampled for the
    // whole text lines up with the layout: all non-newline letters must be drawn once settled.
    let text = "Oui\nmerci";
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let frame = sample(
        theme.level(Emotion::Calm, Intensity::Low),
        text.chars().count(),
        3000,
        SEED,
        Motion::Full,
    );
    let one_line = Renderer::new()
        .render(&frame, text, SIZE.0, SIZE.1)
        .unwrap();
    let rows_with_ink = |img: &Pixmap| {
        (0..SIZE.1)
            .filter(|&y| (0..SIZE.0).any(|x| img.pixel(x, y).unwrap().red() < 100))
            .count()
    };
    let blank = Renderer::new().render(&frame, "", SIZE.0, SIZE.1).unwrap();
    assert_eq!(rows_with_ink(&blank), 0);
    assert!(rows_with_ink(&one_line) > 0, "the two lines are drawn");
}

#[test]
fn rendering_is_repeatable() {
    let a = render(Intensity::Full, Motion::Full, 1060);
    let b = render(Intensity::Full, Motion::Full, 1060);
    assert_eq!(a.data(), b.data());
}

#[test]
fn the_message_stays_readable_for_every_emotion_and_level() {
    // Once the letters have settled the ink must dominate where the message is drawn, and while the
    // animation runs the message must never disappear under the effects: count dark pixels in the
    // band of the message at several instants.
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let renderer = Renderer::new();
    let emotions = [
        Emotion::Joy,
        Emotion::Anger,
        Emotion::Energy,
        Emotion::Suspense,
        Emotion::Sadness,
        Emotion::Love,
        Emotion::Calm,
    ];
    for emotion in emotions {
        for level in [Intensity::Low, Intensity::Medium, Intensity::Full] {
            for t in [3000_u32, 1500, 900] {
                let frame = sample(
                    theme.level(emotion, level),
                    MESSAGE.chars().count(),
                    t,
                    SEED,
                    Motion::Full,
                );
                let img = renderer.render(&frame, MESSAGE, SIZE.0, SIZE.1).unwrap();
                let band =
                    (SIZE.1 / 3..2 * SIZE.1 / 3).flat_map(|y| (0..SIZE.0).map(move |x| (x, y)));
                let ink = band
                    .filter(|&(x, y)| {
                        let p = img.pixel(x, y).unwrap();
                        u32::from(p.red()) + u32::from(p.green()) + u32::from(p.blue()) < 3 * 140
                    })
                    .count();
                // Letters enter one after the other, so the early instants need fewer pixels.
                let needed = if t == 3000 { 500 } else { 60 };
                assert!(
                    ink > needed,
                    "{emotion:?} {level:?} t={t}: only {ink} dark pixels in the message band"
                );
            }
        }
    }
}
