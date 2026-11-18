//! MacControl: trackpad gestures, keyboard shortcuts, mouse buttons and window snapping for macOS.

mod engine;
#[allow(dead_code)]
mod gestures;
mod i18n;
mod mac;
#[allow(dead_code)]
mod mechanics;
mod model;
mod snap;
mod sound;
mod ui;

use eframe::egui;
use std::path::PathBuf;

/// MacControl's icon, a top hat on a cyan-to-blue tile (from assets/icon.svg via
/// scripts/render-icons.sh).
pub const ICON_PNG: &[u8] = include_bytes!("../assets/icon.png");
/// The logo shown inside the window: the same hat on a faint glass tile.
pub const LOGO_PNG: &[u8] = include_bytes!("../assets/logo.png");
/// The menu-bar version: the hat's silhouette as a template image macOS tints itself.
pub const TRAY_PNG: &[u8] = include_bytes!("../assets/tray.png");

fn png_rgba(png: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let img = image::load_from_memory(png).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    Some((img.into_raw(), w, h))
}

pub fn icon_rgba() -> Option<(Vec<u8>, u32, u32)> {
    png_rgba(ICON_PNG)
}

pub fn logo_rgba() -> Option<(Vec<u8>, u32, u32)> {
    png_rgba(LOGO_PNG)
}

pub fn tray_rgba() -> Option<(Vec<u8>, u32, u32)> {
    png_rgba(TRAY_PNG)
}

#[cfg(test)]
mod icon_tests {
    /// All three images are transparent around their shape (the Dock tile has macOS's margin).
    #[test]
    fn icons_are_transparent() {
        for (rgba, w, _) in [super::icon_rgba().unwrap(), super::logo_rgba().unwrap(), super::tray_rgba().unwrap()] {
            assert_eq!(rgba[3], 0, "top-left corner");
            assert_eq!(rgba[(w as usize - 1) * 4 + 3], 0, "top-right corner");
        }
    }
}

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let screenshot = args
        .iter()
        .position(|a| a == "--screenshot")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from);
    // Which page to open on (used to picture each page).
    let page = args.iter().position(|a| a == "--page").and_then(|i| args.get(i + 1)).cloned();

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("MacControl")
        .with_inner_size([1180.0, 760.0])
        .with_min_inner_size([960.0, 620.0])
        .with_fullsize_content_view(true)
        .with_titlebar_shown(false)
        .with_title_shown(false);
    if let Some((rgba, width, height)) = icon_rgba() {
        viewport = viewport.with_icon(egui::IconData { rgba, width, height });
    }
    if screenshot.is_some() {
        // A check takes this picture: never steal focus from what the person is doing.
        viewport = viewport.with_active(false);
    }

    let mut options = eframe::NativeOptions { viewport, ..Default::default() };
    if screenshot.is_some() {
        options.event_loop_builder = Some(Box::new(|b| {
            use winit::platform::macos::EventLoopBuilderExtMacOS;
            b.with_activate_ignoring_other_apps(false);
        }));
    }
    eframe::run_native("MacControl", options, Box::new(move |cc| Ok(Box::new(ui::MacApp::new(cc, screenshot, page)))))
}
