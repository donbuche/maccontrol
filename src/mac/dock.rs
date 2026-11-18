//! MacControl lives in the Dock while its window is open and only in the menu bar
//! while it is closed. Quitting is done from the menu bar item.

use objc2::sel;
use objc2::MainThreadOnly;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSMenuItem};
use crate::i18n::tl;
use objc2_foundation::{MainThreadMarker, NSString};

fn app() -> Option<objc2::rc::Retained<NSApplication>> {
    MainThreadMarker::new().map(NSApplication::sharedApplication)
}

/// Window open: Dock icon and app menu, and bring MacControl to the front.
pub fn show_in_dock() {
    let Some(app) = app() else { return };
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
    #[allow(deprecated)] // `activate` needs macOS 14; MacControl supports 12.
    app.activateIgnoringOtherApps(true);
}

/// Window closed: drop the Dock icon (the menu bar item stays) and hand focus back.
pub fn hide_from_dock() {
    let Some(app) = app() else { return };
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    app.hide(None);
}

/// True when macOS brought MacControl to the front (opened again from Finder, Spotlight…).
pub fn is_active() -> bool {
    app().map(|a| a.isActive()).unwrap_or(false)
}

/// Tags of the app-menu items MacControl changes, to find them again.
const QUIT_TAG: isize = 0x4d43_0001;
const CLOSE_TAG: isize = 0x4d43_0002;

/// Fix the app menu so MacControl only really quits from the menu bar item:
/// "Quit" (⌘Q) closes the window instead, and a "Close window" (⌘W) item is added,
/// which the default menu lacks. Returns false while the menu isn't built yet.
pub fn redirect_quit_item() -> bool {
    let Some(mtm) = MainThreadMarker::new() else { return false };
    let app = NSApplication::sharedApplication(mtm);
    let Some(main) = app.mainMenu() else { return false };
    let Some(app_menu) = main.itemAtIndex(0).and_then(|i| i.submenu()) else { return false };
    let Some(quit) = app_menu.itemArray().iter().find(|i| i.action() == Some(sel!(terminate:))) else { return false };
    unsafe { quit.setAction(Some(sel!(performClose:))) };
    quit.setTag(QUIT_TAG);
    let close = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str(""),
            Some(sel!(performClose:)),
            &NSString::from_str("w"),
        )
    };
    close.setTag(CLOSE_TAG);
    let at = app_menu.indexOfItem(&quit);
    app_menu.insertItem_atIndex(&close, at);
    retitle_menu();
    true
}

/// Puts the app-menu items MacControl added in the current language.
pub fn retitle_menu() {
    let Some(mtm) = MainThreadMarker::new() else { return };
    let app = NSApplication::sharedApplication(mtm);
    let Some(app_menu) = app.mainMenu().and_then(|m| m.itemAtIndex(0)).and_then(|i| i.submenu()) else { return };
    for (tag, text) in [(QUIT_TAG, tl("Close MacControl (keeps running in the menu bar)")), (CLOSE_TAG, tl("Close Window"))] {
        if let Some(item) = app_menu.itemWithTag(tag) {
            item.setTitle(&NSString::from_str(text));
        }
    }
}
