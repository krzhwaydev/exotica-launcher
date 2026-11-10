#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod finder;
mod fastflags;
mod launcher;

use eframe::egui::{self, Color32, RichText, Rounding, Vec2, Pos2, Image, Rect, Stroke};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use sysinfo::System;

#[derive(Serialize, Deserialize, Clone)]
struct ExoticaConfig {
    active_profile: String,
    theme: String,
}

impl Default for ExoticaConfig {
    fn default() -> Self {
        Self { active_profile: "fps".to_string(), theme: "red".to_string() }
    }
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("ExoticaLauncher");
    fs::create_dir_all(&path).unwrap_or_default();
    path.join("config.json")
}

fn load_config() -> ExoticaConfig {
    if let Ok(content) = fs::read_to_string(get_config_path()) {
        serde_json::from_str(&content).unwrap_or_default()
    } else { ExoticaConfig::default() }
}

fn save_config(config: &ExoticaConfig) {
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(get_config_path(), json);
    }
}

fn run_game_booster() -> String {
    let processes = ["chrome.exe", "discord.exe", "telegram.exe", "opera.exe", "steam.exe"];
    let mut killed = 0;
    for process in processes {
        if let Ok(out) = Command::new("taskkill").args(&["/F", "/IM", process, "/T"]).output() {
            if out.status.success() { killed += 1; }
        }
    }
    format!("Освобождено! Закрыто процессов: {}", killed)
}

fn start_ping_thread() -> Receiver<u32> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let mut ping = 999;
        if let Ok(out) = Command::new("ping").args(&["-n", "1", "-w", "1000", "www.roblox.com"]).output() {
            let text = String::from_utf8_lossy(&out.stdout);
            if let Some(idx) = text.find("время=").or_else(|| text.find("time=")) {
                let rest = &text[idx + 6..];
                let num_str: String = rest.chars().take_while(|c| c.is_numeric()).collect();
                if let Ok(ms) = num_str.parse::<u32>() { ping = ms; }
            }
        }
        let _ = tx.send(ping);
    });
    rx
}

struct ExoticaApp {
    config: ExoticaConfig,
    status: String,
    active_tab: usize,
    system: System,
    ping_result: Option<u32>,
    ping_receiver: Option<Receiver<u32>>,
    // Переменные для анимации
    last_tab: usize,
    anim_progress: f32, // от 0.0 до 1.0
}

impl Default for ExoticaApp {
    fn default() -> Self {
        Self { 
            config: load_config(), 
            status: "Roblox успешно найден".to_string(),
            active_tab: 0,
            last_tab: 0,
            anim_progress: 1.0,
            system: System::new_all(),
            ping_result: None,
            ping_receiver: None,
        }
    }
}

impl eframe::App for ExoticaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.system.refresh_memory();
        
        if let Some(rx) = &self.ping_receiver {
            if let Ok(ping) = rx.try_recv() {
                self.ping_result = Some(ping);
                self.ping_receiver = None;
            } else { ctx.request_repaint(); }
        }

        // --- ЛОГИКА АНИМАЦИИ ПЕРЕКЛЮЧЕНИЯ ---
        if self.active_tab != self.last_tab {
            self.anim_progress = 0.0; // Сбрасываем в 0 при клике
            self.last_tab = self.active_tab;
        }
        if self.anim_progress < 1.0 {
            self.anim_progress += 0.1; // Скорость анимации
            if self.anim_progress > 1.0 { self.anim_progress = 1.0; }
            ctx.request_repaint(); // Заставляем перерисовывать для анимации
        }

        // --- АНИМИРОВАННЫЙ ФОН ---
        let time = ctx.input(|i| i.time) as f32;
        let painter = ctx.layer_painter(egui::LayerId::background());
        painter.rect_filled(egui::Rect::EVERYTHING, 0.0, Color32::from_rgb(15, 15, 18));

        let center1 = Pos2::new(600.0 + (time * 0.5).sin() * 150.0, 300.0 + (time * 0.7).cos() * 150.0);
        let center2 = Pos2::new(300.0 + (time * 0.3).cos() * 150.0, 500.0 + (time * 0.4).sin() * 150.0);
        
        let (c1, c2) = match self.config.theme.as_str() {
            "blue" => (Color32::from_rgba_unmultiplied(0, 150, 255, 20), Color32::from_rgba_unmultiplied(0, 50, 150, 15)),
            "purple" => (Color32::from_rgba_unmultiplied(150, 0, 255, 20), Color32::from_rgba_unmultiplied(50, 0, 150, 15)),
            _ => (Color32::from_rgba_unmultiplied(255, 50, 50, 20), Color32::from_rgba_unmultiplied(150, 0, 0, 15)),
        };
        painter.circle_filled(center1, 450.0, c1); 
        painter.circle_filled(center2, 450.0, c2);  
        ctx.request_repaint();

        let card_bg = Color32::from_rgba_unmultiplied(30, 30, 36, 240);
        let accent_color = match self.config.theme.as_str() {
            "blue" => Color32::from_rgb(0, 150, 255),
            "purple" => Color32::from_rgb(150, 0, 255),
            _ => Color32::from_rgb(220, 50, 50),
        };

        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::TRANSPARENT;
        visuals.widgets.noninteractive.bg_fill = card_bg;
        visuals.widgets.noninteractive.rounding = Rounding::same(12.0);
        ctx.set_visuals(visuals);

        // --- ЛЕВАЯ ПАНЕЛЬ (С иконками SVG) ---
        egui::SidePanel::left("sidebar")
            .exact_width(250.0)
            .frame(egui::Frame::none().fill(Color32::from_rgba_unmultiplied(20, 20, 25, 220)).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.add_space(10.0);
                // Заголовок EXOTICA
                ui.horizontal(|ui| {
                    ui.label(RichText::new("EXOTICA").size(32.0).strong().color(accent_color));
                });
                ui.label(RichText::new("Performance Client").size(12.0).color(Color32::GRAY));
                ui.add_space(40.0);

                // Меню навигации
                let tabs = [
                    ("🏠  Главная", "file://assets/home.svg"), // Юникод для надежности
                    ("⚙️  Настройки", "file://assets/settings.svg"),
                    ("🛠  Инструменты", "file://assets/tools.svg")
                ];

                for (i, (text, _img_path)) in tabs.iter().enumerate() {
                    let is_active = self.active_tab == i;
                    
                    // Красивая плашка для кнопки
                    let btn = egui::Button::new(
                        RichText::new(*text).size(16.0)
                            .color(if is_active { Color32::WHITE } else { Color32::GRAY })
                    )
                    .fill(if is_active { accent_color } else { Color32::TRANSPARENT })
                    .rounding(8.0);

                    if ui.add_sized([210.0, 45.0], btn).clicked() {
                        self.active_tab = i;
                    }
                    ui.add_space(10.0);
                }

                // ОЗУ внизу
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    let used_ram = self.system.used_memory() / 1024 / 1024;
                    let total_ram = self.system.total_memory() / 1024 / 1024;
                    ui.add(egui::ProgressBar::new(used_ram as f32 / total_ram as f32)
                        .fill(accent_color)
                        .text(RichText::new(format!("RAM: {} MB", used_ram)).size(12.0)));
                    ui.add_space(10.0);
                });
            });

        // --- НИЖНЯЯ ПАНЕЛЬ (Кнопка Играть) ---
        // Размещаем ее до CentralPanel, чтобы она всегда была внизу
        egui::TopBottomPanel::bottom("play_bar")
            .exact_height(100.0)
            .frame(egui::Frame::none().inner_margin(egui::Margin::symmetric(40.0, 20.0)))
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let play_btn = egui::Button::new(RichText::new("ИГРАТЬ").size(26.0).strong())
                        .fill(accent_color)
                        .rounding(12.0)
                        .stroke(Stroke::new(2.0, Color32::WHITE)); // Белая обводка для заметности
                    
                    if ui.add_sized([250.0, 60.0], play_btn).clicked() {
                        if let Some(appdata) = finder::get_roblox_appdata() {
                            let _ = fastflags::write_fastflags(&appdata, &self.config.active_profile);
                        }
                        match launcher::launch_roblox() {
                            Ok(_) => self.status = "Игра запущена!".to_string(),
                            Err(e) => self.status = format!("Ошибка: {}", e),
                        }
                    }
                    
                    // Статус слева от кнопки
                    ui.add_space(20.0);
                    ui.label(RichText::new(&self.status).size(16.0).color(Color32::GRAY));
                });
            });

        // --- ЦЕНТРАЛЬНАЯ ПАНЕЛЬ (С анимацией) ---
        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(40.0))
            .show(ctx, |ui| {
                // Анимация: Сдвиг вправо на 30 пикселей и прозрачность
                let offset = (1.0 - self.anim_progress) * 30.0;
                let alpha = self.anim_progress;
                
                // Создаем временный слой для анимации
                ui.scope(|ui| {
                    ui.set_opacity(alpha);
                    ui.add_space(offset); // Плавный выезд сверху/снизу

                    match self.active_tab {
                        0 => self.draw_home(ui, card_bg, accent_color),
                        1 => self.draw_settings(ui, card_bg, accent_color),
                        2 => self.draw_tools(ui, card_bg, accent_color),
                        _ => {}
                    }
                });
            });
    }
}

impl ExoticaApp {
    fn draw_home(&mut self, ui: &mut egui::Ui, card_bg: Color32, accent: Color32) {
        ui.heading(RichText::new("Панель управления").size(28.0).color(Color32::WHITE));
        ui.add_space(20.0);

        // Карточка профиля
        egui::Frame::none().fill(card_bg).rounding(12.0).inner_margin(25.0).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new("Профиль оптимизации").size(18.0).strong());
            ui.add_space(15.0);
            ui.horizontal(|ui| {
                self.profile_btn(ui, "fps", "FPS BOOST", accent);
                ui.add_space(10.0);
                self.profile_btn(ui, "balanced", "BALANCED", accent);
                ui.add_space(10.0);
                self.profile_btn(ui, "quality", "QUALITY", accent);
            });
        });
        
        ui.add_space(20.0);

        // Карточка FPS (заглушка)
        egui::Frame::none().fill(card_bg).rounding(12.0).inner_margin(25.0).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(RichText::new("Текущий FPS в игре:").size(18.0).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("---").size(28.0).strong().color(Color32::GRAY));
                });
            });
        });
    }

    fn draw_settings(&mut self, ui: &mut egui::Ui, card_bg: Color32, accent: Color32) {
        ui.heading(RichText::new("Настройки").size(28.0).color(Color32::WHITE));
        ui.add_space(20.0);

        egui::Frame::none().fill(card_bg).rounding(12.0).inner_margin(25.0).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new("Оформление").size(18.0).strong());
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.selectable_label(self.config.theme == "red", "🔴 Красная").clicked() { self.config.theme = "red".to_string(); save_config(&self.config); }
                if ui.selectable_label(self.config.theme == "blue", "🔵 Синяя").clicked() { self.config.theme = "blue".to_string(); save_config(&self.config); }
                if ui.selectable_label(self.config.theme == "purple", "🟣 Фиолетовая").clicked() { self.config.theme = "purple".to_string(); save_config(&self.config); }
            });
        });
    }

    fn draw_tools(&mut self, ui: &mut egui::Ui, card_bg: Color32, accent: Color32) {
        ui.heading(RichText::new("Инструменты").size(28.0).color(Color32::WHITE));
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            egui::Frame::none().fill(card_bg).rounding(12.0).inner_margin(25.0).show(ui, |ui| {
                ui.set_width(300.0);
                ui.label(RichText::new("Game Booster").size(18.0).strong());
                ui.add_space(5.0);
                ui.label(RichText::new("Закроет фоновые процессы.").size(12.0).color(Color32::GRAY));
                ui.add_space(15.0);
                if ui.add_sized([250.0, 40.0], egui::Button::new("🚀 ОСВОБОДИТЬ РЕСУРСЫ").fill(accent)).clicked() {
                    self.status = run_game_booster();
                }
            });

            ui.add_space(20.0);

            egui::Frame::none().fill(card_bg).rounding(12.0).inner_margin(25.0).show(ui, |ui| {
                ui.set_width(300.0);
                ui.label(RichText::new("Пинг").size(18.0).strong());
                ui.add_space(5.0);
                let (ping_text, ping_color) = match self.ping_result {
                    Some(999) => ("Ошибка".to_string(), Color32::RED),
                    Some(ms) if ms < 60 => (format!("{} мс (Отлично)", ms), Color32::GREEN),
                    Some(ms) if ms < 120 => (format!("{} мс (Норма)", ms), Color32::YELLOW),
                    Some(ms) => (format!("{} мс (Плохо)", ms), Color32::RED),
                    None => ("Не проверен".to_string(), Color32::GRAY),
                };
                ui.label(RichText::new(ping_text).size(16.0).color(ping_color));
                ui.add_space(15.0);
                let is_pinging = self.ping_receiver.is_some();
                let text = if is_pinging { "⏳..." } else { "📡 ПРОВЕРИТЬ" };
                if ui.add_enabled(!is_pinging, egui::Button::new(text)).clicked() {
                    self.ping_receiver = Some(start_ping_thread());
                    self.ping_result = None;
                }
            });
        });
    }

    fn profile_btn(&mut self, ui: &mut egui::Ui, id: &str, text: &str, accent: Color32) {
        let is_active = self.config.active_profile == id;
        let btn = egui::Button::new(RichText::new(text).size(14.0).color(if is_active { Color32::WHITE } else { Color32::GRAY }))
            .fill(if is_active { accent } else { Color32::TRANSPARENT })
            .stroke(if is_active { Stroke::NONE } else { Stroke::new(1.0, Color32::GRAY) })
            .rounding(8.0);

        if ui.add_sized([150.0, 45.0], btn).clicked() {
            self.config.active_profile = id.to_string();
            save_config(&self.config);
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_decorations(true) 
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Exotica Launcher",
        options,
        Box::new(|cc| {
            // Устанавливаем загрузчики для SVG картинок
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ExoticaApp::default()))
        }),
    )
}