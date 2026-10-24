//! Emoji of a message are drawn in colour from the bundled emoji font, not as a missing glyph
//! (ADR-0026).
#![cfg(feature = "cpu-render")]

use kd_core::{Emotion, Intensity};
use kd_fx::{
    render::Renderer,
    scene::{sample_sticker, Motion},
    Theme,
};

/// Pixels of a strong red, the colour of a red heart, in the middle band of the frame (the message).
fn red_pixels(message: &str) -> usize {
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let level = theme.level(Emotion::Calm, Intensity::Low);
    let glyphs = message.chars().count();
    let frame = sample_sticker(level, glyphs, 0, 1, Motion::Reduced);
    let pixmap = Renderer::new().render(&frame, message, 360, 360).unwrap();
    let (w, h) = (pixmap.width() as usize, pixmap.height() as usize);
    pixmap
        .pixels()
        .iter()
        .enumerate()
        .filter(|(i, _)| (h / 3..2 * h / 3).contains(&(i / w)))
        .map(|(_, p)| p.demultiply())
        .filter(|p| p.red() > 180 && p.green() < 80 && p.blue() < 90)
        .count()
}

#[test]
fn a_heart_is_drawn_in_colour() {
    let plain = red_pixels("je t'aime");
    let heart = red_pixels("je t'aime ❤️");
    assert!(heart > plain + 100, "heart {heart} vs plain {plain}");
}

#[test]
fn modifiers_draw_nothing_by_themselves() {
    // A skin tone, a joiner and a variation selector alone leave the frame as without them.
    let base = Renderer::new();
    let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
    let level = theme.level(Emotion::Calm, Intensity::Low);
    let frame = sample_sticker(level, 6, 0, 1, Motion::Reduced);
    let a = base.render(&frame, "abc", 200, 200).unwrap();
    let b = base
        .render(&frame, "abc\u{1F3FD}\u{200D}\u{FE0F}", 200, 200)
        .unwrap();
    assert_eq!(a.data(), b.data());
}
