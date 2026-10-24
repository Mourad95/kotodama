//! Full-screen manga props around the message (ADR-0020): suns, flames, neon bolts, spirals,
//! hearts, leaves… scattered over the whole frame in bold black-outlined ink, and the backdrop
//! colour behind them. The theme picks the motif and the colours; the bubble stays clear.
//! No theme or emotion identifiers belong here.

use crate::{
    scene::{ease_out_back, MangaState, Rgb},
    MangaMotif,
};
use std::f32::consts::{PI, TAU};
use tiny_skia::{
    Color, FillRule, GradientStop, LineCap, LineJoin, Paint, Path, PathBuilder, Pixmap, Point,
    RadialGradient, Rect, SpreadMode, Stroke, Transform,
};

/// The outline of every prop: near-black ink, like the lettering's.
const OUTLINE: Rgb = Rgb(18, 18, 26);
const WHITE: Rgb = Rgb(255, 255, 255);

/// A rect in pixels: x, y, width, height.
pub(crate) type Bounds = (f32, f32, f32, f32);

/// One prop shape. Each motif scatters a few of them.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Prop {
    Sun,
    Star,
    Bubble,
    Sparkle,
    Anger,
    Smoke,
    Bolt,
    Shard,
    Crack,
    Spiral,
    Sweat,
    Tear,
    Heart,
    Leaf,
    Sakura,
    Wind,
}

fn props(motif: MangaMotif) -> &'static [Prop] {
    use Prop::*;
    match motif {
        MangaMotif::Radiance => &[Sun, Star, Bubble, Star, Sparkle],
        MangaMotif::Pressure => &[Anger, Smoke, Bolt, Anger],
        MangaMotif::Neon => &[Bolt, Shard, Sparkle, Crack, Bolt],
        MangaMotif::Orbit => &[Shard, Sparkle, Shard, Bolt],
        MangaMotif::Tension => &[Spiral, Sweat, Spiral, Sparkle],
        MangaMotif::Rainfall => &[Tear, Sweat, Tear],
        MangaMotif::Blossom => &[Heart, Sparkle, Heart, Bubble],
        MangaMotif::Breeze => &[Leaf, Sakura, Bubble, Wind, Leaf],
        MangaMotif::Blush => &[Sweat, Spiral, Sweat],
    }
}

/// Draws the backdrop colour of the style over the whole frame, a radial gradient from the bubble
/// outwards, with the bubble cut out so the message stays clear.
pub(crate) fn draw_backdrop(
    image: &mut Pixmap,
    state: &MangaState,
    frame: (f32, f32),
    hole: Option<Bounds>,
) {
    let Some(backdrop) = &state.style.backdrop else {
        return;
    };
    let alpha = backdrop.opacity.clamp(0.0, 1.0) * state.alpha;
    if alpha <= 0.0 {
        return;
    }
    let (w, h) = frame;
    let centre = hole.map_or(Point::from_xy(w / 2.0, h / 2.0), |b| {
        Point::from_xy(b.0 + b.2 / 2.0, b.1 + b.3 / 2.0)
    });
    let reach = {
        let dx = centre.x.max(w - centre.x);
        let dy = centre.y.max(h - centre.y);
        (dx * dx + dy * dy).sqrt().max(1.0)
    };
    let stops = vec![
        GradientStop::new(0.0, color(Rgb::from_hex(&backdrop.inner), alpha)),
        GradientStop::new(1.0, color(Rgb::from_hex(&backdrop.outer), alpha)),
    ];
    let Some(shader) = RadialGradient::new(
        centre,
        0.0,
        centre,
        reach,
        stops,
        SpreadMode::Pad,
        Transform::identity(),
    ) else {
        return;
    };
    let mut pb = PathBuilder::new();
    if let Some(r) = Rect::from_xywh(0.0, 0.0, w, h) {
        pb.push_rect(r);
    }
    if let Some(b) = hole {
        push_round_rect(&mut pb, b, 0.18 * b.2.min(b.3));
    }
    let Some(path) = pb.finish() else {
        return;
    };
    let paint = Paint {
        shader,
        anti_alias: true,
        ..Paint::default()
    };
    image.fill_path(
        &path,
        &paint,
        FillRule::EvenOdd,
        Transform::identity(),
        None,
    );
    if let Some(rays) = &backdrop.rays {
        let alpha = rays.opacity.clamp(0.0, 1.0) * state.alpha;
        let start = hole.map_or(0.12 * w.min(h), |b| 0.5 * (b.2 * b.2 + b.3 * b.3).sqrt());
        speed_lines(image, rays, centre, start, reach, state.progress, alpha);
    }
}

/// Manga speed lines: `rays.count` thin wedges bursting from `start` pixels around `centre` past
/// `reach`, never inside `start`, so the bubble stays clear. They turn slowly with `progress`.
fn speed_lines(
    image: &mut Pixmap,
    rays: &crate::Rays,
    centre: Point,
    start: f32,
    reach: f32,
    progress: f32,
    alpha: f32,
) {
    if alpha <= 0.0 || rays.count == 0 {
        return;
    }
    let n = rays.count as f32;
    let spin = 0.05 * progress;
    let mut pb = PathBuilder::new();
    for i in 0..rays.count {
        let key = 1000 + u64::from(i);
        let angle = (i as f32 + 0.6 * (hash(key, 0) - 0.5)) * TAU / n + spin;
        let half = (0.18 + 0.32 * hash(key, 1)) * PI / n;
        let r0 = start * (1.05 + 0.45 * hash(key, 2));
        let r1 = reach * 1.1;
        let at = |r: f32, a: f32| (centre.x + r * a.cos(), centre.y + r * a.sin());
        let (x0, y0) = at(r0, angle);
        let (x1, y1) = at(r1, angle - half);
        let (x2, y2) = at(r1, angle + half);
        pb.move_to(x0, y0);
        pb.line_to(x1, y1);
        pb.line_to(x2, y2);
        pb.close();
    }
    if let Some(path) = pb.finish() {
        image.fill_path(
            &path,
            &paint(Rgb::from_hex(&rays.color), alpha),
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}

/// Scatters the motif's props over the frame, around `keep` (the bubble, or the message panel),
/// plus the motif's frame-wide marks: a row of flames, rain, a neon tube, zigzag bands.
pub(crate) fn draw_scatter(
    image: &mut Pixmap,
    state: &MangaState,
    frame: (f32, f32),
    keep: Bounds,
    shake: Transform,
) {
    if state.alpha <= 0.0 {
        return;
    }
    let (w, h) = frame;
    let u = w.min(h);
    let style = &state.style;
    let accent = Rgb::from_hex(&style.accent_color);
    let palette: Vec<Rgb> = if style.palette.is_empty() {
        vec![accent]
    } else {
        style.palette.iter().map(|c| Rgb::from_hex(c)).collect()
    };
    let mut pen = Pen {
        image,
        alpha: state.alpha,
        u,
    };
    let p = state.progress;
    let strength = state.strength.clamp(0.0, 1.0);
    match style.motif {
        MangaMotif::Pressure => {
            smoke(&mut pen, accent, p, strength, frame, keep, shake);
            flames(&mut pen, &palette, p, strength, frame, shake);
        }
        MangaMotif::Rainfall => rain(&mut pen, &palette, p, strength, frame, keep, shake),
        MangaMotif::Blush => blush_bands(&mut pen, &palette, p, strength, frame, keep, shake),
        MangaMotif::Neon => neon_tube(&mut pen, accent, &palette, p, strength, keep, shake),
        _ => {}
    }
    // A jittered grid over the frame: each cell may hold one prop, never over the bubble.
    let kinds = props(style.motif);
    let cell = u * 0.40;
    let cols = (w / cell).ceil().max(1.0) as u32;
    let rows = (h / cell).ceil().max(1.0) as u32;
    let margin = cell * 0.3;
    let clear = (
        keep.0 - margin,
        keep.1 - margin,
        keep.2 + 2.0 * margin,
        keep.3 + 2.0 * margin,
    );
    let shown = 0.15 + 0.70 * strength;
    for row in 0..rows {
        for col in 0..cols {
            let key = (row * 64 + col) as u64;
            if hash(key, 0) > shown {
                continue;
            }
            let x = (col as f32 + 0.5 + 0.6 * (hash(key, 1) - 0.5)) * w / cols as f32;
            let y = (row as f32 + 0.5 + 0.6 * (hash(key, 2) - 0.5)) * h / rows as f32;
            if inside(clear, x, y) {
                continue;
            }
            let index = (hash(key, 3) * kinds.len() as f32) as usize % kinds.len();
            let kind = kinds[index];
            // The vein-pop mark always takes the accent, like the 💢 it draws.
            let tint = if kind == Prop::Anger {
                accent
            } else {
                palette[(index + (hash(key, 4) * 3.0) as usize) % palette.len()]
            };
            // Each prop pops in at its own time, then bobs and wobbles in place.
            let delay = 0.25 * hash(key, 5);
            let pop = ease_out_back(((p - delay) / 0.08).clamp(0.0, 1.0));
            if pop <= 0.0 {
                continue;
            }
            let phase = TAU * hash(key, 6);
            let r = cell * (0.24 + 0.12 * hash(key, 7)) * (0.75 + 0.25 * strength) * pop;
            let bob = 0.06 * r * (p * TAU * 3.0 + phase).sin();
            let turn = 0.5 * (hash(key, 8) - 0.5) + 0.08 * (p * TAU * 2.0 + phase).sin();
            let at = Transform::from_translate(x, y + bob)
                .pre_rotate(turn.to_degrees())
                .post_concat(shake);
            draw_prop(&mut pen, kind, r, tint, p + hash(key, 9), at);
        }
    }
}

/// Bold drawing in pixels: fills edged with the black outline.
pub(crate) struct Pen<'a> {
    image: &'a mut Pixmap,
    alpha: f32,
    u: f32,
}

impl Pen<'_> {
    fn fill(&mut self, path: &Path, rgb: Rgb, alpha: f32, at: Transform) {
        self.image.fill_path(
            path,
            &paint(rgb, alpha * self.alpha),
            FillRule::Winding,
            at,
            None,
        );
    }

    fn stroke(&mut self, path: &Path, rgb: Rgb, width: f32, alpha: f32, at: Transform) {
        let stroke = Stroke {
            width: width.max(0.6),
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            ..Stroke::default()
        };
        self.image
            .stroke_path(path, &paint(rgb, alpha * self.alpha), &stroke, at, None);
    }

    /// The outline width for a prop of radius `r`.
    fn ink(&self, r: f32) -> f32 {
        (r * 0.13).clamp(0.004 * self.u, 0.012 * self.u)
    }

    /// Fills `path` with `rgb`, edged in black ink.
    fn inked(&mut self, path: &Path, rgb: Rgb, r: f32, at: Transform) {
        self.fill(path, rgb, 1.0, at);
        let width = self.ink(r);
        self.stroke(path, OUTLINE, width, 1.0, at);
    }

    /// Strokes `path` in `rgb` over a wider black stroke, like an inked brush line.
    fn brush(&mut self, path: &Path, rgb: Rgb, width: f32, at: Transform) {
        let edge = self.ink(width * 3.0);
        self.stroke(path, OUTLINE, width + 2.0 * edge, 1.0, at);
        self.stroke(path, rgb, width, 1.0, at);
    }
}

fn draw_prop(pen: &mut Pen<'_>, kind: Prop, r: f32, tint: Rgb, t: f32, at: Transform) {
    if r < 1.0 {
        return;
    }
    match kind {
        Prop::Sun => {
            if let Some(rays) = star_path(12, r, r * 0.68) {
                pen.inked(&rays, tint, r, at);
            }
            if let Some(disc) = circle(0.0, 0.0, r * 0.58) {
                pen.inked(&disc, lighten(tint, 0.35), r, at);
            }
            highlight(pen, -r * 0.2, -r * 0.2, r * 0.14, at);
        }
        Prop::Star => {
            if let Some(star) = star_path(5, r, r * 0.45) {
                pen.inked(&star, tint, r, at);
            }
            highlight(pen, -r * 0.12, -r * 0.2, r * 0.1, at);
        }
        Prop::Bubble => {
            if let Some(disc) = circle(0.0, 0.0, r * 0.7) {
                pen.fill(&disc, lighten(tint, 0.7), 0.55, at);
                let width = pen.ink(r);
                pen.stroke(&disc, OUTLINE, width, 1.0, at);
            }
            highlight(pen, -r * 0.28, -r * 0.28, r * 0.13, at);
        }
        Prop::Sparkle => {
            if let Some(star) = star_path(4, r * 0.8, r * 0.2) {
                pen.inked(&star, lighten(tint, 0.25), r, at);
            }
        }
        Prop::Anger => {
            // The vein-pop mark: four bold crescents around a cross-shaped gap, throbbing.
            let beat = 1.0 + 0.08 * (t * TAU * 5.0).sin().max(0.0);
            anger_mark(pen, r, tint, at.pre_scale(beat, beat));
        }
        Prop::Smoke => {
            const PUFF: [(f32, f32, f32); 5] = [
                (-0.55, 0.18, 0.40),
                (-0.18, -0.22, 0.50),
                (0.32, -0.12, 0.46),
                (0.62, 0.24, 0.34),
                (0.05, 0.30, 0.42),
            ];
            let lobes = PUFF.map(|(x, y, s)| (x * r, y * r, s * r));
            puff(pen, &lobes, r, smoke_tone(tint), at);
        }
        Prop::Bolt => {
            const BOLT: [(f32, f32); 7] = [
                (0.15, -1.0),
                (-0.5, 0.12),
                (-0.05, 0.12),
                (-0.25, 1.0),
                (0.55, -0.22),
                (0.08, -0.22),
                (0.35, -1.0),
            ];
            if let Some(bolt) = polygon(&BOLT, r) {
                glow(pen, r * 1.3, tint, 0.25, at);
                pen.inked(&bolt, tint, r, at);
            }
        }
        Prop::Shard => {
            const SHARD: [(f32, f32); 4] = [(0.0, -1.0), (0.45, 0.1), (0.0, 0.8), (-0.35, -0.05)];
            if let Some(shard) = polygon(&SHARD, r) {
                pen.inked(&shard, tint, r, at);
            }
            highlight(pen, -r * 0.05, -r * 0.35, r * 0.08, at);
        }
        Prop::Crack => {
            // Impact cracks: a few broken lines from one point, no fill.
            let mut pb = PathBuilder::new();
            for i in 0..5 {
                let a = i as f32 / 5.0 * TAU + 0.3;
                pb.move_to(0.0, 0.0);
                let (mx, my) = (r * 0.5 * (a + 0.2).cos(), r * 0.5 * (a + 0.2).sin());
                pb.line_to(mx, my);
                pb.line_to(r * a.cos(), r * a.sin());
            }
            if let Some(path) = pb.finish() {
                pen.brush(&path, lighten(tint, 0.6), r * 0.05, at);
            }
        }
        Prop::Spiral => {
            let mut pb = PathBuilder::new();
            for i in 0..=48 {
                let a = i as f32 / 48.0 * TAU * 2.5 + t * TAU;
                let d = r * (0.08 + 0.92 * i as f32 / 48.0);
                let (x, y) = (d * a.cos(), d * a.sin());
                if i == 0 {
                    pb.move_to(x, y);
                } else {
                    pb.line_to(x, y);
                }
            }
            if let Some(path) = pb.finish() {
                pen.brush(&path, tint, r * 0.1, at);
            }
        }
        Prop::Sweat | Prop::Tear => {
            let s = if kind == Prop::Tear { r } else { r * 0.75 };
            if let Some(drop) = drop_path(s) {
                pen.inked(&drop, lighten(tint, 0.2), r, at);
            }
            highlight(pen, -s * 0.22, s * 0.25, s * 0.12, at);
        }
        Prop::Heart => {
            if let Some(heart) = heart_path(r) {
                // A gentle beat.
                let beat = 1.0 + 0.06 * (t * TAU * 4.0).sin().max(0.0);
                let at = at.pre_scale(beat, beat);
                pen.inked(&heart, tint, r, at);
                highlight(pen, -r * 0.42, -r * 0.28, r * 0.14, at);
            }
        }
        Prop::Leaf => {
            let mut pb = PathBuilder::new();
            pb.move_to(0.0, -r);
            pb.quad_to(r * 0.75, 0.0, 0.0, r);
            pb.quad_to(-r * 0.75, 0.0, 0.0, -r);
            pb.close();
            if let Some(leaf) = pb.finish() {
                pen.inked(&leaf, tint, r, at);
            }
            let mut pb = PathBuilder::new();
            pb.move_to(0.0, -r * 0.7);
            pb.line_to(0.0, r * 1.15);
            if let Some(vein) = pb.finish() {
                let width = pen.ink(r) * 0.8;
                pen.stroke(&vein, OUTLINE, width, 1.0, at);
            }
        }
        Prop::Sakura => {
            for i in 0..5 {
                let petal = at.pre_rotate(72.0 * i as f32);
                let mut pb = PathBuilder::new();
                pb.move_to(0.0, 0.0);
                pb.cubic_to(-r * 0.45, -r * 0.3, -r * 0.3, -r * 0.95, -r * 0.08, -r);
                pb.line_to(0.0, -r * 0.85);
                pb.line_to(r * 0.08, -r);
                pb.cubic_to(r * 0.3, -r * 0.95, r * 0.45, -r * 0.3, 0.0, 0.0);
                pb.close();
                if let Some(path) = pb.finish() {
                    pen.inked(&path, tint, r * 0.8, petal);
                }
            }
            if let Some(core) = circle(0.0, 0.0, r * 0.14) {
                pen.fill(&core, Rgb(255, 214, 92), 1.0, at);
            }
        }
        Prop::Wind => {
            let mut pb = PathBuilder::new();
            pb.move_to(-r * 1.2, r * 0.2);
            pb.cubic_to(-r * 0.4, r * 0.5, r * 0.5, r * 0.1, r * 0.6, -r * 0.3);
            pb.cubic_to(r * 0.7, -r * 0.7, r * 0.15, -r * 0.75, r * 0.1, -r * 0.4);
            pb.cubic_to(r * 0.06, -r * 0.15, r * 0.35, -r * 0.1, r * 0.4, -r * 0.3);
            if let Some(path) = pb.finish() {
                pen.brush(&path, WHITE, r * 0.1, at);
            }
        }
    }
}

/// The vein-pop mark (💢) of radius `r`: four filled crescents, each bowing toward the centre,
/// edged in black ink with a light rim, leaving a cross-shaped gap between them.
pub(crate) fn anger_mark(pen: &mut Pen<'_>, r: f32, tint: Rgb, at: Transform) {
    for i in 0..4 {
        let rot = at.pre_rotate(90.0 * i as f32);
        let mut pb = PathBuilder::new();
        pb.move_to(-r * 0.84, -r * 0.12);
        pb.quad_to(-r * 0.14, -r * 0.14, -r * 0.12, -r * 0.84);
        pb.line_to(-r * 0.45, -r * 0.96);
        pb.quad_to(-r * 0.56, -r * 0.56, -r * 0.96, -r * 0.45);
        pb.close();
        if let Some(vein) = pb.finish() {
            pen.inked(&vein, tint, r, rot);
        }
        let mut pb = PathBuilder::new();
        pb.move_to(-r * 0.74, -r * 0.24);
        pb.quad_to(-r * 0.30, -r * 0.30, -r * 0.24, -r * 0.74);
        if let Some(rim) = pb.finish() {
            let width = r * 0.07;
            pen.stroke(&rim, lighten(tint, 0.55), width, 0.9, rot);
        }
    }
}

/// Draws the vein-pop mark of radius `r` pixels at `at`, faded by `alpha`: the bubble accents use
/// the same mark as the scattered props.
pub(crate) fn draw_anger_mark(image: &mut Pixmap, r: f32, tint: Rgb, alpha: f32, at: Transform) {
    let mut pen = Pen {
        image,
        alpha,
        u: r * 8.0,
    };
    anger_mark(&mut pen, r, tint, at);
}

/// Freezes the full-screen scene inside a settled bubble: the backdrop, its speed lines and, at the
/// top writing level, the motif's props, all clipped to `clip` (the bubble's panel) and kept off
/// `card` (the reading area). The scene grows with the writing `level` (1 to 3); `dark` dims it.
pub(crate) fn draw_frozen_scene(
    image: &mut Pixmap,
    style: &crate::MangaStyle,
    level: u8,
    clip: &Path,
    card: Bounds,
    dark: bool,
) {
    let Some(backdrop) = &style.backdrop else {
        return;
    };
    let level = level.clamp(1, 3);
    let (w, h) = (image.width(), image.height());
    let (Some(mut scene), Some(mut mask)) = (Pixmap::new(w, h), tiny_skia::Mask::new(w, h)) else {
        return;
    };
    // Level 1 is a tint, level 2 adds the speed lines, level 3 the props as well.
    let mut style = style.clone();
    if level < 2 {
        style.backdrop = Some(crate::Backdrop {
            rays: None,
            ..backdrop.clone()
        });
    }
    let state = MangaState {
        style,
        progress: 0.42,
        alpha: [0.45, 0.75, 1.0][usize::from(level - 1)],
        strength: crate::scene::level_strength(level),
    };
    let frame = (w as f32, h as f32);
    draw_backdrop(&mut scene, &state, frame, None);
    if level == 3 {
        draw_scatter(&mut scene, &state, frame, card, Transform::identity());
    }
    if dark {
        // Dim the scene toward the dark reading surface.
        if let Some(rect) = Rect::from_xywh(0.0, 0.0, frame.0, frame.1) {
            scene.fill_rect(
                rect,
                &paint(Rgb(20, 22, 30), 0.45),
                Transform::identity(),
                None,
            );
        }
    }
    mask.fill_path(clip, FillRule::Winding, true, Transform::identity());
    image.draw_pixmap(
        0,
        0,
        scene.as_ref(),
        &tiny_skia::PixmapPaint::default(),
        Transform::identity(),
        Some(&mask),
    );
}

/// The two tones of a smoke puff: a light top and a shaded underside, a cool grey warmed by `tint`.
fn smoke_tone(tint: Rgb) -> (Rgb, Rgb) {
    let shade = mix(Rgb(150, 144, 162), tint, 0.18);
    (lighten(shade, 0.82), shade)
}

/// A cel-shaded cloud made of overlapping `lobes` (x, y, radius in pixels): one black contour
/// around all of them, a shaded underside and a light top, like an inked smoke puff.
fn puff(pen: &mut Pen<'_>, lobes: &[(f32, f32, f32)], r: f32, tone: (Rgb, Rgb), at: Transform) {
    let (light, shade) = tone;
    let width = 2.0 * pen.ink(r);
    // Outlines first, then every fill over them: only the outer contour stays visible.
    for &(x, y, s) in lobes {
        if let Some(lobe) = circle(x, y, s) {
            pen.stroke(&lobe, OUTLINE, width, 1.0, at);
        }
    }
    for &(x, y, s) in lobes {
        if let Some(lobe) = circle(x, y, s) {
            pen.fill(&lobe, shade, 1.0, at);
        }
    }
    // The lit part sits up and to the left inside each lobe, leaving a shaded crescent below.
    for &(x, y, s) in lobes {
        if let Some(lobe) = circle(x - s * 0.10, y - s * 0.14, s * 0.80) {
            pen.fill(&lobe, light, 1.0, at);
        }
    }
    if let Some(&(x, y, s)) = lobes.iter().max_by(|a, b| a.2.total_cmp(&b.2)) {
        highlight(pen, x - s * 0.38, y - s * 0.40, s * 0.12, at);
    }
}

/// Columns of smoke rising from the bottom of the frame: puffs are born small behind the flames,
/// swell and sway as they climb, then shrink away. Each column is one merged cloud.
fn smoke(
    pen: &mut Pen<'_>,
    accent: Rgb,
    p: f32,
    strength: f32,
    frame: (f32, f32),
    keep: Bounds,
    shake: Transform,
) {
    let (w, h) = frame;
    let u = pen.u;
    let columns = (1.0 + 5.0 * strength).round() as u32;
    let per_column = 9;
    let tone = smoke_tone(accent);
    let margin = 0.03 * u;
    for c in 0..columns {
        let key = 1300 + u64::from(c);
        let base = (c as f32 + 0.5 + 0.5 * (hash(key, 0) - 0.5)) * w / columns as f32;
        let speed = 0.9 + 0.5 * hash(key, 1);
        let lean = (hash(key, 2) - 0.5) * 0.25 * w;
        let mut lobes = Vec::new();
        let mut big = 0.0_f32;
        for j in 0..per_column {
            let life = (hash(key, 3) + p * speed + j as f32 / per_column as f32).fract();
            let grow =
                (life / 0.12).clamp(0.0, 1.0) * (1.0 - ((life - 0.78) / 0.22).clamp(0.0, 1.0));
            let s = u * (0.03 + 0.06 * life) * grow * (0.7 + 0.3 * strength);
            if s < 1.0 {
                continue;
            }
            let phase = TAU * hash(key, 10 + j as u64);
            let x = base + lean * life + 0.05 * u * (life * TAU * 1.5 + phase).sin();
            let y = h * (0.95 - 0.5 * life);
            let reach = 1.6 * s + margin;
            if x + reach > keep.0
                && x - reach < keep.0 + keep.2
                && y + reach > keep.1
                && y - reach < keep.1 + keep.3
            {
                continue;
            }
            // Three lobes per puff, tilted at random.
            for (k, &(dx, dy, ds)) in [(-0.55, 0.15, 0.7), (0.0, -0.2, 1.0), (0.6, 0.1, 0.75)]
                .iter()
                .enumerate()
            {
                let a = phase + k as f32;
                let (ox, oy) = (dx * s, dy * s);
                lobes.push((x + ox * a.cos() * 0.3 + ox * 0.7, y + oy, ds * s));
            }
            big = big.max(s);
        }
        if !lobes.is_empty() {
            puff(pen, &lobes, big, tone, shake);
        }
    }
}

/// `a` mixed toward `b` by `amount` (0 to 1).
fn mix(a: Rgb, b: Rgb, amount: f32) -> Rgb {
    let t = amount.clamp(0.0, 1.0);
    let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Rgb(m(a.0, b.0), m(a.1, b.1), m(a.2, b.2))
}

/// A row of flames along the bottom of the frame, licking up and down.
fn flames(
    pen: &mut Pen<'_>,
    palette: &[Rgb],
    p: f32,
    strength: f32,
    frame: (f32, f32),
    shake: Transform,
) {
    let (w, h) = frame;
    let n = 7;
    let step = w / (n - 1) as f32;
    let outer = palette[0];
    let inner = palette.get(1).copied().unwrap_or(lighten(outer, 0.5));
    for i in 0..n {
        let key = 500 + i as u64;
        let lick = 0.85 + 0.15 * (p * TAU * 6.0 + TAU * hash(key, 0)).sin();
        let height = h * (0.10 + 0.08 * hash(key, 1)) * (0.6 + 0.4 * strength) * lick;
        let x = i as f32 * step + step * 0.3 * (hash(key, 2) - 0.5);
        let at = Transform::from_translate(x, h).post_concat(shake);
        let half = step * 0.62;
        if let Some(flame) = flame_path(half, height) {
            pen.inked(&flame, outer, half, at);
        }
        if let Some(core) = flame_path(half * 0.5, height * 0.55) {
            pen.fill(&core, inner, 1.0, at);
        }
    }
}

/// A flame standing on (0, 0), `half` wide each side and `height` tall, its tip bent to one side.
fn flame_path(half: f32, height: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(-half, 0.0);
    pb.cubic_to(
        -half * 1.1,
        -height * 0.45,
        -half * 0.2,
        -height * 0.55,
        half * 0.15,
        -height,
    );
    pb.cubic_to(
        half * 0.2,
        -height * 0.6,
        half * 1.15,
        -height * 0.45,
        half,
        0.0,
    );
    pb.close();
    pb.finish()
}

/// Rain falling over the whole frame, around the bubble.
fn rain(
    pen: &mut Pen<'_>,
    palette: &[Rgb],
    p: f32,
    strength: f32,
    frame: (f32, f32),
    keep: Bounds,
    shake: Transform,
) {
    let (w, h) = frame;
    let n = (40.0 + 50.0 * strength) as u32;
    let length = pen.u * 0.07;
    let mut pb = PathBuilder::new();
    for i in 0..n {
        let key = 700 + i as u64;
        let x = hash(key, 0) * w;
        let y = ((hash(key, 1) + p * (1.6 + hash(key, 2))) % 1.0) * (h + length) - length;
        if inside(keep, x, y) || inside(keep, x - length * 0.2, y + length) {
            continue;
        }
        pb.move_to(x, y);
        pb.line_to(x - length * 0.2, y + length);
    }
    if let Some(path) = pb.finish() {
        let width = pen.u * 0.006;
        pen.stroke(&path, palette[0], width, 0.75, shake);
    }
}

/// Zigzag bands across the top and bottom of the frame, and vertical gloom lines dropping from the
/// top: the comic panel of embarrassment.
fn blush_bands(
    pen: &mut Pen<'_>,
    palette: &[Rgb],
    p: f32,
    strength: f32,
    frame: (f32, f32),
    keep: Bounds,
    shake: Transform,
) {
    let (w, h) = frame;
    let band = palette.get(1).copied().unwrap_or(Rgb(255, 214, 64));
    let depth = pen.u * 0.07;
    let teeth = 14;
    for (top, base) in [(true, 0.0), (false, h)] {
        let mut pb = PathBuilder::new();
        pb.move_to(-depth, base);
        for i in 0..=teeth * 2 {
            let x = i as f32 / (teeth * 2) as f32 * (w + 2.0 * depth) - depth;
            let reach = if i % 2 == 0 { depth } else { depth * 0.35 };
            pb.line_to(x, if top { base + reach } else { base - reach });
        }
        pb.line_to(w + depth, base);
        pb.close();
        if let Some(path) = pb.finish() {
            pen.inked(&path, band, depth, shake);
        }
    }
    // Gloom lines grow down from the top band.
    let grow = (p / 0.3).clamp(0.0, 1.0);
    let n = (14.0 + 10.0 * strength) as u32;
    let mut pb = PathBuilder::new();
    for i in 0..n {
        let key = 900 + i as u64;
        let x = (i as f32 + 0.5 + 0.4 * (hash(key, 0) - 0.5)) * w / n as f32;
        let end = depth + h * (0.12 + 0.2 * hash(key, 1)) * grow;
        if x >= keep.0 && x <= keep.0 + keep.2 && end > keep.1 {
            continue;
        }
        pb.move_to(x, depth);
        pb.line_to(x, end);
    }
    if let Some(path) = pb.finish() {
        let width = pen.u * 0.005;
        pen.stroke(&path, OUTLINE, width, 0.55, shake);
    }
}

/// A glowing neon tube around the bubble, flickering faintly, with lightning arcs crackling off it.
fn neon_tube(
    pen: &mut Pen<'_>,
    accent: Rgb,
    palette: &[Rgb],
    p: f32,
    strength: f32,
    keep: Bounds,
    shake: Transform,
) {
    let u = pen.u;
    // A subtle flicker only (0.85 to 1), never a full blink: photosensitivity (gpu-effects skill).
    let tick = (p * 24.0) as u64;
    let flicker = 0.85 + 0.15 * hash(tick, 1);
    let second = palette
        .iter()
        .copied()
        .find(|&c| c != accent)
        .unwrap_or(WHITE);
    for (pad, tint) in [(0.025 * u, accent), (0.05 * u, second)] {
        let b = (
            keep.0 - pad,
            keep.1 - pad,
            keep.2 + 2.0 * pad,
            keep.3 + 2.0 * pad,
        );
        let mut pb = PathBuilder::new();
        push_round_rect(&mut pb, b, 0.2 * b.2.min(b.3));
        let Some(tube) = pb.finish() else {
            continue;
        };
        let a = flicker * (0.6 + 0.4 * strength);
        pen.stroke(&tube, tint, 0.045 * u, 0.18 * a, shake);
        pen.stroke(&tube, tint, 0.02 * u, 0.6 * a, shake);
        pen.stroke(&tube, WHITE, 0.006 * u, 0.9 * a, shake);
    }
    // Arcs jump to new places a few times a second.
    let arcs = (2.0 + 4.0 * strength) as u64;
    let jump = (p * 8.0) as u64;
    for i in 0..arcs {
        let key = 1100 + i * 31 + jump * 7;
        let a = TAU * hash(key, 0);
        let (cx, cy) = (keep.0 + keep.2 / 2.0, keep.1 + keep.3 / 2.0);
        // Start on the tube: the bubble's edge along the angle `a`.
        let (dx, dy) = (a.cos(), a.sin());
        let t = (keep.2 / 2.0 / dx.abs().max(1e-3)).min(keep.3 / 2.0 / dy.abs().max(1e-3));
        let (sx, sy) = (cx + dx * (t + 0.05 * u), cy + dy * (t + 0.05 * u));
        let length = u * (0.08 + 0.1 * hash(key, 1));
        let mut pb = PathBuilder::new();
        pb.move_to(sx, sy);
        let steps = 5;
        for s in 1..=steps {
            let along = length * s as f32 / steps as f32;
            let side = u * 0.025 * (hash(key, 10 + s as u64) - 0.5) * 2.0;
            pb.line_to(sx + dx * along - dy * side, sy + dy * along + dx * side);
        }
        if let Some(path) = pb.finish() {
            let tint = palette[(i as usize) % palette.len()];
            pen.stroke(&path, tint, 0.018 * u, 0.3 * flicker, shake);
            pen.stroke(&path, tint, 0.008 * u, 0.9 * flicker, shake);
            pen.stroke(&path, WHITE, 0.003 * u, flicker, shake);
        }
    }
}

fn glow(pen: &mut Pen<'_>, radius: f32, rgb: Rgb, alpha: f32, at: Transform) {
    let stops = vec![
        GradientStop::new(0.0, color(rgb, alpha * pen.alpha)),
        GradientStop::new(1.0, color(rgb, 0.0)),
    ];
    let centre = Point::from_xy(0.0, 0.0);
    if let (Some(shader), Some(disc)) = (
        RadialGradient::new(
            centre,
            0.0,
            centre,
            radius,
            stops,
            SpreadMode::Pad,
            Transform::identity(),
        ),
        circle(0.0, 0.0, radius),
    ) {
        let paint = Paint {
            shader,
            anti_alias: true,
            ..Paint::default()
        };
        pen.image
            .fill_path(&disc, &paint, FillRule::Winding, at, None);
    }
}

fn highlight(pen: &mut Pen<'_>, x: f32, y: f32, r: f32, at: Transform) {
    if let Some(dot) = circle(x, y, r) {
        pen.fill(&dot, WHITE, 0.9, at);
    }
}

fn circle(x: f32, y: f32, r: f32) -> Option<Path> {
    PathBuilder::from_circle(x, y, r.max(0.1))
}

fn polygon(points: &[(f32, f32)], r: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    for (i, &(x, y)) in points.iter().enumerate() {
        if i == 0 {
            pb.move_to(x * r, y * r);
        } else {
            pb.line_to(x * r, y * r);
        }
    }
    pb.close();
    pb.finish()
}

fn star_path(points: u32, outer: f32, inner: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    for i in 0..points * 2 {
        let a = i as f32 / (points * 2) as f32 * TAU - PI / 2.0;
        let d = if i % 2 == 0 { outer } else { inner };
        if i == 0 {
            pb.move_to(d * a.cos(), d * a.sin());
        } else {
            pb.line_to(d * a.cos(), d * a.sin());
        }
    }
    pb.close();
    pb.finish()
}

fn drop_path(r: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, -r);
    pb.cubic_to(-r * 0.3, -r * 0.4, -r * 0.7, 0.1 * r, -r * 0.62, r * 0.45);
    pb.cubic_to(-r * 0.5, r * 0.95, r * 0.5, r * 0.95, r * 0.62, r * 0.45);
    pb.cubic_to(r * 0.7, 0.1 * r, r * 0.3, -r * 0.4, 0.0, -r);
    pb.close();
    pb.finish()
}

fn heart_path(r: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, r * 0.85);
    pb.cubic_to(-r * 1.1, r * 0.1, -r * 0.95, -r * 0.9, 0.0, -r * 0.4);
    pb.cubic_to(r * 0.95, -r * 0.9, r * 1.1, r * 0.1, 0.0, r * 0.85);
    pb.close();
    pb.finish()
}

/// Adds a rounded rect to `pb`, corners of radius `r`.
pub(crate) fn push_round_rect(pb: &mut PathBuilder, b: Bounds, r: f32) {
    let (x, y, w, h) = b;
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    // Cubic approximation of a quarter circle.
    let k = r * 0.552_284_8;
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.cubic_to(x + w - r + k, y, x + w, y + r - k, x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.cubic_to(x + w, y + h - r + k, x + w - r + k, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.cubic_to(x + r - k, y + h, x, y + h - r + k, x, y + h - r);
    pb.line_to(x, y + r);
    pb.cubic_to(x, y + r - k, x + r - k, y, x + r, y);
    pb.close();
}

fn inside(b: Bounds, x: f32, y: f32) -> bool {
    x >= b.0 && x <= b.0 + b.2 && y >= b.1 && y <= b.1 + b.3
}

/// `rgb` mixed toward white by `amount` (0 to 1).
pub(crate) fn lighten(rgb: Rgb, amount: f32) -> Rgb {
    let mix = |c: u8| (c as f32 + (255.0 - c as f32) * amount.clamp(0.0, 1.0)).round() as u8;
    Rgb(mix(rgb.0), mix(rgb.1), mix(rgb.2))
}

/// A deterministic number in [0, 1) for prop `key` and attribute `n` (SplitMix64 finaliser).
fn hash(key: u64, n: u64) -> f32 {
    let mut z = key
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(n.wrapping_mul(0xBF58_476D_1CE4_E5B9))
        .wrapping_add(0x94D0_49BB_1331_11EB);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    ((z >> 40) as f32) / (1u64 << 24) as f32
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Backdrop, MangaStyle};

    fn state(motif: MangaMotif) -> MangaState {
        MangaState {
            style: MangaStyle {
                motif,
                accent_color: "#00E5FF".into(),
                depth: 0.5,
                palette: vec!["#00E5FF".into(), "#FF3DCB".into()],
                backdrop: Some(Backdrop {
                    inner: "#1B4DFF".into(),
                    outer: "#05103A".into(),
                    opacity: 0.9,
                    rays: None,
                }),
            },
            progress: 0.5,
            alpha: 1.0,
            strength: 1.0,
        }
    }

    const KEEP: Bounds = (60.0, 160.0, 120.0, 60.0);

    fn alpha_at(image: &Pixmap, x: u32, y: u32) -> u8 {
        image.pixel(x, y).expect("in frame").alpha()
    }

    #[test]
    fn every_motif_keeps_the_bubble_clear() {
        for motif in [
            MangaMotif::Radiance,
            MangaMotif::Pressure,
            MangaMotif::Orbit,
            MangaMotif::Tension,
            MangaMotif::Rainfall,
            MangaMotif::Blossom,
            MangaMotif::Breeze,
            MangaMotif::Blush,
            MangaMotif::Neon,
        ] {
            let mut image = Pixmap::new(240, 400).expect("pixmap");
            let s = state(motif);
            draw_backdrop(&mut image, &s, (240.0, 400.0), Some(KEEP));
            draw_scatter(&mut image, &s, (240.0, 400.0), KEEP, Transform::identity());
            // The bubble's middle stays transparent; the frame around it is coloured.
            for (x, y) in [(120, 190), (80, 175), (160, 205)] {
                assert_eq!(alpha_at(&image, x, y), 0, "{motif:?} at {x},{y}");
            }
            assert!(alpha_at(&image, 10, 10) > 0, "{motif:?}: no backdrop");
        }
    }

    #[test]
    fn props_appear_with_strength() {
        let count = |strength: f32| {
            let mut image = Pixmap::new(240, 400).expect("pixmap");
            let s = MangaState {
                strength,
                style: MangaStyle {
                    backdrop: None,
                    ..state(MangaMotif::Radiance).style
                },
                ..state(MangaMotif::Radiance)
            };
            draw_scatter(&mut image, &s, (240.0, 400.0), KEEP, Transform::identity());
            image.pixels().iter().filter(|p| p.alpha() > 0).count()
        };
        assert!(count(1.0) > count(0.3));
        assert!(count(0.3) > 0, "level 1 keeps a few props");
    }

    #[test]
    fn no_backdrop_draws_nothing() {
        let mut image = Pixmap::new(40, 40).expect("pixmap");
        let s = MangaState {
            style: MangaStyle {
                backdrop: None,
                ..state(MangaMotif::Neon).style
            },
            ..state(MangaMotif::Neon)
        };
        draw_backdrop(&mut image, &s, (40.0, 40.0), None);
        assert!(image.pixels().iter().all(|p| p.alpha() == 0));
    }

    #[test]
    fn lighten_mixes_toward_white() {
        assert_eq!(lighten(Rgb(0, 100, 255), 0.0), Rgb(0, 100, 255));
        assert_eq!(lighten(Rgb(0, 100, 255), 1.0), Rgb(255, 255, 255));
    }
}
