//! Reads raw finger positions from built-in and Magic trackpads (MultitouchSupport,
//! loaded at run time) and feeds them to the gesture recognizer.

use crate::engine::engine;
use crate::gestures::{Recognizer, Touch};
use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::sync::atomic::Ordering;
use std::sync::Mutex;

#[repr(C)]
#[derive(Clone, Copy)]
struct MtPoint {
    x: f32,
    y: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MtReadout {
    pos: MtPoint,
    vel: MtPoint,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MtTouch {
    frame: i32,
    timestamp: f64,
    identifier: i32,
    state: i32,
    finger_id: i32,
    hand_id: i32,
    normalized: MtReadout,
    size: f32,
    zero1: i32,
    angle: f32,
    major_axis: f32,
    minor_axis: f32,
    mm: MtReadout,
    zero2: [i32; 2],
    unk2: f32,
}

// MultitouchSupport hands over touches 0x60 bytes apart, with the normalised position at
// offset 0x20; these checks keep the struct in step with that layout.
const _: () = assert!(std::mem::size_of::<MtTouch>() == 0x60);
const _: () = assert!(std::mem::offset_of!(MtTouch, normalized) == 0x20);

type Callback = extern "C" fn(*mut c_void, *const MtTouch, i32, f64, i32) -> i32;
type CreateList = unsafe extern "C" fn() -> *const c_void;
type Register = unsafe extern "C" fn(*const c_void, Callback);
type StartDev = unsafe extern "C" fn(*const c_void, i32) -> i32;
type FamilyId = unsafe extern "C" fn(*const c_void, *mut i32) -> i32;

static RECOGNIZERS: Mutex<Option<HashMap<usize, Recognizer>>> = Mutex::new(None);
static LAST_DEVICE: Mutex<usize> = Mutex::new(0);

/// The touch going on now was used for a click: don't also report it as a tap.
pub fn consume_current() {
    let dev = *LAST_DEVICE.lock().unwrap();
    if let Some(map) = RECOGNIZERS.lock().unwrap().as_mut() {
        if let Some(r) = map.get_mut(&dev) {
            r.mark_consumed();
        }
    }
}

extern "C" fn on_frame(device: *mut c_void, touches: *const MtTouch, n: i32, ts: f64, _frame: i32) -> i32 {
    let _ = std::panic::catch_unwind(|| {
        let list: &[MtTouch] = if touches.is_null() || n <= 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(touches, n as usize) }
        };
        // 3 = finger landing, 4 = touching.
        let down: Vec<Touch> = list
            .iter()
            .filter(|t| t.state == 3 || t.state == 4)
            .map(|t| Touch { id: t.identifier, x: t.normalized.pos.x, y: t.normalized.pos.y })
            .collect();
        let eng = engine();
        eng.finger_count.store(down.len(), Ordering::SeqCst);
        if let Ok(mut v) = eng.touches.lock() {
            *v = down.iter().map(|t| (t.x, t.y)).collect();
        }
        *LAST_DEVICE.lock().unwrap() = device as usize;
        let gestures = {
            let mut g = RECOGNIZERS.lock().unwrap();
            let map = g.get_or_insert_with(HashMap::new);
            map.entry(device as usize).or_default().feed(&down, ts)
        };
        for g in gestures {
            eng.gesture(g);
        }
        if eng.ctx.get().is_some() {
            eng.repaint();
        }
    });
    0
}

pub fn start() {
    unsafe {
        let path = CString::new("/System/Library/PrivateFrameworks/MultitouchSupport.framework/MultitouchSupport").unwrap();
        let lib = libc::dlopen(path.as_ptr(), libc::RTLD_NOW);
        if lib.is_null() {
            return;
        }
        let sym = |name: &str| {
            let c = CString::new(name).unwrap();
            libc::dlsym(lib, c.as_ptr())
        };
        let (create, register, startd) = (sym("MTDeviceCreateList"), sym("MTRegisterContactFrameCallback"), sym("MTDeviceStart"));
        if create.is_null() || register.is_null() || startd.is_null() {
            return;
        }
        let family = sym("MTDeviceGetFamilyID");
        let create: CreateList = std::mem::transmute(create);
        let register: Register = std::mem::transmute(register);
        let startd: StartDev = std::mem::transmute(startd);
        let list = create();
        if list.is_null() {
            return;
        }
        let count = super::ffi::CFArrayGetCount(list);
        let mut pads = 0;
        for i in 0..count {
            let dev = super::ffi::CFArrayGetValueAtIndex(list, i);
            if !family.is_null() {
                let f: FamilyId = std::mem::transmute(family);
                let mut id = 0;
                f(dev, &mut id);
                // 112 and 113 are Magic Mice: their surface is not a trackpad.
                if id == 112 || id == 113 {
                    continue;
                }
            }
            register(dev, on_frame);
            startd(dev, 0);
            pads += 1;
        }
        engine().trackpads.store(pads, Ordering::SeqCst);
    }
}
