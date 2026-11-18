//! Moving and resizing other apps' windows through the Accessibility interface.

use super::ffi::*;
use crate::snap::R;
use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::string::CFString;
use std::ffi::c_void;

pub struct Elem(AXUIElementRef);

unsafe impl Send for Elem {}
unsafe impl Sync for Elem {}

impl Drop for Elem {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { CFRelease(self.0) }
        }
    }
}

impl Clone for Elem {
    fn clone(&self) -> Self {
        unsafe { CFRetain(self.0) };
        Elem(self.0)
    }
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CFEqual(self.0, other.0) != 0 }
    }
}

fn attr(el: AXUIElementRef, name: &str) -> Option<CFTypeRef> {
    let key = CFString::new(name);
    let mut out: CFTypeRef = std::ptr::null();
    let err = unsafe { AXUIElementCopyAttributeValue(el, key.as_concrete_TypeRef() as CFStringRef, &mut out) };
    if err == 0 && !out.is_null() {
        Some(out)
    } else {
        None
    }
}

fn string_attr(el: AXUIElementRef, name: &str) -> Option<String> {
    let v = attr(el, name)?;
    unsafe {
        if CFGetTypeID(v) != CFStringGetTypeID() {
            CFRelease(v);
            return None;
        }
        let s = CFString::wrap_under_create_rule(v as _);
        Some(s.to_string())
    }
}

pub fn trusted() -> bool {
    unsafe { AXIsProcessTrusted() }
}

/// Ask macOS to show its "allow accessibility" prompt.
pub fn request_trust() -> bool {
    use core_foundation::dictionary::CFDictionary;
    unsafe {
        let key = CFString::wrap_under_get_rule(kAXTrustedCheckOptionPrompt as _);
        let dict = CFDictionary::from_CFType_pairs(&[(key.as_CFType(), CFBoolean::true_value().as_CFType())]);
        AXIsProcessTrustedWithOptions(dict.as_concrete_TypeRef() as CFTypeRef)
    }
}

impl Elem {
    pub fn system() -> Elem {
        Elem(unsafe { AXUIElementCreateSystemWide() })
    }

    fn child(&self, name: &str) -> Option<Elem> {
        attr(self.0, name).map(Elem)
    }

    #[allow(dead_code)]
    pub fn role(&self) -> String {
        string_attr(self.0, "AXRole").unwrap_or_default()
    }

    pub fn subrole(&self) -> String {
        string_attr(self.0, "AXSubrole").unwrap_or_default()
    }

    #[allow(dead_code)]
    pub fn pid(&self) -> i32 {
        let mut p = 0;
        unsafe { AXUIElementGetPid(self.0, &mut p) };
        p
    }

    pub fn frame(&self) -> Option<R> {
        let pos = attr(self.0, "AXPosition")?;
        let size = match attr(self.0, "AXSize") {
            Some(s) => s,
            None => {
                unsafe { CFRelease(pos) };
                return None;
            }
        };
        let mut p = CGPoint::default();
        let mut s = CGSize::default();
        unsafe {
            AXValueGetValue(pos, kAXValueCGPointType, &mut p as *mut _ as *mut c_void);
            AXValueGetValue(size, kAXValueCGSizeType, &mut s as *mut _ as *mut c_void);
            CFRelease(pos);
            CFRelease(size);
        }
        Some(R::new(p.x as f32, p.y as f32, s.width as f32, s.height as f32))
    }

    /// Returns the Accessibility error code (0 = done).
    fn set_point(&self, x: f32, y: f32) -> i32 {
        let p = CGPoint { x: x as f64, y: y as f64 };
        let key = CFString::new("AXPosition");
        unsafe {
            let v = AXValueCreate(kAXValueCGPointType, &p as *const _ as *const c_void);
            let err = AXUIElementSetAttributeValue(self.0, key.as_concrete_TypeRef() as CFStringRef, v);
            CFRelease(v);
            err
        }
    }

    /// Returns the Accessibility error code (0 = done).
    fn set_size(&self, w: f32, h: f32) -> i32 {
        let s = CGSize { width: w as f64, height: h as f64 };
        let key = CFString::new("AXSize");
        unsafe {
            let v = AXValueCreate(kAXValueCGSizeType, &s as *const _ as *const c_void);
            let err = AXUIElementSetAttributeValue(self.0, key.as_concrete_TypeRef() as CFStringRef, v);
            CFRelease(v);
            err
        }
    }

    /// The application this element belongs to.
    pub fn app(&self) -> Elem {
        Elem(unsafe { AXUIElementCreateApplication(self.pid()) })
    }

    /// Move then resize then move again: apps that refuse a size near a screen edge
    /// accept it once the window has moved.
    ///
    /// Chromium and Electron apps (Chrome, VS Code, Slack…) switch on
    /// `AXEnhancedUserInterface` when they see an accessibility client, and while it is on
    /// they animate or undo outside changes to their windows. It is switched off for the move
    /// and back on afterwards.
    pub fn set_frame(&self, r: R) {
        let app = self.app();
        short_timeout(&app);
        let enhanced = app.get_bool("AXEnhancedUserInterface");
        if enhanced {
            app.set_bool("AXEnhancedUserInterface", false);
        }
        let before = self.frame();
        let e1 = self.set_point(r.x, r.y);
        let e2 = self.set_size(r.w, r.h);
        let e3 = self.set_point(r.x, r.y);
        if enhanced {
            app.set_bool("AXEnhancedUserInterface", true);
        }
        super::log::write(format!(
            "move window: pid {} enhancedUI {} from {:?} to {:?} -> errors {}/{}/{} now {:?}",
            self.pid(), enhanced, before.map(fmt_r), fmt_r(r), e1, e2, e3, self.frame().map(fmt_r)
        ));
    }

    #[allow(dead_code)]
    pub fn set_size_only(&self, w: f32, h: f32) {
        self.set_size(w, h);
    }

    fn set_bool(&self, name: &str, v: bool) {
        let key = CFString::new(name);
        let b = if v { CFBoolean::true_value() } else { CFBoolean::false_value() };
        unsafe {
            AXUIElementSetAttributeValue(self.0, key.as_concrete_TypeRef() as CFStringRef, b.as_concrete_TypeRef() as CFTypeRef);
        }
    }

    fn get_bool(&self, name: &str) -> bool {
        match attr(self.0, name) {
            Some(v) => unsafe {
                if CFGetTypeID(v) != CFBooleanGetTypeID() {
                    CFRelease(v);
                    return false;
                }
                let b = CFBoolean::wrap_under_create_rule(v as _);
                b.into()
            },
            None => false,
        }
    }

    pub fn minimize(&self) {
        self.set_bool("AXMinimized", true);
    }

    pub fn toggle_fullscreen(&self) {
        let now = self.get_bool("AXFullScreen");
        self.set_bool("AXFullScreen", !now);
    }

    pub fn close(&self) {
        if let Some(b) = self.child("AXCloseButton") {
            let key = CFString::new("AXPress");
            unsafe { AXUIElementPerformAction(b.0, key.as_concrete_TypeRef() as CFStringRef) };
        }
    }

    pub fn raise(&self) {
        let key = CFString::new("AXRaise");
        unsafe { AXUIElementPerformAction(self.0, key.as_concrete_TypeRef() as CFStringRef) };
    }
}

fn fmt_r(r: R) -> String {
    format!("({:.0},{:.0} {:.0}x{:.0})", r.x, r.y, r.w, r.h)
}

fn own_pid() -> i32 {
    std::process::id() as i32
}

/// Accessibility questions about MacControl itself would be answered inside this very
/// process, on the calling thread, by its window toolkit, which only allows that on the
/// main thread (asking from a background thread aborted the app). So MacControl never
/// asks about its own windows.
fn is_self(el: &Elem) -> bool {
    el.pid() == own_pid()
}

/// Don't let an app that isn't responding hold a background thread for long.
fn short_timeout(el: &Elem) {
    unsafe { AXUIElementSetMessagingTimeout(el.0, 0.5) };
}

/// The window that has keyboard focus (never one of MacControl's own).
///
/// The frontmost app comes from AppKit and is asked directly. Asking the system-wide element
/// for its "focused application" instead fails with Chromium and Electron apps (VS Code,
/// Slack…), which don't answer that question.
pub fn focused_window() -> Option<Elem> {
    let app = match super::sys::frontmost_pid() {
        Some(pid) => Elem(unsafe { AXUIElementCreateApplication(pid) }),
        None => {
            let sys = Elem::system();
            short_timeout(&sys);
            let Some(app) = sys.child("AXFocusedApplication") else {
                super::log::write("focused window: no frontmost or focused application");
                return None;
            };
            app
        }
    };
    if is_self(&app) {
        return None;
    }
    short_timeout(&app);
    let win = app
        .child("AXFocusedWindow")
        .or_else(|| app.child("AXMainWindow"))
        .or_else(|| first_window(&app));
    if win.is_none() {
        super::log::write(format!("focused window: pid {} has no focused, main or listed window", app.pid()));
    }
    win
}

/// The first standard window in the app's window list (front to back).
fn first_window(app: &Elem) -> Option<Elem> {
    let wins = attr(app.0, "AXWindows")?;
    let n = unsafe { CFArrayGetCount(wins) };
    let mut found = None;
    for i in 0..n {
        let w = unsafe { CFArrayGetValueAtIndex(wins, i) };
        if w.is_null() {
            continue;
        }
        unsafe { CFRetain(w) };
        let el = Elem(w);
        if el.subrole() == "AXStandardWindow" {
            found = Some(el);
            break;
        }
    }
    unsafe { CFRelease(wins) };
    found
}

/// The window under a screen point, found from the window server's list (front to back)
/// and then matched among its app's accessibility windows. MacControl's own windows are
/// skipped without asking the accessibility system about them.
pub fn window_at(x: f32, y: f32) -> Option<Elem> {
    use core_foundation::array::CFArray;
    use core_foundation::base::{CFType, TCFType};
    use core_foundation::dictionary::CFDictionary;
    use core_foundation::number::CFNumber;

    let list = unsafe { CGWindowListCopyWindowInfo(1 | 16, 0) }; // on screen only, no desktop
    if list.is_null() {
        return None;
    }
    let list: CFArray<CFDictionary<CFString, CFType>> = unsafe { CFArray::wrap_under_create_rule(list as _) };
    let num = |d: &CFDictionary<CFString, CFType>, k: &str| -> Option<i64> {
        d.find(CFString::new(k)).and_then(|v| v.downcast::<CFNumber>()).and_then(|n| n.to_i64())
    };
    for d in list.iter() {
        if num(&d, "kCGWindowLayer") != Some(0) {
            continue;
        }
        let Some(bounds) = d.find(CFString::new("kCGWindowBounds")) else { continue };
        let mut r = CGRectRaw::default();
        if unsafe { CGRectMakeWithDictionaryRepresentation(bounds.as_CFTypeRef(), &mut r) } == 0 {
            continue;
        }
        let (bx, by, bw, bh) = (r.x as f32, r.y as f32, r.w as f32, r.h as f32);
        if !(x >= bx && x < bx + bw && y >= by && y < by + bh) {
            continue;
        }
        // The topmost normal window under the point.
        let pid = num(&d, "kCGWindowOwnerPID")? as i32;
        if pid == own_pid() {
            return None;
        }
        let app = Elem(unsafe { AXUIElementCreateApplication(pid) });
        short_timeout(&app);
        let wins = attr(app.0, "AXWindows")?;
        let n = unsafe { CFArrayGetCount(wins) };
        let mut best: Option<(f32, Elem)> = None;
        for i in 0..n {
            let w = unsafe { CFArrayGetValueAtIndex(wins, i) };
            if w.is_null() {
                continue;
            }
            unsafe { CFRetain(w) };
            let el = Elem(w);
            let Some(f) = el.frame() else { continue };
            let err = (f.x - bx).abs() + (f.y - by).abs() + (f.w - bw).abs() + (f.h - bh).abs();
            if best.as_ref().map(|(e, _)| err < *e).unwrap_or(true) {
                best = Some((err, el));
            }
        }
        unsafe { CFRelease(wins) };
        return best.filter(|(e, _)| *e < 8.0).map(|(_, el)| el);
    }
    None
}

#[cfg(test)]
mod tests {
    /// Looking up windows from a background thread must never bring the process down.
    #[test]
    fn lookups_from_a_background_thread_are_safe() {
        let h = std::thread::spawn(|| {
            for (x, y) in [(10.0, 10.0), (400.0, 300.0), (800.0, 600.0), (-50.0, -50.0)] {
                let _ = super::window_at(x, y);
            }
            let _ = super::focused_window();
        });
        h.join().unwrap();
    }
}
