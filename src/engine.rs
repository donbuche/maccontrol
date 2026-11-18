//! The part that keeps running in the background: shared state between the window,
//! the input listeners and the action runner.

use crate::model::{Action, Config, Gesture, KeyCombo, MouseTrigger, TriggerKind};
use crate::snap::{Screen, R};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recording {
    Off,
    Key,
    Mouse,
}

#[derive(Clone, Debug)]
pub enum Recorded {
    Key(KeyCombo),
    Mouse(MouseTrigger),
}

#[derive(Clone, Debug)]
pub struct LastEvent {
    pub what: String,
    pub action: Option<String>,
    pub at: Instant,
}

pub struct Engine {
    pub config: RwLock<Config>,
    /// (bundle id, name) of the frontmost app.
    pub front_app: RwLock<(String, String)>,
    pub screens: RwLock<Vec<Screen>>,
    pub finger_count: AtomicUsize,
    pub touches: Mutex<Vec<(f32, f32)>>,
    pub last_event: Mutex<Option<LastEvent>>,
    pub preview: Mutex<Option<R>>,
    pub recording: Mutex<Recording>,
    pub recorded: Mutex<Option<Recorded>>,
    pub tap_ok: AtomicBool,
    pub trackpads: AtomicUsize,
    pub trusted: AtomicBool,
    pub started: AtomicBool,
    pub ctx: OnceLock<eframe::egui::Context>,
    actions: Mutex<Option<Sender<Action>>>,
    gestures: Mutex<Option<Sender<Gesture>>>,
}

static ENGINE: OnceLock<Arc<Engine>> = OnceLock::new();

pub fn engine() -> &'static Arc<Engine> {
    ENGINE.get_or_init(|| {
        Arc::new(Engine {
            config: RwLock::new(Config::default()),
            front_app: RwLock::new((String::new(), String::new())),
            screens: RwLock::new(vec![]),
            finger_count: AtomicUsize::new(0),
            touches: Mutex::new(vec![]),
            last_event: Mutex::new(None),
            preview: Mutex::new(None),
            recording: Mutex::new(Recording::Off),
            recorded: Mutex::new(None),
            tap_ok: AtomicBool::new(false),
            trackpads: AtomicUsize::new(0),
            trusted: AtomicBool::new(false),
            started: AtomicBool::new(false),
            ctx: OnceLock::new(),
            actions: Mutex::new(None),
            gestures: Mutex::new(None),
        })
    })
}

impl Engine {
    pub fn repaint(&self) {
        if let Some(c) = self.ctx.get() {
            c.request_repaint();
        }
    }

    pub fn paused(&self) -> bool {
        self.config.read().map(|c| c.paused).unwrap_or(false)
    }

    pub fn app_id(&self) -> String {
        self.front_app.read().map(|a| a.0.clone()).unwrap_or_default()
    }

    /// Look up the action for a trigger in the frontmost app.
    pub fn lookup(&self, kind: &TriggerKind) -> Option<Action> {
        if self.paused() {
            return None;
        }
        let app = self.app_id();
        let cfg = self.config.read().ok()?;
        cfg.find(kind, &app).map(|t| t.action.clone())
    }

    pub fn note(&self, what: String, action: Option<&Action>) {
        if let Ok(mut l) = self.last_event.lock() {
            *l = Some(LastEvent { what, action: action.map(|a| a.label()), at: Instant::now() });
        }
        self.repaint();
    }

    pub fn run_action(&self, a: Action) {
        if let Ok(tx) = self.actions.lock() {
            if let Some(tx) = tx.as_ref() {
                let _ = tx.send(a);
            }
        }
    }

    pub fn gesture(&self, g: Gesture) {
        if let Ok(tx) = self.gestures.lock() {
            if let Some(tx) = tx.as_ref() {
                let _ = tx.send(g);
            }
        }
    }

    fn fire_gesture(&self, g: Gesture) {
        let a = self.lookup(&TriggerKind::Trackpad(g));
        self.note(g.label(), a.as_ref());
        if let Some(a) = a {
            self.run_action(a);
        }
    }

    /// Start every background listener. Safe to call more than once.
    pub fn start(self: &Arc<Self>) {
        if self.started.swap(true, Ordering::SeqCst) {
            return;
        }
        // Actions run one after another on their own thread.
        let (atx, arx) = channel::<Action>();
        *self.actions.lock().unwrap() = Some(atx);
        std::thread::Builder::new()
            .name("actions".into())
            .spawn(move || {
                while let Ok(a) = arx.recv() {
                    crate::mac::actions::perform(&a);
                }
            })
            .ok();

        // Gestures: hold a tap back briefly when a double tap could still follow.
        let (gtx, grx) = channel::<Gesture>();
        *self.gestures.lock().unwrap() = Some(gtx);
        let me = self.clone();
        std::thread::Builder::new()
            .name("gestures".into())
            .spawn(move || {
                let mut pending: Option<Gesture> = None;
                loop {
                    let g = match pending.take() {
                        Some(g) => g,
                        None => match grx.recv() {
                            Ok(g) => g,
                            Err(_) => return,
                        },
                    };
                    if let Gesture::Tap(n) = g {
                        if me.lookup(&TriggerKind::Trackpad(Gesture::DoubleTap(n))).is_some() {
                            match grx.recv_timeout(Duration::from_secs_f64(crate::mac::sys::double_click_interval())) {
                                Ok(Gesture::Tap(n2)) if n2 == n => {
                                    me.fire_gesture(Gesture::DoubleTap(n));
                                    continue;
                                }
                                Ok(other) => {
                                    me.fire_gesture(g);
                                    pending = Some(other);
                                    continue;
                                }
                                Err(RecvTimeoutError::Timeout) => {}
                                Err(RecvTimeoutError::Disconnected) => return,
                            }
                        }
                    }
                    me.fire_gesture(g);
                }
            })
            .ok();

        // Frontmost app and screen layout, refreshed in the background.
        let me = self.clone();
        std::thread::Builder::new()
            .name("watch".into())
            .spawn(move || {
                let mut n = 0u32;
                loop {
                    let app = crate::mac::sys::frontmost_app();
                    if let Ok(mut a) = me.front_app.write() {
                        if *a != app {
                            *a = app;
                        }
                    }
                    if n % 8 == 0 {
                        let s = crate::mac::sys::screens();
                        if let Ok(mut w) = me.screens.write() {
                            *w = s;
                        }
                        me.trusted.store(crate::mac::ax::trusted(), Ordering::Relaxed);
                    }
                    n = n.wrapping_add(1);
                    std::thread::sleep(Duration::from_millis(250));
                }
            })
            .ok();

        crate::mac::snapper::start();
        crate::mac::tap::start();
        crate::mac::mt::start();
    }
}
