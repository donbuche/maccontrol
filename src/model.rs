//! What the person configures: triggers, the actions they run, and snapping settings.

use crate::i18n::{tl, tlf};
use crate::mechanics as m;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------- modifiers

pub const MOD_SHIFT: u64 = 0x0002_0000;
pub const MOD_CTRL: u64 = 0x0004_0000;
pub const MOD_OPT: u64 = 0x0008_0000;
pub const MOD_CMD: u64 = 0x0010_0000;
pub const MOD_FN: u64 = 0x0080_0000;
pub const MOD_MASK: u64 = MOD_SHIFT | MOD_CTRL | MOD_OPT | MOD_CMD | MOD_FN;

pub fn mods_label(m: u64) -> String {
    let mut s = String::new();
    if m & MOD_FN != 0 {
        s.push_str("fn ");
    }
    if m & MOD_CTRL != 0 {
        s.push('⌃');
    }
    if m & MOD_OPT != 0 {
        s.push('⌥');
    }
    if m & MOD_SHIFT != 0 {
        s.push('⇧');
    }
    if m & MOD_CMD != 0 {
        s.push('⌘');
    }
    s
}

pub fn key_name(code: u16) -> String {
    let n = match code {
        0 => "A", 1 => "S", 2 => "D", 3 => "F", 4 => "H", 5 => "G", 6 => "Z", 7 => "X",
        8 => "C", 9 => "V", 10 => "§", 11 => "B", 12 => "Q", 13 => "W", 14 => "E", 15 => "R",
        16 => "Y", 17 => "T", 18 => "1", 19 => "2", 20 => "3", 21 => "4", 22 => "6", 23 => "5",
        24 => "=", 25 => "9", 26 => "7", 27 => "-", 28 => "8", 29 => "0", 30 => "]", 31 => "O",
        32 => "U", 33 => "[", 34 => "I", 35 => "P", 36 => "↵", 37 => "L", 38 => "J", 39 => "'",
        40 => "K", 41 => ";", 42 => "\\", 43 => ",", 44 => "/", 45 => "N", 46 => "M", 47 => ".",
        48 => "⇥", 49 => tl("Space"), 50 => "`", 51 => "⌫", 53 => "Esc", 76 => "↵ num",
        96 => "F5", 97 => "F6", 98 => "F7", 99 => "F3", 100 => "F8", 101 => "F9", 103 => "F11",
        105 => "F13", 107 => "F14", 109 => "F10", 111 => "F12", 113 => "F15", 115 => "↖",
        116 => "⇞", 117 => "⌦", 118 => "F4", 119 => "↘", 120 => "F2", 121 => "⇟",
        122 => "F1", 123 => "←", 124 => "→", 125 => "↓", 126 => "↑",
        _ => return tlf("Key {}", &[&code]),
    };
    n.to_string()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyCombo {
    pub key: u16,
    pub mods: u64,
}

/// Inicio, RePág, Fin and AvPág: on a Mac keyboard they are fn + an arrow, and macOS
/// turns that press into these keys before any app sees it. Gives (arrow, key name).
pub fn fn_arrow_key(code: u16) -> Option<(&'static str, &'static str)> {
    match code {
        115 => Some(("←", tl("Home"))),
        116 => Some(("↑", tl("Page Up"))),
        119 => Some(("→", tl("End"))),
        121 => Some(("↓", tl("Page Down"))),
        _ => None,
    }
}

impl KeyCombo {
    pub fn label(&self) -> String {
        format!("{}{}", mods_label(self.display_mods()), key_name(self.key))
    }
    /// Modifiers to show: the fn-arrow keys always carry "fn", since that is how they are typed.
    pub fn display_mods(&self) -> u64 {
        if fn_arrow_key(self.key).is_some() {
            self.mods | MOD_FN
        } else {
            self.mods
        }
    }
}

// ---------------------------------------------------------------- triggers

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn label(self) -> &'static str {
        match self {
            Dir::Up => tl("up"),
            Dir::Down => tl("down"),
            Dir::Left => tl("left"),
            Dir::Right => tl("right"),
        }
    }
    pub fn arrow(self) -> &'static str {
        match self {
            Dir::Up => "↑",
            Dir::Down => "↓",
            Dir::Left => "←",
            Dir::Right => "→",
        }
    }
    pub const ALL: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Corner {
    pub fn label(self) -> &'static str {
        match self {
            Corner::TopLeft => tl("top left"),
            Corner::TopRight => tl("top right"),
            Corner::BottomLeft => tl("bottom left"),
            Corner::BottomRight => tl("bottom right"),
        }
    }
    pub const ALL: [Corner; 4] = [Corner::TopLeft, Corner::TopRight, Corner::BottomLeft, Corner::BottomRight];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

/// A trackpad gesture the recognizer can report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gesture {
    Tap(u8),
    DoubleTap(u8),
    Click(u8),
    Swipe(u8, Dir),
    CornerTap(Corner),
    TipTap(Side),
    /// One finger sliding in from an edge of the trackpad (the edge it starts at).
    EdgeSwipe(Dir),
}

impl Gesture {
    pub fn label(&self) -> String {
        match *self {
            Gesture::Tap(n) => tlf("Tap with {} fingers", &[&n]),
            Gesture::DoubleTap(n) => tlf("Double tap with {} fingers", &[&n]),
            Gesture::Click(n) => tlf("Click with {} fingers", &[&n]),
            Gesture::Swipe(n, d) => tlf("Swipe {} fingers {}", &[&n, &d.label()]),
            Gesture::CornerTap(c) => tlf("1-finger tap in the {} corner", &[&c.label()]),
            Gesture::EdgeSwipe(d) => tl(match d {
                Dir::Left => "Swipe 1 finger in from the left edge",
                Dir::Right => "Swipe 1 finger in from the right edge",
                Dir::Up => "Swipe 1 finger in from the top edge",
                Dir::Down => "Swipe 1 finger in from the bottom edge",
            })
            .into(),
            Gesture::TipTap(Side::Left) => tl("TipTap left (1 finger resting)").into(),
            Gesture::TipTap(Side::Right) => tl("TipTap right (1 finger resting)").into(),
        }
    }
    pub fn fingers(&self) -> u8 {
        match *self {
            Gesture::Tap(n) | Gesture::DoubleTap(n) | Gesture::Click(n) | Gesture::Swipe(n, _) => n,
            Gesture::CornerTap(_) | Gesture::EdgeSwipe(_) => 1,
            Gesture::TipTap(_) => 2,
        }
    }
    /// Every gesture offered in the picker, grouped.
    pub fn catalogue() -> Vec<(&'static str, Vec<Gesture>)> {
        let mut taps = vec![];
        for n in 2..=5 {
            taps.push(Gesture::Tap(n));
        }
        for n in 2..=3 {
            taps.push(Gesture::DoubleTap(n));
        }
        let clicks = (3..=5).map(Gesture::Click).collect();
        let mut swipes = vec![];
        for n in 3..=5 {
            for d in Dir::ALL {
                swipes.push(Gesture::Swipe(n, d));
            }
        }
        let corners = Corner::ALL.iter().map(|c| Gesture::CornerTap(*c)).collect();
        let tip = vec![Gesture::TipTap(Side::Left), Gesture::TipTap(Side::Right)];
        let edges = Dir::ALL.iter().map(|d| Gesture::EdgeSwipe(*d)).collect();
        vec![
            (tl("Taps"), taps),
            (tl("Clicks"), clicks),
            (tl("Swipes"), swipes),
            (tl("Corners"), corners),
            ("TipTap", tip),
            (tl("From the edge"), edges),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Wheel {
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseTrigger {
    /// Button number as macOS counts them: 2 = middle, 3 = back, 4 = forward…
    Button { button: u8, mods: u64 },
    Scroll { dir: Wheel, mods: u64 },
}

pub fn button_name(b: u8) -> String {
    match b {
        2 => tl("Middle button").into(),
        3 => tl("Button 4 (back)").into(),
        4 => tl("Button 5 (forward)").into(),
        n => tlf("Button {}", &[&(n + 1)]),
    }
}

impl MouseTrigger {
    pub fn label(&self) -> String {
        match *self {
            MouseTrigger::Button { button, mods } => {
                let m = mods_label(mods);
                if m.is_empty() {
                    button_name(button)
                } else {
                    format!("{m} + {}", button_name(button))
                }
            }
            MouseTrigger::Scroll { dir, mods } => tlf(
                if dir == Wheel::Up { "{} + wheel up" } else { "{} + wheel down" },
                &[&if mods == 0 { "?".into() } else { mods_label(mods) }],
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TriggerKind {
    Trackpad(Gesture),
    Keyboard(KeyCombo),
    Mouse(MouseTrigger),
}

impl TriggerKind {
    pub fn label(&self) -> String {
        match self {
            TriggerKind::Trackpad(g) => g.label(),
            TriggerKind::Keyboard(k) => k.label(),
            TriggerKind::Mouse(mt) => mt.label(),
        }
    }
}

// ---------------------------------------------------------------- actions

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowPos {
    Maximize,
    LeftHalf,
    RightHalf,
    TopHalf,
    BottomHalf,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    LeftThird,
    CenterThird,
    RightThird,
    LeftTwoThirds,
    RightTwoThirds,
    Center,
}

impl WindowPos {
    pub fn label(self) -> &'static str {
        match self {
            WindowPos::Maximize => tl("Maximize"),
            WindowPos::LeftHalf => tl("Left half"),
            WindowPos::RightHalf => tl("Right half"),
            WindowPos::TopHalf => tl("Top half"),
            WindowPos::BottomHalf => tl("Bottom half"),
            WindowPos::TopLeft => tl("Top-left quarter"),
            WindowPos::TopRight => tl("Top-right quarter"),
            WindowPos::BottomLeft => tl("Bottom-left quarter"),
            WindowPos::BottomRight => tl("Bottom-right quarter"),
            WindowPos::LeftThird => tl("Left third"),
            WindowPos::CenterThird => tl("Center third"),
            WindowPos::RightThird => tl("Right third"),
            WindowPos::LeftTwoThirds => tl("Left two thirds"),
            WindowPos::RightTwoThirds => tl("Right two thirds"),
            WindowPos::Center => tl("Center"),
        }
    }
    pub const ALL: [WindowPos; 15] = [
        WindowPos::Maximize,
        WindowPos::LeftHalf,
        WindowPos::RightHalf,
        WindowPos::TopHalf,
        WindowPos::BottomHalf,
        WindowPos::TopLeft,
        WindowPos::TopRight,
        WindowPos::BottomLeft,
        WindowPos::BottomRight,
        WindowPos::LeftThird,
        WindowPos::CenterThird,
        WindowPos::RightThird,
        WindowPos::LeftTwoThirds,
        WindowPos::RightTwoThirds,
        WindowPos::Center,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SystemAction {
    MissionControl,
    AppExpose,
    ShowDesktop,
    Launchpad,
    LockScreen,
    Screenshot,
    VolumeUp,
    VolumeDown,
    Mute,
    MinimizeWindow,
    MinimizeWindowUnderCursor,
    CloseWindow,
    FullScreen,
    RestoreWindow,
    NextScreen,
    LeftClick,
    RightClick,
    MiddleClick,
}

impl SystemAction {
    pub fn label(self) -> &'static str {
        match self {
            SystemAction::MissionControl => "Mission Control",
            SystemAction::AppExpose => tl("App windows (App Exposé)"),
            SystemAction::ShowDesktop => tl("Show desktop"),
            SystemAction::Launchpad => "Launchpad",
            SystemAction::LockScreen => tl("Lock screen"),
            SystemAction::Screenshot => tl("Capture an area"),
            SystemAction::VolumeUp => tl("Volume up"),
            SystemAction::VolumeDown => tl("Volume down"),
            SystemAction::Mute => tl("Mute / unmute"),
            SystemAction::MinimizeWindow => tl("Minimize window"),
            SystemAction::MinimizeWindowUnderCursor => tl("Minimize the window under the cursor"),
            SystemAction::CloseWindow => tl("Close window"),
            SystemAction::FullScreen => tl("Full screen"),
            SystemAction::RestoreWindow => tl("Restore previous size"),
            SystemAction::NextScreen => tl("Move to the next screen"),
            SystemAction::LeftClick => tl("Left click"),
            SystemAction::RightClick => tl("Right click"),
            SystemAction::MiddleClick => tl("Middle click"),
        }
    }
    pub fn group(self) -> &'static str {
        use SystemAction::*;
        match self {
            MinimizeWindow | MinimizeWindowUnderCursor | CloseWindow | FullScreen | RestoreWindow | NextScreen => tl("Windows"),
            LeftClick | RightClick | MiddleClick => tl("Mouse"),
            VolumeUp | VolumeDown | Mute => tl("Sound"),
            _ => tl("System"),
        }
    }
    pub const ALL: [SystemAction; 18] = [
        SystemAction::MissionControl,
        SystemAction::AppExpose,
        SystemAction::ShowDesktop,
        SystemAction::Launchpad,
        SystemAction::LockScreen,
        SystemAction::Screenshot,
        SystemAction::VolumeUp,
        SystemAction::VolumeDown,
        SystemAction::Mute,
        SystemAction::MinimizeWindow,
        SystemAction::MinimizeWindowUnderCursor,
        SystemAction::CloseWindow,
        SystemAction::FullScreen,
        SystemAction::RestoreWindow,
        SystemAction::NextScreen,
        SystemAction::LeftClick,
        SystemAction::RightClick,
        SystemAction::MiddleClick,
    ];
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Action {
    None,
    Shortcut(KeyCombo),
    Window(WindowPos),
    System(SystemAction),
    OpenApp(String),
    OpenUrl(String),
    TypeText(String),
    Shell(String),
}

impl Action {
    pub fn label(&self) -> String {
        match self {
            Action::None => tl("No action").into(),
            Action::Shortcut(k) => tlf("Press {}", &[&k.label()]),
            Action::Window(w) => w.label().into(),
            Action::System(s) => s.label().into(),
            Action::OpenApp(a) => tlf("Open {}", &[&if a.is_empty() { "app…" } else { a }]),
            Action::OpenUrl(u) => tlf("Open {}", &[&if u.is_empty() { tl("link…") } else { u }]),
            Action::TypeText(t) => tlf("Type “{}”", &[&short(t, 24)]),
            Action::Shell(c) => tlf("Run “{}”", &[&short(c, 24)]),
        }
    }
    pub fn kind_index(&self) -> usize {
        match self {
            Action::None => 0,
            Action::Shortcut(_) => 1,
            Action::Window(_) => 2,
            Action::System(_) => 3,
            Action::OpenApp(_) => 4,
            Action::OpenUrl(_) => 5,
            Action::TypeText(_) => 6,
            Action::Shell(_) => 7,
        }
    }
}

fn short(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(n).collect::<String>())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trigger {
    pub id: u64,
    pub kind: TriggerKind,
    pub action: Action,
    /// Bundle id of the app this trigger is for; `None` = every app.
    pub app: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub note: String,
}

// ---------------------------------------------------------------- snapping

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapSettings {
    pub enabled: bool,
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub corners: bool,
    pub corner_area: f32,
    pub recognition_extra: f32,
    pub left_width: f32,
    pub right_width: f32,
    pub corner_width: f32,
    pub gap: f32,
    pub snap_back: bool,
    pub show_preview: bool,
    pub preview_animation: f32,
    pub delay_enabled: bool,
    pub delay: f32,
    /// RGBA of the preview fill.
    pub preview_color: [u8; 4],
    /// Bundle ids where snapping is off.
    pub excluded_apps: Vec<String>,
}

impl Default for SnapSettings {
    fn default() -> Self {
        Self {
            enabled: m::SNAP_ENABLED,
            left: m::SNAP_LEFT,
            right: m::SNAP_RIGHT,
            top: m::SNAP_TOP,
            corners: m::SNAP_CORNERS,
            corner_area: m::CORNER_AREA,
            recognition_extra: m::RECOGNITION_AREA_EXTRA,
            left_width: m::LEFT_WIDTH,
            right_width: m::RIGHT_WIDTH,
            corner_width: m::CORNER_WIDTH,
            gap: m::GAP,
            snap_back: m::SNAP_BACK,
            show_preview: true,
            preview_animation: m::PREVIEW_ANIMATION,
            delay_enabled: m::SNAP_DELAY_ENABLED,
            delay: m::SNAP_DELAY,
            preview_color: [124, 108, 255, 70],
            excluded_apps: vec![],
        }
    }
}

// ---------------------------------------------------------------- config

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub paused: bool,
    pub triggers: Vec<Trigger>,
    pub snap: SnapSettings,
    /// Apps the person added to the sidebar: (bundle id, name).
    pub apps: Vec<(String, String)>,
    pub next_id: u64,
    /// A short "bip" when clicking inside MacControl's window.
    pub click_sound: bool,
    /// Interface language (the Mac's own when MacControl speaks it, otherwise English).
    pub language: crate::i18n::Lang,
}

impl Default for Config {
    fn default() -> Self {
        let mut c = Config { paused: false, triggers: vec![], snap: SnapSettings::default(), apps: vec![], next_id: 1, click_sound: true, language: crate::i18n::Lang::system() };
        let k = |key, mods| TriggerKind::Keyboard(KeyCombo { key, mods });
        let presets = [
            (TriggerKind::Trackpad(Gesture::Tap(3)), Action::System(SystemAction::MiddleClick)),
            (TriggerKind::Trackpad(Gesture::TipTap(Side::Left)), Action::Shortcut(KeyCombo { key: 33, mods: MOD_CMD })),
            (TriggerKind::Trackpad(Gesture::TipTap(Side::Right)), Action::Shortcut(KeyCombo { key: 30, mods: MOD_CMD })),
            (TriggerKind::Trackpad(Gesture::CornerTap(Corner::TopRight)), Action::System(SystemAction::ShowDesktop)),
            (k(123, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::LeftHalf)),
            (k(124, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::RightHalf)),
            (k(36, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::Maximize)),
            (k(8, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::Center)),
            (TriggerKind::Mouse(MouseTrigger::Button { button: 3, mods: 0 }), Action::Shortcut(KeyCombo { key: 33, mods: MOD_CMD })),
            (TriggerKind::Mouse(MouseTrigger::Button { button: 4, mods: 0 }), Action::Shortcut(KeyCombo { key: 30, mods: MOD_CMD })),
            (TriggerKind::Mouse(MouseTrigger::Button { button: 2, mods: MOD_OPT }), Action::System(SystemAction::MissionControl)),
        ];
        for (kind, action) in presets {
            c.add(kind, action, None);
        }
        c
    }
}

impl Config {
    pub fn add(&mut self, kind: TriggerKind, action: Action, app: Option<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.triggers.push(Trigger { id, kind, action, app, enabled: true, note: String::new() });
        id
    }

    /// The trigger that answers `kind` in `app`: one made for that app wins over a global one.
    pub fn find(&self, kind: &TriggerKind, app: &str) -> Option<&Trigger> {
        let mut global = None;
        for t in &self.triggers {
            if !t.enabled || &t.kind != kind {
                continue;
            }
            match &t.app {
                Some(a) if a == app => return Some(t),
                None if global.is_none() => global = Some(t),
                _ => {}
            }
        }
        global
    }

    pub fn path() -> std::path::PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        std::path::PathBuf::from(home).join("Library/Application Support/MacControl/config.json")
    }

    pub fn load() -> Config {
        std::fs::read_to_string(Self::path())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let p = Self::path();
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let tmp = p.with_extension("json.tmp");
            if std::fs::write(&tmp, s).is_ok() {
                let _ = std::fs::rename(tmp, p);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fn_arrow_keys_show_with_fn() {
        assert_eq!(KeyCombo { key: 116, mods: 0 }.label(), "fn ⇞");
        assert_eq!(KeyCombo { key: 115, mods: 0 }.label(), "fn ↖");
        assert_eq!(KeyCombo { key: 119, mods: 0 }.label(), "fn ↘");
        assert_eq!(KeyCombo { key: 121, mods: MOD_CMD }.label(), "fn ⌘⇟");
        assert_eq!(KeyCombo { key: 124, mods: 0 }.label(), "→");
    }

    #[test]
    fn app_specific_wins() {
        let mut c = Config { triggers: vec![], ..Default::default() };
        let kind = TriggerKind::Trackpad(Gesture::Tap(4));
        c.add(kind, Action::System(SystemAction::Launchpad), None);
        c.add(kind, Action::System(SystemAction::MissionControl), Some("com.apple.Safari".into()));
        assert_eq!(c.find(&kind, "com.apple.Safari").unwrap().action, Action::System(SystemAction::MissionControl));
        assert_eq!(c.find(&kind, "com.apple.Finder").unwrap().action, Action::System(SystemAction::Launchpad));
    }
}
