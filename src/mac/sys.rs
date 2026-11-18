//! Screens, the frontmost app and the list of running apps (AppKit through objc2).

use crate::snap::{Screen, R};
use objc2::rc::autoreleasepool;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};
use objc2_foundation::NSRect;
use std::ffi::{c_char, CStr};

unsafe fn ns_string(s: *mut AnyObject) -> String {
    if s.is_null() {
        return String::new();
    }
    unsafe {
        let c: *const c_char = msg_send![s, UTF8String];
        if c.is_null() {
            String::new()
        } else {
            CStr::from_ptr(c).to_string_lossy().into_owned()
        }
    }
}

/// Every display, in global top-left coordinates.
pub fn screens() -> Vec<Screen> {
    autoreleasepool(|_| unsafe {
        let arr: *mut AnyObject = msg_send![class!(NSScreen), screens];
        if arr.is_null() {
            return vec![];
        }
        let n: usize = msg_send![arr, count];
        let mut out = vec![];
        let mut main_h = 0.0;
        for i in 0..n {
            let s: *mut AnyObject = msg_send![arr, objectAtIndex: i];
            let f: NSRect = msg_send![s, frame];
            let v: NSRect = msg_send![s, visibleFrame];
            if i == 0 {
                main_h = f.size.height;
            }
            // AppKit counts y upwards from the bottom of the main display.
            let flip = |r: NSRect| {
                R::new(
                    r.origin.x as f32,
                    (main_h - r.origin.y - r.size.height) as f32,
                    r.size.width as f32,
                    r.size.height as f32,
                )
            };
            out.push(Screen { frame: flip(f), visible: flip(v) });
        }
        out
    })
}

/// Bundle id and name of the frontmost app.
pub fn frontmost_app() -> (String, String) {
    autoreleasepool(|_| unsafe {
        let ws: *mut AnyObject = msg_send![class!(NSWorkspace), sharedWorkspace];
        let app: *mut AnyObject = msg_send![ws, frontmostApplication];
        if app.is_null() {
            return (String::new(), String::new());
        }
        let b: *mut AnyObject = msg_send![app, bundleIdentifier];
        let n: *mut AnyObject = msg_send![app, localizedName];
        (ns_string(b), ns_string(n))
    })
}

/// Apps with a Dock icon that are running now: (bundle id, name).
pub fn running_apps() -> Vec<(String, String)> {
    autoreleasepool(|_| unsafe {
        let ws: *mut AnyObject = msg_send![class!(NSWorkspace), sharedWorkspace];
        let arr: *mut AnyObject = msg_send![ws, runningApplications];
        let n: usize = msg_send![arr, count];
        let mut out = vec![];
        for i in 0..n {
            let app: *mut AnyObject = msg_send![arr, objectAtIndex: i];
            let policy: isize = msg_send![app, activationPolicy];
            if policy != 0 {
                continue;
            }
            let b: *mut AnyObject = msg_send![app, bundleIdentifier];
            let nm: *mut AnyObject = msg_send![app, localizedName];
            let (b, nm) = (ns_string(b), ns_string(nm));
            if !b.is_empty() {
                out.push((b, nm));
            }
        }
        out.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
        out.dedup();
        out
    })
}

/// The system's double-click interval in seconds (`+[NSEvent doubleClickInterval]`).
pub fn double_click_interval() -> f64 {
    let v: f64 = unsafe { msg_send![class!(NSEvent), doubleClickInterval] };
    if v >= crate::mechanics::DOUBLE_TAP_MIN && v < 5.0 {
        v
    } else {
        crate::mechanics::DOUBLE_TAP_FALLBACK
    }
}
