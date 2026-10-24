//! Original ink primitives shared by live illustrations and persistent bubble decorations.
//! No theme or emotion identifiers belong here: the theme selects a drawing motif.

use crate::{
    scene::{MangaState, Rgb},
    MangaMotif, MangaStyle,
};
use std::f32::consts::{PI, TAU};
use tiny_skia::{FillRule, LineCap, LineJoin, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};

const INK: Rgb = Rgb(27, 32, 49);

/// Draws in a local unit square; all widths scale with the shorter side.
struct InkCanvas<'a> {
    image: &'a mut Pixmap,
    bounds: (f32, f32, f32, f32),
    transform: Transform,
    alpha: f32,
    compact: bool,
}

impl InkCanvas<'_> {
    fn point(&self, x: f32, y: f32) -> (f32, f32) {
        // Static marks follow bounded edge bands, even when native text makes a bubble very tall.
        let local_y = if self.compact {
            let edge = self.bounds.3.min(self.bounds.2 * 0.38);
            if y < 0.5 {
                y * edge
            } else {
                self.bounds.3 - (1.0 - y) * edge
            }
        } else {
            y * self.bounds.3
        };
        (self.bounds.0 + x * self.bounds.2, self.bounds.1 + local_y)
    }
    fn paint(&self, color: Rgb, alpha: f32) -> Paint<'static> {
        let mut p = Paint::default();
        p.set_color_rgba8(
            color.0,
            color.1,
            color.2,
            (255.0 * alpha * self.alpha).clamp(0.0, 255.0) as u8,
        );
        p.anti_alias = true;
        p
    }
    fn fill(&mut self, path: &Path, color: Rgb, alpha: f32) {
        self.image.fill_path(
            path,
            &self.paint(color, alpha),
            FillRule::Winding,
            self.transform,
            None,
        );
    }
    fn stroke(&mut self, path: &Path, color: Rgb, width: f32, alpha: f32) {
        let stroke = Stroke {
            width: (width * self.bounds.2.min(self.bounds.3)).max(0.65),
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            ..Stroke::default()
        };
        self.image.stroke_path(
            path,
            &self.paint(color, alpha),
            &stroke,
            self.transform,
            None,
        );
    }
    fn line(&mut self, points: &[(f32, f32)], color: Rgb, width: f32, alpha: f32) {
        let mut b = PathBuilder::new();
        for (i, &(x, y)) in points.iter().enumerate() {
            let (x, y) = self.point(x, y);
            if i == 0 {
                b.move_to(x, y);
            } else {
                b.line_to(x, y);
            }
        }
        if let Some(p) = b.finish() {
            self.stroke(&p, color, width, alpha);
        }
    }
    fn polygon(&mut self, points: &[(f32, f32)], color: Rgb, alpha: f32) {
        let mut b = PathBuilder::new();
        for (i, &(x, y)) in points.iter().enumerate() {
            let (x, y) = self.point(x, y);
            if i == 0 {
                b.move_to(x, y);
            } else {
                b.line_to(x, y);
            }
        }
        b.close();
        if let Some(p) = b.finish() {
            self.fill(&p, color, alpha);
        }
    }
    fn curve(&mut self, points: [(f32, f32); 4], color: Rgb, width: f32, alpha: f32) {
        let p = points.map(|(x, y)| self.point(x, y));
        let mut b = PathBuilder::new();
        b.move_to(p[0].0, p[0].1);
        b.cubic_to(p[1].0, p[1].1, p[2].0, p[2].1, p[3].0, p[3].1);
        if let Some(p) = b.finish() {
            self.stroke(&p, color, width, alpha);
        }
    }
    fn ellipse(&mut self, x: f32, y: f32, rx: f32, ry: f32, color: Rgb, alpha: f32) {
        let (x, y) = self.point(x, y);
        let rect = tiny_skia::Rect::from_xywh(
            x - rx * self.bounds.2,
            y - ry * self.bounds.3,
            2.0 * rx * self.bounds.2,
            2.0 * ry * self.bounds.3,
        );
        if let Some(p) = rect.and_then(PathBuilder::from_oval) {
            self.fill(&p, color, alpha);
        }
    }
    fn sparkle(&mut self, x: f32, y: f32, r: f32, color: Rgb) {
        self.polygon(
            &[
                (x, y - r),
                (x + r * 0.23, y - r * 0.23),
                (x + r * 0.65, y),
                (x + r * 0.23, y + r * 0.23),
                (x, y + r),
                (x - r * 0.23, y + r * 0.23),
                (x - r * 0.65, y),
                (x - r * 0.23, y - r * 0.23),
            ],
            color,
            0.85,
        );
        self.line(&[(x - r * 0.85, y), (x + r * 0.85, y)], color, 0.002, 0.4);
    }
}

fn pale(c: Rgb) -> Rgb {
    Rgb(244 + c.0 / 24, 244 + c.1 / 24, 244 + c.2 / 24)
}

/// Light panel fill: a tint of the backdrop when the theme has one, so the bubble reads as part
/// of the coloured panel around it.
fn panel_fill(style: &MangaStyle, accent: Rgb) -> Rgb {
    match &style.backdrop {
        Some(b) => crate::scatter::lighten(Rgb::from_hex(&b.inner), 0.90),
        None => pale(accent),
    }
}

/// Slightly irregular panels, with generous clear centres for text.
fn panel(c: &InkCanvas<'_>, motif: MangaMotif) -> Option<Path> {
    let mut b = PathBuilder::new();
    let points: &[(f32, f32)] = if motif == MangaMotif::Pressure {
        &[
            (0.04, 0.19),
            (0.15, 0.08),
            (0.30, 0.11),
            (0.43, 0.05),
            (0.60, 0.10),
            (0.80, 0.06),
            (0.96, 0.16),
            (0.93, 0.43),
            (0.97, 0.70),
            (0.86, 0.91),
            (0.65, 0.87),
            (0.49, 0.94),
            (0.28, 0.89),
            (0.12, 0.94),
            (0.035, 0.77),
            (0.065, 0.47),
        ]
    } else {
        &[]
    };
    if !points.is_empty() {
        for (i, &(x, y)) in points.iter().enumerate() {
            let (x, y) = c.point(x, y);
            if i == 0 {
                b.move_to(x, y);
            } else {
                b.line_to(x, y);
            }
        }
    } else {
        let p = |x, y| c.point(x, y);
        let (x, y) = p(0.12, 0.09);
        b.move_to(x, y);
        let a = p(0.30, 0.055);
        let d = p(0.74, 0.10);
        let e = p(0.86, 0.085);
        b.cubic_to(a.0, a.1, d.0, d.1, e.0, e.1);
        let a = p(0.99, 0.08);
        let d = p(0.975, 0.34);
        let e = p(0.955, 0.66);
        b.cubic_to(a.0, a.1, d.0, d.1, e.0, e.1);
        let a = p(0.98, 0.92);
        let d = p(0.89, 0.92);
        let e = p(0.76, 0.91);
        b.cubic_to(a.0, a.1, d.0, d.1, e.0, e.1);
        let a = p(0.53, 0.95);
        let d = p(0.31, 0.88);
        let e = p(0.13, 0.92);
        b.cubic_to(a.0, a.1, d.0, d.1, e.0, e.1);
        let a = p(0.015, 0.91);
        let d = p(0.04, 0.62);
        let e = p(0.045, 0.36);
        b.cubic_to(a.0, a.1, d.0, d.1, e.0, e.1);
        let a = p(0.035, 0.16);
        let d = p(0.06, 0.10);
        let e = p(0.12, 0.09);
        b.cubic_to(a.0, a.1, d.0, d.1, e.0, e.1);
    }
    b.close();
    b.finish()
}

fn drop(c: &mut InkCanvas<'_>, x: f32, y: f32, r: f32, color: Rgb) {
    let p = |x, y| c.point(x, y);
    let mut b = PathBuilder::new();
    let a = p(x, y - r);
    b.move_to(a.0, a.1);
    let a = p(x - r * 0.8, y + r * 0.35);
    let d = p(x - r * 0.4, y + r);
    let e = p(x, y + r);
    b.cubic_to(a.0, a.1, d.0, d.1, e.0, e.1);
    let a = p(x + r * 0.65, y + r);
    let d = p(x + r * 0.7, y + r * 0.3);
    let e = p(x, y - r);
    b.cubic_to(a.0, a.1, d.0, d.1, e.0, e.1);
    b.close();
    if let Some(p) = b.finish() {
        c.fill(&p, pale(color), 0.92);
        c.stroke(&p, color, 0.006, 0.85);
        c.line(
            &[(x - r * 0.18, y + r * 0.2), (x - r * 0.2, y + r * 0.57)],
            color,
            0.003,
            0.4,
        );
    }
}

fn flower(c: &mut InkCanvas<'_>, x: f32, y: f32, r: f32, color: Rgb) {
    for i in 0..5 {
        let a = i as f32 * TAU / 5.0;
        c.ellipse(
            x + r * 0.65 * a.cos(),
            y + r * 0.65 * a.sin(),
            r * 0.5,
            r * 0.45,
            color,
            0.15,
        );
        c.curve(
            [
                (x, y),
                (x + r * 1.5 * (a - 0.4).cos(), y + r * 1.5 * (a - 0.4).sin()),
                (x + r * 1.5 * (a + 0.4).cos(), y + r * 1.5 * (a + 0.4).sin()),
                (x, y),
            ],
            color,
            0.003,
            0.55,
        );
    }
    c.ellipse(x, y, r * 0.13, r * 0.13, color, 0.8);
}

fn anger(c: &mut InkCanvas<'_>, x: f32, y: f32, r: f32, color: Rgb) {
    let (px, py) = c.point(x, y);
    let radius = r * c.bounds.2.min(c.bounds.3) * if c.compact { 1.7 } else { 1.0 };
    let at = Transform::from_translate(px, py).post_concat(c.transform);
    crate::scatter::draw_anger_mark(c.image, radius, color, c.alpha, at);
}

fn orbit(c: &mut InkCanvas<'_>, color: Rgb, progress: f32, front: bool, strength: f32) {
    for ribbon in 0..3 {
        let mut b = PathBuilder::new();
        for i in 0..75 {
            let angle = (if front { 0.0 } else { PI }) + i as f32 / 74.0 * PI;
            let rx = 0.56 + ribbon as f32 * 0.025;
            let x = 0.5 + rx * angle.cos();
            let y = 0.5 + 0.53 * angle.sin() - 0.13 * angle.cos() + 0.025 * (progress * TAU).sin();
            let (x, y) = c.point(x, y);
            if i == 0 {
                b.move_to(x, y);
            } else {
                b.line_to(x, y);
            }
        }
        if let Some(p) = b.finish() {
            c.stroke(
                &p,
                color,
                (0.018 - ribbon as f32 * 0.005) * strength,
                0.8 - ribbon as f32 * 0.2,
            );
            if ribbon == 0 {
                c.stroke(&p, Rgb(255, 255, 255), 0.004, 0.9);
            }
        }
    }
}

/// Draws the effects behind/around the message in the animated overlay. `panel_drawn` adds the
/// reading panel under the message; the overlay layers leave it out, as the shell's bubble plays it.
pub(crate) fn draw_scene(
    image: &mut Pixmap,
    state: &MangaState,
    ink: Rgb,
    bounds: (f32, f32, f32, f32),
    transform: Transform,
    front: bool,
    panel_drawn: bool,
) {
    if state.alpha <= 0.0 {
        return;
    }
    let mut c = InkCanvas {
        image,
        bounds,
        transform,
        alpha: state.alpha,
        compact: false,
    };
    let style = &state.style;
    let accent = Rgb::from_hex(&style.accent_color);
    let p = state.progress;
    if front {
        if style.motif == MangaMotif::Orbit {
            orbit(&mut c, accent, p, true, state.strength);
        }
        return;
    }
    let line_ink = if style.motif == MangaMotif::Pressure || style.motif == MangaMotif::Tension {
        INK
    } else {
        ink
    };
    // Localised ink rays/hatching: empty reading centre and transparent outer corners.
    match style.motif {
        MangaMotif::Radiance | MangaMotif::Pressure | MangaMotif::Tension | MangaMotif::Power => {
            let count = if style.motif == MangaMotif::Radiance {
                32
            } else {
                54
            };
            for i in 0..count {
                let a = i as f32 / count as f32 * TAU;
                let jitter = ((i * 17 % 13) as f32) / 13.0;
                let r = 0.62 + 0.10 * jitter;
                let end = 0.85 + 0.22 * jitter + 0.025 * (p * 5.0 + i as f32).sin();
                c.line(
                    &[
                        (0.5 + r * a.cos(), 0.5 + r * a.sin()),
                        (0.5 + end * a.cos(), 0.5 + end * a.sin()),
                    ],
                    line_ink,
                    if i % 4 == 0 { 0.007 } else { 0.002 },
                    (0.18 + 0.33 * jitter) * state.strength,
                );
            }
        }
        MangaMotif::Rainfall | MangaMotif::Blush => {
            for i in 0..32 {
                let x = -0.06 + i as f32 / 28.0;
                let length = 0.10 + (i * 13 % 11) as f32 / 45.0;
                c.line(
                    &[(x, -0.38), (x - 0.01, -0.38 + length + 0.07 * p)],
                    ink,
                    0.002,
                    0.25 * state.strength,
                );
            }
        }
        _ => {}
    }
    if style.motif == MangaMotif::Orbit {
        orbit(&mut c, accent, p, false, state.strength);
    }
    if style.motif == MangaMotif::Tension && style.backdrop.is_none() {
        // Sparse offset silhouettes form a local ink aura; a full-screen backdrop already carries
        // the colour, and the aura would cover the bubble.
        for i in (1..=6).rev() {
            let spread = i as f32 * 0.023;
            c.ellipse(0.5, 0.5, 0.50 + spread, 0.45 + spread, accent, 0.035);
        }
    }
    if let Some(body) = panel(&c, style.motif).filter(|_| panel_drawn) {
        let shadow =
            Transform::from_translate(bounds.2 * 0.015, bounds.3 * 0.025).post_concat(transform);
        c.image
            .fill_path(&body, &c.paint(ink, 0.09), FillRule::Winding, shadow, None);
        c.fill(&body, panel_fill(style, accent), 0.97);
        c.stroke(
            &body,
            if matches!(style.motif, MangaMotif::Pressure | MangaMotif::Orbit) {
                INK
            } else {
                ink
            },
            if style.motif == MangaMotif::Pressure {
                0.017
            } else {
                0.008
            },
            0.85,
        );
        if style.depth > 0.25 {
            for i in 0..16 {
                let x = 0.19 + i as f32 * 0.037;
                c.line(
                    &[(x, 0.92), (x + 0.018, 0.97)],
                    ink,
                    0.003,
                    0.35 * style.depth,
                );
            }
        }
    }
    accents(&mut c, style, ink, p, false, state.strength);
}

/// Draws only the permanent decoration, never the message text or an animation frame.
pub(crate) fn draw_bubble(image: &mut Pixmap, style: &MangaStyle, ink: Rgb, dark: bool, level: u8) {
    let bounds = (0.0, 0.0, image.width() as f32, image.height() as f32);
    let mut c = InkCanvas {
        image,
        bounds,
        transform: Transform::identity(),
        alpha: 1.0,
        compact: true,
    };
    let accent = Rgb::from_hex(&style.accent_color);
    let border = if dark { pale(accent) } else { ink };
    if let Some(body) = panel(&c, style.motif) {
        let surface = if dark {
            Rgb(35, 38, 48)
        } else {
            panel_fill(style, accent)
        };
        c.fill(&body, surface, 1.0);
        if style.backdrop.is_some() {
            // The animation's scene stays in the bubble, frozen, around a clear reading card.
            let (x0, y0) = c.point(0.11, 0.20);
            let (x1, y1) = c.point(0.89, 0.80);
            let card = (x0, y0, x1 - x0, y1 - y0);
            crate::scatter::draw_frozen_scene(c.image, style, level, &body, card, dark);
            let radius = 0.22 * card.2.min(card.3);
            let unit = bounds.2.min(bounds.3);
            for (grow, alpha) in [(0.05, 0.35), (0.025, 0.7)] {
                let pad = grow * unit;
                let mut pb = PathBuilder::new();
                crate::scatter::push_round_rect(
                    &mut pb,
                    (
                        card.0 - pad,
                        card.1 - pad,
                        card.2 + 2.0 * pad,
                        card.3 + 2.0 * pad,
                    ),
                    radius + pad,
                );
                if let Some(halo) = pb.finish() {
                    c.fill(&halo, surface, alpha);
                }
            }
            let mut pb = PathBuilder::new();
            crate::scatter::push_round_rect(&mut pb, card, radius);
            if let Some(face) = pb.finish() {
                c.fill(&face, surface, 1.0);
                c.stroke(&face, border, 0.005, 0.35);
            }
        }
        c.stroke(&body, border, 0.009, 0.65);
        // A second incomplete contour gives a hand-inked rather than widget-like edge.
        c.curve(
            [(0.15, 0.14), (0.34, 0.10), (0.65, 0.15), (0.79, 0.13)],
            border,
            0.003,
            0.25,
        );
    }
    accents(
        &mut c,
        style,
        if dark { pale(accent) } else { ink },
        0.42,
        true,
        level as f32 / 3.0,
    );
}

fn accents(
    c: &mut InkCanvas<'_>,
    style: &MangaStyle,
    ink: Rgb,
    p: f32,
    compact: bool,
    strength: f32,
) {
    let accent = Rgb::from_hex(&style.accent_color);
    let drift = if compact {
        0.0
    } else {
        0.025 * (p * TAU).sin()
    };
    let outer = if compact { 0.07 } else { -0.06 };
    let right = if compact { 0.95 } else { 1.02 };
    let top = if compact { 0.20 } else { -0.04 };
    match style.motif {
        MangaMotif::Radiance => {
            c.sparkle(right, top, if compact { 0.055 } else { 0.10 }, accent);
            c.sparkle(
                outer,
                0.77 + drift,
                if compact { 0.035 } else { 0.07 },
                accent,
            );
            if !compact {
                c.sparkle(0.25, -0.18 - drift, 0.045, accent);
            }
        }
        MangaMotif::Pressure => {
            anger(c, right, top, if compact { 0.04 } else { 0.13 }, accent);
            for i in 0..4 {
                let d = i as f32 * 0.035;
                c.line(&[(0.08 + d, 0.84), (0.06 + d, 0.93)], ink, 0.004, 0.55);
            }
        }
        MangaMotif::Orbit => {
            if compact {
                c.curve(
                    [(0.10, 0.78), (0.34, 1.01), (0.72, 1.0), (0.90, 0.78)],
                    accent,
                    0.014,
                    0.7,
                );
                c.curve(
                    [(0.12, 0.81), (0.37, 1.04), (0.72, 1.02), (0.87, 0.86)],
                    ink,
                    0.003,
                    0.5,
                );
            }
            for i in 0..if compact { 2 } else { 5 } {
                let x = if i % 2 == 0 { outer } else { right };
                let y = 0.19 + i as f32 * 0.16 + drift;
                let r = if compact {
                    0.04
                } else {
                    0.07 + 0.02 * i as f32
                };
                c.polygon(
                    &[(x, y - r), (x + r * 0.75, y), (x, y + r), (x - r * 0.3, y)],
                    accent,
                    0.9,
                );
                c.line(
                    &[(x, y - r), (x, y + r), (x + r * 0.75, y)],
                    ink,
                    0.003,
                    0.75,
                );
            }
        }
        MangaMotif::Neon | MangaMotif::Power => {
            // Small electric bolts at opposite corners, accent fill with an ink edge.
            let r = if compact { 0.05 } else { 0.10 };
            for (x, y) in [(right, top), (outer, 0.80 + drift)] {
                let bolt = [
                    (x - r * 0.2, y - r),
                    (x + r * 0.35, y - r * 0.15),
                    (x - r * 0.05, y - r * 0.05),
                    (x + r * 0.3, y + r),
                ];
                c.line(&bolt, accent, 0.012, 0.9 * strength.max(0.4));
                c.line(&bolt, ink, 0.003, 0.8);
            }
        }
        MangaMotif::Tension => {
            for i in 0..if compact { 6 } else { 15 } {
                let x = 0.16 + i as f32 * if compact { 0.025 } else { 0.045 };
                c.line(
                    &[
                        (x, if compact { 0.10 } else { -0.12 }),
                        (x - 0.015, top + 0.09 + (i % 3) as f32 * 0.015),
                    ],
                    ink,
                    0.003,
                    0.4 * strength,
                );
            }
            for i in 0..3 {
                c.ellipse(right, 0.28 + i as f32 * 0.09, 0.006, 0.009, ink, 0.7);
            }
        }
        MangaMotif::Rainfall => {
            for i in 0..if compact { 3 } else { 7 } {
                let x = if compact {
                    0.048 + i as f32 * 0.013
                } else {
                    -0.13 + i as f32 * 0.21
                };
                let y = if compact {
                    0.25
                } else {
                    -0.16 + (i % 2) as f32 * 0.12 + drift
                };
                c.line(&[(x, y - 0.08), (x, y)], ink, 0.002, 0.35);
                drop(c, x, y + 0.03, if compact { 0.018 } else { 0.04 }, accent);
            }
        }
        MangaMotif::Blossom => {
            flower(
                c,
                right,
                if compact { 0.75 } else { 0.90 + drift },
                if compact { 0.025 } else { 0.15 },
                accent,
            );
            if !compact {
                flower(c, -0.02, -0.05 - drift, 0.12, accent);
            }
            blush(c, outer, 0.28, accent, compact);
            if !compact {
                c.sparkle(0.85, -0.18, 0.055, accent);
            }
        }
        MangaMotif::Breeze => {
            for i in 0..2 {
                let y = 0.85 + i as f32 * 0.04;
                c.curve(
                    [
                        (0.16, y),
                        (0.31, y + 0.09),
                        (0.70, y + 0.11),
                        (0.86, y - 0.04),
                    ],
                    accent,
                    0.003,
                    0.5,
                );
            }
            let x = right;
            let y = top + drift;
            c.curve(
                [
                    (x - 0.05, y + 0.05),
                    (x - 0.06, y - 0.07),
                    (x + 0.06, y - 0.07),
                    (x + 0.05, y - 0.04),
                ],
                accent,
                0.007,
                0.7,
            );
            c.curve(
                [
                    (x - 0.05, y + 0.05),
                    (x + 0.05, y + 0.06),
                    (x + 0.07, y - 0.02),
                    (x + 0.05, y - 0.04),
                ],
                accent,
                0.003,
                0.7,
            );
            c.line(
                &[(x - 0.05, y + 0.05), (x + 0.04, y - 0.03)],
                ink,
                0.002,
                0.4,
            );
        }
        MangaMotif::Blush => {
            blush(c, outer, if compact { 0.18 } else { 0.35 }, accent, compact);
            if !compact {
                blush(c, right - 0.10, 0.72, accent, false);
            }
            drop(
                c,
                right,
                if compact { 0.24 } else { 0.13 + drift },
                if compact { 0.04 } else { 0.13 },
                Rgb(74, 102, 157),
            );
            for i in 0..3 {
                c.ellipse(
                    0.75 + i as f32 * 0.04,
                    if compact { 0.84 } else { -0.10 },
                    0.006,
                    0.009,
                    ink,
                    0.6,
                );
            }
        }
    }
}

fn blush(c: &mut InkCanvas<'_>, x: f32, y: f32, accent: Rgb, compact: bool) {
    let n = if compact { 3 } else { 6 };
    for i in 0..n {
        let x = if compact {
            0.04 + i as f32 * 0.012
        } else {
            x + i as f32 * 0.018
        };
        c.line(
            &[(x, y + 0.045), (x + 0.025, y - 0.025)],
            accent,
            0.004,
            0.65,
        );
    }
}
