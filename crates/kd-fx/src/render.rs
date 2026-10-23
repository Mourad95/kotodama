//! CPU reference renderer: draws a [`Frame`] to pixels with `tiny-skia` (ADR-0015).
//!
//! It is the reference the golden-frame tests compare against and the base of the GIF/WebP export.
//! The on-screen `wgpu` renderer draws the same `Frame`.
//!
//! Look: a light background, the message in the level's text colour wrapped to the width and
//! centred, effects tinted from the text colour (speed lines, shockwave, spotlight, tint), the
//! impact flash in black, onomatopoeias in their own colour with a white outline. Background effects
//! are drawn first and the message last, so the words stay readable.

use skrifa::instance::{LocationRef, Size};
use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::{FontRef, GlyphId, MetadataProvider};
use tiny_skia::{
    Color, FillRule, GradientStop, LineCap, Mask, Paint, Path, PathBuilder, Pixmap, Point,
    RadialGradient, SpreadMode, Stroke, Transform,
};

use crate::scene::{Frame, GlyphState, OnomatopoeiaState, ParticleState, Rgb, ScreenState};
use crate::{ParticleKind, ScreenKind};

const FONT: &[u8] = include_bytes!("../assets/fonts/NotoSansJP-Bold-subset.ttf");
/// Message font size as a fraction of the frame height.
const TEXT_SIZE: f32 = 0.07;
/// Widest a line of the message may be, as a fraction of the frame width.
const TEXT_WIDTH: f32 = 0.84;
/// Characters that never start a line: they stay with the word they follow.
const STICKS_TO_PREVIOUS: &str = ",.;:!?…)»”’";
/// Flash opacity from which the message is drawn in white instead of its ink colour.
const FLASH_TEXT_SWITCH: f32 = 0.35;
/// Line height in em.
const LINE_HEIGHT: f32 = 1.35;

/// How letters are drawn this frame: the message font size, the frame shake and whether the frame is
/// composited over a conversation (transparent background, white halo).
#[derive(Clone, Copy)]
struct TextStyle {
    font_px: f32,
    shake: Transform,
    overlay: bool,
}

/// Draws frames. Cheap to build; reuse it across frames.
pub struct Renderer {
    font: FontRef<'static>,
    background: Rgb,
}

/// A glyph outline in font units (y up), ready to be scaled and drawn.
struct Outline(PathBuilder);

impl OutlinePen for Outline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, -y);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, -y);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.quad_to(x1, -y1, x, -y);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.cubic_to(x1, -y1, x2, -y2, x, -y);
    }
    fn close(&mut self) {
        self.0.close();
    }
}

/// One letter placed by the layout, before the frame's animation is applied.
struct Placed {
    index: usize,
    glyph: GlyphId,
    /// Centre of the letter's advance box, in pixels.
    cx: f32,
    /// Baseline, in pixels.
    baseline: f32,
    advance: f32,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// A renderer with the bundled font and a light background.
    pub fn new() -> Self {
        Self {
            font: FontRef::new(FONT).expect("the bundled font is valid"),
            background: crate::scene::LIGHT_BACKGROUND,
        }
    }

    /// Renders `frame` for `message` (the text the frame was sampled for) at `width` x `height`.
    /// Returns `None` for an empty or absurd size.
    pub fn render(&self, frame: &Frame, message: &str, width: u32, height: u32) -> Option<Pixmap> {
        self.draw(frame, message, width, height, false)
    }

    /// Like [`Renderer::render`] but on a transparent background, to be composited over a conversation:
    /// only the effects and the message are drawn, and the letters get a white halo so they stay
    /// readable over any backdrop (light or dark).
    pub fn render_over(
        &self,
        frame: &Frame,
        message: &str,
        width: u32,
        height: u32,
    ) -> Option<Pixmap> {
        self.draw(frame, message, width, height, true)
    }

    fn draw(
        &self,
        frame: &Frame,
        message: &str,
        width: u32,
        height: u32,
        overlay: bool,
    ) -> Option<Pixmap> {
        let mut pixmap = Pixmap::new(width, height)?;
        if !overlay {
            pixmap.fill(color(self.background, 1.0));
        }
        let (w, h) = (width as f32, height as f32);
        let shake = Transform::from_translate(frame.shake.0 * w, frame.shake.1 * h);
        let ink = frame.text_color;
        // During an impact flash the background goes dark: the message switches to white so it stays readable.
        let flash = frame
            .screen
            .iter()
            .filter(|s| s.kind == ScreenKind::ImpactFlash)
            .map(|s| s.alpha)
            .fold(0.0_f32, f32::max);
        let text_ink = if flash >= FLASH_TEXT_SWITCH {
            Rgb(255, 255, 255)
        } else {
            ink
        };

        for s in &frame.screen {
            draw_screen(&mut pixmap, s, ink, w, h, shake);
        }
        for p in &frame.particles {
            draw_particle(&mut pixmap, p, w, h, shake);
        }
        let font_px = TEXT_SIZE * h;
        let style = TextStyle {
            font_px,
            shake,
            overlay,
        };
        let placed = self.layout(message, font_px, w, h);
        for g in &frame.glyphs {
            if let Some(p) = placed.iter().find(|p| p.index == g.index) {
                self.draw_glyph(&mut pixmap, p, g, text_ink, &style);
            }
        }
        for o in &frame.onomatopoeias {
            self.draw_onomatopoeia(&mut pixmap, o, font_px, w, h, shake);
        }
        Some(pixmap)
    }

    /// True if the bundled font has a glyph for every character of `text` (spaces and newlines aside).
    pub fn covers(&self, text: &str) -> bool {
        let map = self.font.charmap();
        text.chars()
            .filter(|c| !c.is_whitespace())
            .all(|c| map.map(c).is_some())
    }

    fn scale(&self, font_px: f32) -> f32 {
        let metrics = self.font.metrics(Size::unscaled(), LocationRef::default());
        font_px / f32::from(metrics.units_per_em)
    }

    fn advance(&self, ch: char, font_px: f32) -> (GlyphId, f32) {
        let gid = self.font.charmap().map(ch).unwrap_or(GlyphId::NOTDEF);
        let units = self
            .font
            .glyph_metrics(Size::unscaled(), LocationRef::default())
            .advance_width(gid)
            .unwrap_or(500.0);
        (gid, units * self.scale(font_px))
    }

    /// Greedy word wrap, centred on the frame.
    fn layout(&self, message: &str, font_px: f32, w: f32, h: f32) -> Vec<Placed> {
        let max_width = TEXT_WIDTH * w;
        let chars: Vec<(usize, char)> = message.chars().enumerate().collect();
        let mut lines: Vec<Vec<(usize, char)>> = vec![Vec::new()];
        let (mut line_w, mut word_start) = (0.0_f32, 0_usize);
        for &(i, ch) in &chars {
            let (_, adv) = self.advance(ch, font_px);
            if ch == '\n' {
                lines.push(Vec::new());
                (line_w, word_start) = (0.0, 0);
                continue;
            }
            let line = lines.last_mut().expect("at least one line");
            if line_w + adv > max_width && ch != ' ' && !STICKS_TO_PREVIOUS.contains(ch) {
                // Break at the last space of the line if there is one, else mid-word.
                let split = (word_start < line.len()).then_some(word_start);
                let mut next = Vec::new();
                if let Some(at) = split.filter(|&at| at > 0) {
                    next = line.split_off(at);
                }
                lines.push(next);
                let carried = lines.last().expect("just pushed");
                line_w = carried
                    .iter()
                    .map(|&(_, c)| self.advance(c, font_px).1)
                    .sum();
                word_start = 0;
            }
            let line = lines.last_mut().expect("at least one line");
            line.push((i, ch));
            line_w += adv;
            if ch == ' ' {
                word_start = line.len();
            }
        }
        let line_px = LINE_HEIGHT * font_px;
        let top = h * 0.5 - line_px * (lines.len() as f32 - 1.0) / 2.0;
        let mut out = Vec::new();
        for (n, line) in lines.iter().enumerate() {
            let advances: Vec<(GlyphId, f32)> = line
                .iter()
                .map(|&(_, c)| self.advance(c, font_px))
                .collect();
            let total: f32 = advances.iter().map(|a| a.1).sum();
            let mut x = (w - total) / 2.0;
            let baseline = top + n as f32 * line_px + font_px * 0.35;
            for (&(index, _), &(glyph, advance)) in line.iter().zip(&advances) {
                out.push(Placed {
                    index,
                    glyph,
                    cx: x + advance / 2.0,
                    baseline,
                    advance,
                });
                x += advance;
            }
        }
        out
    }

    fn outline(&self, glyph: GlyphId) -> Option<Path> {
        let mut b = Outline(PathBuilder::new());
        let outline = self.font.outline_glyphs().get(glyph)?;
        let settings = DrawSettings::unhinted(Size::unscaled(), LocationRef::default());
        outline.draw(settings, &mut b).ok()?;
        b.0.finish()
    }

    fn draw_glyph(
        &self,
        pixmap: &mut Pixmap,
        p: &Placed,
        g: &GlyphState,
        ink: Rgb,
        style: &TextStyle,
    ) {
        let TextStyle {
            font_px,
            shake,
            overlay,
        } = *style;
        if g.alpha <= 0.0 {
            return;
        }
        let Some(path) = self.outline(p.glyph) else {
            return;
        };
        let s = self.scale(font_px);
        let centre_y = p.baseline - font_px * 0.3;
        let at = Transform::from_translate(
            p.cx - p.advance / 2.0 + g.dx * font_px,
            p.baseline + g.dy * font_px,
        );
        // Scale and rotate around the middle of the letter.
        let pivot = (p.advance / 2.0, centre_y - p.baseline);
        let t = at
            .pre_translate(pivot.0, pivot.1)
            .pre_rotate(g.rotation.to_degrees())
            .pre_scale(g.scale, g.scale)
            .pre_translate(-pivot.0, -pivot.1)
            .pre_scale(s, s)
            .post_concat(shake);
        if overlay {
            // White halo: the message stays readable over a dark or busy conversation.
            let mut halo = Paint::default();
            halo.set_color(color(Rgb(255, 255, 255), 0.9 * g.alpha));
            let stroke = Stroke {
                width: 0.14 * font_px / s,
                line_cap: LineCap::Round,
                line_join: tiny_skia::LineJoin::Round,
                ..Stroke::default()
            };
            pixmap.stroke_path(&path, &halo, &stroke, t, None);
        }
        if g.glow > 0.0 {
            let mut glow = Paint::default();
            glow.set_color(color(ink, 0.45 * g.glow * g.alpha));
            let stroke = Stroke {
                width: 0.14 * font_px / s,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };
            pixmap.stroke_path(&path, &glow, &stroke, t, None);
        }
        let mut paint = Paint::default();
        paint.set_color(color(ink, g.alpha));
        paint.anti_alias = true;
        pixmap.fill_path(&path, &paint, FillRule::Winding, t, None);
    }

    fn draw_onomatopoeia(
        &self,
        pixmap: &mut Pixmap,
        o: &OnomatopoeiaState,
        font_px: f32,
        w: f32,
        h: f32,
        shake: Transform,
    ) {
        if o.alpha <= 0.0 || o.scale <= 0.0 {
            return;
        }
        let size = font_px * o.scale;
        let s = self.scale(size);
        let glyphs: Vec<(GlyphId, f32)> = o.text.chars().map(|c| self.advance(c, size)).collect();
        let total: f32 = glyphs.iter().map(|g| g.1).sum();
        // Keep the whole word on screen: clamp its centre by its measured width.
        let half = (total / 2.0 + 0.03 * w).min(w / 2.0);
        let x_px = (o.x * w).clamp(half, w - half);
        let at = Transform::from_translate(x_px, o.y * h)
            .pre_rotate(o.rotation.to_degrees())
            .post_concat(shake);
        let mut x = -total / 2.0;
        for (gid, adv) in glyphs {
            if let Some(path) = self.outline(gid) {
                let t = at.pre_translate(x, size * 0.35).pre_scale(s, s);
                let mut halo = Paint::default();
                halo.set_color(color(Rgb(255, 255, 255), o.alpha));
                let stroke = Stroke {
                    width: 0.16 * size / s,
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                };
                pixmap.stroke_path(&path, &halo, &stroke, t, None);
                let mut paint = Paint::default();
                paint.set_color(color(o.color, o.alpha));
                paint.anti_alias = true;
                pixmap.fill_path(&path, &paint, FillRule::Winding, t, None);
            }
            x += adv;
        }
    }
}

fn color(rgb: Rgb, alpha: f32) -> Color {
    Color::from_rgba8(
        rgb.0,
        rgb.1,
        rgb.2,
        (alpha.clamp(0.0, 1.0) * 255.0).round() as u8,
    )
}

fn paint(rgb: Rgb, alpha: f32) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color(color(rgb, alpha));
    p.anti_alias = true;
    p
}

fn full_rect(w: f32, h: f32) -> Path {
    PathBuilder::from_rect(tiny_skia::Rect::from_xywh(-w, -h, 3.0 * w, 3.0 * h).expect("non-empty"))
}

fn draw_screen(pixmap: &mut Pixmap, s: &ScreenState, ink: Rgb, w: f32, h: f32, shake: Transform) {
    let centre = Point::from_xy(w / 2.0, h / 2.0);
    match s.kind {
        ScreenKind::ImpactFlash => {
            pixmap.fill_path(
                &full_rect(w, h),
                &paint(Rgb(0, 0, 0), s.alpha),
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
        ScreenKind::Tint => {
            pixmap.fill_path(
                &full_rect(w, h),
                &paint(ink, 0.18 * s.alpha),
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
        ScreenKind::Spotlight => {
            let stops = vec![
                GradientStop::new(0.0, Color::from_rgba8(0, 0, 0, 0)),
                GradientStop::new(0.45, Color::from_rgba8(0, 0, 0, 0)),
                GradientStop::new(1.0, color(Rgb(0, 0, 0), 0.7 * s.alpha)),
            ];
            if let Some(shader) = RadialGradient::new(
                centre,
                0.0,
                centre,
                h * 0.8,
                stops,
                SpreadMode::Pad,
                Transform::identity(),
            ) {
                let p = Paint {
                    shader,
                    ..Paint::default()
                };
                pixmap.fill_path(
                    &full_rect(w, h),
                    &p,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
            }
        }
        ScreenKind::Shockwave => {
            let radius = s.progress.powf(0.6) * h * 0.7;
            if radius > 1.0 {
                if let Some(ring) = PathBuilder::from_circle(centre.x, centre.y, radius) {
                    let stroke = Stroke {
                        width: 0.035 * h * (1.0 - 0.5 * s.progress),
                        ..Stroke::default()
                    };
                    pixmap.stroke_path(&ring, &paint(ink, 0.55 * s.alpha), &stroke, shake, None);
                }
            }
        }
        ScreenKind::SpeedLines => {
            // Thin wedges converging on the message; angles are fixed so the lines are stable.
            let (inner, outer) = (h * 0.22, h * 0.95);
            let mut pb = PathBuilder::new();
            for i in 0..56_u32 {
                let a =
                    (i as f32 + 0.5 * ((i * 7 % 5) as f32 / 5.0)) / 56.0 * std::f32::consts::TAU;
                let half = 0.012 + 0.012 * ((i * 13 % 7) as f32 / 7.0);
                let r0 = inner + h * 0.12 * ((i * 11 % 6) as f32 / 6.0);
                for (r, da) in [(r0, 0.0), (outer, -half), (outer, half)] {
                    let (x, y) = (centre.x + r * (a + da).cos(), centre.y + r * (a + da).sin());
                    if r == r0 {
                        pb.move_to(x, y)
                    } else {
                        pb.line_to(x, y)
                    }
                }
                pb.close();
            }
            if let Some(path) = pb.finish() {
                pixmap.fill_path(
                    &path,
                    &paint(ink, 0.55 * s.alpha),
                    FillRule::Winding,
                    shake,
                    None,
                );
            }
        }
        ScreenKind::Halftone => draw_halftone(pixmap, s, ink, w, h, shake),
        ScreenKind::Burst => draw_burst(pixmap, s, ink, w, h, shake),
        ScreenKind::Shake => {}
    }
}

/// Screentone: staggered rows of dots whose size grows from nothing near the centre (where the message
/// is) to a full dot at the edges, so the middle of the frame stays clean.
fn draw_halftone(pixmap: &mut Pixmap, s: &ScreenState, ink: Rgb, w: f32, h: f32, shake: Transform) {
    let step = 0.03 * h;
    let half_diagonal = (w * w + h * h).sqrt() / 2.0;
    let mut dots = PathBuilder::new();
    let mut row = 0_u32;
    let mut y = step / 2.0;
    while y < h + step {
        let offset = if row.is_multiple_of(2) {
            0.0
        } else {
            step / 2.0
        };
        let mut x = offset;
        while x < w + step {
            let d = ((x - w / 2.0).powi(2) + (y - h / 2.0).powi(2)).sqrt() / half_diagonal;
            let grow = ((d - 0.35) / 0.65).clamp(0.0, 1.0);
            let r = step * 0.5 * grow * grow * s.strength.clamp(0.0, 1.0);
            if r > 0.8 {
                dots.push_circle(x, y, r);
            }
            x += step;
        }
        y += step * 0.9;
        row += 1;
    }
    if let Some(path) = dots.finish() {
        pixmap.fill_path(
            &path,
            &paint(ink, 0.4 * s.alpha / s.strength.max(0.01)),
            FillRule::Winding,
            shake,
            None,
        );
    }
}

/// Manga explosion: a spiky pale shape (a tint of the ink colour) with an ink outline, popping in with
/// a little overshoot. Drawn behind the message, so the words sit on a clean light area.
fn draw_burst(pixmap: &mut Pixmap, s: &ScreenState, ink: Rgb, w: f32, h: f32, shake: Transform) {
    const SPIKES: u32 = 18;
    let entrance = (s.progress / 0.12).clamp(0.0, 1.0);
    let grow = crate::scene::ease_out_back(entrance);
    let base = 0.5 * w.min(h * 0.7) * 1.05 * grow;
    if base < 2.0 {
        return;
    }
    let mut pb = PathBuilder::new();
    for i in 0..SPIKES * 2 {
        let spike = i % 2 == 0;
        // Deterministic irregularity so the shape looks drawn by hand, not stamped.
        let wobble = 0.88 + 0.24 * (((u64::from(i) * 2_654_435_761) >> 7) % 100) as f32 / 100.0;
        let radius = if spike { base * wobble } else { base * 0.74 };
        let angle = i as f32 / (SPIKES * 2) as f32 * std::f32::consts::TAU;
        let (x, y) = (
            w / 2.0 + radius * 1.1 * angle.cos(),
            h / 2.0 + radius * 1.0 * angle.sin(),
        );
        if i == 0 {
            pb.move_to(x, y);
        } else {
            pb.line_to(x, y);
        }
    }
    pb.close();
    if let Some(path) = pb.finish() {
        let pale = Rgb(
            (u16::from(ink.0) / 10 + 230) as u8,
            (u16::from(ink.1) / 10 + 230) as u8,
            (u16::from(ink.2) / 10 + 230) as u8,
        );
        pixmap.fill_path(
            &path,
            &paint(pale, 0.92 * s.alpha),
            FillRule::Winding,
            shake,
            None,
        );
        let stroke = Stroke {
            width: 0.007 * h,
            line_join: tiny_skia::LineJoin::Miter,
            ..Stroke::default()
        };
        pixmap.stroke_path(&path, &paint(ink, 0.85 * s.alpha), &stroke, shake, None);
    }
}

fn star(cx: f32, cy: f32, r: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    for i in 0..8 {
        let a = i as f32 * std::f32::consts::FRAC_PI_4 - std::f32::consts::FRAC_PI_2;
        let rr = if i % 2 == 0 { r } else { r * 0.3 };
        let (x, y) = (cx + rr * a.cos(), cy + rr * a.sin());
        if i == 0 {
            pb.move_to(x, y)
        } else {
            pb.line_to(x, y)
        }
    }
    pb.close();
    pb.finish()
}

fn heart(cx: f32, cy: f32, r: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(cx, cy + r);
    pb.cubic_to(
        cx - 1.6 * r,
        cy,
        cx - 0.9 * r,
        cy - 1.2 * r,
        cx,
        cy - 0.4 * r,
    );
    pb.cubic_to(cx + 0.9 * r, cy - 1.2 * r, cx + 1.6 * r, cy, cx, cy + r);
    pb.close();
    pb.finish()
}

fn draw_particle(pixmap: &mut Pixmap, p: &ParticleState, w: f32, h: f32, shake: Transform) {
    if p.alpha <= 0.0 {
        return;
    }
    let (x, y, r) = (p.x * w, p.y * h, p.size * h);
    let spin = shake.pre_rotate_at(p.rotation.to_degrees(), x, y);
    let fill = paint(p.color, p.alpha);
    let path = match p.kind {
        ParticleKind::Confetti => {
            tiny_skia::Rect::from_xywh(x - r, y - r * 0.5, r * 2.0, r).map(PathBuilder::from_rect)
        }
        ParticleKind::Stars => star(x, y, r * 1.6),
        ParticleKind::Hearts => heart(x, y, r),
        ParticleKind::Petals => {
            let mut pb = PathBuilder::new();
            pb.move_to(x - r, y);
            pb.quad_to(x, y - r * 1.2, x + r, y);
            pb.quad_to(x, y + r * 1.2, x - r, y);
            pb.close();
            pb.finish()
        }
        ParticleKind::Rain => tiny_skia::Rect::from_xywh(x - r * 0.06, y - r, r * 0.12, r * 2.0)
            .map(PathBuilder::from_rect),
        ParticleKind::Aura | ParticleKind::Embers => PathBuilder::from_circle(x, y, r),
    };
    if let Some(path) = path {
        pixmap.fill_path(&path, &fill, FillRule::Winding, spin, None::<&Mask>);
    }
}
