//! The content of each section.

use super::icons::{self, Icon};
use super::theme::*;
use super::{MacApp, Page, RecTarget};
use crate::engine::{engine, Recorded};
use crate::i18n::{self, tl, tlf};
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
            if app.page.is_rules() && primary_button(ui, tl("+  New rule")).clicked() {
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
        child.label(RichText::new(tl("Welcome to MacControl")).size(30.0).strong().color(TEXT));
        child.label(
            RichText::new(tl("Control your Mac with trackpad gestures, keyboard shortcuts and mouse buttons, and place windows just by dragging them to the edges of the screen."))
                .size(14.0)
                .color(MUTED),
        );
        child.add_space(4.0);
        child.horizontal(|ui| {
            let n = cfg.triggers.iter().filter(|t| t.enabled).count();
            pill(ui, &tlf("{} active rules", &[&n]), GREEN);
            if cfg.snap.enabled {
                pill(ui, tl("Window snapping on"), Color32::from_rgb(120, 220, 255));
            }
            if cfg.paused {
                pill(ui, tl("Paused"), AMBER);
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
                        ui.label(RichText::new(tl("First step: grant Accessibility permission")).size(15.0).strong());
                        ui.label(RichText::new(tl("Without it, macOS won't let MacControl read gestures and keys or move windows.")).color(MUTED));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if primary_button(ui, tl("Grant permission")).clicked() {
                            crate::mac::ax::request_trust();
                        }
                    });
                });
            });
            ui.add_space(14.0);
        }

        section_title(ui, tl("What do you want to set up?"));
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
                        tlf("{} active rules", &[&n])
                    } else if cfg.snap.enabled {
                        tl("On").to_string()
                    } else {
                        tl("Off").to_string()
                    };
                    pr.text(Pos2::new(r2.left() + 18.0, r2.bottom() - 22.0), egui::Align2::LEFT_CENTER, info, egui::FontId::proportional(12.0), FAINT);
                    let arrow_x = r2.right() - 22.0 + 4.0 * h;
                    pr.text(Pos2::new(arrow_x, r2.bottom() - 22.0), egui::Align2::RIGHT_CENTER, tl("Set up  →"), egui::FontId::proportional(12.5), lerp_color(MUTED, Color32::WHITE, h));
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
        section_title(ui, tl("How it works"));
        ui.add_space(4.0);
        let steps = [
            (tl("Choose what you do"), tl("A finger gesture, a key combination or a mouse button.")),
            (tl("Choose what happens"), tl("Place a window, open an app, send a shortcut, turn the volume up…")),
            (tl("That's it"), tl("It works in every app, or only in the ones you choose.")),
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
                section_title(ui, &tlf("Your rules ({})", &[&ids.len()]));
                ui.add_space(2.0);
                rule_list(app, ui, cfg, &ids);
                ui.add_space(10.0);
                ideas(app, ui, cfg, tl("More ideas"));
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

/// tl("Works in"): every app or one in particular.
fn scope_bar(app: &mut MacApp, ui: &mut egui::Ui, cfg: &mut Config) {
    egui::Frame::new()
        .fill(glass(9))
        .stroke(Stroke::new(1.0, glass(20)))
        .corner_radius(16)
        .inner_margin(egui::Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(tl("Works in")).color(MUTED));
                ui.add_space(4.0);
                if chip(ui, tl("All apps"), app.scope.is_none()).clicked() {
                    app.scope = None;
                    app.select(None);
                }
                let mut remove = None;
                for (b, n) in cfg.apps.clone() {
                    let on = app.scope.as_deref() == Some(b.as_str());
                    let r = chip(ui, &n, on).on_hover_text(tl("Right-click to remove it"));
                    if r.clicked() {
                        app.scope = Some(b.clone());
                        app.select(None);
                    }
                    r.context_menu(|ui| {
                        if ui.button(tl("Remove this app and its rules")).clicked() {
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
                let add = ghost_button(ui, tl("+ A specific app"));
                egui::Popup::menu(&add).show(|ui| {
                    ui.set_min_width(230.0);
                    ui.label(RichText::new(tl("Apps open now")).size(11.0).color(FAINT));
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        let running = app.running_apps.clone();
                        if running.is_empty() {
                            ui.label(RichText::new(tl("Open the app you want and come back here.")).color(MUTED));
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
                    ui.label(RichText::new(tl("Rules for a specific app take priority over general ones")).size(11.5).color(FAINT));
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
            let title = match &app.scope {
                None => tl("You have no rules for all apps yet").to_string(),
                Some(b) => tlf("You have no rules for {} yet", &[&app_name(cfg, &app.running_apps, b)]),
            };
            ui.label(RichText::new(title).size(18.0).strong());
            ui.label(RichText::new(tl("Start with one of these ideas or create your own.")).color(MUTED));
            ui.add_space(10.0);
            if primary_button(ui, tl("+  Create a rule")).clicked() {
                add_trigger(app, cfg, None);
            }
            ui.add_space(10.0);
        });
        ideas(app, ui, cfg, tl("Ideas to get started"));
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
            if ghost_button(ui, &text).on_hover_text(tl("Add this rule")).clicked() {
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
            Gesture::Tap(n) => tlf("{}-finger tap", &[n]),
            Gesture::DoubleTap(n) => tlf("{}-finger double tap", &[n]),
            Gesture::Click(n) => tlf("{}-finger click", &[n]),
            Gesture::Swipe(n, d) => tlf("{} fingers {}", &[n, &d.arrow()]),
            Gesture::CornerTap(_) => tl("Corner tap").into(),
            Gesture::EdgeSwipe(d) => tlf("1 finger from edge {}", &[&d.arrow()]),
            Gesture::TipTap(Side::Left) => tl("TipTap left").into(),
            Gesture::TipTap(Side::Right) => tl("TipTap right").into(),
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
        TriggerKind::Keyboard(k) if k.key == 0xFFFF => tl("New shortcut: press a combination").into(),
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
        let sub = if none { tl("Choose what it should do →").to_string() } else { format!("→  {}", t.action.label()) };
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
                ui.label(RichText::new(tl("Try your gestures here")).strong());
                let pads = eng.trackpads.load(Ordering::Relaxed);
                match &last {
                    Some(l) if l.at.elapsed().as_secs() < 8 => {
                        ui.label(RichText::new(&l.what).color(Color32::WHITE).size(14.0));
                        ui.label(RichText::new(l.action.clone().map(|a| format!("→ {a}")).unwrap_or_else(|| tl("This gesture has no rule yet").into())).color(MUTED).size(12.0));
                    }
                    _ => {
                        ui.label(RichText::new(tl("Put your fingers on the trackpad to see which gesture MacControl recognizes.")).color(MUTED).size(12.5));
                        if pads == 0 && eng.started.load(Ordering::Relaxed) {
                            ui.label(RichText::new(tl("No trackpad detected.")).color(AMBER).size(12.0));
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
                ui.label(RichText::new(tl("Edit rule")).size(17.0).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    toggle(ui, &mut t.enabled);
                    ui.label(RichText::new(if t.enabled { tl("Enabled") } else { tl("Disabled") }).size(12.0).color(MUTED));
                });
            });
            ui.add_space(10.0);

            let hint1 = match t.kind {
                TriggerKind::Trackpad(_) => tl("The gesture you'll make on the trackpad"),
                TriggerKind::Keyboard(_) => tl("The key combination you'll press"),
                TriggerKind::Mouse(_) => tl("The button or wheel you'll use"),
            };
            step_title(ui, 1, tl("When I…"), hint1);
            ui.add_space(6.0);
            ui.indent("s1", |ui| match &mut t.kind {
                TriggerKind::Trackpad(g) => gesture_picker(ui, g),
                TriggerKind::Keyboard(k) => {
                    let recording = rec == Some(RecTarget::TriggerKey(id));
                    if recording {
                        recording_box(ui, tl("Press the key combination now…"));
                    } else if k.key == 0xFFFF {
                        ui.label(RichText::new(tl("No combination yet")).color(AMBER));
                    } else {
                        keycaps(ui, &combo_parts(k.display_mods(), Some(key_name(k.key))), true);
                        if let Some((arrow, name)) = fn_arrow_key(k.key) {
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(tlf(
                                    "⚠ On a Mac, fn + {} is the {} key. While this rule is on, {} will stop working in every app.",
                                    &[&arrow, &name, &name],
                                ))
                                .size(12.0)
                                .color(AMBER),
                            );
                        }
                    }
                    ui.add_space(6.0);
                    if recording {
                        if ghost_button(ui, tl("Cancel")).clicked() {
                            stop_rec = true;
                        }
                    } else if ghost_button(ui, tl("●  Record combination")).clicked() {
                        start_rec = Some(RecTarget::TriggerKey(id));
                    }
                }
                TriggerKind::Mouse(m) => mouse_picker(ui, m, rec == Some(RecTarget::TriggerMouse(id)), &mut start_rec, &mut stop_rec, id),
            });
            divider(ui);

            step_title(ui, 2, tl("MacControl will…"), tl("What will happen each time"));
            ui.add_space(6.0);
            ui.indent("s2", |ui| action_editor(ui, &mut t.action, rec == Some(RecTarget::ActionKey(id)), &mut start_rec, &mut stop_rec, id));
            divider(ui);

            step_title(ui, 3, tl("Works in…"), tl("Every app or just one"));
            ui.add_space(6.0);
            ui.indent("s3", |ui| {
                let cur = match &t.app {
                    None => tl("All apps").to_string(),
                    Some(b) => apps_choice.iter().find(|(x, _)| x == b).map(|(_, n)| n.clone()).unwrap_or(b.clone()),
                };
                egui::ComboBox::from_id_salt("scope").selected_text(cur).width(240.0).show_ui(ui, |ui| {
                    ui.selectable_value(&mut t.app, None, tl("All apps"));
                    for (b, n) in &apps_choice {
                        ui.selectable_value(&mut t.app, Some(b.clone()), n);
                    }
                });
            });
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new(tl("Changes are saved automatically.")).size(11.5).color(FAINT));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if danger_button(ui, tl("Delete rule")).clicked() {
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
            egui::ComboBox::from_id_salt("gesture").selected_text(tl("Choose another gesture")).width(220.0).height(380.0).show_ui(ui, |ui| {
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
        p.text(r.center_bottom() + Vec2::new(0.0, -9.0), egui::Align2::CENTER_CENTER, tl("click"), egui::FontId::proportional(10.0), MUTED);
    }
    if matches!(g, Gesture::DoubleTap(_)) {
        p.text(r.center_bottom() + Vec2::new(0.0, -9.0), egui::Align2::CENTER_CENTER, "×2", egui::FontId::proportional(10.0), MUTED);
    }
}

fn mods_chips(ui: &mut egui::Ui, mods: &mut u64) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for (bit, label) in [(MOD_CTRL, "⌃ control"), (MOD_OPT, tl("⌥ option")), (MOD_SHIFT, tl("⇧ shift")), (MOD_CMD, tl("⌘ command"))] {
            if chip(ui, label, *mods & bit != 0).clicked() {
                *mods ^= bit;
            }
        }
    });
}

fn mouse_picker(ui: &mut egui::Ui, m: &mut MouseTrigger, recording: bool, start: &mut Option<RecTarget>, stop: &mut bool, id: u64) {
    let is_scroll = matches!(m, MouseTrigger::Scroll { .. });
    ui.horizontal(|ui| {
        if chip(ui, tl("A button"), !is_scroll).clicked() && is_scroll {
            *m = MouseTrigger::Button { button: 2, mods: 0 };
        }
        if chip(ui, tl("The wheel + keys"), is_scroll).clicked() && !is_scroll {
            *m = MouseTrigger::Scroll { dir: Wheel::Up, mods: MOD_OPT };
        }
    });
    ui.add_space(6.0);
    match m {
        MouseTrigger::Button { button, mods } => {
            if recording {
                recording_box(ui, tl("Press a mouse button now (middle, side…)"));
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
                    if ghost_button(ui, tl("Cancel")).clicked() {
                        *stop = true;
                    }
                } else if ghost_button(ui, tl("●  Record button")).clicked() {
                    *start = Some(RecTarget::TriggerMouse(id));
                }
            });
            ui.add_space(4.0);
            ui.label(RichText::new(tl("While holding down (optional)")).size(12.0).color(MUTED));
            mods_chips(ui, mods);
        }
        MouseTrigger::Scroll { dir, mods } => {
            ui.horizontal(|ui| {
                if chip(ui, tl("Wheel up"), *dir == Wheel::Up).clicked() {
                    *dir = Wheel::Up;
                }
                if chip(ui, tl("Wheel down"), *dir == Wheel::Down).clicked() {
                    *dir = Wheel::Down;
                }
            });
            ui.add_space(4.0);
            ui.label(RichText::new(tl("While holding down at least one of these keys")).size(12.0).color(MUTED));
            mods_chips(ui, mods);
            if *mods == 0 {
                ui.label(RichText::new(tl("Choose at least one key: the wheel alone keeps scrolling as usual.")).size(12.0).color(AMBER));
            }
        }
    }
}

const ACTION_KINDS: [&str; 8] = [
    "Nothing yet",
    "Press a keyboard shortcut",
    "Place the window",
    "A system action",
    "Open an app",
    "Open a link or file",
    "Type some text",
    "Run a command",
];

fn action_editor(ui: &mut egui::Ui, a: &mut Action, recording: bool, start: &mut Option<RecTarget>, stop: &mut bool, id: u64) {
    let mut kind = a.kind_index();
    egui::ComboBox::from_id_salt("action_kind").selected_text(tl(ACTION_KINDS[kind])).width(260.0).show_ui(ui, |ui| {
        for (i, k) in ACTION_KINDS.iter().enumerate() {
            ui.selectable_value(&mut kind, i, tl(k));
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
            ui.label(RichText::new(tl("Choose above what you want to happen.")).color(AMBER));
        }
        Action::Shortcut(k) => {
            if recording {
                recording_box(ui, tl("Press the shortcut MacControl should send now…"));
            } else if k.key == 0xFFFF {
                ui.label(RichText::new(tl("Record the shortcut to send")).color(MUTED));
            } else {
                keycaps(ui, &combo_parts(k.display_mods(), Some(key_name(k.key))), true);
            }
            ui.add_space(6.0);
            if recording {
                if ghost_button(ui, tl("Cancel")).clicked() {
                    *stop = true;
                }
            } else if ghost_button(ui, tl("●  Record shortcut")).clicked() {
                *start = Some(RecTarget::ActionKey(id));
            }
        }
        Action::Window(pos) => window_grid(ui, pos),
        Action::System(s) => {
            egui::ComboBox::from_id_salt("system").selected_text(s.label()).width(260.0).height(420.0).show_ui(ui, |ui| {
                for group in [tl("System"), tl("Windows"), tl("Sound"), tl("Mouse")] {
                    ui.label(RichText::new(group).size(11.0).color(FAINT));
                    for it in SystemAction::ALL.iter().filter(|x| x.group() == group) {
                        ui.selectable_value(s, *it, it.label());
                    }
                    ui.separator();
                }
            });
        }
        Action::OpenApp(name) => {
            ui.label(RichText::new(tl("App name (for example “Safari” or “Notes”)")).size(12.0).color(MUTED));
            ui.add(egui::TextEdit::singleline(name).hint_text("Safari").desired_width(f32::INFINITY));
        }
        Action::OpenUrl(u) => {
            ui.label(RichText::new(tl("Web address or path to a file or folder")).size(12.0).color(MUTED));
            ui.add(egui::TextEdit::singleline(u).hint_text("https://www.apple.com").desired_width(f32::INFINITY));
        }
        Action::TypeText(t) => {
            ui.label(RichText::new(tl("Text to type wherever the cursor is")).size(12.0).color(MUTED));
            ui.add(egui::TextEdit::multiline(t).desired_rows(3).desired_width(f32::INFINITY));
        }
        Action::Shell(c) => {
            ui.label(RichText::new(tl("Terminal command (run with sh)")).size(12.0).color(MUTED));
            ui.add(egui::TextEdit::singleline(c).code_editor().hint_text(tl("say hello")).desired_width(f32::INFINITY));
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
    ui.label(RichText::new(tlf("The active window will go to: {}", &[&pos.label()])).color(MUTED).size(12.0));
}

/// Where a window would go, as fractions of a 1000-point screen (for the small drawings).
fn unit_target(p: WindowPos, s: &SnapSettings, current: Option<R>) -> R {
    let t = target_rect(p, R::new(0.0, 0.0, 1000.0, 1000.0), s, current);
    R::new(t.x / 1000.0, t.y / 1000.0, t.w / 1000.0, t.h / 1000.0)
}

fn short_pos(p: WindowPos) -> &'static str {
    match p {
        WindowPos::Maximize => tl("Maximize"),
        WindowPos::LeftHalf => tl("L. half"),
        WindowPos::RightHalf => tl("R. half"),
        WindowPos::TopHalf => tl("Top"),
        WindowPos::BottomHalf => tl("Bottom"),
        WindowPos::TopLeft => tl("Top L."),
        WindowPos::TopRight => tl("Top R."),
        WindowPos::BottomLeft => tl("Bottom L."),
        WindowPos::BottomRight => tl("Bottom R."),
        WindowPos::LeftThird => tl("L. 1/3"),
        WindowPos::CenterThird => tl("Mid 1/3"),
        WindowPos::RightThird => tl("R. 1/3"),
        WindowPos::LeftTwoThirds => tl("L. 2/3"),
        WindowPos::RightTwoThirds => tl("R. 2/3"),
        WindowPos::Center => tl("Center"),
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
                            ui.label(RichText::new(tl("Snap when dragging")).size(16.0).strong());
                            ui.label(RichText::new(tl("Drag a window by its title bar to an edge and let go.")).color(MUTED).size(12.5));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            toggle(ui, &mut s.enabled);
                        });
                    });
                    ui.add_space(12.0);
                    screen_diagram(app, ui, s);
                    ui.add_space(10.0);
                    ui.label(RichText::new(tl("Which edges work")).size(12.0).color(MUTED));
                    ui.horizontal_wrapped(|ui| {
                        if chip(ui, tl("◧  Left: half"), s.left).clicked() {
                            s.left = !s.left;
                        }
                        if chip(ui, tl("◨  Right: half"), s.right).clicked() {
                            s.right = !s.right;
                        }
                        if chip(ui, tl("⬒  Top: maximize"), s.top).clicked() {
                            s.top = !s.top;
                        }
                        if chip(ui, tl("◰  Corners: quarters"), s.corners).clicked() {
                            s.corners = !s.corners;
                        }
                    });
                });
                ui.add_space(10.0);
                card(ui, |ui| {
                    ui.set_width(ui.available_width());
                    section_title(ui, tl("Exceptions"));
                    ui.label(RichText::new(tl("Apps whose windows don't snap")).color(MUTED).size(12.0));
                    ui.add_space(4.0);
                    let mut remove = None;
                    ui.horizontal_wrapped(|ui| {
                        for b in &s.excluded_apps {
                            let n = app_name(&Config::default(), &app.running_apps, b);
                            if chip(ui, &format!("{n}  ×"), false).on_hover_text(tl("Remove")).clicked() {
                                remove = Some(b.clone());
                            }
                        }
                        let add = ghost_button(ui, tl("+ Add app"));
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
                    step_title(ui, 1, tl("Sizes"), tl("How much space the window takes when it snaps"));
                    ui.add_space(6.0);
                    slider(ui, tl("Left half"), &mut s.left_width, 0.2..=0.8);
                    slider(ui, tl("Right half"), &mut s.right_width, 0.2..=0.8);
                    slider(ui, tl("Quarters (corners)"), &mut s.corner_width, 0.2..=0.8);
                    px_slider(ui, tl("Gap around"), &mut s.gap, 0.0..=40.0);
                });
                ui.add_space(10.0);
                card(ui, |ui| {
                    ui.set_width(ui.available_width());
                    step_title(ui, 2, tl("Detection"), tl("How close to the edge you need to get"));
                    ui.add_space(6.0);
                    px_slider(ui, tl("Corner area"), &mut s.corner_area, 5.0..=150.0);
                    px_slider(ui, tl("Widen the edges"), &mut s.recognition_extra, 0.0..=40.0);
                    switch_row(ui, tl("Wait a moment before snapping"), tl("Avoids snapping when you brush an edge by accident"), &mut s.delay_enabled);
                    if s.delay_enabled {
                        ui.add(egui::Slider::new(&mut s.delay, 0.05..=1.0).suffix(" s").step_by(0.05));
                    }
                });
                ui.add_space(10.0);
                card(ui, |ui| {
                    ui.set_width(ui.available_width());
                    step_title(ui, 3, tl("Behavior"), "");
                    ui.add_space(6.0);
                    switch_row(ui, tl("Restore the size when pulled away"), tl("Dragging it off the edge brings back its previous size"), &mut s.snap_back);
                    ui.add_space(4.0);
                    switch_row(ui, tl("Show preview"), tl("A rectangle shows where the window will land"), &mut s.show_preview);
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
                        ui.add(egui::Slider::new(&mut s.preview_animation, 0.0..=0.6).suffix(" s").text(tl("Animation")));
                    }
                    ui.add_space(8.0);
                    if ghost_button(ui, tl("Restore the defaults")).clicked() {
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
    ui.label(RichText::new(tl("Hover over the colored zones to see where the window would go.")).size(11.5).color(FAINT));
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
                section_title(ui, tl("Permissions"));
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(tl("Accessibility")).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if trusted {
                            pill(ui, tl("Granted"), GREEN);
                        } else {
                            pill(ui, tl("Pending"), AMBER);
                        }
                    });
                });
                ui.label(
                    RichText::new(tl("MacControl needs it to read shortcuts and mouse buttons and to move other apps' windows."))
                        .size(12.0)
                        .color(MUTED),
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if primary_button(ui, tl("Request permission")).clicked() {
                        crate::mac::ax::request_trust();
                    }
                    if ghost_button(ui, tl("Open System Settings")).clicked() {
                        let _ = std::process::Command::new("/usr/bin/open")
                            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
                            .spawn();
                    }
                });
            });
            cols[0].add_space(10.0);
            card(&mut cols[0], |ui| {
                ui.set_width(ui.available_width());
                section_title(ui, tl("Status"));
                ui.add_space(4.0);
                status_row(ui, tl("Keyboard and mouse"), if tap { (tl("Listening"), GREEN) } else { (tl("Stopped"), AMBER) });
                status_row(ui, "Trackpads", if pads > 0 { (if pads == 1 { tl("1 detected") } else { tl("Several detected") }, GREEN) } else { (tl("None"), AMBER) });
                status_row(ui, tl("Window snapping"), if cfg.snap.enabled { (tl("On"), GREEN) } else { (tl("Off"), MUTED) });
                ui.horizontal(|ui| {
                    ui.label(RichText::new(tl("Frontmost app")).color(MUTED));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(if front.is_empty() { "—".to_string() } else { front.clone() });
                    });
                });
            });
            card(&mut cols[1], |ui| {
                ui.set_width(ui.available_width());
                section_title(ui, "General");
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(tl("Language")).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        i18n::lang_picker(ui, &mut cfg.language);
                    });
                });
                ui.add_space(8.0);
                let mut active = !cfg.paused;
                if switch_row(ui, tl("MacControl active"), tl("Pauses every rule and window snapping"), &mut active) {
                    cfg.paused = !active;
                }
                ui.add_space(6.0);
                switch_row(ui, tl("Click sound"), tl("A little “beep” when you click inside MacControl"), &mut cfg.click_sound);
                ui.add_space(8.0);
                ui.label(RichText::new(tl("When you close the window, MacControl keeps working from its icon in the menu bar.")).size(12.0).color(MUTED));
                ui.add_space(6.0);
                ui.label(RichText::new(tl("Settings saved in")).size(11.5).color(FAINT));
                ui.label(RichText::new(home_relative(&Config::path())).size(11.5).color(MUTED));
                ui.add_space(8.0);
                if app.confirm_reset {
                    ui.label(RichText::new(tl("Are you sure? All your rules will be deleted.")).color(AMBER));
                    ui.horizontal(|ui| {
                        if danger_button(ui, tl("Yes, reset")).clicked() {
                            let language = cfg.language;
                            *cfg = Config::default();
                            cfg.language = language;
                            app.confirm_reset = false;
                            app.select(None);
                            app.scope = None;
                        }
                        if ghost_button(ui, tl("Cancel")).clicked() {
                            app.confirm_reset = false;
                        }
                    });
                } else if danger_button(ui, tl("Reset everything")).clicked() {
                    app.confirm_reset = true;
                }
            });
            cols[1].add_space(10.0);
            card(&mut cols[1], |ui| {
                ui.set_width(ui.available_width());
                section_title(ui, tl("About"));
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(44.0), Sense::hover());
                    app.draw_logo(ui, r);
                    ui.add_space(4.0);
                    ui.label(RichText::new("MacControl").strong().size(18.0).color(TEXT));
                    let (a, _) = accent();
                    pill(ui, &tlf("Version {}", &[&env!("CARGO_PKG_VERSION")]), lerp_color(a, Color32::WHITE, 0.45));
                });
                ui.add_space(6.0);
                ui.label(
                    RichText::new(tl("MacControl is a free app for smart management of the trackpad, keyboard shortcuts and window snapping on macOS."))
                        .size(12.5)
                        .color(MUTED),
                );
                ui.add_space(10.0);
                ui.spacing_mut().interact_size.y = 16.0;
                egui::Grid::new("about").num_columns(2).spacing(Vec2::new(14.0, 5.0)).min_col_width(60.0).show(ui, |ui| {
                    let key = |ui: &mut egui::Ui, text: &str| {
                        ui.label(RichText::new(text).size(12.5).color(MUTED));
                    };
                    key(ui, tl("Author"));
                    ui.label(RichText::new(AUTHOR).size(12.5));
                    ui.end_row();
                    key(ui, tl("Contact"));
                    let link = ui.visuals().hyperlink_color;
                    ui.hyperlink_to(RichText::new(EMAIL).size(12.5).color(link), format!("mailto:{EMAIL}"));
                    ui.end_row();
                    key(ui, tl("License"));
                    ui.label(RichText::new("MIT").size(12.5));
                    ui.end_row();
                });
                ui.add_space(8.0);
                ui.label(
                    RichText::new(tl("Click sound: “audio-volume-change” by Lucas McCallister, from the freedesktop.org sound theme (CC-BY-SA license)."))
                        .size(11.0)
                        .color(FAINT),
                );
            });
        });
    });
}

const AUTHOR: &str = "Joan Galtés i Moreno";
const EMAIL: &str = "donbuche@gmail.com";

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
