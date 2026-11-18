//! Small line icons drawn with the painter (no image files needed).

use eframe::egui::{Color32, Painter, Pos2, Rect, Stroke, StrokeKind, Vec2};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Icon {
    Home,
    Trackpad,
    Keyboard,
    Mouse,
    Snap,
    Settings,
    Apps,
    Plus,
    Bolt,
}

pub fn draw(p: &Painter, icon: Icon, rect: Rect, color: Color32) {
    let s = Stroke::new(1.6, color);
    let c = rect.center();
    let w = rect.width();
    match icon {
        Icon::Home => {
            let v = |x: f32, y: f32| c + Vec2::new(w * x, w * y);
            p.line_segment([v(-0.40, 0.02), v(0.0, -0.36)], s);
            p.line_segment([v(0.0, -0.36), v(0.40, 0.02)], s);
            p.line_segment([v(-0.28, -0.08), v(-0.28, 0.36)], s);
            p.line_segment([v(0.28, -0.08), v(0.28, 0.36)], s);
            p.line_segment([v(-0.28, 0.36), v(0.28, 0.36)], s);
            p.rect_filled(Rect::from_min_max(v(-0.08, 0.12), v(0.08, 0.36)), 1.5, color);
        }
        Icon::Trackpad => {
            let r = Rect::from_center_size(c, Vec2::new(w * 0.86, w * 0.64));
            p.rect_stroke(r, 3.0, s, StrokeKind::Middle);
            p.circle_filled(c + Vec2::new(-w * 0.12, -w * 0.04), w * 0.06, color);
            p.circle_filled(c + Vec2::new(w * 0.12, -w * 0.04), w * 0.06, color);
        }
        Icon::Keyboard => {
            let r = Rect::from_center_size(c, Vec2::new(w * 0.9, w * 0.6));
            p.rect_stroke(r, 3.0, s, StrokeKind::Middle);
            for row in 0..2 {
                for col in 0..4 {
                    let x = r.left() + w * 0.16 + col as f32 * w * 0.19;
                    let y = r.top() + w * 0.16 + row as f32 * w * 0.14;
                    p.circle_filled(Pos2::new(x, y), w * 0.04, color);
                }
            }
            p.line_segment([Pos2::new(r.left() + w * 0.22, r.bottom() - w * 0.13), Pos2::new(r.right() - w * 0.22, r.bottom() - w * 0.13)], s);
        }
        Icon::Mouse => {
            let r = Rect::from_center_size(c, Vec2::new(w * 0.56, w * 0.86));
            p.rect_stroke(r, w * 0.28, s, StrokeKind::Middle);
            p.line_segment([Pos2::new(c.x, r.top() + w * 0.12), Pos2::new(c.x, r.top() + w * 0.3)], s);
        }
        Icon::Snap => {
            let r = Rect::from_center_size(c, Vec2::new(w * 0.9, w * 0.7));
            p.rect_stroke(r, 3.0, s, StrokeKind::Middle);
            let half = Rect::from_min_max(r.min + Vec2::splat(2.5), Pos2::new(c.x - 1.0, r.bottom() - 2.5));
            p.rect_filled(half, 1.5, color);
        }
        Icon::Settings => {
            p.circle_stroke(c, w * 0.18, s);
            for i in 0..8 {
                let a = i as f32 * std::f32::consts::TAU / 8.0;
                let d = Vec2::new(a.cos(), a.sin());
                p.line_segment([c + d * w * 0.28, c + d * w * 0.42], Stroke::new(2.2, color));
            }
            p.circle_stroke(c, w * 0.31, Stroke::new(1.4, color));
        }
        Icon::Apps => {
            let g = w * 0.32;
            for (dx, dy) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
                let cc = c + Vec2::new(dx * g * 0.62, dy * g * 0.62);
                p.rect_stroke(Rect::from_center_size(cc, Vec2::splat(g * 0.9)), 2.0, s, StrokeKind::Middle);
            }
        }
        Icon::Plus => {
            p.line_segment([c - Vec2::new(w * 0.3, 0.0), c + Vec2::new(w * 0.3, 0.0)], Stroke::new(2.0, color));
            p.line_segment([c - Vec2::new(0.0, w * 0.3), c + Vec2::new(0.0, w * 0.3)], Stroke::new(2.0, color));
        }
        Icon::Bolt => {
            let v = |x: f32, y: f32| c + Vec2::new(w * x, w * y);
            p.add(eframe::egui::Shape::convex_polygon(vec![v(0.10, -0.44), v(-0.22, 0.08), v(0.06, 0.08)], color, Stroke::NONE));
            p.add(eframe::egui::Shape::convex_polygon(vec![v(-0.10, 0.44), v(-0.06, -0.08), v(0.22, -0.08)], color, Stroke::NONE));
        }
    }
}
