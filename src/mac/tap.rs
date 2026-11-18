#![allow(non_upper_case_globals)]
//! Listens to every key press, mouse button and scroll in the system (an event tap),
//! runs the matching trigger and swallows the original input when it did.

use super::ffi::*;
use crate::engine::{engine, Recorded, Recording};
use crate::model::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicPtr, AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

static PORT: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
/// Keycode whose key-up must be swallowed (+1, 0 = none).
static SWALLOW_KEY_UP: AtomicI64 = AtomicI64::new(0);
/// Bit per mouse button whose button-up must be swallowed.
static SWALLOW_BUTTONS: AtomicU32 = AtomicU32::new(0);
static SWALLOW_LEFT_UP: AtomicBool = AtomicBool::new(false);
static LAST_SCROLL: Mutex<Option<Instant>> = Mutex::new(None);

/// Keys that macOS always reports with the fn flag set.
fn strip_fn(key: u16, flags: u64) -> u64 {
    let special = matches!(key, 96..=126 | 51 | 53 | 36 | 48 | 76);
    if special {
        flags & !MOD_FN
    } else {
        flags
    }
}

extern "C" fn callback(_proxy: CGEventTapProxy, etype: u32, event: CGEventRef, _user: *mut c_void) -> CGEventRef {
    if etype == kCGEventTapDisabledByTimeout || etype == kCGEventTapDisabledByUserInput {
        let p = PORT.load(Ordering::SeqCst);
        if !p.is_null() {
            unsafe { CGEventTapEnable(p, true) };
        }
        return event;
    }
    unsafe {
        if CGEventGetIntegerValueField(event, kCGEventSourceUserData) == OWN_EVENT_MARK {
            return event;
        }
    }
    let swallow = std::panic::catch_unwind(|| handle(etype, event)).unwrap_or(false);
    if swallow {
        std::ptr::null_mut()
    } else {
        event
    }
}

fn handle(etype: u32, event: CGEventRef) -> bool {
    let eng = engine();
    let flags = unsafe { CGEventGetFlags(event) } & MOD_MASK;
    match etype {
        kCGEventKeyDown => {
            let key = unsafe { CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode) } as u16;
            let repeat = unsafe { CGEventGetIntegerValueField(event, kCGKeyboardEventAutorepeat) } != 0;
            let mods = strip_fn(key, flags);
            if *eng.recording.lock().unwrap() == Recording::Key {
                if !repeat {
                    *eng.recorded.lock().unwrap() = Some(Recorded::Key(KeyCombo { key, mods }));
                    *eng.recording.lock().unwrap() = Recording::Off;
                    eng.repaint();
                }
                SWALLOW_KEY_UP.store(key as i64 + 1, Ordering::SeqCst);
                return true;
            }
            let combo = KeyCombo { key, mods };
            let kind = TriggerKind::Keyboard(combo);
            if let Some(a) = eng.lookup(&kind) {
                if !repeat {
                    super::log::write(format!("key {} in {} -> {}", combo.label(), eng.app_id(), a.label()));
                    eng.note(combo.label(), Some(&a));
                    eng.run_action(a);
                }
                SWALLOW_KEY_UP.store(key as i64 + 1, Ordering::SeqCst);
                return true;
            }
            false
        }
        kCGEventKeyUp => {
            let key = unsafe { CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode) };
            SWALLOW_KEY_UP.compare_exchange(key + 1, 0, Ordering::SeqCst, Ordering::SeqCst).is_ok()
        }
        kCGEventLeftMouseDown => {
            let p = unsafe { CGEventGetLocation(event) };
            super::snapper::send(super::snapper::Msg::Down(p.x as f32, p.y as f32));
            // Clicking the trackpad with 3 or more fingers.
            let n = eng.finger_count.load(Ordering::SeqCst);
            if n >= 3 {
                let g = Gesture::Click(n.min(5) as u8);
                if let Some(a) = eng.lookup(&TriggerKind::Trackpad(g)) {
                    super::mt::consume_current();
                    eng.note(g.label(), Some(&a));
                    eng.run_action(a);
                    SWALLOW_LEFT_UP.store(true, Ordering::SeqCst);
                    return true;
                }
            }
            false
        }
        kCGEventLeftMouseDragged => {
            let p = unsafe { CGEventGetLocation(event) };
            super::snapper::send(super::snapper::Msg::Drag(p.x as f32, p.y as f32));
            false
        }
        kCGEventLeftMouseUp => {
            let p = unsafe { CGEventGetLocation(event) };
            super::snapper::send(super::snapper::Msg::Up(p.x as f32, p.y as f32));
            SWALLOW_LEFT_UP.swap(false, Ordering::SeqCst)
        }
        kCGEventOtherMouseDown => {
            let button = unsafe { CGEventGetIntegerValueField(event, kCGMouseEventButtonNumber) } as u8;
            let mods = flags & !MOD_FN;
            if *eng.recording.lock().unwrap() == Recording::Mouse {
                *eng.recorded.lock().unwrap() = Some(Recorded::Mouse(MouseTrigger::Button { button, mods }));
                *eng.recording.lock().unwrap() = Recording::Off;
                eng.repaint();
                SWALLOW_BUTTONS.fetch_or(1 << (button.min(31)), Ordering::SeqCst);
                return true;
            }
            let t = MouseTrigger::Button { button, mods };
            if let Some(a) = eng.lookup(&TriggerKind::Mouse(t)) {
                eng.note(t.label(), Some(&a));
                eng.run_action(a);
                SWALLOW_BUTTONS.fetch_or(1 << (button.min(31)), Ordering::SeqCst);
                return true;
            }
            false
        }
        kCGEventOtherMouseUp => {
            let button = unsafe { CGEventGetIntegerValueField(event, kCGMouseEventButtonNumber) } as u32;
            let bit = 1u32 << button.min(31);
            SWALLOW_BUTTONS.fetch_and(!bit, Ordering::SeqCst) & bit != 0
        }
        kCGEventScrollWheel => {
            let mods = flags & !MOD_FN;
            if mods == 0 {
                return false;
            }
            let delta = unsafe { CGEventGetIntegerValueField(event, kCGScrollWheelEventDeltaAxis1) };
            if delta == 0 {
                return false;
            }
            let dir = if delta > 0 { Wheel::Up } else { Wheel::Down };
            let t = MouseTrigger::Scroll { dir, mods };
            if let Some(a) = eng.lookup(&TriggerKind::Mouse(t)) {
                let mut last = LAST_SCROLL.lock().unwrap();
                let now = Instant::now();
                if last.map(|l| now - l > Duration::from_millis(120)).unwrap_or(true) {
                    *last = Some(now);
                    eng.note(t.label(), Some(&a));
                    eng.run_action(a);
                }
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn start() {
    std::thread::Builder::new()
        .name("event-tap".into())
        .spawn(|| loop {
            if !super::ax::trusted() {
                std::thread::sleep(Duration::from_secs(1));
                continue;
            }
            let events = mask(kCGEventKeyDown)
                | mask(kCGEventKeyUp)
                | mask(kCGEventLeftMouseDown)
                | mask(kCGEventLeftMouseUp)
                | mask(kCGEventLeftMouseDragged)
                | mask(kCGEventOtherMouseDown)
                | mask(kCGEventOtherMouseUp)
                | mask(kCGEventScrollWheel);
            // Session tap, inserted at the head, able to change events.
            let port = unsafe { CGEventTapCreate(1, 0, 0, events, callback, std::ptr::null_mut()) };
            if port.is_null() {
                std::thread::sleep(Duration::from_secs(2));
                continue;
            }
            PORT.store(port, Ordering::SeqCst);
            unsafe {
                let src = CFMachPortCreateRunLoopSource(std::ptr::null(), port, 0);
                CFRunLoopAddSource(CFRunLoopGetCurrent(), src, kCFRunLoopCommonModes);
                CGEventTapEnable(port, true);
            }
            engine().tap_ok.store(true, Ordering::SeqCst);
            engine().repaint();
            unsafe { CFRunLoopRun() };
            engine().tap_ok.store(false, Ordering::SeqCst);
        })
        .ok();
}
