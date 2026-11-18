//! Window snapping: watches a window being dragged, shows where it will land when the
//! cursor reaches a screen edge or corner, and places it there on release.

use super::ax::{self, Elem};
use crate::engine::engine;
use crate::mechanics as m;
use crate::model::WindowPos;
use crate::snap::{screen_at, target_rect, zone_at, R};
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[allow(dead_code)]
pub enum Msg {
    Down(f32, f32),
    Drag(f32, f32),
    Up(f32, f32),
}

static TX: Mutex<Option<Sender<Msg>>> = Mutex::new(None);

pub fn send(m: Msg) {
    if let Ok(tx) = TX.lock() {
        if let Some(tx) = tx.as_ref() {
            let _ = tx.send(m);
        }
    }
}

struct Drag {
    win: Elem,
    start: R,
    moving: bool,
    checked: Instant,
    zone: Option<WindowPos>,
    zone_since: Instant,
    target: Option<R>,
}

fn set_preview(r: Option<R>) {
    let eng = engine();
    let mut p = eng.preview.lock().unwrap();
    if *p != r {
        *p = r;
        drop(p);
        eng.repaint();
    }
}

pub fn start() {
    let (tx, rx) = channel::<Msg>();
    *TX.lock().unwrap() = Some(tx);
    std::thread::Builder::new()
        .name("snapper".into())
        .spawn(move || {
            let mut drag: Option<Drag> = None;
            while let Ok(msg) = rx.recv() {
                let eng = engine();
                let settings = eng.config.read().unwrap().snap.clone();
                match msg {
                    Msg::Down(x, y) => {
                        drag = None;
                        if !settings.enabled || eng.paused() || settings.excluded_apps.contains(&eng.app_id()) {
                            continue;
                        }
                        if let Some(win) = ax::window_at(x, y) {
                            if win.subrole() != "AXStandardWindow" && !win.subrole().is_empty() {
                                continue;
                            }
                            if let Some(f) = win.frame() {
                                let now = Instant::now();
                                drag = Some(Drag { win, start: f, moving: false, checked: now, zone: None, zone_since: now, target: None });
                            }
                        }
                    }
                    Msg::Drag(x, y) => {
                        let Some(d) = drag.as_mut() else { continue };
                        if !d.moving {
                            if d.checked.elapsed() < Duration::from_millis(25) {
                                continue;
                            }
                            d.checked = Instant::now();
                            let Some(f) = d.win.frame() else { continue };
                            let moved = (f.x - d.start.x).abs().max((f.y - d.start.y).abs());
                            let same_size = (f.w - d.start.w).abs() < 1.0 && (f.h - d.start.h).abs() < 1.0;
                            if !same_size {
                                // It is being resized, not moved.
                                drag = None;
                                continue;
                            }
                            if moved <= m::MOVE_THRESHOLD {
                                continue;
                            }
                            d.moving = true;
                            // Dragging a snapped window away gives it back its old size.
                            if settings.snap_back {
                                if let Some(old) = super::take_saved(&d.win) {
                                    // Placed with x = cursor.x + w / -1.5, y = cursor.y - 8 (title bar
                                    // under the cursor), then kept inside the screen.
                                    let mut nx = x + old.w / m::SNAP_BACK_X_DIVISOR;
                                    let ny = y - m::SNAP_BACK_Y_OFFSET;
                                    let screens = eng.screens.read().unwrap().clone();
                                    if let Some(scr) = screen_at(&screens, x, y) {
                                        let v = scr.visible;
                                        if nx + old.w > v.right() {
                                            nx = v.right() - old.w;
                                        }
                                        if nx < v.x {
                                            nx = v.x;
                                        }
                                    }
                                    let r = R::new(nx, ny, old.w, old.h);
                                    d.win.set_frame(r);
                                    d.start = r;
                                }
                            }
                        }
                        let screens = eng.screens.read().unwrap().clone();
                        let Some(scr) = screen_at(&screens, x, y) else { continue };
                        let zone = zone_at(x, y, &scr, screens.len(), &settings);
                        if zone != d.zone {
                            d.zone = zone;
                            d.zone_since = Instant::now();
                        }
                        let ready = !settings.delay_enabled || d.zone_since.elapsed().as_secs_f32() >= settings.delay;
                        d.target = match (zone, ready) {
                            (Some(z), true) => Some(target_rect(z, scr.visible, &settings, None)),
                            _ => None,
                        };
                        set_preview(if settings.show_preview { d.target } else { None });
                    }
                    Msg::Up(_, _) => {
                        set_preview(None);
                        if let Some(d) = drag.take() {
                            if d.moving {
                                if let Some(t) = d.target {
                                    // Let the app finish its own drag first.
                                    std::thread::sleep(Duration::from_millis(30));
                                    let before = d.win.frame().unwrap_or(d.start);
                                    super::remember(&d.win, R::new(before.x, before.y, d.start.w, d.start.h));
                                    if let Some(z) = d.zone {
                                        super::place_frame(&d.win, t, z);
                                    }
                                    let label = d.zone.map(|z| z.label()).unwrap_or("");
                                    eng.note(crate::i18n::tlf("Window snapped: {}", &[&label]), None);
                                }
                            }
                        }
                    }
                }
            }
        })
        .ok();
}
