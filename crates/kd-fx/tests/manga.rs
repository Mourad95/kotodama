//! Manga illustrations must survive the overlay-to-bubble transition without hiding native text.
#![cfg(feature = "cpu-render")]

use kd_core::{Emotion, Intensity};
use kd_fx::{
    render::Renderer,
    scene::{duration_ms, sample, Motion},
    Theme,
};

fn theme() -> Theme {
    Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap()
}

#[test]
fn every_situation_has_a_distinct_illustration_and_static_bubble() {
    let theme = theme();
    let renderer = Renderer::new();
    let mut images = Vec::new();
    for emotion in Emotion::ALL {
        let level = theme.level(emotion, Intensity::Full);
        assert!(level.manga.is_some(), "{emotion}: missing illustration");
        let bubble = renderer.render_bubble(level, 320, 140, false).unwrap();
        assert_eq!(bubble.pixel(0, 0).unwrap().alpha(), 0);
        // The text's safe area must be pale, opaque and undecorated for accessible native text.
        for y in 45..95 {
            for x in 65..255 {
                let pixel = bubble.pixel(x, y).unwrap();
                assert!(
                    pixel.alpha() > 245
                        && pixel.red() > 225
                        && pixel.green() > 225
                        && pixel.blue() > 225,
                    "{emotion}: decoration in text safe area ({x}, {y})"
                );
            }
        }
        assert!(
            !images.contains(&bubble.data().to_vec()),
            "{emotion}: duplicate bubble"
        );
        images.push(bubble.data().to_vec());
        let dark = renderer.render_bubble(level, 320, 140, true).unwrap();
        assert!(dark.pixel(160, 70).unwrap().red() < 65);
    }
}

#[test]
fn long_messages_finish_and_reduced_motion_keeps_the_illustration_static_only_in_the_bubble() {
    let theme = theme();
    for emotion in Emotion::ALL {
        for mode in [Motion::Full, Motion::Gentle, Motion::Reduced] {
            let level = theme.level(emotion, Intensity::Full);
            let end = duration_ms(level, 2000, mode);
            assert!(end <= 4500, "{emotion}: {end}");
            let frame = sample(level, 2000, end, 3, mode);
            assert!(frame.glyphs.iter().all(|g| g.alpha == 1.0));
            assert_eq!(frame, sample(level, 2000, end + 1000, 3, mode));
            if mode == Motion::Reduced {
                assert!(frame.manga.is_none());
            }
        }
    }
}

#[test]
fn invalid_manga_settings_are_rejected_and_legacy_levels_still_load() {
    let original = include_str!("../../../themes/anime/theme.json");
    let mut value: serde_json::Value = serde_json::from_str(original).unwrap();
    value["emotions"]["joie"]["levels"][0]["manga"]["depth"] = serde_json::json!(1.1);
    assert!(Theme::from_json(&value.to_string()).is_err());
    for motion in value["emotions"].as_object_mut().unwrap().values_mut() {
        for level in motion["levels"].as_array_mut().unwrap() {
            level.as_object_mut().unwrap().remove("manga");
        }
    }
    assert!(Theme::from_json(&value.to_string()).is_ok());
}

#[test]
fn tall_bubbles_keep_the_reading_surface_behind_first_and_last_lines() {
    let theme = theme();
    for emotion in Emotion::ALL {
        let image = Renderer::new()
            .render_bubble(theme.level(emotion, Intensity::Full), 320, 900, false)
            .unwrap();
        for y in [28, 60, 450, 840, 872] {
            let pixel = image.pixel(160, y).unwrap();
            assert!(
                pixel.alpha() > 245 && pixel.red() > 225,
                "{emotion}: no clear reading surface at {y}"
            );
        }
    }
}

#[test]
fn the_frozen_scene_grows_with_the_writing_level() {
    let theme = theme();
    let renderer = Renderer::new();
    // Ink of the frame: how far its opaque pixels are from white, summed.
    let ink = |emotion: Emotion, intensity: Intensity| -> u64 {
        let bubble = renderer
            .render_bubble(theme.level(emotion, intensity), 320, 140, false)
            .unwrap();
        bubble
            .pixels()
            .iter()
            .map(|p| {
                let away = 765 - (u32::from(p.red()) + u32::from(p.green()) + u32::from(p.blue()));
                u64::from(away * u32::from(p.alpha()) / 255)
            })
            .sum()
    };
    for emotion in Emotion::ALL {
        if theme
            .level(emotion, Intensity::Full)
            .manga
            .as_ref()
            .and_then(|m| m.backdrop.as_ref())
            .is_none()
        {
            continue;
        }
        let (low, medium, full) = (
            ink(emotion, Intensity::Low),
            ink(emotion, Intensity::Medium),
            ink(emotion, Intensity::Full),
        );
        assert!(
            low < medium && medium < full,
            "{emotion}: {low} {medium} {full}"
        );
    }
}
