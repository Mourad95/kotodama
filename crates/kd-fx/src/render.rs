//! CPU reference renderer: draws a [`Frame`] to pixels with `tiny-skia` (ADR-0015).
//!
//! It is the reference the golden-frame tests compare against and the base of the GIF/WebP export.
//! The on-screen `wgpu` renderer draws the same `Frame`.
//!
//! Look: a light background, the message in the level's text colour wrapped to the width and
//! centred, effects tinted from the text colour (speed lines, shockwave, spotlight, tint), the
//! impact flash in black, onomatopoeias in their own colour with a white outline. Background effects
//! are drawn first and the message last, so the words stay readable.
//!
//! A frame can be drawn in two [`Layer`]s (ADR-0017): the soft screen effects can be drawn small and
//! scaled up by the phone, while the hard-edged effects, particles and manga ink are drawn at screen
//! resolution and stay sharp. The overlay layers never draw the message: it lives in the native
//! bubble (ADR-0019).
//!
//! Sizes are expressed in a *unit* of the frame (see [`size_unit`]): its height for a 9:16 portrait
//! frame, less for a wide or short one, so the message and the effects keep their proportions on any
//! screen shape.

use skrifa::instance::{LocationRef, Size};
use skrifa::outline::{DrawSettings, OutlinePen};
use skrifa::{FontRef, GlyphId, MetadataProvider};
use tiny_skia::{
    Color, FillRule, GradientStop, LineCap, LinearGradient, Mask, Paint, Path, PathBuilder, Pixmap,
    Point, RadialGradient, SpreadMode, Stroke, Transform,
};

use crate::scene::{Frame, GlyphState, OnomatopoeiaState, ParticleState, Rgb, ScreenState};
use crate::{Level, ParticleKind, ScreenKind};

const FONT: &[u8] = include_bytes!("../assets/fonts/NotoSansJP-Bold-subset.ttf");
/// Message font size as a fraction of the size unit.
const TEXT_SIZE: f32 = 0.058;
/// Widest a line of the message may be, as a fraction of the frame width.
const TEXT_WIDTH: f32 = 0.80;
/// Characters that never start a line: they stay with the word they follow.
const STICKS_TO_PREVIOUS: &str = ",.;:!?…)»”’";
/// Flash opacity from which the message is drawn in white instead of its ink colour.
const FLASH_TEXT_SWITCH: f32 = 0.35;
/// Line height in em.
const LINE_HEIGHT: f32 = 1.35;

/// How letters are drawn this frame: the message font size, the frame shake and the manga lettering
/// (extrusion depth and accent colour).
#[derive(Clone, Copy)]
struct TextStyle {
    font_px: f32,
    shake: Transform,
    depth: f32,
    accent: Rgb,
}

/// Which part of a frame to draw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    /// Everything, on the light background (export, golden frames).
    Opaque,
    /// Everything, on a transparent background, to composite over a conversation.
    Over,
    /// Only the soft screen effects (flash, tint, spotlight), on a transparent background. They have
    /// no hard edge: draw this layer small and scale it up.
    Back,
    /// Everything else, on a transparent background: the hard-edged screen effects (speed lines,
    /// screentone, burst, shockwave), the particles, the manga ink around the stage (ADR-0018) and the
    /// onomatopoeias. The overlay layers never draw the message itself (ADR-0019): the shell shows it in
    /// its bubble. Its inked lines must stay sharp: draw it at screen resolution, over [`Layer::Back`].
    Front,
}

/// The length every size of a frame is expressed in: the frame height for a 9:16 portrait frame; for a
/// wider or shorter frame, the larger side without exceeding the height a 9:16 frame of that width
/// would have.
pub fn size_unit(width: f32, height: f32) -> f32 {
    (width / 0.5625).min(width.max(height))
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
        self.render_layer(frame, message, width, height, Layer::Opaque)
    }

    /// Like [`Renderer::render`] but on a transparent background, to be composited over a conversation:
    /// only the effects are drawn, never the message nor the panel behind it (ADR-0019); the shell
    /// shows the message in its bubble.
    pub fn render_over(
        &self,
        frame: &Frame,
        message: &str,
        width: u32,
        height: u32,
    ) -> Option<Pixmap> {
        self.render_layer(frame, message, width, height, Layer::Over)
    }

    /// Renders a static manga bubble decoration: the inked panel and the level's onomatopoeias stamped
    /// on its corners. The shell draws accessible native text above it. No message text is accepted
    /// or rasterised here. `dark` selects a dark, opaque reading area.
    pub fn render_bubble(
        &self,
        level: &Level,
        width: u32,
        height: u32,
        dark: bool,
    ) -> Option<Pixmap> {
        if width == 0 || height == 0 || width > 2048 || height > 2048 {
            return None;
        }
        let mut image = Pixmap::new(width, height)?;
        if let Some(style) = &level.manga {
            crate::manga::draw_bubble(
                &mut image,
                style,
                Rgb::from_hex(&level.text_color),
                dark,
                level.level,
            );
        }
        self.stamp_onomatopoeias(&mut image, level);
        Some(image)
    }

    /// Stamps the level's onomatopoeias, still, on the bubble's corners (ADR-0019): the first one
    /// top right, the next bottom left, then the other two corners. Their size follows the bubble's
    /// edge band, not its height, so a long message does not blow them up.
    fn stamp_onomatopoeias(&self, image: &mut Pixmap, level: &Level) {
        const CORNERS: [(f32, f32, f32); 4] = [
            (1.0, 0.0, -0.14),
            (0.0, 1.0, 0.10),
            (0.0, 0.0, -0.08),
            (1.0, 1.0, 0.12),
        ];
        let (w, h) = (image.width() as f32, image.height() as f32);
        let edge = h.min(w * 0.38);
        for (o, &(cx, cy, rotation)) in level.onomatopoeias.iter().zip(CORNERS.iter()) {
            let size = edge * 0.38 * o.scale.clamp(0.4, 0.6);
            // Keep the word inside the bubble, its centre in the corner's padding band.
            let y = if cy < 0.5 {
                size * 0.78
            } else {
                h - size * 0.58
            };
            let stamp = OnomatopoeiaState {
                text: o.text.clone(),
                color: Rgb::from_hex(&o.color),
                scale: 1.0,
                entrance: 1.0,
                rotation,
                alpha: 0.92,
                x: cx,
                y: y / h,
            };
            self.draw_onomatopoeia(image, &stamp, size, w, h, Transform::identity());
        }
    }

    /// Draws one [`Layer`] of `frame` at `width` x `height`. Two layers of the same frame line up when
    /// stretched to the same screen area, whatever their pixel sizes.
    pub fn render_layer(
        &self,
        frame: &Frame,
        message: &str,
        width: u32,
        height: u32,
        layer: Layer,
    ) -> Option<Pixmap> {
        self.draw(frame, message, width, height, layer, None)
    }

    /// Like [`Renderer::render_layer`] with no message, the effects anchored on `stage`: the message
    /// bubble's place on screen (ADR-0020). The ink, the bursts and the particles that cling to the
    /// message frame the bubble; the soft fills and the other particles still cover the whole frame,
    /// and the foreground ones fade where they would cross the bubble. An empty or invalid stage
    /// falls back to the centre of the frame.
    pub fn render_layer_on(
        &self,
        frame: &Frame,
        width: u32,
        height: u32,
        layer: Layer,
        stage: Stage,
    ) -> Option<Pixmap> {
        self.draw(frame, "", width, height, layer, stage.valid())
    }

    fn draw(
        &self,
        frame: &Frame,
        message: &str,
        width: u32,
        height: u32,
        layer: Layer,
        stage: Option<Stage>,
    ) -> Option<Pixmap> {
        let mut pixmap = Pixmap::new(width, height)?;
        if layer == Layer::Opaque {
            pixmap.fill(color(self.background, 1.0));
        }
        let soft = layer != Layer::Front;
        let sharp = layer != Layer::Back;
        let (w, h) = (width as f32, height as f32);
        let u = size_unit(w, h);
        let shake = Transform::from_translate(frame.shake.0 * w, frame.shake.1 * h);
        let ink = frame.text_color;
        // The bubble in pixels, at least STAGE_MIN_HEIGHT tall so a one-line bubble still gets a
        // readable frame of ink; `hole` is its real rect, kept clear of the hard-edged effects.
        let stage_px = stage.map(|s| {
            let height = (s.height * h).max(STAGE_MIN_HEIGHT * h);
            (
                s.x * w,
                (s.y + s.height / 2.0) * h - height / 2.0,
                s.width * w,
                height,
            )
        });
        let hole = stage.map(|s| (s.x * w, s.y * h, s.width * w, s.height * h));
        let focus = stage_px.map_or(Point::from_xy(w / 2.0, h / 2.0), |b| {
            Point::from_xy(b.0 + b.2 / 2.0, b.1 + b.3 / 2.0)
        });
        let area = Area {
            w,
            h,
            u,
            shake,
            focus,
            hole,
        };
        // Effects tied to the message move with it: drawn as if centred, then shifted onto the stage.
        let follow = shake.pre_translate(focus.x - w / 2.0, focus.y - h / 2.0);
        // The coloured panel behind everything, the bubble cut out of it (ADR-0020).
        if soft {
            if let Some(m) = &frame.manga {
                crate::scatter::draw_backdrop(&mut pixmap, m, (w, h), hole);
            }
        }
        // Soft effects first, hard-edged ones over them: the same order whether the frame is drawn
        // whole or as Back then Front.
        for s in &frame.screen {
            if s.kind.is_soft() && soft || !s.kind.is_soft() && sharp {
                draw_screen(&mut pixmap, s, ink, &area);
            }
        }
        if !sharp {
            return Some(pixmap);
        }
        // The overlay layers leave the message to the shell's bubble (ADR-0019): the ink then frames
        // an empty stage in the middle of the screen.
        let message = if layer == Layer::Opaque { message } else { "" };
        // Fit long messages into the central stage before measuring the illustration around them.
        let mut font_px = TEXT_SIZE * u;
        let mut placed = self.layout(message, font_px, w, h);
        while placed
            .last()
            .zip(placed.first())
            .is_some_and(|(last, first)| last.baseline - first.baseline + font_px > h * 0.52)
            && font_px > 3.0
        {
            font_px *= 0.88;
            placed = self.layout(message, font_px, w, h);
        }
        let style = TextStyle {
            font_px,
            shake,
            depth: frame
                .manga
                .as_ref()
                .map_or(0.0, |m| m.style.depth * m.alpha),
            accent: frame
                .manga
                .as_ref()
                .map_or(ink, |m| Rgb::from_hex(&m.style.accent_color)),
        };
        // During an impact flash the background goes dark: the message switches to white so it stays readable.
        let flash = frame
            .screen
            .iter()
            .filter(|s| s.kind == ScreenKind::ImpactFlash)
            .map(|s| s.alpha)
            .fold(0.0_f32, f32::max);
        let text_ink = if flash >= FLASH_TEXT_SWITCH && frame.manga.is_none() {
            Rgb(255, 255, 255)
        } else {
            ink
        };
        let (mut left, mut right, mut top, mut bottom) = (w * 0.25, w * 0.75, h * 0.46, h * 0.54);
        if !placed.is_empty() {
            left = placed
                .iter()
                .map(|p| p.cx - p.advance / 2.0)
                .fold(w, f32::min);
            right = placed
                .iter()
                .map(|p| p.cx + p.advance / 2.0)
                .fold(0.0, f32::max);
            top = placed.first().expect("nonempty layout").baseline - font_px;
            bottom = placed.last().expect("nonempty layout").baseline + font_px * 0.25;
        }
        let panel_width = (right - left + font_px * 1.1).min(w * 0.94);
        let panel_height = (bottom - top + font_px * 1.1).max(h * 0.19);
        let bounds = stage_px.unwrap_or((
            (w - panel_width) / 2.0,
            h * 0.5 - panel_height / 2.0,
            panel_width,
            panel_height,
        ));
        let panel = layer == Layer::Opaque;
        let particle = |pixmap: &mut Pixmap, p: &ParticleState| {
            if p.kind.is_anchored() {
                draw_particle(pixmap, p, w, h, u, follow);
            } else if p.depth > 0.68 && hole.is_some_and(|b| covers(b, p.x * w, p.y * h)) {
                // A foreground particle crossing the bubble would hide the message: it fades there.
                let faded = ParticleState {
                    alpha: p.alpha * 0.25,
                    ..*p
                };
                draw_particle(pixmap, &faded, w, h, u, shake);
            } else {
                draw_particle(pixmap, p, w, h, u, shake);
            }
        };
        for p in frame.particles.iter().filter(|p| p.depth <= 0.68) {
            particle(&mut pixmap, p);
        }
        if let Some(m) = &frame.manga {
            crate::manga::draw_scene(&mut pixmap, m, ink, bounds, shake, false, panel);
            crate::scatter::draw_scatter(&mut pixmap, m, (w, h), hole.unwrap_or(bounds), shake);
        }
        for g in &frame.glyphs {
            if let Some(p) = placed
                .get(g.index)
                .filter(|p| p.index == g.index)
                .or_else(|| placed.iter().find(|p| p.index == g.index))
            {
                self.draw_glyph(&mut pixmap, p, g, text_ink, &style);
            }
        }
        if let Some(m) = &frame.manga {
            crate::manga::draw_scene(&mut pixmap, m, ink, bounds, shake, true, panel);
        }
        for p in frame.particles.iter().filter(|p| p.depth > 0.68) {
            particle(&mut pixmap, p);
        }
        for o in &frame.onomatopoeias {
            match hole {
                Some(b) => {
                    let (placed, size) = self.frame_word(o, b, font_px, w, h);
                    self.draw_onomatopoeia(&mut pixmap, &placed, size, w, h, shake);
                }
                None => self.draw_onomatopoeia(&mut pixmap, o, font_px, w, h, follow),
            }
        }
        Some(pixmap)
    }

    /// Width of `text` lettered at `size` pixels, its outline included.
    fn word_width(&self, text: &str, size: f32) -> f32 {
        text.chars().map(|c| self.advance(c, size).1).sum::<f32>() + 0.5 * size
    }

    /// Places an onomatopoeia as a giant word framing the bubble `hole` (pixels), like a manga panel
    /// title: in the free space above the bubble when the theme places it in the upper half, below
    /// otherwise. A side too short for a word gives its words to the other side, stacked in two rows.
    /// The word grows to fill its row and most of the screen width, never past its theme scale times
    /// [`WORD_MAX_SCALE`]. Returns the placed word (its `scale` reduced to the entrance bounce) and its
    /// settled size in pixels.
    fn frame_word(
        &self,
        o: &OnomatopoeiaState,
        hole: (f32, f32, f32, f32),
        font_px: f32,
        w: f32,
        h: f32,
    ) -> (OnomatopoeiaState, f32) {
        let gap = 0.02 * h;
        let above = (gap, (hole.1 - gap).max(gap));
        let below = ((hole.1 + hole.3 + gap).min(h - gap), h - gap);
        let tall = |r: (f32, f32)| r.1 - r.0 >= WORD_MIN_ROOM * h;
        let wants_above = o.y < 0.5;
        let (room, shared) = match (wants_above, tall(above), tall(below)) {
            (true, true, _) | (false, true, false) => (above, !tall(below)),
            _ => (below, !tall(above)),
        };
        // Two words share a side: the upper-half word takes the row nearer the top.
        let room = if shared {
            let mid = (room.0 + room.1) / 2.0;
            if wants_above {
                (room.0, mid)
            } else {
                (mid, room.1)
            }
        } else {
            room
        };
        let base = o.scale / o.entrance.max(1e-3);
        let size = (font_px * base * WORD_MAX_SCALE)
            .min(0.62 * (room.1 - room.0))
            .min(WORD_WIDTH * w / self.word_width(&o.text, 1.0).max(1e-3))
            .max(1.0);
        let placed = OnomatopoeiaState {
            x: 0.5 + 0.3 * (o.x - 0.5),
            y: ((room.0 + room.1) / 2.0) / h,
            scale: o.entrance,
            ..o.clone()
        };
        (placed, size)
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
            depth,
            accent,
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
            .pre_concat(Transform::from_row(
                1.0,
                0.0,
                -crate::letter_slant(depth),
                1.0,
                0.0,
                0.0,
            ))
            .pre_scale(s, s)
            .post_concat(shake);
        if depth > 0.0 {
            // Layered outline offsets imitate an inked side face without a 3D mesh.
            let (dx, dy) = crate::EXTRUSION_STEP;
            for i in (1..=crate::extrusion_steps(depth)).rev() {
                let side = t.post_translate(i as f32 * font_px * dx, i as f32 * font_px * dy);
                let stroke = Stroke {
                    width: font_px * 0.035 / s,
                    line_join: tiny_skia::LineJoin::Round,
                    ..Stroke::default()
                };
                pixmap.stroke_path(
                    &path,
                    &paint(Rgb(24, 31, 48), g.alpha * 0.8),
                    &stroke,
                    side,
                    None,
                );
                pixmap.fill_path(
                    &path,
                    &paint(accent, g.alpha * 0.9),
                    FillRule::Winding,
                    side,
                    None,
                );
            }
            let stroke = Stroke {
                width: font_px * 0.045 / s,
                line_join: tiny_skia::LineJoin::Round,
                ..Stroke::default()
            };
            pixmap.stroke_path(&path, &paint(Rgb(255, 255, 255), g.alpha), &stroke, t, None);
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
        place: Transform,
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
        // `place` may move the word onto the bubble: clamp where it lands, not where it was laid out.
        let (dx, dy) = (place.tx, place.ty);
        let x_px = (o.x * w + dx).clamp(half, w - half) - dx;
        let y_px = (o.y * h + dy).clamp(size * 0.7, (h - size * 0.5).max(size * 0.7)) - dy;
        let at = Transform::from_translate(x_px, y_px)
            .pre_rotate(o.rotation.to_degrees())
            .post_concat(place);
        // The word is lettered opaque on its own small layer, then faded as a whole: a fading word
        // does not show its rim through its outline, and the Back/Front split rounds the same way.
        let (hw, hh) = (total / 2.0 + 0.4 * size, size);
        let corners = [(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)].map(|(x, y)| {
            let mut p = Point::from_xy(x, y);
            at.map_point(&mut p);
            p
        });
        let x0 = corners
            .iter()
            .map(|p| p.x)
            .fold(f32::MAX, f32::min)
            .floor()
            .max(0.0);
        let y0 = corners
            .iter()
            .map(|p| p.y)
            .fold(f32::MAX, f32::min)
            .floor()
            .max(0.0);
        let x1 = corners
            .iter()
            .map(|p| p.x)
            .fold(f32::MIN, f32::max)
            .ceil()
            .min(w);
        let y1 = corners
            .iter()
            .map(|p| p.y)
            .fold(f32::MIN, f32::max)
            .ceil()
            .min(h);
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let Some(mut word) = Pixmap::new((x1 - x0) as u32, (y1 - y0) as u32) else {
            return;
        };
        let at = at.post_translate(-x0, -y0);
        let mut x = -total / 2.0;
        let mut paths = Vec::new();
        for (gid, adv) in glyphs {
            if let Some(path) = self.outline(gid) {
                paths.push((path, at.pre_translate(x, size * 0.35).pre_scale(s, s)));
            }
            x += adv;
        }
        // Manga lettering, each pass over the whole word: a white rim, a black drop shadow, a thick
        // black outline, then the colour, so a bright word reads on any panel colour.
        let outline = Stroke {
            width: 0.20 * size / s,
            line_cap: LineCap::Round,
            line_join: tiny_skia::LineJoin::Round,
            ..Stroke::default()
        };
        let rim = Stroke {
            width: 0.36 * size / s,
            ..outline.clone()
        };
        let white = paint(Rgb(255, 255, 255), 1.0);
        let black = paint(LETTERING_INK, 1.0);
        for (path, t) in &paths {
            word.stroke_path(
                path,
                &white,
                &rim,
                t.post_translate(size * 0.10, size * 0.10),
                None,
            );
            word.stroke_path(path, &white, &rim, *t, None);
        }
        for (path, t) in &paths {
            let shadow = t.post_translate(size * 0.10, size * 0.10);
            word.stroke_path(path, &black, &outline, shadow, None);
            word.fill_path(path, &black, FillRule::Winding, shadow, None);
        }
        for (path, t) in &paths {
            word.stroke_path(path, &black, &outline, *t, None);
        }
        // The colour runs from a light tint at the top of the letters to a deeper shade at the bottom.
        let (mut top, mut bottom) = (
            Point::from_xy(0.0, -0.55 * size),
            Point::from_xy(0.0, 0.35 * size),
        );
        at.map_point(&mut top);
        at.map_point(&mut bottom);
        let fill = LinearGradient::new(
            top,
            bottom,
            vec![
                GradientStop::new(0.0, color(crate::scatter::lighten(o.color, 0.55), 1.0)),
                GradientStop::new(0.55, color(o.color, 1.0)),
                GradientStop::new(1.0, color(shade(o.color, 0.7), 1.0)),
            ],
            SpreadMode::Pad,
            Transform::identity(),
        )
        .map_or_else(
            || paint(o.color, 1.0),
            |shader| Paint {
                shader,
                anti_alias: true,
                ..Paint::default()
            },
        );
        for (path, t) in &paths {
            if let Some(path) = path.clone().transform(*t) {
                word.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
            }
        }
        pixmap.draw_pixmap(
            x0 as i32,
            y0 as i32,
            word.as_ref(),
            &tiny_skia::PixmapPaint {
                opacity: o.alpha.clamp(0.0, 1.0),
                ..tiny_skia::PixmapPaint::default()
            },
            Transform::identity(),
            None,
        );
    }
}

/// Share of the screen width a framing onomatopoeia may span.
const WORD_WIDTH: f32 = 0.9;
/// A framing onomatopoeia never grows past its theme scale times this.
const WORD_MAX_SCALE: f32 = 4.0;
/// The least free height, as a share of the screen, for a side of the bubble to hold its own word.
const WORD_MIN_ROOM: f32 = 0.16;

/// The outline of the onomatopoeia lettering; its fill must contrast with it (theme tests).
pub const LETTERING_INK: Rgb = Rgb(18, 18, 26);

/// `rgb` darkened toward black, keeping `keep` of each channel.
fn shade(rgb: Rgb, keep: f32) -> Rgb {
    let k = |c: u8| (f32::from(c) * keep).round() as u8;
    Rgb(k(rgb.0), k(rgb.1), k(rgb.2))
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

/// Where screen effects are drawn: the frame size in pixels, its size unit, the frame shake, the
/// point the effects gather on (the frame centre, or the bubble's) and the bubble rect to keep clear.
struct Area {
    w: f32,
    h: f32,
    u: f32,
    shake: Transform,
    focus: Point,
    hole: Option<(f32, f32, f32, f32)>,
}

impl Area {
    fn centre(&self) -> Point {
        self.focus
    }

    /// Distance from the focus to the farthest corner: a radial effect of this radius fills the frame.
    fn reach(&self) -> f32 {
        let dx = self.focus.x.max(self.w - self.focus.x);
        let dy = self.focus.y.max(self.h - self.focus.y);
        (dx * dx + dy * dy).sqrt()
    }

    /// Half the diagonal of the bubble to keep clear, or 0 without one.
    fn hole_radius(&self) -> f32 {
        self.hole
            .map_or(0.0, |b| (b.2 * b.2 + b.3 * b.3).sqrt() / 2.0)
    }
}

/// The message bubble's rect, in fractions of the frame (0 to 1, from the top left corner).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stage {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Width.
    pub width: f32,
    /// Height.
    pub height: f32,
}

impl Stage {
    /// The stage clipped to the frame, or `None` if it is not finite or ends up empty.
    fn valid(self) -> Option<Self> {
        let finite = [self.x, self.y, self.width, self.height]
            .iter()
            .all(|v| v.is_finite());
        if !finite {
            return None;
        }
        let (x0, y0) = (self.x.clamp(0.0, 1.0), self.y.clamp(0.0, 1.0));
        let x1 = (self.x + self.width).clamp(0.0, 1.0);
        let y1 = (self.y + self.height).clamp(0.0, 1.0);
        (x1 - x0 > 0.0 && y1 - y0 > 0.0).then_some(Self {
            x: x0,
            y: y0,
            width: x1 - x0,
            height: y1 - y0,
        })
    }
}

/// Smallest height of the inked frame around a bubble, as a fraction of the frame height.
const STAGE_MIN_HEIGHT: f32 = 0.14;

/// True if the point lies inside `bounds` (x, y, width, height).
fn covers(bounds: (f32, f32, f32, f32), x: f32, y: f32) -> bool {
    x >= bounds.0 && x <= bounds.0 + bounds.2 && y >= bounds.1 && y <= bounds.1 + bounds.3
}

fn full_rect(w: f32, h: f32) -> Path {
    PathBuilder::from_rect(tiny_skia::Rect::from_xywh(-w, -h, 3.0 * w, 3.0 * h).expect("non-empty"))
}

/// A paint filled with a radial gradient centred on `centre`, `stops` placed along `radius`.
fn radial(centre: Point, radius: f32, stops: Vec<GradientStop>) -> Option<Paint<'static>> {
    let shader = RadialGradient::new(
        centre,
        0.0,
        centre,
        radius,
        stops,
        SpreadMode::Pad,
        Transform::identity(),
    )?;
    Some(Paint {
        shader,
        anti_alias: true,
        ..Paint::default()
    })
}

/// A lighter version of `ink`: `keep` of the colour mixed into white.
fn tint_of(ink: Rgb, keep: f32) -> Rgb {
    let mix = |c: u8| (255.0 - (255.0 - f32::from(c)) * keep).round() as u8;
    Rgb(mix(ink.0), mix(ink.1), mix(ink.2))
}

fn draw_screen(pixmap: &mut Pixmap, s: &ScreenState, ink: Rgb, area: &Area) {
    let (w, h, u, shake) = (area.w, area.h, area.u, area.shake);
    let centre = area.centre();
    let whole = full_rect(w, h);
    match s.kind {
        ScreenKind::ImpactFlash => {
            // The flash darkens the screen around the bubble, never the message itself.
            let mut flash = PathBuilder::new();
            flash.push_rect(
                tiny_skia::Rect::from_xywh(-w, -h, 3.0 * w, 3.0 * h).expect("non-empty"),
            );
            if let Some(cut) = area
                .hole
                .and_then(|b| tiny_skia::Rect::from_xywh(b.0, b.1, b.2, b.3))
            {
                flash.push_rect(cut);
            }
            pixmap.fill_path(
                &flash.finish().unwrap_or_else(|| whole.clone()),
                &paint(Rgb(0, 0, 0), s.alpha),
                FillRule::EvenOdd,
                Transform::identity(),
                None,
            );
        }
        ScreenKind::Tint => {
            // A vignette rather than a flat veil: the colour gathers at the edges, the message stays clear.
            let stops = vec![
                GradientStop::new(0.0, color(ink, 0.05 * s.alpha)),
                GradientStop::new(1.0, color(ink, 0.24 * s.alpha)),
            ];
            if let Some(p) = radial(centre, area.reach(), stops) {
                pixmap.fill_path(&whole, &p, FillRule::Winding, Transform::identity(), None);
            }
        }
        ScreenKind::Spotlight => {
            let stops = vec![
                GradientStop::new(0.0, Color::from_rgba8(0, 0, 0, 0)),
                GradientStop::new(0.4, Color::from_rgba8(0, 0, 0, 0)),
                GradientStop::new(1.0, color(Rgb(0, 0, 0), 0.7 * s.alpha)),
            ];
            if let Some(p) = radial(centre, area.reach() * 1.1, stops) {
                pixmap.fill_path(&whole, &p, FillRule::Winding, Transform::identity(), None);
            }
        }
        ScreenKind::Shockwave => {
            let radius = s.progress.powf(0.6) * u * 0.7;
            if radius > 1.0 {
                let oval = tiny_skia::Rect::from_xywh(
                    centre.x - radius,
                    centre.y - radius * 0.42,
                    radius * 2.0,
                    radius * 0.84,
                );
                if let Some(ring) = oval.and_then(PathBuilder::from_oval) {
                    let stroke = Stroke {
                        width: 0.012 * u * (1.0 - 0.5 * s.progress),
                        ..Stroke::default()
                    };
                    let perspective = shake.pre_rotate_at(-10.0, centre.x, centre.y);
                    pixmap.stroke_path(
                        &ring,
                        &paint(ink, 0.55 * s.alpha),
                        &stroke,
                        perspective,
                        None,
                    );
                }
            }
        }
        ScreenKind::SpeedLines => {
            // Thin wedges converging on the message, fading out before they reach it; angles are fixed
            // so the lines are stable.
            let (inner, outer) = ((u * 0.2).max(area.hole_radius()), area.reach() * 1.1);
            let mut pb = PathBuilder::new();
            for i in 0..64_u32 {
                let a =
                    (i as f32 + 0.5 * ((i * 7 % 5) as f32 / 5.0)) / 64.0 * std::f32::consts::TAU;
                let half = 0.006 + 0.01 * ((i * 13 % 7) as f32 / 7.0);
                let r0 = inner + u * 0.14 * ((i * 11 % 6) as f32 / 6.0);
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
            let stops = vec![
                GradientStop::new(inner / outer, color(ink, 0.0)),
                GradientStop::new(
                    ((inner + 0.25 * u) / outer).min(1.0),
                    color(ink, 0.45 * s.alpha),
                ),
                GradientStop::new(1.0, color(ink, 0.6 * s.alpha)),
            ];
            if let (Some(path), Some(p)) = (pb.finish(), radial(centre, outer, stops)) {
                pixmap.fill_path(&path, &p, FillRule::Winding, shake, None);
            }
        }
        ScreenKind::Halftone => draw_halftone(pixmap, s, ink, area),
        ScreenKind::Burst => draw_burst(pixmap, s, ink, area),
        ScreenKind::Shake => {}
    }
}

/// Screentone: staggered rows of dots whose size grows from nothing near the centre (where the message
/// is) to a full dot at the edges, so the middle of the frame stays clean.
fn draw_halftone(pixmap: &mut Pixmap, s: &ScreenState, ink: Rgb, area: &Area) {
    let (w, h) = (area.w, area.h);
    let step = 0.026 * area.u;
    let reach = area.reach();
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
            let d = ((x - area.focus.x).powi(2) + (y - area.focus.y).powi(2)).sqrt() / reach;
            let grow = ((d - 0.35) / 0.65).clamp(0.0, 1.0);
            let r = step * 0.5 * grow * grow * s.strength.clamp(0.0, 1.0);
            if r > 0.5 && !area.hole.is_some_and(|b| covers(b, x, y)) {
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
            &paint(ink, 0.3 * s.alpha / s.strength.max(0.01)),
            FillRule::Winding,
            area.shake,
            None,
        );
    }
}

/// The outline of a manga burst of `base` radius centred on the frame.
fn burst_path(area: &Area, base: f32) -> Option<Path> {
    const SPIKES: u32 = 18;
    let mut pb = PathBuilder::new();
    for i in 0..SPIKES * 2 {
        let spike = i % 2 == 0;
        // Deterministic irregularity so the shape looks drawn by hand, not stamped.
        let wobble = 0.88 + 0.24 * (((u64::from(i) * 2_654_435_761) >> 7) % 100) as f32 / 100.0;
        let radius = if spike { base * wobble } else { base * 0.76 };
        let angle = i as f32 / (SPIKES * 2) as f32 * std::f32::consts::TAU;
        let (x, y) = (
            area.focus.x + radius * 1.1 * angle.cos(),
            area.focus.y + radius * angle.sin(),
        );
        if i == 0 {
            pb.move_to(x, y);
        } else {
            pb.line_to(x, y);
        }
    }
    pb.close();
    pb.finish()
}

/// Manga explosion: a spiky shape, white at its heart and tinted towards its tips, with a fine ink
/// outline and a fainter inner contour, popping in with a little overshoot. Drawn behind the message,
/// so the words sit on a clean light area.
fn draw_burst(pixmap: &mut Pixmap, s: &ScreenState, ink: Rgb, area: &Area) {
    let entrance = (s.progress / 0.12).clamp(0.0, 1.0);
    let grow = crate::scene::ease_out_back(entrance);
    // Around a bubble the burst grows from its edge, so its spikes show past the message.
    let size = area.hole.map_or(area.w.min(area.h * 0.7), |b| {
        (b.2 * 0.62).max(b.3 * 1.25).min(area.w)
    });
    let base = 0.5 * size * 1.05 * grow;
    if base < 2.0 {
        return;
    }
    let Some(path) = burst_path(area, base) else {
        return;
    };
    let stops = vec![
        GradientStop::new(0.0, color(Rgb(255, 255, 255), 0.95 * s.alpha)),
        GradientStop::new(0.55, color(tint_of(ink, 0.06), 0.93 * s.alpha)),
        GradientStop::new(1.0, color(tint_of(ink, 0.22), 0.9 * s.alpha)),
    ];
    if let Some(fill) = radial(area.centre(), base * 1.2, stops) {
        // The bubble's rect is cut out of the fill (even-odd), so the burst never hides the message.
        let cut = area.hole.and_then(|b| {
            let mut pb = PathBuilder::new();
            pb.push_path(&path);
            pb.push_rect(tiny_skia::Rect::from_xywh(b.0, b.1, b.2, b.3)?);
            pb.finish()
        });
        match cut {
            Some(cut) => pixmap.fill_path(&cut, &fill, FillRule::EvenOdd, area.shake, None),
            None => pixmap.fill_path(&path, &fill, FillRule::Winding, area.shake, None),
        }
    }
    let stroke = |width: f32| Stroke {
        width: width * area.u,
        line_join: tiny_skia::LineJoin::Round,
        ..Stroke::default()
    };
    pixmap.stroke_path(
        &path,
        &paint(ink, 0.7 * s.alpha),
        &stroke(0.0045),
        area.shake,
        None,
    );
    if let Some(inner) = burst_path(area, base * 0.9) {
        pixmap.stroke_path(
            &inner,
            &paint(ink, 0.22 * s.alpha),
            &stroke(0.0025),
            area.shake,
            None,
        );
    }
}

fn star(cx: f32, cy: f32, r: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    for i in 0..8 {
        let a = i as f32 * std::f32::consts::FRAC_PI_4 - std::f32::consts::FRAC_PI_2;
        let rr = if i % 2 == 0 { r } else { r * 0.28 };
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

/// A soft disc of `rgb` fading from `alpha` at its centre to nothing at `radius`.
fn glow(pixmap: &mut Pixmap, x: f32, y: f32, radius: f32, rgb: Rgb, alpha: f32, t: Transform) {
    let stops = vec![
        GradientStop::new(0.0, color(rgb, alpha)),
        GradientStop::new(0.4, color(rgb, alpha * 0.45)),
        GradientStop::new(1.0, color(rgb, 0.0)),
    ];
    if let (Some(disc), Some(p)) = (
        PathBuilder::from_circle(x, y, radius),
        radial(Point::from_xy(x, y), radius, stops),
    ) {
        pixmap.fill_path(&disc, &p, FillRule::Winding, t, None::<&Mask>);
    }
}

fn draw_particle(pixmap: &mut Pixmap, p: &ParticleState, w: f32, h: f32, u: f32, shake: Transform) {
    if p.alpha <= 0.0 {
        return;
    }
    let (x, y, r) = (p.x * w, p.y * h, p.size * u);
    let spin = shake.pre_rotate_at(p.rotation.to_degrees(), x, y);
    let fill = paint(p.color, p.alpha);
    if p.kind == ParticleKind::Sparks {
        draw_spark(pixmap, p, x, y, r, spin);
        return;
    }
    let path = match p.kind {
        ParticleKind::Confetti => {
            // Paper turning in the air: its apparent height follows the spin.
            let flip = (p.rotation * 1.7).cos().abs().max(0.15);
            tiny_skia::Rect::from_xywh(x - r, y - r * 0.5 * flip, r * 2.0, r * flip)
                .map(PathBuilder::from_rect)
        }
        ParticleKind::Stars => {
            glow(pixmap, x, y, r * 2.4, p.color, 0.3 * p.alpha, shake);
            star(x, y, r * 1.6)
        }
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
        ParticleKind::Aura | ParticleKind::Embers | ParticleKind::Sparks => {
            let mut b = PathBuilder::new();
            b.move_to(x - r * 0.35, y + r);
            b.line_to(x - r * 0.6, y - r * 0.15);
            b.line_to(x + r * 0.4, y - r * 1.5);
            b.line_to(x + r * 0.5, y + r * 0.3);
            b.close();
            b.finish()
        }
    };
    if let Some(path) = path {
        if p.depth > 0.68 {
            // Broad faint silhouette separates the nearest plane without costly full-frame blur.
            let stroke = Stroke {
                width: r * 0.3,
                line_join: tiny_skia::LineJoin::Round,
                ..Stroke::default()
            };
            pixmap.stroke_path(&path, &paint(p.color, p.alpha * 0.12), &stroke, spin, None);
        }
        pixmap.fill_path(&path, &fill, FillRule::Winding, spin, None::<&Mask>);
        if !matches!(p.kind, ParticleKind::Rain) {
            let stroke = Stroke {
                width: (r * 0.07).max(0.5),
                ..Stroke::default()
            };
            pixmap.stroke_path(
                &path,
                &paint(Rgb(32, 36, 50), p.alpha * 0.35),
                &stroke,
                spin,
                None,
            );
            pixmap.stroke_path(
                &path,
                &paint(Rgb(255, 255, 255), p.alpha * 0.5),
                &Stroke {
                    width: (r * 0.04).max(0.4),
                    ..Stroke::default()
                },
                spin.post_translate(-r * 0.12, -r * 0.10),
                None,
            );
        }
    }
    if p.kind == ParticleKind::Hearts {
        // A small highlight makes the heart look rounded rather than flat.
        if let Some(dot) = PathBuilder::from_circle(x - 0.4 * r, y - 0.35 * r, 0.16 * r) {
            pixmap.fill_path(
                &dot,
                &paint(Rgb(255, 255, 255), 0.55 * p.alpha),
                FillRule::Winding,
                spin,
                None::<&Mask>,
            );
        }
    }
}

/// An electric spark: a short zigzag bolt along the particle's heading, glowing in its colour with a
/// white-hot core.
fn draw_spark(pixmap: &mut Pixmap, p: &ParticleState, x: f32, y: f32, r: f32, spin: Transform) {
    const ZIGZAG: [(f32, f32); 5] = [
        (-1.6, 0.0),
        (-0.6, 0.55),
        (0.0, -0.45),
        (0.7, 0.4),
        (1.6, -0.1),
    ];
    let mut pb = PathBuilder::new();
    for (i, &(dx, dy)) in ZIGZAG.iter().enumerate() {
        let (px, py) = (x + dx * r * 1.4, y + dy * r);
        if i == 0 {
            pb.move_to(px, py);
        } else {
            pb.line_to(px, py);
        }
    }
    let Some(bolt) = pb.finish() else {
        return;
    };
    glow(pixmap, x, y, r * 3.0, p.color, 0.35 * p.alpha, spin);
    let stroke = |width: f32| Stroke {
        width: (r * width).max(0.6),
        line_cap: LineCap::Round,
        line_join: tiny_skia::LineJoin::Round,
        ..Stroke::default()
    };
    pixmap.stroke_path(
        &bolt,
        &paint(p.color, 0.35 * p.alpha),
        &stroke(0.9),
        spin,
        None,
    );
    pixmap.stroke_path(&bolt, &paint(p.color, p.alpha), &stroke(0.4), spin, None);
    pixmap.stroke_path(
        &bolt,
        &paint(Rgb(255, 255, 255), 0.9 * p.alpha),
        &stroke(0.15),
        spin,
        None,
    );
}
