//! The content of each section.

use super::icons::{self, Icon};
use super::theme::*;
use super::{MacApp, Page, RecTarget};
use crate::engine::{engine, Recorded};
use crate::model::*;
use crate::snap::{target_rect, R};
use eframe::egui::{self, Color32, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Vec2};
use std::sync::atomic::Ordering;

fn app_name(cfg: &Config, running: &[(String, String)], id: &str) -> String {
    cfg.apps
        .iter()
        .chain(running.iter())
        .find(|(b, _)| b == id)
        .map(|(_, n)| n.clone())
        .unwrap_or_else(|| id.to_string())
}

fn fade_in(ui: &mut egui::Ui, since: std::time::Instant) {
    let t = ease(since.elapsed().as_secs_f32() / 0.3);
    ui.multiply_opacity(t);
    ui.add_space((1.0 - t) * 10.0);
}

// ------------------------------------------------------------------ header

pub fn header(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    if app.page == Page::Home {
        return;
    }
    ui.horizontal(|ui| {
        let pal = app.page.palette();
        let (tile, _) = ui.allocate_exact_size(Vec2::splat(48.0), Sense::hover());
        icon_tile(ui.painter(), tile, pal.a, pal.b);
        icons::draw(ui.painter(), app.page.icon(), tile.shrink(11.0), Color32::WHITE);
        ui.add_space(6.0);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 3.0;
            ui.label(RichText::new(app.page.title()).size(28.0).strong().color(TEXT));
            ui.label(RichText::new(app.page.subtitle()).color(MUTED));
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if app.page.is_rules() && primary_button(ui, "+  Nueva regla").clicked() {
                add_trigger(app, cfg, None);
            }
        });
    });
}

/// Ready-made rules offered as one-click ideas.
fn presets(page: Page) -> Vec<(TriggerKind, Action)> {
    let k = |key, mods| TriggerKind::Keyboard(KeyCombo { key, mods });
    let tp = TriggerKind::Trackpad;
    let mb = |button, mods| TriggerKind::Mouse(MouseTrigger::Button { button, mods });
    let back = Action::Shortcut(KeyCombo { key: 33, mods: MOD_CMD });
    let fwd = Action::Shortcut(KeyCombo { key: 30, mods: MOD_CMD });
    match page {
        Page::Trackpad => vec![
            (tp(Gesture::Tap(3)), Action::System(SystemAction::MiddleClick)),
            (tp(Gesture::TipTap(Side::Left)), back.clone()),
            (tp(Gesture::TipTap(Side::Right)), fwd.clone()),
            (tp(Gesture::CornerTap(Corner::TopRight)), Action::System(SystemAction::ShowDesktop)),
            (tp(Gesture::Tap(4)), Action::System(SystemAction::Launchpad)),
            (tp(Gesture::Swipe(5, Dir::Up)), Action::Window(WindowPos::Maximize)),
            (tp(Gesture::Click(3)), Action::Shortcut(KeyCombo { key: 13, mods: MOD_CMD })),
        ],
        Page::Keyboard => vec![
            (k(123, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::LeftHalf)),
            (k(124, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::RightHalf)),
            (k(36, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::Maximize)),
            (k(8, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::Center)),
            (k(126, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::TopHalf)),
            (k(125, MOD_CTRL | MOD_OPT), Action::Window(WindowPos::BottomHalf)),
            (k(45, MOD_CTRL | MOD_OPT), Action::System(SystemAction::NextScreen)),
        ],
        _ => vec![
            (mb(3, 0), back),
            (mb(4, 0), fwd),
            (mb(2, MOD_OPT), Action::System(SystemAction::MissionControl)),
            (TriggerKind::Mouse(MouseTrigger::Scroll { dir: Wheel::Up, mods: MOD_OPT }), Action::System(SystemAction::VolumeUp)),
            (TriggerKind::Mouse(MouseTrigger::Scroll { dir: Wheel::Down, mods: MOD_OPT }), Action::System(SystemAction::VolumeDown)),
        ],
    }
}

fn add_trigger(app: &mut MacApp, cfg: &mut Config, preset: Option<(TriggerKind, Action)>) {
    let id = match preset {
        Some((kind, action)) => cfg.add(kind, action, app.scope.clone()),
        None => {
            let used: Vec<TriggerKind> = cfg.triggers.iter().filter(|t| t.app == app.scope).map(|t| t.kind).collect();
            let kind = match app.page {
                Page::Trackpad => {
                    let all: Vec<Gesture> = Gesture::catalogue().into_iter().flat_map(|(_, g)| g).collect();
                    let g = all.iter().find(|g| !used.contains(&TriggerKind::Trackpad(**g))).copied().unwrap_or(Gesture::Tap(4));
                    TriggerKind::Trackpad(g)
                }
                Page::Keyboard => TriggerKind::Keyboard(KeyCombo { key: 0xFFFF, mods: 0 }),
                _ => TriggerKind::Mouse(MouseTrigger::Button { button: 2, mods: MOD_CMD }),
            };
            let id = cfg.add(kind, Action::None, app.scope.clone());
            match app.page {
                Page::Keyboard => app.start_recording(RecTarget::TriggerKey(id)),
                Page::Mouse => app.start_recording(RecTarget::TriggerMouse(id)),
                _ => {}
            }
            id
        }
    };
    app.select(Some(id));
}

// ------------------------------------------------------------------ home

pub fn home(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    egui::ScrollArea::vertical().id_salt("home").show(ui, |ui| {
        // Hero.
        let w = ui.available_width();
        let (hero, _) = ui.allocate_exact_size(Vec2::new(w, 170.0), Sense::hover());
        let (a, b) = accent();
        let p = ui.painter().with_clip_rect(hero);
        glass_panel(&p, hero, 24.0, 0.0, false);
        glow(&p, hero.left_center() + Vec2::new(110.0, 0.0), 150.0, Color32::from_rgba_unmultiplied(a.r(), a.g(), a.b(), 90));
        glow(&p, hero.right_top() + Vec2::new(-120.0, 10.0), 220.0, Color32::from_rgba_unmultiplied(b.r(), b.g(), b.b(), 60));
        let logo = Rect::from_center_size(hero.left_center() + Vec2::new(105.0, 0.0), Vec2::splat(112.0));
        app.draw_logo(ui, logo);
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_max(Pos2::new(logo.right() + 34.0, hero.top() + 34.0), hero.max - Vec2::new(28.0, 20.0))));
        child.spacing_mut().item_spacing.y = 6.0;
        child.label(RichText::new("Bienvenido a MacControl").size(30.0).strong().color(TEXT));
        child.label(
            RichText::new("Controla tu Mac con gestos del trackpad, atajos de teclado y botones del ratón, y coloca las ventanas solo con arrastrarlas a los bordes de la pantalla.")
                .size(14.0)
                .color(MUTED),
        );
        child.add_space(4.0);
        child.horizontal(|ui| {
            let n = cfg.triggers.iter().filter(|t| t.enabled).count();
            pill(ui, &format!("{n} reglas activas"), GREEN);
            if cfg.snap.enabled {
                pill(ui, "Ajuste de ventanas activado", Color32::from_rgb(120, 220, 255));
            }
            if cfg.paused {
                pill(ui, "En pausa", AMBER);
            }
        });
        ui.add_space(14.0);

        // First step when the permission is missing.
        if !app.permissions_ok() {
            card(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(40.0), Sense::hover());
                    icon_tile(ui.painter(), r, AMBER, Color32::from_rgb(255, 120, 90));
                    ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, "!", egui::FontId::proportional(20.0), Color32::WHITE);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(RichText::new("Primer paso: da permiso de Accesibilidad").size(15.0).strong());
                        ui.label(RichText::new("Sin él, macOS no deja que MacControl lea gestos y teclas ni mueva ventanas.").color(MUTED));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if primary_button(ui, "Conceder permiso").clicked() {
                            crate::mac::ax::request_trust();
                        }
                    });
                });
            });
            ui.add_space(14.0);
        }

        section_title(ui, "¿Qué quieres configurar?");
        ui.add_space(4.0);
        let gap = 14.0;
        let cw = (ui.available_width() - gap) / 2.0;
        let features = [Page::Trackpad, Page::Keyboard, Page::Mouse, Page::Snapping];
        let mut go = None;
        for row in features.chunks(2) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = gap;
                for p in row {
                    let (r, resp) = ui.allocate_exact_size(Vec2::new(cw, 132.0), Sense::click());
                    let h = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.18);
                    let pal = p.palette();
                    let pr = ui.painter();
                    let r2 = glass_panel(pr, r, 20.0, h, false);
                    let clip = pr.with_clip_rect(r2);
                    glow(&clip, r2.right_top() + Vec2::new(-30.0, 20.0), 140.0, Color32::from_rgba_unmultiplied(pal.a.r(), pal.a.g(), pal.a.b(), (40.0 + 50.0 * h) as u8));
                    let tile = Rect::from_min_size(r2.min + Vec2::new(18.0, 18.0), Vec2::splat(46.0));
                    icon_tile(pr, tile, pal.a, pal.b);
                    icons::draw(pr, p.icon(), tile.shrink(11.0), Color32::WHITE);
                    pr.text(Pos2::new(tile.right() + 14.0, tile.top() + 4.0), egui::Align2::LEFT_TOP, p.title(), egui::FontId::proportional(17.0), TEXT);
                    let g = pr.layout(p.blurb().to_string(), egui::FontId::proportional(12.5), MUTED, r2.width() - tile.width() - 52.0);
                    pr.galley(Pos2::new(tile.right() + 14.0, tile.top() + 28.0), g, MUTED);
                    let info = if p.is_rules() {
                        let n = cfg.triggers.iter().filter(|t| p.matches(&t.kind) && t.enabled).count();
                        format!("{n} reglas activas")
                    } else if cfg.snap.enabled {
                        "Activado".to_string()
                    } else {
                        "Desactivado".to_string()
                    };
                    pr.text(Pos2::new(r2.left() + 18.0, r2.bottom() - 22.0), egui::Align2::LEFT_CENTER, info, egui::FontId::proportional(12.0), FAINT);
                    let arrow_x = r2.right() - 22.0 + 4.0 * h;
                    pr.text(Pos2::new(arrow_x, r2.bottom() - 22.0), egui::Align2::RIGHT_CENTER, "Configurar  →", egui::FontId::proportional(12.5), lerp_color(MUTED, Color32::WHITE, h));
                    if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        go = Some(*p);
                    }
                }
            });
            ui.add_space(gap - 8.0);
        }
        if let Some(p) = go {
            app.go(p);
        }

        ui.add_space(8.0);
        section_title(ui, "Cómo funciona");
        ui.add_space(4.0);
        let steps = [
            ("Elige qué haces", "Un gesto con los dedos, una combinación de teclas o un botón del ratón."),
            ("Elige qué pasa", "Colocar una ventana, abrir una app, enviar un atajo, subir el volumen…"),
            ("Y listo", "Funciona en todas las apps, o solo en las que tú elijas."),
        ];
        let sw = (ui.available_width() - 2.0 * gap) / 3.0;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for (i, (t, d)) in steps.iter().enumerate() {
                ui.allocate_ui_with_layout(Vec2::new(sw, 110.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                    card(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.set_min_height(74.0);
                        step_title(ui, i + 1, t, "");
                        ui.label(RichText::new(*d).size(12.5).color(MUTED));
                    });
                });
            }
        });
    });
}

// ------------------------------------------------------------------ rules pages

pub fn triggers(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    // Pick up a recorded key or button.
    if let Some(target) = app.rec {
        let got = engine().recorded.lock().unwrap().take();
        let got = got.or_else(|| local_key(ui).map(Recorded::Key));
        if let Some(r) = got {
            apply_recorded(cfg, target, r);
            app.stop_recording();
        }
    }

    scope_bar(app, ui, cfg);
    ui.add_space(12.0);

    let ids: Vec<u64> = cfg.triggers.iter().filter(|t| app.page.matches(&t.kind) && t.app == app.scope).map(|t| t.id).collect();
    if ids.is_empty() {
        empty_state(app, ui, cfg);
        return;
    }
    if app.selected.map(|s| !ids.contains(&s)).unwrap_or(true) {
        app.select(ids.first().copied());
    }

    let avail = ui.available_size();
    let list_w = (avail.x * 0.5).clamp(330.0, 540.0);
    ui.horizontal_top(|ui| {
        ui.allocate_ui_with_layout(Vec2::new(list_w, avail.y), egui::Layout::top_down(egui::Align::Min), |ui| {
            egui::ScrollArea::vertical().id_salt("list").show(ui, |ui| {
                if app.page == Page::Trackpad {
                    live_trackpad(ui);
                    ui.add_space(10.0);
                }
                section_title(ui, &format!("Tus reglas ({})", ids.len()));
                ui.add_space(2.0);
                rule_list(app, ui, cfg, &ids);
                ui.add_space(10.0);
                ideas(app, ui, cfg, "Más ideas");
            });
        });
        ui.add_space(12.0);
        ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), avail.y), egui::Layout::top_down(egui::Align::Min), |ui| {
            egui::ScrollArea::vertical().id_salt("editor").show(ui, |ui| {
                fade_in(ui, app.selected_at);
                editor(app, ui, cfg);
            });
        });
    });
}

/// "Funciona en": every app or one in particular.
fn scope_bar(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    egui::Frame::new()
        .fill(glass(9))
        .stroke(Stroke::new(1.0, glass(20)))
        .corner_radius(16)
        .inner_margin(egui::Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Funciona en").color(MUTED));
                ui.add_space(4.0);
                if chip(ui, "Todas las apps", app.scope.is_none()).clicked() {
                    app.scope = None;
                    app.select(None);
                }
                let mut remove = None;
                for (b, n) in cfg.apps.clone() {
                    let on = app.scope.as_deref() == Some(b.as_str());
                    let r = chip(ui, &n, on).on_hover_text("Clic derecho para quitarla");
                    if r.clicked() {
                        app.scope = Some(b.clone());
                        app.select(None);
                    }
                    r.context_menu(|ui| {
                        if ui.button("Quitar esta app y sus reglas").clicked() {
                            remove = Some(b.clone());
                            ui.close();
                        }
                    });
                }
                if let Some(r) = remove {
                    cfg.apps.retain(|(b, _)| *b != r);
                    cfg.triggers.retain(|t| t.app.as_deref() != Some(r.as_str()));
                    if app.scope.as_deref() == Some(r.as_str()) {
                        app.scope = None;
                    }
                }
                let add = ghost_button(ui, "+ Una app concreta");
                egui::Popup::menu(&add).show(|ui| {
                    ui.set_min_width(230.0);
                    ui.label(RichText::new("Apps abiertas ahora").size(11.0).color(FAINT));
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        let running = app.running_apps.clone();
                        if running.is_empty() {
                            ui.label(RichText::new("Abre la app que quieras y vuelve aquí.").color(MUTED));
                        }
                        for (b, n) in running {
                            if cfg.apps.iter().any(|(x, _)| *x == b) {
                                continue;
                            }
                            if ui.button(&n).clicked() {
                                cfg.apps.push((b.clone(), n.clone()));
                                app.scope = Some(b);
                                app.select(None);
                                ui.close();
                            }
                        }
                    });
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("Las reglas de una app concreta mandan sobre las generales").size(11.5).color(FAINT));
                });
            });
        });
}

fn empty_state(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    card(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.vertical_centered(|ui| {
            ui.add_space(16.0);
            let pal = app.page.palette();
            let (r, _) = ui.allocate_exact_size(Vec2::splat(64.0), Sense::hover());
            glow(ui.painter(), r.center(), 90.0, Color32::from_rgba_unmultiplied(pal.a.r(), pal.a.g(), pal.a.b(), 70));
            icon_tile(ui.painter(), r, pal.a, pal.b);
            icons::draw(ui.painter(), app.page.icon(), r.shrink(15.0), Color32::WHITE);
            ui.add_space(8.0);
            let where_ = match &app.scope {
                None => "para todas las apps".to_string(),
                Some(b) => format!("para {}", app_name(cfg, &app.running_apps, b)),
            };
            ui.label(RichText::new(format!("Aún no tienes reglas {where_}")).size(18.0).strong());
            ui.label(RichText::new("Empieza con una de estas ideas o crea la tuya.").color(MUTED));
            ui.add_space(10.0);
            if primary_button(ui, "+  Crear una regla").clicked() {
                add_trigger(app, cfg, None);
            }
            ui.add_space(10.0);
        });
        ideas(app, ui, cfg, "Ideas para empezar");
    });
}

/// One-click ready-made rules that aren't added yet.
fn ideas(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config, title: &str) {
    let have: Vec<TriggerKind> = cfg.triggers.iter().filter(|t| t.app == app.scope).map(|t| t.kind).collect();
    let list: Vec<(TriggerKind, Action)> = presets(app.page).into_iter().filter(|(k, _)| !have.contains(k)).collect();
    if list.is_empty() {
        return;
    }
    section_title(ui, title);
    ui.add_space(2.0);
    let mut chosen = None;
    ui.horizontal_wrapped(|ui| {
        for (k, a) in list {
            let text = format!("+  {}  →  {}", short_kind(&k), a.label());
            if ghost_button(ui, &text).on_hover_text("Añadir esta regla").clicked() {
                chosen = Some((k, a));
            }
        }
    });
    if let Some(p) = chosen {
        add_trigger(app, cfg, Some(p));
    }
}

fn short_kind(k: &TriggerKind) -> String {
    match k {
        TriggerKind::Trackpad(g) => match g {
            Gesture::Tap(n) => format!("Toque {n} dedos"),
            Gesture::DoubleTap(n) => format!("Doble toque {n} dedos"),
            Gesture::Click(n) => format!("Clic {n} dedos"),
            Gesture::Swipe(n, d) => format!("{n} dedos {}", d.arrow()),
            Gesture::CornerTap(_) => "Toque en esquina".into(),
            Gesture::EdgeSwipe(d) => format!("1 dedo desde borde {}", d.arrow()),
            Gesture::TipTap(Side::Left) => "TipTap izq.".into(),
            Gesture::TipTap(Side::Right) => "TipTap dcha.".into(),
        },
        _ => k.label(),
    }
}

fn apply_recorded(cfg: &mut Config, target: RecTarget, r: Recorded) {
    match (target, r) {
        (RecTarget::TriggerKey(id), Recorded::Key(k)) => {
            if let Some(t) = cfg.triggers.iter_mut().find(|t| t.id == id) {
                t.kind = TriggerKind::Keyboard(k);
            }
        }
        (RecTarget::ActionKey(id), Recorded::Key(k)) => {
            if let Some(t) = cfg.triggers.iter_mut().find(|t| t.id == id) {
                t.action = Action::Shortcut(k);
            }
        }
        (RecTarget::TriggerMouse(id), Recorded::Mouse(m)) => {
            if let Some(t) = cfg.triggers.iter_mut().find(|t| t.id == id) {
                t.kind = TriggerKind::Mouse(m);
            }
        }
        _ => {}
    }
}

/// When the system-wide listener isn't running yet, record keys typed into this window.
fn local_key(ui: &egui::Ui) -> Option<KeyCombo> {
    if engine().tap_ok.load(Ordering::Relaxed) {
        return None;
    }
    ui.input(|i| {
        for e in &i.events {
            if let egui::Event::Key { key, pressed: true, modifiers, .. } = e {
                if let Some(code) = mac_keycode(*key) {
                    let mut mods = 0;
                    if modifiers.mac_cmd || modifiers.command {
                        mods |= MOD_CMD;
                    }
                    if modifiers.alt {
                        mods |= MOD_OPT;
                    }
                    if modifiers.ctrl && !modifiers.command {
                        mods |= MOD_CTRL;
                    }
                    if modifiers.shift {
                        mods |= MOD_SHIFT;
                    }
                    return Some(KeyCombo { key: code, mods });
                }
            }
        }
        None
    })
}

fn mac_keycode(k: egui::Key) -> Option<u16> {
    use egui::Key::*;
    Some(match k {
        A => 0, S => 1, D => 2, F => 3, H => 4, G => 5, Z => 6, X => 7, C => 8, V => 9, B => 11, Q => 12,
        W => 13, E => 14, R => 15, Y => 16, T => 17, O => 31, U => 32, I => 34, P => 35, L => 37, J => 38,
        K => 40, N => 45, M => 46,
        Num1 => 18, Num2 => 19, Num3 => 20, Num4 => 21, Num5 => 23, Num6 => 22, Num7 => 26, Num8 => 28,
        Num9 => 25, Num0 => 29,
        Enter => 36, Tab => 48, Space => 49, Backspace => 51, Escape => 53, Delete => 117,
        Home => 115, End => 119, PageUp => 116, PageDown => 121,
        ArrowLeft => 123, ArrowRight => 124, ArrowDown => 125, ArrowUp => 126,
        F1 => 122, F2 => 120, F3 => 99, F4 => 118, F5 => 96, F6 => 97, F7 => 98, F8 => 100,
        F9 => 101, F10 => 109, F11 => 103, F12 => 111,
        Minus => 27, Comma => 43, Period => 47, Slash => 44, Semicolon => 41,
        OpenBracket => 33, CloseBracket => 30, Backslash => 42,
        _ => return None,
    })
}

fn kind_badge(kind: &TriggerKind) -> String {
    match kind {
        TriggerKind::Trackpad(g) => match g {
            Gesture::Swipe(_, d) => format!("{}{}", g.fingers(), d.arrow()),
            Gesture::CornerTap(_) => "◰".into(),
            Gesture::EdgeSwipe(d) => format!("⇤{}", d.arrow()),
            Gesture::TipTap(Side::Left) => "◐".into(),
            Gesture::TipTap(Side::Right) => "◑".into(),
            _ => g.fingers().to_string(),
        },
        TriggerKind::Keyboard(_) => "⌘".into(),
        TriggerKind::Mouse(MouseTrigger::Scroll { .. }) => "↕".into(),
        TriggerKind::Mouse(MouseTrigger::Button { button, .. }) => format!("B{}", button + 1),
    }
}

fn trigger_title(kind: &TriggerKind) -> String {
    match kind {
        TriggerKind::Keyboard(k) if k.key == 0xFFFF => "Nuevo atajo: pulsa una combinación".into(),
        TriggerKind::Keyboard(k) => combo_parts(k.display_mods(), Some(key_name(k.key))).join(" "),
        _ => kind.label(),
    }
}

fn rule_list(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config, ids: &[u64]) {
    for id in ids {
        let id = *id;
        let Some(idx) = cfg.triggers.iter().position(|t| t.id == id) else { continue };
        let selected = app.selected == Some(id);
        let t = &mut cfg.triggers[idx];
        let w = ui.available_width();
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 68.0), Sense::click());
        let h = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered(), 0.15);
        let p = ui.painter();
        let r = glass_panel(p, rect, 16.0, h * 0.6, selected);
        let (a, b) = accent();
        let badge = Rect::from_min_size(r.min + Vec2::new(12.0, 13.0), Vec2::splat(42.0));
        if t.enabled {
            icon_tile(p, badge, a, b);
        } else {
            p.rect_filled(badge, 12.0, glass(18));
        }
        p.text(badge.center(), egui::Align2::CENTER_CENTER, kind_badge(&t.kind), egui::FontId::proportional(15.0), if t.enabled { Color32::WHITE } else { FAINT });
        let tc = if t.enabled { TEXT } else { MUTED };
        let max_w = r.width() - 140.0;
        let g = p.layout(trigger_title(&t.kind), egui::FontId::proportional(14.5), tc, max_w);
        p.galley(Pos2::new(badge.right() + 14.0, r.top() + 14.0), g, tc);
        let none = matches!(t.action, Action::None);
        let sub = if none { "Elige qué debe hacer →".to_string() } else { format!("→  {}", t.action.label()) };
        let g2 = p.layout(sub, egui::FontId::proportional(12.5), if none { AMBER } else { MUTED }, max_w);
        p.galley(Pos2::new(badge.right() + 14.0, r.top() + 38.0), g2, MUTED);
        let sw = Rect::from_min_size(Pos2::new(r.right() - 56.0, r.center().y - 12.0), Vec2::new(42.0, 24.0));
        let mut on = t.enabled;
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(sw));
        if toggle(&mut child, &mut on).changed() {
            t.enabled = on;
        }
        if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            app.select(Some(id));
            app.stop_recording();
        }
        ui.add_space(4.0);
    }
}

// ------------------------------------------------------------------ live trackpad

fn live_trackpad(ui: &mut egui::Ui) {
    let eng = engine();
    let touches = eng.touches.lock().unwrap().clone();
    let last = eng.last_event.lock().unwrap().clone();
    card(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            let (r, _) = ui.allocate_exact_size(Vec2::new(120.0, 78.0), Sense::hover());
            ui.painter().rect_filled(r, 12.0, Color32::from_black_alpha(60));
            ui.painter().rect_stroke(r, 12.0, Stroke::new(1.0, glass(40)), StrokeKind::Inside);
            let (a, b) = accent();
            for (x, y) in &touches {
                let p = Pos2::new(r.left() + x * r.width(), r.bottom() - y * r.height());
                glow(ui.painter(), p, 16.0, Color32::from_rgba_unmultiplied(a.r(), a.g(), a.b(), 160));
                ui.painter().circle_filled(p, 5.5, b);
            }
            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 3.0;
                ui.label(RichText::new("Prueba tus gestos aquí").strong());
                let pads = eng.trackpads.load(Ordering::Relaxed);
                match &last {
                    Some(l) if l.at.elapsed().as_secs() < 8 => {
                        ui.label(RichText::new(&l.what).color(Color32::WHITE).size(14.0));
                        ui.label(RichText::new(l.action.clone().map(|a| format!("→ {a}")).unwrap_or_else(|| "Este gesto aún no tiene regla".into())).color(MUTED).size(12.0));
                    }
                    _ => {
                        ui.label(RichText::new("Pon los dedos en el trackpad: verás qué gesto reconoce MacControl.").color(MUTED).size(12.5));
                        if pads == 0 && eng.started.load(Ordering::Relaxed) {
                            ui.label(RichText::new("No se ha detectado ningún trackpad.").color(AMBER).size(12.0));
                        }
                    }
                }
            });
        });
    });
}

// ------------------------------------------------------------------ editor

fn editor(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    let Some(id) = app.selected else { return };
    let Some(idx) = cfg.triggers.iter().position(|t| t.id == id) else {
        app.selected = None;
        return;
    };
    let mut delete = false;
    let apps_choice: Vec<(String, String)> = {
        let mut v = cfg.apps.clone();
        for a in &app.running_apps {
            if !v.iter().any(|(b, _)| *b == a.0) {
                v.push(a.clone());
            }
        }
        v
    };
    let rec = app.rec;
    let mut start_rec: Option<RecTarget> = None;
    let mut stop_rec = false;
    {
        let t = &mut cfg.triggers[idx];
        card(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(RichText::new("Editar regla").size(17.0).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    toggle(ui, &mut t.enabled);
                    ui.label(RichText::new(if t.enabled { "Activada" } else { "Desactivada" }).size(12.0).color(MUTED));
                });
            });
            ui.add_space(10.0);

            let hint1 = match t.kind {
                TriggerKind::Trackpad(_) => "El gesto que harás en el trackpad",
                TriggerKind::Keyboard(_) => "La combinación de teclas que pulsarás",
                TriggerKind::Mouse(_) => "El botón o la rueda que usarás",
            };
            step_title(ui, 1, "Cuando yo…", hint1);
            ui.add_space(6.0);
            ui.indent("s1", |ui| match &mut t.kind {
                TriggerKind::Trackpad(g) => gesture_picker(ui, g),
                TriggerKind::Keyboard(k) => {
                    let recording = rec == Some(RecTarget::TriggerKey(id));
                    if recording {
                        recording_box(ui, "Pulsa ahora la combinación de teclas…");
                    } else if k.key == 0xFFFF {
                        ui.label(RichText::new("Todavía no hay combinación").color(AMBER));
                    } else {
                        keycaps(ui, &combo_parts(k.display_mods(), Some(key_name(k.key))), true);
                        if let Some((arrow, name)) = fn_arrow_key(k.key) {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(format!(
                                    "⚠ En el Mac, fn + {arrow} es la tecla {name}. Mientras esta regla esté activa, {name} dejará de funcionar en todas las apps."
                                ))
                                .size(12.0)
                                .color(AMBER),
                            );
                        }
                    }
                    ui.add_space(6.0);
                    if recording {
                        if ghost_button(ui, "Cancelar").clicked() {
                            stop_rec = true;
                        }
                    } else if ghost_button(ui, "●  Grabar combinación").clicked() {
                        start_rec = Some(RecTarget::TriggerKey(id));
                    }
                }
                TriggerKind::Mouse(m) => mouse_picker(ui, m, rec == Some(RecTarget::TriggerMouse(id)), &mut start_rec, &mut stop_rec, id),
            });
            divider(ui);

            step_title(ui, 2, "MacControl hará…", "Lo que pasará cada vez");
            ui.add_space(6.0);
            ui.indent("s2", |ui| action_editor(ui, &mut t.action, rec == Some(RecTarget::ActionKey(id)), &mut start_rec, &mut stop_rec, id));
            divider(ui);

            step_title(ui, 3, "Funciona en…", "Todas las apps o solo una");
            ui.add_space(6.0);
            ui.indent("s3", |ui| {
                let cur = match &t.app {
                    None => "Todas las apps".to_string(),
                    Some(b) => apps_choice.iter().find(|(x, _)| x == b).map(|(_, n)| n.clone()).unwrap_or(b.clone()),
                };
                egui::ComboBox::from_id_salt("scope").selected_text(cur).width(240.0).show_ui(ui, |ui| {
                    ui.selectable_value(&mut t.app, None, "Todas las apps");
                    for (b, n) in &apps_choice {
                        ui.selectable_value(&mut t.app, Some(b.clone()), n);
                    }
                });
            });
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Los cambios se guardan solos.").size(11.5).color(FAINT));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if danger_button(ui, "Eliminar regla").clicked() {
                        delete = true;
                    }
                });
            });
        });
    }
    if let Some(r) = start_rec {
        app.start_recording(r);
    }
    if stop_rec {
        app.stop_recording();
    }
    if delete {
        cfg.triggers.retain(|t| t.id != id);
        let next = cfg.triggers.iter().find(|t| app.page.matches(&t.kind) && t.app == app.scope).map(|t| t.id);
        app.select(next);
        app.stop_recording();
    }
    // Keep the rule visible in its list when its app changed.
    if let Some(t) = cfg.triggers.iter().find(|t| t.id == id) {
        if t.app != app.scope {
            app.scope = t.app.clone();
            if let Some(b) = &t.app {
                if !cfg.apps.iter().any(|(x, _)| x == b) {
                    let name = apps_choice.iter().find(|(x, _)| x == b).map(|(_, n)| n.clone()).unwrap_or(b.clone());
                    cfg.apps.push((b.clone(), name));
                }
            }
        }
    }
}

fn divider(ui: &mut egui::Ui) {
    ui.add_space(12.0);
    let w = ui.available_width();
    let (r, _) = ui.allocate_exact_size(Vec2::new(w, 1.0), Sense::hover());
    ui.painter().rect_filled(r, 0.0, glass(18));
    ui.add_space(12.0);
}

fn recording_box(ui: &mut egui::Ui, text: &str) {
    let t = ui.input(|i| i.time) as f32;
    let a = 0.55 + 0.45 * (t * 4.0).sin().abs();
    let (ac, _) = accent();
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(ac.r(), ac.g(), ac.b(), 40))
        .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(ac.r(), ac.g(), ac.b(), (200.0 * a) as u8)))
        .corner_radius(12)
        .inner_margin(10)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.painter().circle_filled(ui.cursor().min + Vec2::new(6.0, 9.0), 5.0, RED.gamma_multiply(a));
                ui.add_space(16.0);
                ui.label(RichText::new(text).color(TEXT));
            });
        });
    ui.ctx().request_repaint();
}

fn gesture_picker(ui: &mut egui::Ui, g: &mut Gesture) {
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(Vec2::new(96.0, 64.0), Sense::hover());
        draw_gesture(ui, r, *g);
        ui.vertical(|ui| {
            ui.label(RichText::new(g.label()).size(15.0).strong());
            egui::ComboBox::from_id_salt("gesture").selected_text("Elegir otro gesto").width(220.0).height(380.0).show_ui(ui, |ui| {
                for (group, items) in Gesture::catalogue() {
                    ui.label(RichText::new(group).size(11.0).color(FAINT));
                    for it in items {
                        ui.selectable_value(g, it, it.label());
                    }
                    ui.separator();
                }
            });
        });
    });
}

/// A small trackpad drawing with fingers and the movement.
fn draw_gesture(ui: &egui::Ui, r: Rect, g: Gesture) {
    let p = ui.painter();
    p.rect_filled(r, 10.0, Color32::from_black_alpha(60));
    p.rect_stroke(r, 10.0, Stroke::new(1.0, glass(45)), StrokeKind::Inside);
    let c = r.center();
    let n = g.fingers() as usize;
    let (a, b) = accent();
    let finger = |pos: Pos2| {
        glow(p, pos, 12.0, Color32::from_rgba_unmultiplied(a.r(), a.g(), a.b(), 150));
        p.circle_filled(pos, 4.5, b);
    };
    match g {
        Gesture::CornerTap(corner) => {
            let pos = match corner {
                Corner::TopLeft => r.left_top() + Vec2::new(12.0, 12.0),
                Corner::TopRight => r.right_top() + Vec2::new(-12.0, 12.0),
                Corner::BottomLeft => r.left_bottom() + Vec2::new(12.0, -12.0),
                Corner::BottomRight => r.right_bottom() + Vec2::new(-12.0, -12.0),
            };
            finger(pos);
        }
        Gesture::EdgeSwipe(d) => {
            let (start, v) = match d {
                Dir::Left => (Pos2::new(r.left() + 8.0, c.y), Vec2::new(30.0, 0.0)),
                Dir::Right => (Pos2::new(r.right() - 8.0, c.y), Vec2::new(-30.0, 0.0)),
                Dir::Up => (Pos2::new(c.x, r.top() + 8.0), Vec2::new(0.0, 22.0)),
                Dir::Down => (Pos2::new(c.x, r.bottom() - 8.0), Vec2::new(0.0, -22.0)),
            };
            finger(start);
            p.arrow(start + v * 0.3, v, Stroke::new(2.0, TEXT));
        }
        Gesture::TipTap(side) => {
            p.circle_filled(c, 4.5, MUTED);
            let dx = if side == Side::Left { -18.0 } else { 18.0 };
            finger(c + Vec2::new(dx, 0.0));
        }
        _ => {
            let spread = 12.0;
            let x0 = c.x - spread * (n as f32 - 1.0) / 2.0;
            for i in 0..n {
                finger(Pos2::new(x0 + i as f32 * spread, c.y));
            }
        }
    }
    if let Gesture::Swipe(_, d) = g {
        let v = match d {
            Dir::Up => Vec2::new(0.0, -18.0),
            Dir::Down => Vec2::new(0.0, 18.0),
            Dir::Left => Vec2::new(-26.0, 0.0),
            Dir::Right => Vec2::new(26.0, 0.0),
        };
        let from = c + v * 0.2 + if v.y == 0.0 { Vec2::new(0.0, 14.0) } else { Vec2::new(30.0, 0.0) };
        p.arrow(from, v, Stroke::new(2.0, TEXT));
    }
    if matches!(g, Gesture::Click(_)) {
        p.text(r.center_bottom() + Vec2::new(0.0, -9.0), egui::Align2::CENTER_CENTER, "clic", egui::FontId::proportional(10.0), MUTED);
    }
    if matches!(g, Gesture::DoubleTap(_)) {
        p.text(r.center_bottom() + Vec2::new(0.0, -9.0), egui::Align2::CENTER_CENTER, "×2", egui::FontId::proportional(10.0), MUTED);
    }
}

fn mods_chips(ui: &mut egui::Ui, mods: &mut u64) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for (bit, label) in [(MOD_CTRL, "⌃ control"), (MOD_OPT, "⌥ opción"), (MOD_SHIFT, "⇧ mayús"), (MOD_CMD, "⌘ comando")] {
            if chip(ui, label, *mods & bit != 0).clicked() {
                *mods ^= bit;
            }
        }
    });
}

fn mouse_picker(ui: &mut egui::Ui, m: &mut MouseTrigger, recording: bool, start: &mut Option<RecTarget>, stop: &mut bool, id: u64) {
    let is_scroll = matches!(m, MouseTrigger::Scroll { .. });
    ui.horizontal(|ui| {
        if chip(ui, "Un botón", !is_scroll).clicked() && is_scroll {
            *m = MouseTrigger::Button { button: 2, mods: 0 };
        }
        if chip(ui, "La rueda + teclas", is_scroll).clicked() && !is_scroll {
            *m = MouseTrigger::Scroll { dir: Wheel::Up, mods: MOD_OPT };
        }
    });
    ui.add_space(6.0);
    match m {
        MouseTrigger::Button { button, mods } => {
            if recording {
                recording_box(ui, "Pulsa ahora un botón del ratón (central, lateral…)");
            } else {
                ui.label(RichText::new(MouseTrigger::Button { button: *button, mods: *mods }.label()).size(15.0).strong());
            }
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("button").selected_text(button_name(*button)).width(170.0).show_ui(ui, |ui| {
                    for b in 2..=7u8 {
                        ui.selectable_value(button, b, button_name(b));
                    }
                });
                if recording {
                    if ghost_button(ui, "Cancelar").clicked() {
                        *stop = true;
                    }
                } else if ghost_button(ui, "●  Grabar botón").clicked() {
                    *start = Some(RecTarget::TriggerMouse(id));
                }
            });
            ui.add_space(4.0);
            ui.label(RichText::new("Mientras mantengo pulsadas (opcional)").size(12.0).color(MUTED));
            mods_chips(ui, mods);
        }
        MouseTrigger::Scroll { dir, mods } => {
            ui.horizontal(|ui| {
                if chip(ui, "Rueda hacia arriba", *dir == Wheel::Up).clicked() {
                    *dir = Wheel::Up;
                }
                if chip(ui, "Rueda hacia abajo", *dir == Wheel::Down).clicked() {
                    *dir = Wheel::Down;
                }
            });
            ui.add_space(4.0);
            ui.label(RichText::new("Mientras mantengo pulsada al menos una de estas teclas").size(12.0).color(MUTED));
            mods_chips(ui, mods);
            if *mods == 0 {
                ui.label(RichText::new("Elige al menos una tecla: la rueda sola sigue desplazando como siempre.").size(12.0).color(AMBER));
            }
        }
    }
}

const ACTION_KINDS: [&str; 8] = [
    "Nada todavía",
    "Pulsar un atajo de teclado",
    "Colocar la ventana",
    "Una acción del sistema",
    "Abrir una app",
    "Abrir un enlace o archivo",
    "Escribir un texto",
    "Ejecutar un comando",
];

fn action_editor(ui: &mut egui::Ui, a: &mut Action, recording: bool, start: &mut Option<RecTarget>, stop: &mut bool, id: u64) {
    let mut kind = a.kind_index();
    egui::ComboBox::from_id_salt("action_kind").selected_text(ACTION_KINDS[kind]).width(260.0).show_ui(ui, |ui| {
        for (i, k) in ACTION_KINDS.iter().enumerate() {
            ui.selectable_value(&mut kind, i, *k);
        }
    });
    if kind != a.kind_index() {
        *a = match kind {
            0 => Action::None,
            1 => Action::Shortcut(KeyCombo { key: 0xFFFF, mods: 0 }),
            2 => Action::Window(WindowPos::LeftHalf),
            3 => Action::System(SystemAction::MissionControl),
            4 => Action::OpenApp(String::new()),
            5 => Action::OpenUrl(String::new()),
            6 => Action::TypeText(String::new()),
            _ => Action::Shell(String::new()),
        };
        if kind == 1 {
            *start = Some(RecTarget::ActionKey(id));
        }
    }
    ui.add_space(8.0);
    match a {
        Action::None => {
            ui.label(RichText::new("Elige arriba qué quieres que pase.").color(AMBER));
        }
        Action::Shortcut(k) => {
            if recording {
                recording_box(ui, "Pulsa ahora el atajo que MacControl debe enviar…");
            } else if k.key == 0xFFFF {
                ui.label(RichText::new("Graba el atajo que se enviará").color(MUTED));
            } else {
                keycaps(ui, &combo_parts(k.display_mods(), Some(key_name(k.key))), true);
            }
            ui.add_space(6.0);
            if recording {
                if ghost_button(ui, "Cancelar").clicked() {
                    *stop = true;
                }
            } else if ghost_button(ui, "●  Grabar atajo").clicked() {
                *start = Some(RecTarget::ActionKey(id));
            }
        }
        Action::Window(pos) => window_grid(ui, pos),
        Action::System(s) => {
            egui::ComboBox::from_id_salt("system").selected_text(s.label()).width(260.0).height(420.0).show_ui(ui, |ui| {
                for group in ["Sistema", "Ventanas", "Sonido", "Ratón"] {
                    ui.label(RichText::new(group).size(11.0).color(FAINT));
                    for it in SystemAction::ALL.iter().filter(|x| x.group() == group) {
                        ui.selectable_value(s, *it, it.label());
                    }
                    ui.separator();
                }
            });
        }
        Action::OpenApp(name) => {
            ui.label(RichText::new("Nombre de la app (por ejemplo «Safari» o «Notas»)").size(12.0).color(MUTED));
            ui.add(egui::TextEdit::singleline(name).hint_text("Safari").desired_width(f32::INFINITY));
        }
        Action::OpenUrl(u) => {
            ui.label(RichText::new("Dirección web o ruta de un archivo o carpeta").size(12.0).color(MUTED));
            ui.add(egui::TextEdit::singleline(u).hint_text("https://www.apple.com/es").desired_width(f32::INFINITY));
        }
        Action::TypeText(t) => {
            ui.label(RichText::new("Texto que se escribirá donde esté el cursor").size(12.0).color(MUTED));
            ui.add(egui::TextEdit::multiline(t).desired_rows(3).desired_width(f32::INFINITY));
        }
        Action::Shell(c) => {
            ui.label(RichText::new("Orden de Terminal (se ejecuta con sh)").size(12.0).color(MUTED));
            ui.add(egui::TextEdit::singleline(c).code_editor().hint_text("say hola").desired_width(f32::INFINITY));
        }
    }
}

/// Grid of small screen pictures, one per window position.
fn window_grid(ui: &mut egui::Ui, pos: &mut WindowPos) {
    let s = SnapSettings::default();
    let cols = ((ui.available_width() + 8.0) / 74.0).floor().max(3.0) as usize;
    let (a, b) = accent();
    egui::Grid::new("wingrid").spacing(Vec2::new(8.0, 8.0)).show(ui, |ui| {
        for (i, p) in WindowPos::ALL.iter().enumerate() {
            let (r, resp) = ui.allocate_exact_size(Vec2::new(66.0, 54.0), Sense::click());
            let sel = *pos == *p;
            let h = ui.ctx().animate_bool_with_time(resp.id.with("h"), resp.hovered() || sel, 0.15);
            ui.painter().rect_filled(r, 10.0, glass((12.0 + 16.0 * h) as u8));
            if sel {
                ui.painter().rect_stroke(r, 10.0, Stroke::new(1.5, lerp_color(a, Color32::WHITE, 0.3)), StrokeKind::Inside);
            }
            let screen = Rect::from_min_size(r.min + Vec2::new(5.0, 5.0), Vec2::new(56.0, 33.0));
            ui.painter().rect_stroke(screen, 4.0, Stroke::new(1.0, glass(70)), StrokeKind::Inside);
            let t = unit_target(*p, &s, Some(R::new(0.0, 0.0, 500.0, 600.0)));
            let inner = screen.shrink(2.5);
            let tr = Rect::from_min_size(
                inner.min + Vec2::new(t.x * inner.width(), t.y * inner.height()),
                Vec2::new(t.w * inner.width(), t.h * inner.height()),
            );
            if sel {
                gradient_rounded(ui.painter(), tr.shrink(0.5), 2.0, a, b);
            } else {
                ui.painter().rect_filled(tr.shrink(0.5), 2.0, glass(70));
            }
            ui.painter().text(r.center_bottom() + Vec2::new(0.0, -8.0), egui::Align2::CENTER_CENTER, short_pos(*p), egui::FontId::proportional(9.5), if sel { TEXT } else { MUTED });
            if resp.on_hover_text(p.label()).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                *pos = *p;
            }
            if (i + 1) % cols == 0 {
                ui.end_row();
            }
        }
    });
    ui.add_space(4.0);
    ui.label(RichText::new(format!("La ventana activa irá a: {}", pos.label())).color(MUTED).size(12.0));
}

/// Where a window would go, as fractions of a 1000-point screen (for the small drawings).
fn unit_target(p: WindowPos, s: &SnapSettings, current: Option<R>) -> R {
    let t = target_rect(p, R::new(0.0, 0.0, 1000.0, 1000.0), s, current);
    R::new(t.x / 1000.0, t.y / 1000.0, t.w / 1000.0, t.h / 1000.0)
}

fn short_pos(p: WindowPos) -> &'static str {
    match p {
        WindowPos::Maximize => "Maximizar",
        WindowPos::LeftHalf => "Mitad izq.",
        WindowPos::RightHalf => "Mitad dcha.",
        WindowPos::TopHalf => "Arriba",
        WindowPos::BottomHalf => "Abajo",
        WindowPos::TopLeft => "Sup. izq.",
        WindowPos::TopRight => "Sup. dcha.",
        WindowPos::BottomLeft => "Inf. izq.",
        WindowPos::BottomRight => "Inf. dcha.",
        WindowPos::LeftThird => "1/3 izq.",
        WindowPos::CenterThird => "1/3 centro",
        WindowPos::RightThird => "1/3 dcha.",
        WindowPos::LeftTwoThirds => "2/3 izq.",
        WindowPos::RightTwoThirds => "2/3 dcha.",
        WindowPos::Center => "Centrar",
    }
}

// ------------------------------------------------------------------ snapping page

pub fn snapping(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    let s = &mut cfg.snap;
    let avail = ui.available_size();
    ui.horizontal_top(|ui| {
        let left_w = (avail.x * 0.56).max(380.0);
        ui.allocate_ui_with_layout(Vec2::new(left_w, avail.y), egui::Layout::top_down(egui::Align::Min), |ui| {
            egui::ScrollArea::vertical().id_salt("snapl").show(ui, |ui| {
                card(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            ui.label(RichText::new("Ajustar al arrastrar").size(16.0).strong());
                            ui.label(RichText::new("Arrastra una ventana por su barra de título hasta un borde y suéltala.").color(MUTED).size(12.5));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            toggle(ui, &mut s.enabled);
                        });
                    });
                    ui.add_space(12.0);
                    screen_diagram(app, ui, s);
                    ui.add_space(10.0);
                    ui.label(RichText::new("Qué bordes funcionan").size(12.0).color(MUTED));
                    ui.horizontal_wrapped(|ui| {
                        if chip(ui, "◧  Izquierda: mitad", s.left).clicked() {
                            s.left = !s.left;
                        }
                        if chip(ui, "◨  Derecha: mitad", s.right).clicked() {
                            s.right = !s.right;
                        }
                        if chip(ui, "⬒  Arriba: maximizar", s.top).clicked() {
                            s.top = !s.top;
                        }
                        if chip(ui, "◰  Esquinas: cuartos", s.corners).clicked() {
                            s.corners = !s.corners;
                        }
                    });
                });
                ui.add_space(10.0);
                card(ui, |ui| {
                    ui.set_width(ui.available_width());
                    section_title(ui, "Excepciones");
                    ui.label(RichText::new("Apps en las que las ventanas no se ajustan").color(MUTED).size(12.0));
                    ui.add_space(4.0);
                    let mut remove = None;
                    ui.horizontal_wrapped(|ui| {
                        for b in &s.excluded_apps {
                            let n = app_name(&Config::default(), &app.running_apps, b);
                            if chip(ui, &format!("{n}  ×"), false).on_hover_text("Quitar").clicked() {
                                remove = Some(b.clone());
                            }
                        }
                        let add = ghost_button(ui, "+ Añadir app");
                        egui::Popup::menu(&add).show(|ui| {
                            egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                                for (b, n) in &app.running_apps {
                                    if !s.excluded_apps.contains(b) && ui.button(n).clicked() {
                                        s.excluded_apps.push(b.clone());
                                        ui.close();
                                    }
                                }
                            });
                        });
                    });
                    if let Some(r) = remove {
                        s.excluded_apps.retain(|x| *x != r);
                    }
                });
            });
        });
        ui.add_space(12.0);
        ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), avail.y), egui::Layout::top_down(egui::Align::Min), |ui| {
            egui::ScrollArea::vertical().id_salt("snapopts").show(ui, |ui| {
                card(ui, |ui| {
                    ui.set_width(ui.available_width());
                    step_title(ui, 1, "Tamaños", "Cuánto ocupa la ventana al ajustarse");
                    ui.add_space(6.0);
                    slider(ui, "Mitad izquierda", &mut s.left_width, 0.2..=0.8);
                    slider(ui, "Mitad derecha", &mut s.right_width, 0.2..=0.8);
                    slider(ui, "Cuartos (esquinas)", &mut s.corner_width, 0.2..=0.8);
                    px_slider(ui, "Margen alrededor", &mut s.gap, 0.0..=40.0);
                });
                ui.add_space(10.0);
                card(ui, |ui| {
                    ui.set_width(ui.available_width());
                    step_title(ui, 2, "Detección", "Qué tan cerca del borde hay que llegar");
                    ui.add_space(6.0);
                    px_slider(ui, "Zona de las esquinas", &mut s.corner_area, 5.0..=150.0);
                    px_slider(ui, "Ampliar los bordes", &mut s.recognition_extra, 0.0..=40.0);
                    switch_row(ui, "Esperar un momento antes de ajustar", "Evita ajustes al rozar un borde sin querer", &mut s.delay_enabled);
                    if s.delay_enabled {
                        ui.add(egui::Slider::new(&mut s.delay, 0.05..=1.0).suffix(" s").step_by(0.05));
                    }
                });
                ui.add_space(10.0);
                card(ui, |ui| {
                    ui.set_width(ui.available_width());
                    step_title(ui, 3, "Comportamiento", "");
                    ui.add_space(6.0);
                    switch_row(ui, "Recuperar el tamaño al separarla", "Al sacarla del borde vuelve a su tamaño de antes", &mut s.snap_back);
                    ui.add_space(4.0);
                    switch_row(ui, "Mostrar vista previa", "Un rectángulo indica dónde quedará la ventana", &mut s.show_preview);
                    if s.show_preview {
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Color").color(MUTED));
                            for c in [[124u8, 108, 255, 70], [61, 214, 140, 70], [255, 181, 71, 70], [80, 170, 255, 70], [255, 107, 150, 70]] {
                                let (r, resp) = ui.allocate_exact_size(Vec2::splat(24.0), Sense::click());
                                ui.painter().circle_filled(r.center(), 9.0, Color32::from_rgb(c[0], c[1], c[2]));
                                if s.preview_color == c {
                                    ui.painter().circle_stroke(r.center(), 11.5, Stroke::new(1.5, TEXT));
                                }
                                if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                                    s.preview_color = c;
                                }
                            }
                        });
                        ui.add(egui::Slider::new(&mut s.preview_animation, 0.0..=0.6).suffix(" s").text("Animación"));
                    }
                    ui.add_space(8.0);
                    if ghost_button(ui, "Volver a los valores originales").clicked() {
                        let excluded = s.excluded_apps.clone();
                        *s = SnapSettings::default();
                        s.excluded_apps = excluded;
                    }
                });
            });
        });
    });
}

fn slider(ui: &mut egui::Ui, label: &str, v: &mut f32, range: std::ops::RangeInclusive<f32>) {
    ui.label(RichText::new(label).color(TEXT));
    let mut pct = *v * 100.0;
    let r = (*range.start() * 100.0)..=(*range.end() * 100.0);
    if ui.add(egui::Slider::new(&mut pct, r).suffix(" %").step_by(1.0)).changed() {
        *v = pct / 100.0;
    }
    ui.add_space(2.0);
}

fn px_slider(ui: &mut egui::Ui, label: &str, v: &mut f32, range: std::ops::RangeInclusive<f32>) {
    ui.label(RichText::new(label).color(TEXT));
    ui.add(egui::Slider::new(v, range).suffix(" px").step_by(1.0));
    ui.add_space(2.0);
}

/// A monitor with its snap zones; hovering a zone shows where the window would go.
fn screen_diagram(app: &mut MacApp, ui: &mut egui::Ui, s: &SnapSettings) {
    let w = ui.available_width().min(580.0);
    let h = w * 0.58;
    let (outer, _) = ui.allocate_exact_size(Vec2::new(w, h + 26.0), Sense::hover());
    let (a, b) = accent();
    let bezel = Rect::from_min_size(outer.min, Vec2::new(w, h));
    {
        let p = ui.painter();
        p.add(egui::Shadow { offset: [0, 12], blur: 30, spread: 0, color: Color32::from_black_alpha(90) }.as_shape(bezel, 16.0));
        p.rect_filled(bezel, 16.0, Color32::from_rgb(10, 11, 18));
        p.rect_stroke(bezel, 16.0, Stroke::new(1.0, glass(40)), StrokeKind::Inside);
        let neck = Rect::from_center_size(Pos2::new(bezel.center().x, bezel.bottom() + 9.0), Vec2::new(w * 0.12, 18.0));
        corner_gradient(p, neck, [glass(40), glass(40), glass(20), glass(20)]);
        let foot = Rect::from_center_size(Pos2::new(bezel.center().x, bezel.bottom() + 21.0), Vec2::new(w * 0.3, 6.0));
        p.rect_filled(foot, 3.0, glass(45));
    }
    let screen = bezel.shrink(10.0);
    {
        let p = ui.painter().with_clip_rect(screen);
        corner_gradient(&p, screen, [lerp_color(a, Color32::BLACK, 0.55), lerp_color(b, Color32::BLACK, 0.6), lerp_color(b, Color32::BLACK, 0.75), lerp_color(a, Color32::BLACK, 0.7)]);
        glow(&p, screen.left_top() + Vec2::new(w * 0.2, h * 0.25), w * 0.4, Color32::from_rgba_unmultiplied(a.r(), a.g(), a.b(), 60));
    }
    let p = ui.painter();
    let menubar = Rect::from_min_size(screen.min, Vec2::new(screen.width(), 12.0));
    p.rect_filled(menubar, 0.0, Color32::from_black_alpha(90));
    let visible = Rect::from_min_max(Pos2::new(screen.left(), menubar.bottom()), screen.max);
    let e = 9.0;
    let c = (s.corner_area / 30.0 * 24.0).clamp(10.0, 60.0);
    let mut zones: Vec<(WindowPos, Rect, bool)> = vec![
        (WindowPos::LeftHalf, Rect::from_min_size(screen.min, Vec2::new(e, screen.height())), s.left),
        (WindowPos::RightHalf, Rect::from_min_size(Pos2::new(screen.right() - e, screen.top()), Vec2::new(e, screen.height())), s.right),
        (WindowPos::Maximize, Rect::from_min_size(screen.min, Vec2::new(screen.width(), e)), s.top),
    ];
    if s.corners {
        zones.push((WindowPos::TopLeft, Rect::from_min_size(screen.min, Vec2::splat(c)), true));
        zones.push((WindowPos::TopRight, Rect::from_min_size(Pos2::new(screen.right() - c, screen.top()), Vec2::splat(c)), true));
        zones.push((WindowPos::BottomLeft, Rect::from_min_size(Pos2::new(screen.left(), screen.bottom() - c), Vec2::splat(c)), true));
        zones.push((WindowPos::BottomRight, Rect::from_min_size(screen.max - Vec2::splat(c), Vec2::splat(c)), true));
    }
    let hover_pos = ui.input(|i| i.pointer.hover_pos());
    let mut hovered = None;
    for (pos, r, on) in zones.iter().rev() {
        if *on && hover_pos.map(|h| r.contains(h)).unwrap_or(false) {
            hovered = Some(*pos);
            break;
        }
    }
    if hovered.is_some() {
        app.snap_hover = hovered;
    }
    let shown = app.snap_hover.unwrap_or(WindowPos::LeftHalf);
    if s.enabled {
        let t = unit_target(shown, s, None);
        let goal = Rect::from_min_size(
            visible.min + Vec2::new(t.x * visible.width(), t.y * visible.height()),
            Vec2::new(t.w * visible.width(), t.h * visible.height()),
        )
        .shrink(4.0);
        // Animate the preview between zones.
        let ctx = ui.ctx();
        let ax = ctx.animate_value_with_time(ui.make_persistent_id("dg_x0"), goal.min.x, 0.22);
        let ay = ctx.animate_value_with_time(ui.make_persistent_id("dg_y0"), goal.min.y, 0.22);
        let bx = ctx.animate_value_with_time(ui.make_persistent_id("dg_x1"), goal.max.x, 0.22);
        let by = ctx.animate_value_with_time(ui.make_persistent_id("dg_y1"), goal.max.y, 0.22);
        let tr = Rect::from_min_max(Pos2::new(ax, ay), Pos2::new(bx, by));
        let [r, g, bb, _] = s.preview_color;
        let col = Color32::from_rgb(r, g, bb);
        p.rect_filled(tr, 8.0, Color32::from_rgba_unmultiplied(r, g, bb, 70));
        p.rect_stroke(tr, 8.0, Stroke::new(2.0, col), StrokeKind::Inside);
        p.text(tr.center(), egui::Align2::CENTER_CENTER, shown.label(), egui::FontId::proportional(13.5), Color32::WHITE);
    }
    for (pos, r, on) in &zones {
        if !*on || !s.enabled {
            p.rect_filled(*r, 3.0, glass(12));
        } else if Some(*pos) == hovered || *pos == shown {
            gradient_rounded(p, *r, 3.0, a, b);
        } else {
            p.rect_filled(*r, 3.0, Color32::from_rgba_unmultiplied(a.r(), a.g(), a.b(), 110));
        }
    }
    ui.label(RichText::new("Pasa el ratón por las zonas de color para ver dónde quedaría la ventana.").size(11.5).color(FAINT));
}

// ------------------------------------------------------------------ settings page

pub fn settings(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    let eng = engine();
    let trusted = app.permissions_ok() && app.screenshot.is_none();
    let tap = eng.tap_ok.load(Ordering::Relaxed);
    let pads = eng.trackpads.load(Ordering::Relaxed);
    let front = eng.front_app.read().unwrap().1.clone();
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.columns(2, |cols| {
            card(&mut cols[0], |ui| {
                ui.set_width(ui.available_width());
                section_title(ui, "Permisos");
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Accesibilidad").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if trusted {
                            pill(ui, "Concedido", GREEN);
                        } else {
                            pill(ui, "Pendiente", AMBER);
                        }
                    });
                });
                ui.label(
                    RichText::new("MacControl lo necesita para leer atajos, botones del ratón y para mover las ventanas de otras apps.")
                        .size(12.0)
                        .color(MUTED),
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if primary_button(ui, "Pedir permiso").clicked() {
                        crate::mac::ax::request_trust();
                    }
                    if ghost_button(ui, "Abrir Ajustes del Sistema").clicked() {
                        let _ = std::process::Command::new("/usr/bin/open")
                            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
                            .spawn();
                    }
                });
            });
            cols[0].add_space(10.0);
            card(&mut cols[0], |ui| {
                ui.set_width(ui.available_width());
                section_title(ui, "Estado");
                ui.add_space(4.0);
                status_row(ui, "Teclado y ratón", if tap { ("Escuchando", GREEN) } else { ("Detenido", AMBER) });
                status_row(ui, "Trackpads", if pads > 0 { (if pads == 1 { "1 detectado" } else { "Varios detectados" }, GREEN) } else { ("Ninguno", AMBER) });
                status_row(ui, "Ajuste de ventanas", if cfg.snap.enabled { ("Activado", GREEN) } else { ("Desactivado", MUTED) });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("App en primer plano").color(MUTED));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(if front.is_empty() { "—".to_string() } else { front.clone() });
                    });
                });
            });
            card(&mut cols[1], |ui| {
                ui.set_width(ui.available_width());
                section_title(ui, "General");
                ui.add_space(4.0);
                let mut active = !cfg.paused;
                if switch_row(ui, "MacControl activo", "Pausa todas las reglas y el ajuste de ventanas", &mut active) {
                    cfg.paused = !active;
                }
                ui.add_space(6.0);
                switch_row(ui, "Sonido al hacer clic", "Un pequeño «bip» al pulsar dentro de MacControl", &mut cfg.click_sound);
                ui.add_space(8.0);
                ui.label(RichText::new("Al cerrar la ventana, MacControl sigue funcionando desde su icono en la barra de menús.").size(12.0).color(MUTED));
                ui.add_space(6.0);
                ui.label(RichText::new("Configuración guardada en").size(11.5).color(FAINT));
                ui.label(RichText::new(home_relative(&Config::path())).size(11.5).color(MUTED));
                ui.add_space(8.0);
                if app.confirm_reset {
                    ui.label(RichText::new("¿Seguro? Se borrarán todas tus reglas.").color(AMBER));
                    ui.horizontal(|ui| {
                        if danger_button(ui, "Sí, restablecer").clicked() {
                            *cfg = Config::default();
                            app.confirm_reset = false;
                            app.select(None);
                            app.scope = None;
                        }
                        if ghost_button(ui, "Cancelar").clicked() {
                            app.confirm_reset = false;
                        }
                    });
                } else if danger_button(ui, "Restablecer todo").clicked() {
                    app.confirm_reset = true;
                }
            });
            cols[1].add_space(10.0);
            card(&mut cols[1], |ui| {
                ui.set_width(ui.available_width());
                section_title(ui, "Acerca de");
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover());
                    app.draw_logo(ui, r);
                    ui.vertical(|ui| {
                        ui.label(RichText::new(concat!("MacControl ", env!("CARGO_PKG_VERSION"))).strong().size(15.0));
                        ui.label(RichText::new("Trackpad, teclado, ratón y ajuste de ventanas para macOS.").size(12.0).color(MUTED));
                    });
                });
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Sonido de clic: «audio-volume-change» del tema de sonidos de freedesktop.org, de Lucas McCallister (licencia CC-BY-SA).")
                        .size(11.0)
                        .color(FAINT),
                );
            });
        });
    });
}

fn status_row(ui: &mut egui::Ui, label: &str, (text, color): (&str, Color32)) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(MUTED));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| pill(ui, text, color));
    });
}

#[allow(dead_code)]
fn unused_icon(_: Icon) {}

/// A path with the home folder written as "~", the way macOS shows it.
fn home_relative(p: &std::path::Path) -> String {
    let full = p.display().to_string();
    match std::env::var("HOME") {
        Ok(home) if !home.is_empty() && full.starts_with(&home) => format!("~{}", &full[home.len()..]),
        _ => full,
    }
}
