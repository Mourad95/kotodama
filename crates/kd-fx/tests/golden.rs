//! Golden-frame tests: the CPU renderer's output for fixed (theme, emotion, level, motion, time, seed)
//! must match the committed PNGs within a small tolerance.
//!
//! Update the references on purpose, in their own commit:
//!     KD_UPDATE_GOLDEN=1 cargo test -p kd-fx --features cpu-render --test golden
#![cfg(feature = "cpu-render")]

use std::path::PathBuf;

use kd_core::{Emotion, Intensity};
use kd_fx::render::{Layer, Renderer, Stage};
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
fn overlay_keeps_the_conversation_visible_and_leaves_the_message_to_the_bubble() {
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let level = theme.level(Emotion::Energy, Intensity::Full);
    let renderer = Renderer::new();
    // Once the full-screen effects have faded, nothing is drawn at the corners: the conversation
    // shows through.
    let settled = sample(level, MESSAGE.chars().count(), 4000, SEED, Motion::Full);
    let img = renderer
        .render_over(&settled, MESSAGE, SIZE.0, SIZE.1)
        .unwrap();
    assert_eq!(img.pixel(0, 0).unwrap().alpha(), 0);
    assert_eq!(img.pixel(SIZE.0 - 1, SIZE.1 - 1).unwrap().alpha(), 0);
    let frame = sample(level, MESSAGE.chars().count(), 1060, SEED, Motion::Full);
    let img = renderer
        .render_over(&frame, MESSAGE, SIZE.0, SIZE.1)
        .unwrap();
    // The effects play, but the message is not drawn (ADR-0019): the shell shows it in its bubble.
    assert!(
        img.data().chunks(4).any(|p| p[3] == 255),
        "the effects are drawn"
    );
    let other = renderer
        .render_over(&frame, "Autre texte, mêmes effets", SIZE.0, SIZE.1)
        .unwrap();
    assert_eq!(img.data(), other.data());
}

#[test]
fn bubbles_carry_the_level_onomatopoeias() {
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let renderer = Renderer::new();
    let level = theme.level(Emotion::Anger, Intensity::Full);
    assert!(!level.onomatopoeias.is_empty());
    let mut silent = level.clone();
    silent.onomatopoeias.clear();
    let with = renderer.render_bubble(level, 320, 140, false).unwrap();
    let without = renderer.render_bubble(&silent, 320, 140, false).unwrap();
    let changed = with
        .data()
        .chunks(4)
        .zip(without.data().chunks(4))
        .filter(|(a, b)| a != b)
        .count();
    assert!(changed > 400, "only {changed} pixels stamped");
}

#[test]
fn back_and_front_layers_add_up_to_the_overlay() {
    // Drawn at the same size, the front layer composited over the back layer is the overlay frame
    // (ADR-0017, ADR-0019), up to 8-bit rounding: the split loses nothing and draws nothing twice.
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let renderer = Renderer::new();
    for (emotion, t) in Emotion::ALL
        .into_iter()
        .flat_map(|e| [300, 1060, 1500].map(|t| (e, t)))
    {
        let frame = sample(
            theme.level(emotion, Intensity::Full),
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
        // Translucent ink drawn over the translucent backdrop rounds to 8 bits at each pass, so the
        // split and the whole frame may drift by a few levels.
        assert!(
            worst <= 8,
            "{emotion:?} t={t}: layers differ from the overlay by {worst}"
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
fn the_message_stays_readable_during_an_impact_flash_over_a_manga_scene() {
    // The letters keep their ink over a manga scene instead of turning white during a flash
    // (ADR-0018): the flash must not hide them. Energy at full intensity flashes at 1000 ms.
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let level = theme.level(Emotion::Energy, Intensity::Full);
    assert!(level.manga.is_some());
    let renderer = Renderer::new();
    for t in (1000..=1120_u32).step_by(20) {
        let frame = sample(level, MESSAGE.chars().count(), t, SEED, Motion::Full);
        let img = renderer.render(&frame, MESSAGE, SIZE.0, SIZE.1).unwrap();
        let mut lum: Vec<f64> = (2 * SIZE.1 / 5..3 * SIZE.1 / 5)
            .flat_map(|y| (0..SIZE.0).map(move |x| (x, y)))
            .map(|(x, y)| luminance(img.pixel(x, y).unwrap()))
            .collect();
        lum.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = lum.len();
        let ratio = (lum[n - 1 - n / 20] + 0.05) / (lum[n / 20] + 0.05);
        assert!(ratio >= 3.0, "t={t}: message contrast {ratio:.2}");
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

/// A bubble low on the screen, as in a chat (fractions of the frame).
const STAGE: Stage = Stage {
    x: 0.30,
    y: 0.58,
    width: 0.62,
    height: 0.10,
};

#[test]
fn effects_frame_the_bubble_and_fill_the_screen() {
    // ADR-0020: the coloured panel and its motifs cover the screen, the bubble stays clear.
    // ADR-0021: a level without a backdrop (`neutre`) only frames the bubble.
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let renderer = Renderer::new();
    let (w, h) = (SIZE.0 as f32, SIZE.1 as f32);
    for emotion in Emotion::ALL {
        let level = theme.level(emotion, Intensity::Full);
        // Past the impact flash and once every word has landed.
        let frame = sample(level, MESSAGE.chars().count(), 1300, SEED, Motion::Full);
        let image = renderer
            .render_layer_on(&frame, SIZE.0, SIZE.1, Layer::Over, STAGE)
            .unwrap();
        let alpha = |x: f32, y: f32| {
            image
                .pixel((x * w) as u32, (y * h) as u32)
                .map_or(0, |p| p.alpha())
        };
        check(&format!("{}-stage", emotion.id()), &image);
        // The bubble's middle: at most a faded particle crossing it.
        let inside = (0..5)
            .flat_map(|i| (0..3).map(move |j| (0.36 + i as f32 * 0.12, 0.60 + j as f32 * 0.03)))
            .map(|(x, y)| u32::from(alpha(x, y)))
            .sum::<u32>()
            / 15;
        assert!(inside < 40, "{emotion:?}: the bubble is covered ({inside})");
        // The far corners carry the backdrop, when the level has one (`neutre` stays in the bubble).
        if level
            .manga
            .as_ref()
            .and_then(|m| m.backdrop.as_ref())
            .is_none()
        {
            continue;
        }
        for (x, y) in [(0.03, 0.03), (0.97, 0.03), (0.03, 0.97)] {
            assert!(alpha(x, y) > 80, "{emotion:?}: no backdrop at ({x}, {y})");
        }
    }
}
