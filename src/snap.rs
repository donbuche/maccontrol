//! Window snapping geometry: which screen zone the cursor is in, and where a window goes.
//! Coordinates are global points with the origin at the top-left of the main display.

use crate::mechanics as m;
use crate::model::{SnapSettings, WindowPos};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct R {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl R {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        R { x, y, w, h }
    }
    pub fn right(&self) -> f32 {
        self.x + self.w
    }
    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
}

/// One display: its whole frame and the part windows may use (no menu bar, no Dock).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Screen {
    pub frame: R,
    pub visible: R,
}

pub fn screen_at(screens: &[Screen], x: f32, y: f32) -> Option<Screen> {
    screens.iter().find(|s| s.frame.contains(x, y)).copied().or_else(|| {
        // The cursor can sit exactly on the far edge; take the nearest screen.
        screens
            .iter()
            .min_by(|a, b| {
                let da = dist(&a.frame, x, y);
                let db = dist(&b.frame, x, y);
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    })
}

fn dist(r: &R, x: f32, y: f32) -> f32 {
    let dx = (r.x - x).max(0.0).max(x - r.right());
    let dy = (r.y - y).max(0.0).max(y - r.bottom());
    dx * dx + dy * dy
}

/// Width of the strip along each edge that starts a snap.
pub fn edge_width(s: &SnapSettings, screen_count: usize) -> f32 {
    s.recognition_extra + if screen_count > 1 { m::EDGE_MULTI_DISPLAY } else { m::EDGE_SINGLE_DISPLAY }
}

/// The snap zone under the cursor, if any.
pub fn zone_at(x: f32, y: f32, screen: &Screen, screen_count: usize, s: &SnapSettings) -> Option<WindowPos> {
    if !s.enabled {
        return None;
    }
    let f = screen.frame;
    let e = edge_width(s, screen_count);
    let near_left = x <= f.x + e;
    let near_right = x >= f.right() - 1.0 - e;
    let near_top = y <= f.y + e;
    let near_bottom = y >= f.bottom() - 1.0 - e;
    if !(near_left || near_right || near_top || near_bottom) {
        return None;
    }
    if s.corners {
        let c = s.corner_area;
        let top_band = y <= f.y + c;
        let bottom_band = y >= f.bottom() - c;
        let left_band = x <= f.x + c;
        let right_band = x >= f.right() - c;
        if (near_left && top_band) || (near_top && left_band) {
            return Some(WindowPos::TopLeft);
        }
        if (near_right && top_band) || (near_top && right_band) {
            return Some(WindowPos::TopRight);
        }
        if (near_left && bottom_band) || (near_bottom && left_band) {
            return Some(WindowPos::BottomLeft);
        }
        if (near_right && bottom_band) || (near_bottom && right_band) {
            return Some(WindowPos::BottomRight);
        }
    }
    if near_left && s.left {
        return Some(WindowPos::LeftHalf);
    }
    if near_right && s.right {
        return Some(WindowPos::RightHalf);
    }
    if near_top && s.top {
        return Some(WindowPos::Maximize);
    }
    None
}

/// Where a window placed at `pos` goes inside the usable area `v`.
pub fn target_rect(pos: WindowPos, v: R, s: &SnapSettings, current: Option<R>) -> R {
    let lw = s.left_width.clamp(0.1, 0.9);
    let rw = s.right_width.clamp(0.1, 0.9);
    let cw = s.corner_width.clamp(0.1, 0.9);
    let third = v.w / 3.0;
    let raw = match pos {
        WindowPos::Maximize => return v,
        WindowPos::LeftHalf => R::new(v.x, v.y, v.w * lw, v.h),
        WindowPos::RightHalf => R::new(v.right() - v.w * rw, v.y, v.w * rw, v.h),
        WindowPos::TopHalf => R::new(v.x, v.y, v.w, v.h / 2.0),
        WindowPos::BottomHalf => R::new(v.x, v.y + v.h / 2.0, v.w, v.h / 2.0),
        WindowPos::TopLeft => R::new(v.x, v.y, v.w * cw, v.h / 2.0),
        WindowPos::TopRight => R::new(v.right() - v.w * cw, v.y, v.w * cw, v.h / 2.0),
        WindowPos::BottomLeft => R::new(v.x, v.y + v.h / 2.0, v.w * cw, v.h / 2.0),
        WindowPos::BottomRight => R::new(v.right() - v.w * cw, v.y + v.h / 2.0, v.w * cw, v.h / 2.0),
        WindowPos::LeftThird => R::new(v.x, v.y, third, v.h),
        WindowPos::CenterThird => R::new(v.x + third, v.y, third, v.h),
        WindowPos::RightThird => R::new(v.x + 2.0 * third, v.y, third, v.h),
        WindowPos::LeftTwoThirds => R::new(v.x, v.y, 2.0 * third, v.h),
        WindowPos::RightTwoThirds => R::new(v.x + third, v.y, 2.0 * third, v.h),
        WindowPos::Center => {
            let c = current.unwrap_or(R::new(0.0, 0.0, v.w * 0.6, v.h * 0.7));
            let w = c.w.min(v.w);
            let h = c.h.min(v.h);
            return R::new(v.x + (v.w - w) / 2.0, v.y + (v.h - h) / 2.0, w, h);
        }
    };
    let g = s.gap.max(0.0);
    R::new(raw.x + g, raw.y + g, (raw.w - 2.0 * g).max(50.0), (raw.h - 2.0 * g).max(50.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn scr() -> Screen {
        Screen { frame: R::new(0.0, 0.0, 1440.0, 900.0), visible: R::new(0.0, 25.0, 1440.0, 800.0) }
    }
    #[test]
    fn zones() {
        let s = SnapSettings::default();
        let sc = scr();
        assert_eq!(zone_at(0.0, 400.0, &sc, 1, &s), Some(WindowPos::LeftHalf));
        assert_eq!(zone_at(1439.0, 400.0, &sc, 1, &s), Some(WindowPos::RightHalf));
        assert_eq!(zone_at(700.0, 0.0, &sc, 1, &s), Some(WindowPos::Maximize));
        assert_eq!(zone_at(0.0, 10.0, &sc, 1, &s), Some(WindowPos::TopLeft));
        assert_eq!(zone_at(1439.0, 899.0, &sc, 1, &s), Some(WindowPos::BottomRight));
        assert_eq!(zone_at(700.0, 400.0, &sc, 1, &s), None);
        // Bottom edge away from the corners does nothing.
        assert_eq!(zone_at(700.0, 899.0, &sc, 1, &s), None);
    }
    #[test]
    fn halves() {
        let s = SnapSettings::default();
        let v = scr().visible;
        assert_eq!(target_rect(WindowPos::LeftHalf, v, &s, None), R::new(0.0, 25.0, 720.0, 800.0));
        assert_eq!(target_rect(WindowPos::BottomRight, v, &s, None), R::new(720.0, 425.0, 720.0, 400.0));
    }
}
