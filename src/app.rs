mod configshandler;

use std::{default, fs};
use eframe::egui::{self, Pos2};
use configshandler::{Config, Template, DefaultTemplate, load_config, load_templates, find_template_by_label, configs_dir};


enum Screen {
    Home,
    FirstConfiguration{
        editor_text: String,
        editor_body: String,
        editor_label: String,
        default_template: DefaultTemplate,
    },
}

pub struct App{
    pub screen: Screen,
    pub config: Config,
    // pub sectors: Vec<Sector>,
}

impl Default for App {
    fn default() -> Self {
        let config_path = configshandler::configs_dir().join("config.json");
        let templates_path = configshandler::configs_dir().join("templates.json");

        // Sprawdzamy i zapamiętujemy TERAZ, zanim cokolwiek utworzymy
        let config_existed = fs::metadata(&config_path).is_ok();

        if !config_existed {
            configshandler::create_default_config(&config_path)
                .expect("Failed to create default config");
        }

        let config: Config = load_config(&config_path).expect("Failed to load config");
        let templates: Vec<Template> = load_templates(&templates_path).expect("Failed to load templates");

        let default_template = DefaultTemplate {
            new_reservation: find_template_by_label(&templates, &config.new_reservation_name)
                .cloned().expect("Default template 'new_reservation' not found"),
            update_reservation: find_template_by_label(&templates, &config.update_reservation_name)
                .cloned().expect("Default template 'update_reservation' not found"),
            delete_reservation: find_template_by_label(&templates, &config.delete_reservation_name)
                .cloned().expect("Default template 'delete_reservation' not found"),
        };

        let screen = if config_existed {
            Screen::Home
        } else {
            Screen::FirstConfiguration {
                editor_text: String::new(),
                editor_body: String::new(),
                editor_label: String::new(),
                default_template,
            }
        };

        Self { screen, config }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.screen {
            // Ekran główny, który wyświetla powitanie lub inne informacje
            Screen::Home => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Home Screen");
                });
            }
            // Ekran konfiguracji, który wyświetla szablony rezerwacji i pozwala użytkownikowi wybrać domyślny szablon
            Screen::FirstConfiguration { editor_text, editor_body, editor_label, default_template, .. } => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let heading = ui.heading("Hej, wybierz szablon rezerwacji, który chcesz wybrać jako domyślny");
                    
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(30, 30, 30))
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(12.0))
                        .show(ui, |ui| {
                            ui.label("To jest zawartość w szarym panelu");
                        });
                    
                    ui.horizontal(|ui| {
                   // Panel lewy - podgląd/lista
                        egui::Frame::none()
                           .fill(egui::Color32::from_rgb(30, 30, 30))
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.set_min_width(200.0);
                                ui.label(format!("Tytuł:\n{}\n\nTreść:\n{}",default_template.new_reservation.subject, default_template.new_reservation.body));
                            });

                      // Panel prawy - edytor tekstu
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(30, 30, 30))
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.set_min_width(300.0);
                                ui.add(
                                 egui::TextEdit::multiline(editor_text)
                                     .desired_width(f32::INFINITY)
                                     .desired_rows(15),
                                );
                            });
                    });

                });
            }
        }
    }
}
