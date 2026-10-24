//! Export of a sticker as an animated GIF, to share through the system share sheet (ADR-0025,
//! docs/legal.md L15, L25). The GIF plays the sticker animation once, then holds its last frame,
//! the settled scene, before it loops (messaging apps loop GIFs).

use crate::render::{Layer, Renderer};
use crate::scene::{self, Motion};
use crate::Level;

/// Time between two frames, in hundredths of a second (GIF delays are in that unit): 20 fps.
const FRAME_DELAY_CS: u16 = 5;
/// How long the last frame stays, in hundredths of a second, before the GIF loops.
const HOLD_CS: u16 = 250;
/// Most frames in one GIF, so a long message stays a few megabytes.
const MAX_FRAMES: u32 = 90;
/// Colour quantization effort: 1 is the best and slowest, 30 the fastest.
const QUANTIZE_SPEED: i32 = 10;

/// A GIF could not be made.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    /// The side is zero, too large for GIF, or the frame could not be allocated.
    #[error("unsupported GIF size")]
    Size,
    /// The encoder failed.
    #[error("GIF encoding failed: {0}")]
    Encode(String),
}

/// Encodes the sticker of `message` (ADR-0023) as a `side` x `side` looping GIF. With reduced
/// motion the GIF is the settled scene, still (one frame). Deterministic for the same inputs.
pub fn sticker_gif(
    renderer: &Renderer,
    level: &Level,
    message: &str,
    side: u32,
    seed: u64,
    motion: Motion,
) -> Result<Vec<u8>, ExportError> {
    let gif_side = u16::try_from(side).map_err(|_| ExportError::Size)?;
    if gif_side == 0 {
        return Err(ExportError::Size);
    }
    let glyphs = message.chars().count();
    let instants: Vec<u32> = if motion == Motion::Reduced {
        vec![0]
    } else {
        let end = scene::duration_ms(level, glyphs, motion);
        let step = u32::from(FRAME_DELAY_CS) * 10;
        let count = (end / step + 1).min(MAX_FRAMES);
        let step = if count == MAX_FRAMES {
            end / (MAX_FRAMES - 1)
        } else {
            step
        };
        let mut instants: Vec<u32> = (0..count).map(|i| (i * step).min(end)).collect();
        if instants.last() != Some(&end) {
            instants.push(end);
        }
        instants
    };
    let mut out = Vec::new();
    {
        let mut encoder = gif::Encoder::new(&mut out, gif_side, gif_side, &[])
            .map_err(|e| ExportError::Encode(e.to_string()))?;
        encoder
            .set_repeat(gif::Repeat::Infinite)
            .map_err(|e| ExportError::Encode(e.to_string()))?;
        let last = instants.len() - 1;
        let mut previous: Option<Vec<u8>> = None;
        for (i, &t) in instants.iter().enumerate() {
            let frame = scene::sample_sticker(level, glyphs, t, seed, motion);
            let pixmap = renderer
                .render_layer(&frame, message, side, side, Layer::Opaque)
                .ok_or(ExportError::Size)?;
            let rgba = pixmap.take();
            let mut delta = rgba.clone();
            if let Some(previous) = &previous {
                transparent_where_unchanged(&mut delta, previous);
            }
            let mut gif_frame =
                gif::Frame::from_rgba_speed(gif_side, gif_side, &mut delta, QUANTIZE_SPEED);
            gif_frame.dispose = gif::DisposalMethod::Keep;
            gif_frame.delay = if i == last { HOLD_CS } else { FRAME_DELAY_CS };
            encoder
                .write_frame(&gif_frame)
                .map_err(|e| ExportError::Encode(e.to_string()))?;
            previous = Some(rgba);
        }
    }
    Ok(out)
}

/// Makes the pixels of `frame` that equal those of `previous` fully transparent, so the GIF
/// keeps the previous frame there: only what moved is stored, which keeps the file small.
fn transparent_where_unchanged(frame: &mut [u8], previous: &[u8]) {
    for (pixel, before) in frame.chunks_exact_mut(4).zip(previous.chunks_exact(4)) {
        if pixel == before {
            pixel[3] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Theme;
    use kd_core::{Emotion, Intensity};

    fn level() -> Level {
        let theme = Theme::from_json(include_str!("../../../themes/anime/theme.json")).unwrap();
        theme.level(Emotion::Joy, Intensity::Full).clone()
    }

    fn frames(gif: &[u8]) -> Vec<u16> {
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::Indexed);
        let mut decoder = options.read_info(gif).unwrap();
        let mut delays = Vec::new();
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            delays.push(frame.delay);
        }
        delays
    }

    #[test]
    fn plays_once_then_holds_the_settled_scene() {
        let renderer = Renderer::new();
        let gif = sticker_gif(&renderer, &level(), "trop bien", 96, 7, Motion::Full).unwrap();
        assert_eq!(&gif[..6], b"GIF89a");
        let delays = frames(&gif);
        assert!(delays.len() > 5 && delays.len() as u32 <= MAX_FRAMES + 1);
        assert_eq!(*delays.last().unwrap(), HOLD_CS);
        assert!(delays[..delays.len() - 1]
            .iter()
            .all(|&d| d == FRAME_DELAY_CS));
    }

    #[test]
    fn only_changed_pixels_stay_opaque() {
        let previous = [1, 2, 3, 255, 9, 9, 9, 255];
        let mut frame = [1, 2, 3, 255, 8, 9, 9, 255];
        transparent_where_unchanged(&mut frame, &previous);
        assert_eq!(frame, [1, 2, 3, 0, 8, 9, 9, 255]);
    }

    #[test]
    fn reduced_motion_is_one_still_frame() {
        let renderer = Renderer::new();
        let gif = sticker_gif(&renderer, &level(), "trop bien", 64, 7, Motion::Reduced).unwrap();
        assert_eq!(frames(&gif).len(), 1);
    }

    #[test]
    fn is_deterministic_and_rejects_bad_sizes() {
        let renderer = Renderer::new();
        let a = sticker_gif(&renderer, &level(), "salut", 48, 3, Motion::Gentle).unwrap();
        let b = sticker_gif(&renderer, &level(), "salut", 48, 3, Motion::Gentle).unwrap();
        assert_eq!(a, b);
        assert!(sticker_gif(&renderer, &level(), "salut", 0, 3, Motion::Full).is_err());
        assert!(sticker_gif(&renderer, &level(), "salut", 70_000, 3, Motion::Full).is_err());
    }
}
