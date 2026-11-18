pub mod actions;
pub mod ax;
pub mod dock;
pub mod ffi;
pub mod log;
pub mod mt;
pub mod snapper;
pub mod sys;
pub mod tap;

use crate::snap::R;
use ax::Elem;
use std::sync::Mutex;

/// Frames windows had before MacControl moved them (for "restore" and snap-back).
static SAVED: Mutex<Vec<(Elem, R)>> = Mutex::new(Vec::new());

pub fn remember(win: &Elem, frame: R) {
    let mut s = SAVED.lock().unwrap();
    if let Some(e) = s.iter_mut().find(|(w, _)| w == win) {
        // Keep the first size: snapping twice should still restore the original.
        let _ = e;
        return;
    }
    s.push((win.clone(), frame));
    if s.len() > 64 {
        s.remove(0);
    }
}

pub fn take_saved(win: &Elem) -> Option<R> {
    let mut s = SAVED.lock().unwrap();
    let i = s.iter().position(|(w, _)| w == win)?;
    Some(s.remove(i).1)
}

/// Put a window at `target`; if it keeps a different size (more than 8 pt off: it has a
/// minimum size) move it again so it lines up with the side it was sent to.
pub fn place_frame(win: &Elem, target: R, pos: crate::model::WindowPos) {
    use crate::model::WindowPos::*;
    win.set_frame(target);
    let Some(got) = win.frame() else { return };
    let tol = crate::mechanics::SIZE_TOLERANCE;
    if (got.w - target.w).abs() <= tol && (got.h - target.h).abs() <= tol {
        return;
    }
    let right = matches!(pos, RightHalf | TopRight | BottomRight | RightThird | RightTwoThirds);
    let bottom = matches!(pos, BottomHalf | BottomLeft | BottomRight);
    let center = matches!(pos, Center | CenterThird);
    let mut x = target.x;
    let mut y = target.y;
    if right {
        x = target.right() - got.w;
    } else if center {
        x = target.x + (target.w - got.w) / 2.0;
    }
    if bottom {
        y = target.bottom() - got.h;
    }
    win.set_frame(R::new(x, y, got.w, got.h));
}
