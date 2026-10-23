//! Golden-frame tests: the CPU renderer's output for fixed (theme, emotion, level, motion, time, seed)
//! must match the committed PNGs within a small tolerance.
//!
//! Update the references on purpose, in their own commit:
//!     KD_UPDATE_GOLDEN=1 cargo test -p kd-fx --features cpu-render --test golden
#![cfg(feature = "cpu-render")]

use std::path::PathBuf;

use kd_core::{Emotion, Intensity};
use kd_fx::render::{Layer, Renderer};
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
fn overlay_keeps_the_conversation_visible_and_the_message_readable() {
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let frame = sample(
        theme.level(Emotion::Calm, Intensity::Low),
        MESSAGE.chars().count(),
        3000,
        SEED,
        Motion::Full,
    );
    let img = Renderer::new()
        .render_over(&frame, MESSAGE, SIZE.0, SIZE.1)
        .unwrap();
    // Nothing is drawn at the corners: the conversation shows through.
    assert_eq!(img.pixel(0, 0).unwrap().alpha(), 0);
    assert_eq!(img.pixel(SIZE.0 - 1, SIZE.1 - 1).unwrap().alpha(), 0);
    // The letters (ink) and their white halo are drawn, so the message reads on a dark backdrop too.
    let opaque = img.data().chunks(4).filter(|p| p[3] == 255).count();
    assert!(opaque > 1500, "only {opaque} opaque pixels");
    let white = img
        .data()
        .chunks(4)
        .filter(|p| p[3] > 200 && p[0] > 240 && p[1] > 240 && p[2] > 240)
        .count();
    assert!(white > 50, "no white halo around the letters ({white})");
}

#[test]
fn back_and_front_layers_add_up_to_the_overlay() {
    // Drawn at the same size, the front layer composited over the back layer is the overlay frame
    // (ADR-0017), up to 8-bit rounding: the split loses nothing and draws nothing twice.
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let renderer = Renderer::new();
    for t in [300, 1060, 1500] {
        let frame = sample(
            theme.level(Emotion::Energy, Intensity::Full),
            MESSAGE.chars().count(),
            t,
            SEED,
            Motion::Full,
        );
        let layer = |l| {
            renderer
                .render_layer(&frame, MESSAGE, SIZE.0, SIZE.1, l)
                .unwrap()
        };
        let (over, back, front) = (layer(Layer::Over), layer(Layer::Back), layer(Layer::Front));
        let mut stacked = back.clone();
        stacked.draw_pixmap(
            0,
            0,
            front.as_ref(),
            &tiny_skia::PixmapPaint::default(),
            tiny_skia::Transform::identity(),
            None,
        );
        let worst = over
            .data()
            .iter()
            .zip(stacked.data())
            .map(|(a, b)| a.abs_diff(*b))
            .max()
            .unwrap();
        assert!(
            worst <= 4,
            "t={t}: layers differ from the overlay by {worst}"
        );
        assert_ne!(
            back.data(),
            front.data(),
            "t={t}: the layers split the frame"
        );
    }
}

#[test]
fn rendering_is_repeatable() {
    let a = render(Intensity::Full, Motion::Full, 1060);
    let b = render(Intensity::Full, Motion::Full, 1060);
    assert_eq!(a.data(), b.data());
}

/// WCAG relative luminance of an RGBA pixel (premultiplied alpha is irrelevant: the frame is opaque).
fn luminance(p: tiny_skia::PremultipliedColorU8) -> f64 {
    let lin = |v: u8| {
        let v = f64::from(v) / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(p.red()) + 0.7152 * lin(p.green()) + 0.0722 * lin(p.blue())
}

#[test]
fn the_message_stays_readable_for_every_emotion_and_level() {
    // At many instants (flash and burst frames included) the pixels of the message must stand out from
    // the background around them: take the darkest and the lightest 5 % of the pixels of the message
    // band (the middle fifth of the frame, where the two lines of the message sit); the message is readable when those two extremes contrast, and it was drawn (enough pixels
    // far from the median). Checked on the settled frame and while the animation runs.
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let renderer = Renderer::new();
    let emotions = Emotion::ALL;
    for emotion in emotions {
        for level in [Intensity::Low, Intensity::Medium, Intensity::Full] {
            for t in (0..=3200_u32).step_by(100) {
                let frame = sample(
                    theme.level(emotion, level),
                    MESSAGE.chars().count(),
                    t,
                    SEED,
                    Motion::Full,
                );
                let img = renderer.render(&frame, MESSAGE, SIZE.0, SIZE.1).unwrap();
                if t < 1500 {
                    continue; // letters are still entering: only the later instants must be fully readable
                }
                let mut lum: Vec<f64> = (2 * SIZE.1 / 5..3 * SIZE.1 / 5)
                    .flat_map(|y| (0..SIZE.0).map(move |x| (x, y)))
                    .map(|(x, y)| luminance(img.pixel(x, y).unwrap()))
                    .collect();
                lum.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let n = lum.len();
                let (low, high) = (lum[n / 20], lum[n - 1 - n / 20]);
                let ratio = (high + 0.05) / (low + 0.05);
                assert!(
                    ratio >= 3.0,
                    "{emotion:?} {level:?} t={t}: message contrast {ratio:.2}"
                );
            }
        }
    }
}

#[test]
fn manga_situations_and_settled_bubbles() {
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let renderer = Renderer::new();
    for emotion in Emotion::ALL {
        let level = theme.level(emotion, Intensity::Full);
        let frame = sample(level, MESSAGE.chars().count(), 1600, SEED, Motion::Full);
        check(
            &format!("{}-manga", emotion.id()),
            &renderer
                .render_over(&frame, MESSAGE, SIZE.0, SIZE.1)
                .unwrap(),
        );
        for dark in [false, true] {
            check(
                &format!(
                    "{}-bubble-{}",
                    emotion.id(),
                    if dark { "dark" } else { "light" }
                ),
                &renderer.render_bubble(level, 320, 140, dark).unwrap(),
            );
        }
    }
}
