//! Renders actual animation keyframes and permanent bubble decorations for visual review.
//! cargo run -p kd-fx --features cpu-render --example manga_preview -- output/manga
use kd_core::{Emotion, Intensity};
use kd_fx::{
    render::Renderer,
    scene::{sample, Motion},
    Theme,
};

fn main() {
    let out = std::path::PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "output/manga".into()),
    );
    std::fs::create_dir_all(&out).expect("create preview directory");
    let theme =
        Theme::from_json(include_str!("../../../themes/anime/theme.json")).expect("valid theme");
    let renderer = Renderer::new();
    let messages = [
        "On a réussi !",
        "Ça suffit !",
        "On va tout donner !",
        "Il faut que je te dise…",
        "Je suis déçu…",
        "Tu me manques.",
        "On prend notre temps.",
        "Vous avez tout entendu ?",
    ];
    for (emotion, message) in Emotion::ALL.into_iter().zip(messages) {
        let level = theme.level(emotion, Intensity::Full);
        for t in [400, 1000, 1600, 2400] {
            let frame = sample(level, message.chars().count(), t, 1, Motion::Full);
            renderer
                .render_over(&frame, message, 360, 640)
                .expect("draw overlay")
                .save_png(out.join(format!("{}-{t}.png", emotion.id())))
                .expect("write overlay");
        }
        for dark in [false, true] {
            renderer
                .render_bubble(level, 320, 128, dark)
                .expect("draw bubble")
                .save_png(out.join(format!(
                    "{}-bubble-{}.png",
                    emotion.id(),
                    if dark { "dark" } else { "light" }
                )))
                .expect("write bubble");
        }
    }
}
