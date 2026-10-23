//! Renders frames of a theme to PNG files, to look at an animation without a phone.
//!
//!     cargo run -p kd-fx --features cpu-render --example render_frames -- \
//!         themes/anime/theme.json energie 3 "Franchement, c'est trop bien !" out/ [seed] [t_ms ...]
//!
//! Arguments: theme file, emotion id, level (1-3), message, output directory, optional seed and
//! instants in milliseconds (default: a spread over the first two seconds). `KD_SIZE=1080x1500` changes
//! the frame size (default 540x960). `KD_LAYERS=1` composites the frame like the phone does (ADR-0017):
//! the back layer drawn at 0.4 of the width and smoothed up, the front layer at full size over it.

use std::path::PathBuf;

use kd_core::{Emotion, Intensity};
use kd_fx::render::{Layer, Renderer};
use kd_fx::scene::{sample, Motion};
use kd_fx::Theme;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 5 {
        eprintln!("usage: render_frames <theme.json> <emotion> <level 1-3> <message> <out dir> [seed] [t_ms ...]");
        std::process::exit(2);
    }
    let theme = Theme::from_json(&std::fs::read_to_string(&args[0]).expect("read theme"))
        .expect("valid theme");
    let emotion = Emotion::from_id(&args[1]).expect("emotion id");
    let intensity = match args[2].as_str() {
        "1" => Intensity::Low,
        "2" => Intensity::Medium,
        _ => Intensity::Full,
    };
    let (message, out) = (&args[3], PathBuf::from(&args[4]));
    let seed: u64 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(1);
    let times: Vec<u32> = if args.len() > 6 {
        args[6..].iter().filter_map(|s| s.parse().ok()).collect()
    } else {
        vec![0, 200, 400, 700, 1000, 1100, 1300, 1700, 2200]
    };
    std::fs::create_dir_all(&out).expect("create output directory");
    let level = theme.level(emotion, intensity);
    let renderer = Renderer::new();
    let (width, height) = std::env::var("KD_SIZE")
        .ok()
        .and_then(|v| {
            let (w, h) = v.split_once('x')?;
            Some((w.parse().ok()?, h.parse().ok()?))
        })
        .unwrap_or((540, 960));
    for t in times {
        let frame = sample(level, message.chars().count(), t, seed, Motion::Full);
        let pixmap = if std::env::var_os("KD_LAYERS").is_some() {
            layered(&renderer, &frame, message, width, height)
        } else {
            renderer
                .render(&frame, message, width, height)
                .expect("render")
        };
        let path = out.join(format!("{}-{}-{t:05}.png", args[1], args[2]));
        pixmap.save_png(&path).expect("write png");
        println!("{}", path.display());
    }
}

/// The frame as the phone shows it: the back layer drawn small and scaled up, the front layer over it.
fn layered(
    renderer: &Renderer,
    frame: &kd_fx::scene::Frame,
    message: &str,
    width: u32,
    height: u32,
) -> tiny_skia::Pixmap {
    let small = ((width as f32 * 0.4) as u32).max(1);
    let small_h = ((height as f32 * 0.4) as u32).max(1);
    let back = renderer
        .render_layer(frame, message, small, small_h, Layer::Back)
        .expect("render back");
    let front = renderer
        .render_layer(frame, message, width, height, Layer::Front)
        .expect("render front");
    let mut out = tiny_skia::Pixmap::new(width, height).expect("frame size");
    out.fill(tiny_skia::Color::from_rgba8(242, 242, 245, 255));
    let paint = tiny_skia::PixmapPaint {
        quality: tiny_skia::FilterQuality::Bicubic,
        ..Default::default()
    };
    let scale = tiny_skia::Transform::from_scale(
        width as f32 / small as f32,
        height as f32 / small_h as f32,
    );
    out.draw_pixmap(0, 0, back.as_ref(), &paint, scale, None);
    out.draw_pixmap(
        0,
        0,
        front.as_ref(),
        &tiny_skia::PixmapPaint::default(),
        tiny_skia::Transform::identity(),
        None,
    );
    out
}
