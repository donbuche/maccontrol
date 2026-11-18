//! MacControl's window.

mod icons;
mod pages;
pub mod theme;

use crate::engine::{engine, Recording};
use crate::model::{Config, TriggerKind};
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};
use icons::Icon;
use std::path::PathBuf;
use std::time::Instant;
use theme::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Home,
    Trackpad,
    Keyboard,
    Mouse,
    Snapping,
    Settings,
}

pub const PAGES: [Page; 6] = [Page::Home, Page::Trackpad, Page::Keyboard, Page::Mouse, Page::Snapping, Page::Settings];

fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

impl Page {
    pub fn title(self) -> &'static str {
        match self {
            Page::Home => "Inicio",
            Page::Trackpad => "Gestos del trackpad",
            Page::Keyboard => "Atajos de teclado",
            Page::Mouse => "Botones del ratón",
            Page::Snapping => "Ajuste de ventanas",
            Page::Settings => "Ajustes",
        }
    }
    pub fn nav(self) -> &'static str {
        match self {
            Page::Home => "Inicio",
            Page::Trackpad => "Trackpad",
            Page::Keyboard => "Teclado",
            Page::Mouse => "Ratón",
            Page::Snapping => "Ventanas",
            Page::Settings => "Ajustes",
        }
    }
    pub fn subtitle(self) -> &'static str {
        match self {
            Page::Home => "Todo lo que MacControl puede hacer por ti, de un vistazo.",
            Page::Trackpad => "Crea reglas: «cuando hago este gesto con los dedos, MacControl hace esto».",
            Page::Keyboard => "Crea reglas: «cuando pulso estas teclas, MacControl hace esto», en cualquier app.",
            Page::Mouse => "Da una función a los botones extra o a la rueda de cualquier ratón.",
            Page::Snapping => "Arrastra una ventana a un borde o esquina de la pantalla y se coloca sola.",
            Page::Settings => "Permisos, estado, sonido y preferencias generales.",
        }
    }
    pub fn blurb(self) -> &'static str {
        match self {
            Page::Home => "",
            Page::Trackpad => "Toques, clics y deslizamientos con 2 a 5 dedos, esquinas y TipTap.",
            Page::Keyboard => "Combinaciones de teclas globales que hacen lo que quieras.",
            Page::Mouse => "Botones laterales, botón central y rueda con teclas.",
            Page::Snapping => "Bordes para mitades, arriba para maximizar y esquinas para cuartos.",
            Page::Settings => "",
        }
    }
    pub fn icon(self) -> Icon {
        match self {
            Page::Home => Icon::Home,
            Page::Trackpad => Icon::Trackpad,
            Page::Keyboard => Icon::Keyboard,
            Page::Mouse => Icon::Mouse,
            Page::Snapping => Icon::Snap,
            Page::Settings => Icon::Settings,
        }
    }
    pub fn palette(self) -> Palette {
        let (a, b, t, bo) = match self {
            Page::Home => (0x8B5CF6, 0xEC4899, 0x2A1252, 0x0D0A24),
            Page::Trackpad => (0x7C5CFF, 0x4F8BFF, 0x1C1852, 0x0A0D26),
            Page::Keyboard => (0x22D3EE, 0x3B82F6, 0x0B2C4A, 0x071223),
            Page::Mouse => (0xF472B6, 0xFB923C, 0x3C1236, 0x170A1C),
            Page::Snapping => (0x34D399, 0x22D3EE, 0x08362F, 0x061A1F),
            Page::Settings => (0xA78BFA, 0x60A5FA, 0x22264A, 0x0B0D1C),
        };
        Palette { a: rgb(a), b: rgb(b), bg_top: rgb(t), bg_bottom: rgb(bo) }
    }
    pub fn matches(self, k: &TriggerKind) -> bool {
        matches!(
            (self, k),
            (Page::Trackpad, TriggerKind::Trackpad(_)) | (Page::Keyboard, TriggerKind::Keyboard(_)) | (Page::Mouse, TriggerKind::Mouse(_))
        )
    }
    pub fn is_rules(self) -> bool {
        matches!(self, Page::Trackpad | Page::Keyboard | Page::Mouse)
    }
}

/// What a recorded key or button press should be written into.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RecTarget {
    TriggerKey(u64),
    TriggerMouse(u64),
    ActionKey(u64),
}

pub struct MacApp {
    pub page: Page,
    pub scope: Option<String>,
    pub selected: Option<u64>,
    pub selected_at: Instant,
    pub rec: Option<RecTarget>,
    pub running_apps: Vec<(String, String)>,
    apps_checked: Option<Instant>,
    pub screenshot: Option<PathBuf>,
    frames: u64,
    shot_sent: bool,
    dirty: Option<Instant>,
    overlay: Option<Rect>,
    last_frame: Instant,
    tray: Option<tray_icon::TrayIcon>,
    menu_ids: Vec<(tray_icon::menu::MenuId, &'static str)>,
    hidden: bool,
    /// ⌘Q already turned into "close the window".
    quit_redirected: bool,
    /// Since the window was closed, macOS has seen MacControl in the background at least once.
    seen_inactive: bool,
    quitting: bool,
    pub confirm_reset: bool,
    pub snap_hover: Option<crate::model::WindowPos>,
    pub logo: Option<egui::TextureHandle>,
    page_at: Instant,
    pal_from: Palette,
    pal_to: Palette,
}

impl MacApp {
    pub fn new(cc: &eframe::CreationContext<'_>, screenshot: Option<PathBuf>, page: Option<String>) -> Self {
        theme::apply(&cc.egui_ctx);
        let eng = engine();
        let _ = eng.ctx.set(cc.egui_ctx.clone());
        let start = match page.as_deref() {
            Some("trackpad") => Page::Trackpad,
            Some("keyboard") => Page::Keyboard,
            Some("mouse") => Page::Mouse,
            Some("snapping") => Page::Snapping,
            Some("settings") => Page::Settings,
            _ => Page::Home,
        };
        let long_ago = Instant::now() - std::time::Duration::from_secs(5);
        let mut app = MacApp {
            page: start,
            scope: None,
            selected: None,
            selected_at: long_ago,
            rec: None,
            running_apps: vec![],
            apps_checked: None,
            screenshot: screenshot.clone(),
            frames: 0,
            shot_sent: false,
            dirty: None,
            overlay: None,
            last_frame: Instant::now(),
            tray: None,
            menu_ids: vec![],
            hidden: false,
            quit_redirected: false,
            seen_inactive: false,
            quitting: false,
            confirm_reset: false,
            snap_hover: None,
            logo: crate::logo_rgba().map(|(px, w, h)| {
                let img = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &px);
                cc.egui_ctx.load_texture("maccontrol-icon", img, egui::TextureOptions::LINEAR)
            }),
            page_at: long_ago,
            pal_from: start.palette(),
            pal_to: start.palette(),
        };
        if screenshot.is_none() {
            *eng.config.write().unwrap() = Config::load();
            eng.start();
            crate::sound::init();
            app.make_tray(&cc.egui_ctx);
            cc.egui_ctx.send_viewport_cmd(egui::ViewportCommand::RequestPaintWhileHidden);
        } else {
            // A picture for the check: sample rules.
            *eng.config.write().unwrap() = Config::default();
        }
        app.selected = eng.config.read().unwrap().triggers.iter().find(|t| app.page.matches(&t.kind)).map(|t| t.id);
        app
    }

    /// Switch section with a fade/slide and a colour blend.
    pub fn go(&mut self, p: Page) {
        if self.page == p {
            return;
        }
        self.pal_from = self.palette_now();
        self.pal_to = p.palette();
        self.page = p;
        self.page_at = Instant::now();
        self.stop_recording();
        let cfg = engine().config.read().unwrap();
        self.selected = cfg.triggers.iter().find(|t| p.matches(&t.kind) && t.app == self.scope).map(|t| t.id);
        self.selected_at = Instant::now();
    }

    pub fn select(&mut self, id: Option<u64>) {
        if self.selected != id {
            self.selected = id;
            self.selected_at = Instant::now();
        }
    }

    fn palette_now(&self) -> Palette {
        self.pal_from.lerp(self.pal_to, ease(self.page_at.elapsed().as_secs_f32() / 0.55))
    }

    /// The app icon, cropped to its tile.
    pub fn draw_logo(&self, ui: &egui::Ui, r: Rect) {
        match &self.logo {
            Some(t) => {
                let uv = Rect::from_min_max(Pos2::new(0.07, 0.07), Pos2::new(0.93, 0.95));
                ui.painter().image(t.id(), r, uv, Color32::WHITE);
            }
            None => {
                let (a, b) = accent();
                icon_tile(ui.painter(), r, a, b);
                icons::draw(ui.painter(), Icon::Bolt, r.shrink(r.width() * 0.22), Color32::WHITE);
            }
        }
    }

    fn make_tray(&mut self, ctx: &egui::Context) {
        use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
        let menu = Menu::new();
        let open = MenuItem::new("Abrir MacControl", true, None);
        let pause = MenuItem::new("Pausar / reanudar", true, None);
        let quit = MenuItem::new("Salir de MacControl", true, None);
        let _ = menu.append_items(&[&open, &pause, &PredefinedMenuItem::separator(), &quit]);
        self.menu_ids = vec![(open.id().clone(), "open"), (pause.id().clone(), "pause"), (quit.id().clone(), "quit")];
        let c = ctx.clone();
        MenuEvent::set_event_handler(Some(move |e: MenuEvent| {
            MENU_EVENTS.lock().unwrap().push(e.id);
            c.request_repaint();
        }));
        let icon = crate::tray_rgba().and_then(|(px, w, h)| tray_icon::Icon::from_rgba(px, w, h).ok());
        let mut b = tray_icon::TrayIconBuilder::new().with_menu(Box::new(menu)).with_tooltip("MacControl");
        if let Some(i) = icon {
            b = b.with_icon_templated(i);
        }
        self.tray = b.build().ok();
    }

    fn handle_menu(&mut self, ctx: &egui::Context) {
        let ids: Vec<_> = std::mem::take(&mut *MENU_EVENTS.lock().unwrap());
        for id in ids {
            let what = self.menu_ids.iter().find(|(i, _)| *i == id).map(|(_, w)| *w);
            match what {
                Some("open") => self.show_window(ctx),
                Some("pause") => {
                    let mut c = engine().config.write().unwrap();
                    c.paused = !c.paused;
                    drop(c);
                    self.dirty = Some(Instant::now());
                }
                Some("quit") => {
                    self.quitting = true;
                    engine().config.read().unwrap().save();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                _ => {}
            }
        }
    }

    /// Bring the window back, with its Dock icon.
    fn show_window(&mut self, ctx: &egui::Context) {
        self.hidden = false;
        crate::mac::dock::show_in_dock();
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = Some(Instant::now());
    }

    pub fn start_recording(&mut self, target: RecTarget) {
        let eng = engine();
        *eng.recorded.lock().unwrap() = None;
        *eng.recording.lock().unwrap() = match target {
            RecTarget::TriggerMouse(_) => Recording::Mouse,
            _ => Recording::Key,
        };
        self.rec = Some(target);
    }

    pub fn stop_recording(&mut self) {
        *engine().recording.lock().unwrap() = Recording::Off;
        self.rec = None;
    }

    pub fn permissions_ok(&self) -> bool {
        let eng = engine();
        self.screenshot.is_some() || eng.trusted.load(std::sync::atomic::Ordering::Relaxed) || crate::mac::ax::trusted()
    }

    fn sidebar(&mut self, ui: &mut egui::Ui, cfg: &mut Config) {
        ui.add_space(42.0);
        ui.horizontal(|ui| {
            ui.add_space(6.0);
            let (r, _) = ui.allocate_exact_size(Vec2::splat(42.0), Sense::hover());
            let (a, _) = accent();
            glow(ui.painter(), r.center(), 40.0, Color32::from_rgba_unmultiplied(a.r(), a.g(), a.b(), 70));
            self.draw_logo(ui, r);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.add_space(3.0);
                ui.label(egui::RichText::new("MacControl").size(18.0).strong().color(TEXT));
                ui.label(egui::RichText::new("Tu Mac, a tu manera").size(11.5).color(MUTED));
            });
        });
        ui.add_space(26.0);

        // Navigation with a sliding highlight.
        let item_h = 46.0;
        let w = ui.available_width();
        let (area, _) = ui.allocate_exact_size(Vec2::new(w, item_h * PAGES.len() as f32), Sense::hover());
        let sel = PAGES.iter().position(|p| *p == self.page).unwrap_or(0) as f32;
        let y = ui.ctx().animate_value_with_time(ui.make_persistent_id("nav_hl"), area.top() + sel * item_h, 0.28);
        let hl = Rect::from_min_size(Pos2::new(area.left(), y), Vec2::new(w, item_h - 6.0));
        glass_panel(ui.painter(), hl, 13.0, 0.0, false);
        let mut go_to = None;
        for (i, p) in PAGES.iter().enumerate() {
            let r = Rect::from_min_size(Pos2::new(area.left(), area.top() + i as f32 * item_h), Vec2::new(w, item_h - 6.0));
            let resp = ui.interact(r, ui.make_persistent_id(("nav", i)), Sense::click());
            let hov = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered() && self.page != *p, 0.15);
            if hov > 0.0 {
                ui.painter().rect_filled(r, 13.0, glass((10.0 * hov) as u8));
            }
            let pal = p.palette();
            let tile = Rect::from_center_size(Pos2::new(r.left() + 24.0, r.center().y), Vec2::splat(28.0));
            icon_tile(ui.painter(), tile, pal.a, pal.b);
            icons::draw(ui.painter(), p.icon(), tile.shrink(6.0), Color32::WHITE);
            let selected = self.page == *p;
            ui.painter().text(
                Pos2::new(r.left() + 48.0, r.center().y),
                egui::Align2::LEFT_CENTER,
                p.nav(),
                egui::FontId::proportional(14.0),
                if selected { Color32::WHITE } else { MUTED },
            );
            if p.is_rules() {
                let n = cfg.triggers.iter().filter(|t| p.matches(&t.kind) && t.enabled).count();
                ui.painter().text(Pos2::new(r.right() - 14.0, r.center().y), egui::Align2::RIGHT_CENTER, n.to_string(), egui::FontId::proportional(12.0), FAINT);
            }
            if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                go_to = Some(*p);
            }
        }
        if let Some(p) = go_to {
            self.go(p);
        }

        // Status at the bottom.
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.add_space(16.0);
            card(ui, |ui| {
                ui.set_width(ui.available_width());
                let ok = self.permissions_ok();
                let mut active = !cfg.paused;
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new(if active { "Activo" } else { "En pausa" }).strong());
                        if !ok {
                            ui.label(egui::RichText::new("Falta un permiso").size(11.0).color(AMBER));
                        } else if active {
                            ui.label(egui::RichText::new("Escuchando").size(11.0).color(GREEN));
                        } else {
                            ui.label(egui::RichText::new("No hace nada").size(11.0).color(MUTED));
                        }
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if toggle(ui, &mut active).changed() {
                            cfg.paused = !active;
                        }
                    });
                });
            });
        });
    }

    /// The translucent rectangle that shows where a dragged window will land.
    fn snap_overlay(&mut self, ctx: &egui::Context, cfg: &Config) {
        let target = *engine().preview.lock().unwrap();
        let dt = self.last_frame.elapsed().as_secs_f32().min(0.1);
        self.last_frame = Instant::now();
        let Some(t) = target else {
            self.overlay = None;
            return;
        };
        let goal = Rect::from_min_size(Pos2::new(t.x, t.y), Vec2::new(t.w, t.h));
        let cur = match self.overlay {
            None => Rect::from_center_size(goal.center(), goal.size() * 0.92),
            Some(r) => r,
        };
        let k = if cfg.snap.preview_animation <= 0.01 { 1.0 } else { (dt / cfg.snap.preview_animation * 4.0).clamp(0.0, 1.0) };
        let next = Rect::from_min_max(cur.min + (goal.min - cur.min) * k, cur.max + (goal.max - cur.max) * k);
        self.overlay = Some(next);
        if (next.min - goal.min).length() > 0.5 || (next.max - goal.max).length() > 0.5 {
            ctx.request_repaint();
        }
        let [r, g, b, a] = cfg.snap.preview_color;
        let fill = Color32::from_rgba_unmultiplied(r, g, b, a);
        let border = Color32::from_rgba_unmultiplied(r, g, b, 230);
        let builder = egui::ViewportBuilder::default()
            .with_title("MacControl snap")
            .with_position(next.min)
            .with_inner_size(next.size())
            .with_decorations(false)
            .with_transparent(true)
            .with_has_shadow(false)
            .with_mouse_passthrough(true)
            .with_window_level(egui::WindowLevel::AlwaysOnTop)
            .with_taskbar(false)
            .with_active(false)
            .with_resizable(false);
        ctx.show_viewport_immediate(egui::ViewportId::from_hash_of("snap_preview"), builder, |ui, _| {
            egui::CentralPanel::no_frame().show(ui, |ui| {
                let rect = ui.max_rect().shrink(2.0);
                let radius = if crate::mechanics::PREVIEW_ROUNDED { crate::mechanics::PREVIEW_RADIUS } else { 0.0 };
                ui.painter().rect_filled(rect, radius, fill);
                ui.painter().rect_stroke(rect, radius, Stroke::new(crate::mechanics::PREVIEW_BORDER, border), egui::StrokeKind::Inside);
            });
        });
    }

    fn screenshot_step(&mut self, ctx: &egui::Context) {
        let Some(path) = self.screenshot.clone() else { return };
        self.frames += 1;
        ctx.request_repaint();
        if self.frames >= 120 && !self.shot_sent {
            self.shot_sent = true;
            let cb = egui::ScreenshotCallback::new(move |img: std::sync::Arc<egui::ColorImage>| {
                let [w, h] = img.size;
                let mut buf = Vec::with_capacity(w * h * 4);
                for p in &img.pixels {
                    buf.extend_from_slice(&[p.r(), p.g(), p.b(), 255]);
                }
                if let Some(dir) = path.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                let _ = image::save_buffer(&path, &buf, w as u32, h as u32, image::ColorType::Rgba8);
                std::process::exit(0);
            });
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(cb));
        }
        if self.frames > 600 {
            std::process::exit(1);
        }
    }
}

static MENU_EVENTS: std::sync::Mutex<Vec<tray_icon::menu::MenuId>> = std::sync::Mutex::new(Vec::new());

impl eframe::App for MacApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_menu(ctx);
        if self.screenshot.is_none() {
            if !self.quit_redirected {
                self.quit_redirected = crate::mac::dock::redirect_quit_item();
            }
            if self.hidden {
                // Opened again from Finder, Spotlight or Launchpad while closed: show the window.
                // Only a switch from background to front counts: right after closing,
                // macOS may still report the app as active for a moment.
                let active = crate::mac::dock::is_active();
                if active && self.seen_inactive {
                    self.show_window(ctx);
                } else {
                    self.seen_inactive |= !active;
                    ctx.request_repaint_after(std::time::Duration::from_millis(400));
                }
            }
        }
        if let Some(t) = self.dirty {
            if t.elapsed().as_secs_f32() > 0.6 && self.screenshot.is_none() {
                engine().config.read().unwrap().save();
                self.dirty = None;
            } else {
                ctx.request_repaint_after(std::time::Duration::from_millis(700));
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.screenshot_step(&ctx);

        // Closing the window keeps MacControl running in the menu bar.
        if ctx.input(|i| i.viewport().close_requested()) && !self.quitting && self.screenshot.is_none() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            crate::mac::dock::hide_from_dock();
            self.hidden = true;
            self.seen_inactive = false;
            engine().config.read().unwrap().save();
        }

        if self.screenshot.is_none() && self.apps_checked.map(|t| t.elapsed().as_secs() > 5).unwrap_or(true) {
            self.running_apps = crate::mac::sys::running_apps();
            self.apps_checked = Some(Instant::now());
        }

        let mut cfg = engine().config.read().unwrap().clone();
        let before = serde_json::to_string(&cfg).unwrap_or_default();

        self.snap_overlay(&ctx, &cfg);

        if !self.hidden {
            // A soft "bip" for clicks inside the window.
            if cfg.click_sound && self.screenshot.is_none() && ctx.input(|i| i.pointer.primary_pressed()) {
                crate::sound::click();
            }

            let pal = self.palette_now();
            theme::set_palette(pal);
            if ctx.global_style().visuals.selection.bg_fill != pal.a.gamma_multiply(0.8) {
                ctx.global_style_mut(|s| {
                    s.visuals.selection.bg_fill = pal.a.gamma_multiply(0.8);
                    s.visuals.selection.stroke = egui::Stroke::new(1.0, pal.b);
                });
            }
            let time = ctx.input(|i| i.time) as f32;
            theme::background(ui.painter(), ui.max_rect(), pal, time);

            egui::Panel::left("sidebar")
                .exact_size(232.0)
                .resizable(false)
                .frame(
                    egui::Frame::new()
                        .fill(glass(9))
                        .inner_margin(egui::Margin { left: 14, right: 14, top: 0, bottom: 0 })
                        .stroke(Stroke::new(1.0, glass(18))),
                )
                .show(ui, |ui| self.sidebar(ui, &mut cfg));

            egui::CentralPanel::default()
                .frame(egui::Frame::new().fill(Color32::TRANSPARENT).inner_margin(egui::Margin { left: 30, right: 30, top: 34, bottom: 20 }))
                .show(ui, |ui| {
                    // Each section arrives with a short fade and slide.
                    let t = ease(self.page_at.elapsed().as_secs_f32() / 0.35);
                    ui.multiply_opacity(t);
                    ui.add_space((1.0 - t) * 18.0);
                    pages::header(self, ui, &mut cfg);
                    ui.add_space(16.0);
                    match self.page {
                        Page::Home => pages::home(self, ui, &mut cfg),
                        Page::Trackpad | Page::Keyboard | Page::Mouse => pages::triggers(self, ui, &mut cfg),
                        Page::Snapping => pages::snapping(self, ui, &mut cfg),
                        Page::Settings => pages::settings(self, ui, &mut cfg),
                    }
                });

            let animating = self.page_at.elapsed().as_secs_f32() < 0.6 || self.selected_at.elapsed().as_secs_f32() < 0.4;
            if animating {
                ctx.request_repaint();
            } else if ctx.input(|i| i.focused) {
                // The background glows drift slowly while the window is in front.
                ctx.request_repaint_after(std::time::Duration::from_millis(50));
            }
        }

        let after = serde_json::to_string(&cfg).unwrap_or_default();
        if after != before {
            *engine().config.write().unwrap() = cfg;
            self.mark_dirty();
        }
        if self.rec.is_some() || self.page == Page::Trackpad {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
    }

    fn on_exit(&mut self) {
        if self.screenshot.is_none() {
            engine().config.read().unwrap().save();
        }
    }
}
