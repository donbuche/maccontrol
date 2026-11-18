//! Carries out the actions a trigger is set to.

use super::ax::{self, Elem};
use super::ffi::*;
use crate::engine::engine;
use crate::model::*;
use crate::snap::{screen_at, target_rect, R};
use std::process::Command;
use std::time::Duration;

unsafe fn post(ev: CGEventRef) {
    if ev.is_null() {
        return;
    }
    unsafe {
        CGEventSetIntegerValueField(ev, kCGEventSourceUserData, OWN_EVENT_MARK);
        CGEventPost(0, ev);
        CFRelease(ev as CFTypeRef);
    }
}

pub fn press_keys(k: KeyCombo) {
    unsafe {
        let down = CGEventCreateKeyboardEvent(std::ptr::null(), k.key, true);
        CGEventSetFlags(down, k.mods);
        post(down);
        let up = CGEventCreateKeyboardEvent(std::ptr::null(), k.key, false);
        CGEventSetFlags(up, k.mods);
        post(up);
    }
}

fn type_text(text: &str) {
    for ch in text.encode_utf16().collect::<Vec<u16>>().chunks(16) {
        unsafe {
            let down = CGEventCreateKeyboardEvent(std::ptr::null(), 0, true);
            CGEventKeyboardSetUnicodeString(down, ch.len(), ch.as_ptr());
            post(down);
            let up = CGEventCreateKeyboardEvent(std::ptr::null(), 0, false);
            CGEventKeyboardSetUnicodeString(up, ch.len(), ch.as_ptr());
            post(up);
        }
        std::thread::sleep(Duration::from_millis(4));
    }
}

fn click(button: u32) {
    unsafe {
        let here = CGEventCreate(std::ptr::null());
        let p = CGEventGetLocation(here);
        CFRelease(here as CFTypeRef);
        let (d, u) = match button {
            0 => (kCGEventLeftMouseDown, kCGEventLeftMouseUp),
            1 => (kCGEventRightMouseDown, kCGEventRightMouseUp),
            _ => (kCGEventOtherMouseDown, kCGEventOtherMouseUp),
        };
        for t in [d, u] {
            let ev = CGEventCreateMouseEvent(std::ptr::null(), t, p, button);
            if button >= 2 {
                CGEventSetIntegerValueField(ev, kCGMouseEventButtonNumber, button as i64);
            }
            post(ev);
        }
    }
}

fn open(args: &[&str]) {
    let _ = Command::new("/usr/bin/open").args(args).spawn();
}

fn osascript(script: &str) {
    let _ = Command::new("/usr/bin/osascript").arg("-e").arg(script).spawn();
}

fn place(win: &Elem, pos: WindowPos) {
    let Some(f) = win.frame() else { return };
    let eng = engine();
    let screens = eng.screens.read().unwrap().clone();
    let (cx, cy) = f.center();
    let Some(scr) = screen_at(&screens, cx, cy) else { return };
    let settings = eng.config.read().unwrap().snap.clone();
    super::remember(win, f);
    super::place_frame(win, target_rect(pos, scr.visible, &settings, Some(f)), pos);
}

fn next_screen(win: &Elem) {
    let Some(f) = win.frame() else { return };
    let screens = engine().screens.read().unwrap().clone();
    if screens.len() < 2 {
        return;
    }
    let (cx, cy) = f.center();
    let i = screens.iter().position(|s| s.frame.contains(cx, cy)).unwrap_or(0);
    let from = screens[i].visible;
    let to = screens[(i + 1) % screens.len()].visible;
    let fx = (f.x - from.x) / from.w;
    let fy = (f.y - from.y) / from.h;
    let w = (f.w / from.w * to.w).min(to.w);
    let h = (f.h / from.h * to.h).min(to.h);
    win.set_frame(R::new(to.x + fx * to.w, to.y + fy * to.h, w, h));
}

pub fn perform(a: &Action) {
    match a {
        Action::None => {}
        Action::Shortcut(k) => press_keys(*k),
        Action::TypeText(t) => type_text(t),
        Action::Window(pos) => {
            if let Some(w) = ax::focused_window() {
                place(&w, *pos);
            }
        }
        Action::OpenApp(name) if !name.trim().is_empty() => {
            let n = name.trim();
            if n.ends_with(".app") || n.starts_with('/') {
                open(&[n]);
            } else {
                open(&["-a", n]);
            }
        }
        Action::OpenUrl(u) if !u.trim().is_empty() => {
            let u = u.trim();
            let full = if u.contains("://") || u.starts_with('/') || u.starts_with('~') {
                u.to_string()
            } else {
                format!("https://{u}")
            };
            open(&[&full]);
        }
        Action::Shell(cmd) if !cmd.trim().is_empty() => {
            let _ = Command::new("/bin/sh").arg("-c").arg(cmd).spawn();
        }
        Action::System(s) => system(*s),
        _ => {}
    }
}

fn system(s: SystemAction) {
    use SystemAction::*;
    match s {
        MissionControl => open(&["-a", "Mission Control"]),
        ShowDesktop => open(&["-a", "Mission Control", "--args", "1"]),
        AppExpose => open(&["-a", "Mission Control", "--args", "2"]),
        Launchpad => open(&["-a", "Launchpad"]),
        LockScreen => press_keys(KeyCombo { key: 12, mods: MOD_CTRL | MOD_CMD }),
        Screenshot => press_keys(KeyCombo { key: 21, mods: MOD_SHIFT | MOD_CMD }),
        VolumeUp => osascript("set volume output volume ((output volume of (get volume settings)) + 6)"),
        VolumeDown => osascript("set volume output volume ((output volume of (get volume settings)) - 6)"),
        Mute => osascript("set volume output muted (not (output muted of (get volume settings)))"),
        MinimizeWindow => {
            if let Some(w) = ax::focused_window() {
                w.minimize()
            }
        }
        MinimizeWindowUnderCursor => {
            let p = unsafe {
                let here = CGEventCreate(std::ptr::null());
                let p = CGEventGetLocation(here);
                CFRelease(here as CFTypeRef);
                p
            };
            if let Some(w) = ax::window_at(p.x as f32, p.y as f32) {
                w.minimize()
            }
        }
        CloseWindow => {
            if let Some(w) = ax::focused_window() {
                w.close()
            }
        }
        FullScreen => {
            if let Some(w) = ax::focused_window() {
                w.toggle_fullscreen()
            }
        }
        RestoreWindow => {
            if let Some(w) = ax::focused_window() {
                if let Some(old) = super::take_saved(&w) {
                    w.set_frame(old);
                }
            }
        }
        NextScreen => {
            if let Some(w) = ax::focused_window() {
                next_screen(&w);
                w.raise();
            }
        }
        LeftClick => click(0),
        RightClick => click(1),
        MiddleClick => click(2),
    }
}
