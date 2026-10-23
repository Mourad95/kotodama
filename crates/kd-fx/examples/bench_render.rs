//! Times the two layers the phone draws (ADR-0017) for every emotion at full level: the back layer at
//! half the screen width, the front layer at the full width.
//!
//!     cargo run --release -p kd-fx --features cpu-render --example bench_render -- themes/anime/theme.json

use std::time::Instant;

use kd_core::{Emotion, Intensity};
use kd_fx::render::{Layer, Renderer};
use kd_fx::scene::{sample, Motion};
use kd_fx::Theme;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "themes/anime/theme.json".into());
    let theme =
        Theme::from_json(&std::fs::read_to_string(path).expect("read theme")).expect("valid theme");
    let renderer = Renderer::new();
    let message = "Franchement, c'est trop bien !";
    let (w, h) = (1080_u32, 1500_u32);
    for (layer, scale) in [(Layer::Back, 2), (Layer::Front, 1)] {
        println!("{layer:?} at {}x{}", w / scale, h / scale);
        for emotion in Emotion::ALL {
            let level = theme.level(emotion, Intensity::Full);
            let (mut total, mut worst, mut n) = (0.0_f64, 0.0_f64, 0);
            for t in (0..2400).step_by(40) {
                let frame = sample(level, message.chars().count(), t, 1, Motion::Full);
                let start = Instant::now();
                let pixmap = renderer
                    .render_layer(&frame, message, w / scale, h / scale, layer)
                    .expect("render");
                let ms = start.elapsed().as_secs_f64() * 1000.0;
                std::hint::black_box(pixmap);
                (total, worst, n) = (total + ms, worst.max(ms), n + 1);
            }
            println!(
                "  {}: avg {:.1} ms, worst {:.1} ms",
                emotion.id(),
                total / f64::from(n),
                worst
            );
        }
    }
}
