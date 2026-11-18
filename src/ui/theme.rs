//! MacControl's look: deep gradient backgrounds with soft colour glows, translucent
//! "glass" panels, and an accent colour pair per section that blends as you move around.

use eframe::egui::{self, epaint::Mesh, Color32, CornerRadius, FontId, Pos2, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui, Vec2};
use std::cell::Cell;

pub const TEXT: Color32 = Color32::from_rgb(244, 245, 250);
pub const MUTED: Color32 = Color32::from_rgba_premultiplied(178, 182, 200, 220);
pub const FAINT: Color32 = Color32::from_rgba_premultiplied(130, 134, 156, 200);
pub const GREEN: Color32 = Color32::from_rgb(74, 222, 160);
pub const RED: Color32 = Color32::from_rgb(255, 120, 130);
pub const AMBER: Color32 = Color32::from_rgb(255, 196, 92);

/// Kept for code that still names the old constants.
pub const ACCENT: Color32 = Color32::from_rgb(139, 92, 246);
pub const ACCENT2: Color32 = Color32::from_rgb(236, 92, 190);

/// The colours of one section.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    pub a: Color32,
    pub b: Color32,
    pub bg_top: Color32,
    pub bg_bottom: Color32,
}

impl Palette {
    pub fn lerp(self, o: Palette, t: f32) -> Palette {
        Palette { a: lerp_color(self.a, o.a, t), b: lerp_color(self.b, o.b, t), bg_top: lerp_color(self.bg_top, o.bg_top, t), bg_bottom: lerp_color(self.bg_bottom, o.bg_bottom, t) }
    }
}

thread_local! {
    static CURRENT: Cell<Palette> = const { Cell::new(Palette {
        a: ACCENT,
        b: ACCENT2,
        bg_top: Color32::from_rgb(26, 16, 54),
        bg_bottom: Color32::from_rgb(12, 10, 28),
    }) };
}

pub fn set_palette(p: Palette) {
    CURRENT.with(|c| c.set(p));
}

/// The accent pair of the section on screen.
pub fn accent() -> (Color32, Color32) {
    CURRENT.with(|c| {
        let p = c.get();
        (p.a, p.b)
    })
}

pub fn glass(alpha: u8) -> Color32 {
    Color32::from_white_alpha(alpha)
}

pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    let mut style = (*ctx.global_style()).clone();
    let v = &mut style.visuals;
    *v = egui::Visuals::dark();
    v.panel_fill = Color32::TRANSPARENT;
    v.window_fill = Color32::from_rgb(30, 26, 52);
    v.extreme_bg_color = Color32::from_black_alpha(70);
    v.faint_bg_color = glass(10);
    v.override_text_color = Some(TEXT);
    v.selection.bg_fill = Color32::from_rgba_unmultiplied(160, 120, 255, 110);
    v.selection.stroke = Stroke::new(1.0, Color32::from_rgb(190, 160, 255));
    v.window_corner_radius = CornerRadius::same(14);
    v.menu_corner_radius = CornerRadius::same(12);
    v.window_stroke = Stroke::new(1.0, glass(30));
    v.popup_shadow = egui::Shadow { offset: [0, 10], blur: 30, spread: 0, color: Color32::from_black_alpha(140) };
    for w in [&mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open, &mut v.widgets.noninteractive] {
        w.corner_radius = CornerRadius::same(10);
    }
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, glass(22));
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);
    v.widgets.inactive.bg_fill = glass(16);
    v.widgets.inactive.weak_bg_fill = glass(16);
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, glass(26));
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);
    v.widgets.hovered.bg_fill = glass(28);
    v.widgets.hovered.weak_bg_fill = glass(28);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, glass(60));
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    v.widgets.active.bg_fill = glass(40);
    v.widgets.active.weak_bg_fill = glass(40);
    v.widgets.open.weak_bg_fill = glass(28);
    v.slider_trailing_fill = true;
    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);
    style.spacing.interact_size.y = 30.0;
    style.spacing.slider_width = 200.0;
    style.spacing.combo_width = 220.0;
    use egui::TextStyle::*;
    style.text_styles.insert(Heading, FontId::proportional(28.0));
    style.text_styles.insert(Body, FontId::proportional(13.5));
    style.text_styles.insert(Button, FontId::proportional(13.5));
    style.text_styles.insert(Small, FontId::proportional(11.0));
    ctx.set_global_style(style);
}

pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgba_premultiplied(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()), l(a.a(), b.a()))
}

pub fn ease(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

// ------------------------------------------------------------------ painting helpers

/// Rectangle with a colour at each corner (top-left, top-right, bottom-right, bottom-left).
pub fn corner_gradient(p: &egui::Painter, r: Rect, c: [Color32; 4]) {
    let mut m = Mesh::default();
    m.colored_vertex(r.left_top(), c[0]);
    m.colored_vertex(r.right_top(), c[1]);
    m.colored_vertex(r.right_bottom(), c[2]);
    m.colored_vertex(r.left_bottom(), c[3]);
    m.add_triangle(0, 1, 2);
    m.add_triangle(0, 2, 3);
    p.add(Shape::mesh(m));
}

/// A soft round glow: full colour in the middle fading to nothing.
pub fn glow(p: &egui::Painter, c: Pos2, radius: f32, color: Color32) {
    let mut m = Mesh::default();
    m.colored_vertex(c, color);
    let n = 48;
    for i in 0..=n {
        let a = i as f32 / n as f32 * std::f32::consts::TAU;
        m.colored_vertex(c + Vec2::new(a.cos(), a.sin()) * radius, Color32::TRANSPARENT);
    }
    for i in 1..=n as u32 {
        m.add_triangle(0, i, i + 1);
    }
    p.add(Shape::mesh(m));
}

fn rounded_points(r: Rect, radius: f32) -> Vec<Pos2> {
    let rad = radius.min(r.width() / 2.0).min(r.height() / 2.0);
    let mut pts = vec![];
    let corners = [
        (Pos2::new(r.right() - rad, r.top() + rad), -90.0f32),
        (Pos2::new(r.right() - rad, r.bottom() - rad), 0.0),
        (Pos2::new(r.left() + rad, r.bottom() - rad), 90.0),
        (Pos2::new(r.left() + rad, r.top() + rad), 180.0),
    ];
    for (c, start) in corners {
        for k in 0..=8 {
            let a = (start + k as f32 * 90.0 / 8.0).to_radians();
            pts.push(c + Vec2::new(a.cos(), a.sin()) * rad);
        }
    }
    pts
}

/// Rounded rectangle filled with a left-to-right gradient (smooth, no banding).
pub fn gradient_rounded(p: &egui::Painter, r: Rect, radius: f32, a: Color32, b: Color32) {
    let pts = rounded_points(r, radius);
    let mut m = Mesh::default();
    let col = |q: Pos2| {
        let t = ((q.x - r.left()) / r.width().max(1.0) * 0.8 + (q.y - r.top()) / r.height().max(1.0) * 0.2).clamp(0.0, 1.0);
        lerp_color(a, b, t)
    };
    m.colored_vertex(r.center(), col(r.center()));
    for q in &pts {
        m.colored_vertex(*q, col(*q));
    }
    let n = pts.len() as u32;
    for i in 0..n {
        m.add_triangle(0, 1 + i, 1 + (i + 1) % n);
    }
    p.add(Shape::mesh(m));
}

/// The window background: deep vertical gradient with three drifting colour glows.
pub fn background(p: &egui::Painter, r: Rect, pal: Palette, t: f32) {
    corner_gradient(p, r, [pal.bg_top, lerp_color(pal.bg_top, pal.bg_bottom, 0.35), pal.bg_bottom, lerp_color(pal.bg_bottom, pal.bg_top, 0.25)]);
    let w = r.width();
    let h = r.height();
    let s1 = (t * 0.11).sin();
    let c1 = (t * 0.09).cos();
    let fade = |c: Color32, a: f32| Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (a * 255.0) as u8);
    glow(p, r.min + Vec2::new(w * (0.78 + 0.05 * s1), h * (0.10 + 0.05 * c1)), w * 0.42, fade(pal.a, 0.34));
    glow(p, r.min + Vec2::new(w * (0.30 + 0.04 * c1), h * (0.95 - 0.04 * s1)), w * 0.45, fade(pal.b, 0.26));
    glow(p, r.min + Vec2::new(w * (0.05 + 0.03 * s1), h * 0.35), w * 0.30, fade(lerp_color(pal.a, pal.b, 0.5), 0.18));
}

// ------------------------------------------------------------------ containers

/// A translucent rounded panel ("glass").
pub fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::new()
        .fill(glass(13))
        .stroke(Stroke::new(1.0, glass(26)))
        .corner_radius(CornerRadius::same(18))
        .inner_margin(egui::Margin::same(18))
        .shadow(egui::Shadow { offset: [0, 10], blur: 28, spread: 0, color: Color32::from_black_alpha(60) })
        .show(ui, add)
        .inner
}

/// Paints a glass panel in `rect`; `hover` (0..1) lightens it and lifts it a little.
pub fn glass_panel(p: &egui::Painter, rect: Rect, radius: f32, hover: f32, selected: bool) -> Rect {
    let r = rect.translate(Vec2::new(0.0, -2.0 * hover));
    let fill = glass((13.0 + 12.0 * hover + if selected { 10.0 } else { 0.0 }) as u8);
    p.add(egui::Shadow { offset: [0, 8], blur: 22, spread: 0, color: Color32::from_black_alpha((40.0 + 30.0 * hover) as u8) }.as_shape(r, radius));
    p.rect_filled(r, radius, fill);
    let (a, _) = accent();
    let stroke = if selected { Stroke::new(1.5, lerp_color(a, Color32::WHITE, 0.25)) } else { Stroke::new(1.0, glass((26.0 + 30.0 * hover) as u8)) };
    p.rect_stroke(r, radius, stroke, StrokeKind::Inside);
    r
}

pub fn section_title(ui: &mut Ui, text: &str) {
    ui.label(egui::RichText::new(text.to_uppercase()).size(11.0).color(FAINT).strong());
}

/// Numbered step heading: a gradient dot with the number, then the text.
pub fn step_title(ui: &mut Ui, n: usize, text: &str, hint: &str) {
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::splat(26.0), Sense::hover());
        let (a, b) = accent();
        gradient_rounded(ui.painter(), r, 13.0, a, b);
        ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, n.to_string(), FontId::proportional(13.0), Color32::WHITE);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            ui.label(egui::RichText::new(text).size(15.0).strong().color(TEXT));
            if !hint.is_empty() {
                ui.label(egui::RichText::new(hint).size(11.5).color(MUTED));
            }
        });
    });
}

// ------------------------------------------------------------------ widgets

fn hover_t(ui: &Ui, resp: &Response) -> f32 {
    ui.ctx().animate_bool_with_time(resp.id.with("hov"), resp.hovered(), 0.15)
}

/// The main button: accent gradient pill with a soft glow.
pub fn primary_button(ui: &mut Ui, text: &str) -> Response {
    let font = FontId::proportional(13.5);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font, Color32::WHITE);
    let size = Vec2::new(galley.size().x + 36.0, 36.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    let h = hover_t(ui, &resp);
    let (a, b) = accent();
    let r = rect.translate(Vec2::new(0.0, -1.5 * h));
    ui.painter().add(egui::Shadow { offset: [0, 6], blur: (14.0 + 10.0 * h) as u8, spread: 0, color: Color32::from_rgba_unmultiplied(a.r(), a.g(), a.b(), (90.0 + 60.0 * h) as u8) }.as_shape(r, 18.0));
    gradient_rounded(ui.painter(), r, 18.0, lerp_color(a, Color32::WHITE, 0.12 * h), lerp_color(b, Color32::WHITE, 0.12 * h));
    ui.painter().rect_stroke(r, 18.0, Stroke::new(1.0, glass(60)), StrokeKind::Inside);
    ui.painter().galley(r.center() - galley.size() / 2.0, galley, Color32::WHITE);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A quiet translucent button.
pub fn ghost_button(ui: &mut Ui, text: &str) -> Response {
    let font = FontId::proportional(13.0);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font, TEXT);
    let size = Vec2::new(galley.size().x + 26.0, 32.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    let h = hover_t(ui, &resp);
    ui.painter().rect_filled(rect, 16.0, glass((16.0 + 16.0 * h) as u8));
    ui.painter().rect_stroke(rect, 16.0, Stroke::new(1.0, glass((30.0 + 40.0 * h) as u8)), StrokeKind::Inside);
    ui.painter().galley(rect.center() - galley.size() / 2.0, galley, TEXT);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn danger_button(ui: &mut Ui, text: &str) -> Response {
    let font = FontId::proportional(13.0);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font, RED);
    let size = Vec2::new(galley.size().x + 26.0, 32.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    let h = hover_t(ui, &resp);
    ui.painter().rect_filled(rect, 16.0, Color32::from_rgba_unmultiplied(255, 110, 120, (22.0 + 30.0 * h) as u8));
    ui.painter().rect_stroke(rect, 16.0, Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 120, 130, 110)), StrokeKind::Inside);
    ui.painter().galley(rect.center() - galley.size() / 2.0, galley, RED);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// On/off switch, filled with the accent gradient when on.
pub fn toggle(ui: &mut Ui, on: &mut bool) -> Response {
    let size = Vec2::new(42.0, 24.0);
    let (rect, mut resp) = ui.allocate_exact_size(size, Sense::click());
    if resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }
    let t = ui.ctx().animate_bool_with_time(resp.id, *on, 0.18);
    let p = ui.painter();
    p.rect_filled(rect, 12.0, glass(30));
    if t > 0.0 {
        let (a, b) = accent();
        let fade = |c: Color32| Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (255.0 * t) as u8);
        gradient_rounded(p, rect, 12.0, fade(a), fade(b));
    }
    p.rect_stroke(rect, 12.0, Stroke::new(1.0, glass(40)), StrokeKind::Inside);
    let x = egui::lerp(rect.left() + 12.0..=rect.right() - 12.0, t);
    let c = Pos2::new(x, rect.center().y);
    p.circle_filled(c + Vec2::new(0.0, 1.0), 9.5, Color32::from_black_alpha(60));
    p.circle_filled(c, 9.0, Color32::WHITE);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A labelled row with a switch on the right.
pub fn switch_row(ui: &mut Ui, title: &str, hint: &str, on: &mut bool) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(egui::RichText::new(title).color(TEXT));
            if !hint.is_empty() {
                ui.label(egui::RichText::new(hint).size(11.5).color(MUTED));
            }
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            changed = toggle(ui, on).changed();
        });
    });
    changed
}

/// Keyboard keys drawn as little glass caps.
pub fn keycaps(ui: &mut Ui, parts: &[String], big: bool) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for p in parts {
            let font = FontId::proportional(if big { 17.0 } else { 12.5 });
            let galley = ui.painter().layout_no_wrap(p.clone(), font, TEXT);
            let h = if big { 38.0 } else { 24.0 };
            let w = (galley.size().x + if big { 22.0 } else { 14.0 }).max(h);
            let (rect, _) = ui.allocate_exact_size(Vec2::new(w, h), Sense::hover());
            let pt = ui.painter();
            pt.rect_filled(rect.translate(Vec2::new(0.0, 3.0)), 9.0, Color32::from_black_alpha(90));
            corner_gradient(pt, rect.shrink(0.5), [glass(46), glass(40), glass(22), glass(26)]);
            pt.rect_stroke(rect, 9.0, Stroke::new(1.0, glass(60)), StrokeKind::Inside);
            pt.galley(rect.center() - galley.size() / 2.0, galley, TEXT);
        }
    });
}

/// Splits a combo into separate caps.
pub fn combo_parts(mods: u64, key: Option<String>) -> Vec<String> {
    use crate::model::*;
    let mut v = vec![];
    if mods & MOD_FN != 0 {
        v.push("fn".to_string());
    }
    if mods & MOD_CTRL != 0 {
        v.push("⌃".into());
    }
    if mods & MOD_OPT != 0 {
        v.push("⌥".into());
    }
    if mods & MOD_SHIFT != 0 {
        v.push("⇧".into());
    }
    if mods & MOD_CMD != 0 {
        v.push("⌘".into());
    }
    if let Some(k) = key {
        v.push(k);
    }
    v
}

/// Small rounded label with a coloured dot.
pub fn pill(ui: &mut Ui, text: &str, color: Color32) {
    let font = FontId::proportional(11.5);
    let galley = ui.painter().layout_no_wrap(text.to_string(), font, color);
    let size = Vec2::new(galley.size().x + 24.0, 22.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(rect, 11.0, Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 36));
    ui.painter().rect_stroke(rect, 11.0, Stroke::new(1.0, Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 70)), StrokeKind::Inside);
    ui.painter().circle_filled(Pos2::new(rect.left() + 11.0, rect.center().y), 3.0, color);
    ui.painter().galley(Pos2::new(rect.left() + 18.0, rect.center().y - galley.size().y / 2.0), galley, color);
}

/// Toggleable chip (modifier keys, options).
pub fn chip(ui: &mut Ui, text: &str, on: bool) -> Response {
    let font = FontId::proportional(13.0);
    let col = if on { Color32::WHITE } else { MUTED };
    let galley = ui.painter().layout_no_wrap(text.to_string(), font, col);
    let size = Vec2::new((galley.size().x + 22.0).max(32.0), 30.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    let h = hover_t(ui, &resp);
    let t = ui.ctx().animate_bool_with_time(resp.id.with("on"), on, 0.18);
    ui.painter().rect_filled(rect, 15.0, glass((14.0 + 14.0 * h) as u8));
    if t > 0.0 {
        let (a, b) = accent();
        let fade = |c: Color32| Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (235.0 * t) as u8);
        gradient_rounded(ui.painter(), rect, 15.0, fade(a), fade(b));
    }
    ui.painter().rect_stroke(rect, 15.0, Stroke::new(1.0, glass((28.0 + 30.0 * h) as u8)), StrokeKind::Inside);
    ui.painter().galley(rect.center() - galley.size() / 2.0, galley, col);
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A rounded icon tile with a gradient, as used for sections.
pub fn icon_tile(p: &egui::Painter, r: Rect, a: Color32, b: Color32) {
    p.add(egui::Shadow { offset: [0, 4], blur: 12, spread: 0, color: Color32::from_rgba_unmultiplied(a.r(), a.g(), a.b(), 80) }.as_shape(r, r.width() * 0.28));
    gradient_rounded(p, r, r.width() * 0.28, a, b);
    // Glossy top half.
    let top = Rect::from_min_max(r.min, Pos2::new(r.right(), r.center().y));
    p.rect_filled(top.shrink2(Vec2::new(1.0, 1.0)), CornerRadius { nw: (r.width() * 0.26) as u8, ne: (r.width() * 0.26) as u8, sw: 0, se: 0 }, glass(26));
}
