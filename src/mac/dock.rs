//! MacControl lives in the Dock while its window is open and only in the menu bar
//! while it is closed. Quitting is done from the menu bar item.

use objc2::sel;
use objc2::MainThreadOnly;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSMenuItem};
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
    quit.setTitle(&NSString::from_str("Cerrar MacControl (sigue en la barra de menús)"));
    let close = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str("Cerrar ventana"),
            Some(sel!(performClose:)),
            &NSString::from_str("w"),
        )
    };
    let at = app_menu.indexOfItem(&quit);
    app_menu.insertItem_atIndex(&close, at);
    true
}
