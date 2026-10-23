//! Quality rules every bundled theme must meet: all emotions covered, the bundled font can draw every
//! onomatopoeia, colours are readable on the light background.
#![cfg(feature = "cpu-render")]

use kd_core::{Emotion, Intensity};
use kd_fx::render::Renderer;
use kd_fx::scene::{Rgb, LIGHT_BACKGROUND};
use kd_fx::Theme;

const ANIME: &str = include_str!("../../../themes/anime/theme.json");
const ALL: [Emotion; 8] = Emotion::ALL;
const LEVELS: [Intensity; 3] = [Intensity::Low, Intensity::Medium, Intensity::Full];

/// WCAG relative luminance.
fn luminance(c: Rgb) -> f64 {
    let lin = |v: u8| {
        let v = f64::from(v) / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(c.0) + 0.7152 * lin(c.1) + 0.0722 * lin(c.2)
}

fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[test]
fn every_emotion_has_its_own_motion() {
    let theme = Theme::from_json(ANIME).unwrap();
    for e in ALL {
        assert!(
            theme.emotions.contains_key(&e),
            "{e:?} falls back to calme: add it to the theme"
        );
    }
}

#[test]
fn the_font_draws_every_onomatopoeia() {
    let theme = Theme::from_json(ANIME).unwrap();
    let renderer = Renderer::new();
    for e in ALL {
        for i in LEVELS {
            for o in &theme.level(e, i).onomatopoeias {
                assert!(
                    renderer.covers(&o.text),
                    "{e:?} {i:?}: the font lacks a glyph for {:?}",
                    o.text
                );
            }
        }
    }
}

#[test]
fn colours_are_readable_on_the_light_background() {
    let theme = Theme::from_json(ANIME).unwrap();
    for e in ALL {
        for i in LEVELS {
            let level = theme.level(e, i);
            // Message text: WCAG AA for normal text. Onomatopoeias are large text: AA large (3:1).
            let text = contrast(Rgb::from_hex(&level.text_color), LIGHT_BACKGROUND);
            assert!(text >= 4.5, "{e:?} {i:?}: text contrast {text:.2}");
            for o in &level.onomatopoeias {
                let c = contrast(Rgb::from_hex(&o.color), LIGHT_BACKGROUND);
                assert!(c >= 3.0, "{e:?} {i:?}: onomatopoeia contrast {c:.2}");
            }
        }
    }
}
