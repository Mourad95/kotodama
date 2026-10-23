//! Renders frames of a theme to PNG files, to look at an animation without a phone.
//!
//!     cargo run -p kd-fx --features cpu-render --example render_frames -- \
//!         themes/anime/theme.json energie 3 "Franchement, c'est trop bien !" out/ [seed] [t_ms ...]
//!
//! Arguments: theme file, emotion id, level (1-3), message, output directory, optional seed and
//! instants in milliseconds (default: a spread over the first two seconds).

use std::path::PathBuf;

use kd_core::{Emotion, Intensity};
use kd_fx::render::Renderer;
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
    for t in times {
        let frame = sample(level, message.chars().count(), t, seed, Motion::Full);
        let pixmap = renderer.render(&frame, message, 540, 960).expect("render");
        let path = out.join(format!("{}-{}-{t:05}.png", args[1], args[2]));
        pixmap.save_png(&path).expect("write png");
        println!("{}", path.display());
    }
}
