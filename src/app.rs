mod configshandler;

use std::{default, fs};
use eframe::egui::{self, Pos2};
use configshandler::{Config, Template, DefaultTemplate, load_config, load_templates, find_template_by_label};

enum Screen {
    Home,
    Configuration{
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
        let mut screen;
        // let mut sectors: Vec<Sector> = Vec::new();
        

        if fs::metadata("configs/config.json").is_err() {
            configshandler::create_default_config("configs/config.json")
                .expect("Failed to create default config");
            screen = Screen::Configuration{
                editor_text: String::new(),
                editor_body: String::new(),
                editor_label: String::new(),
                default_template: DefaultTemplate {
                    new_reservation: Template::none(),
                    update_reservation: Template::none(),
                    delete_reservation: Template::none(),
                },
            };
        } else {
            screen = Screen::Home;
        }

        let config: Config = load_config("configs/config.json").expect("Failed to load config");
        let templates: Vec<Template> = load_templates("configs/templates.json").expect("Failed to load templates");
        let default_template: DefaultTemplate;
        default_template = DefaultTemplate {
            new_reservation: find_template_by_label(&templates, &config.new_reservation_name).cloned().expect("Default template 'new_reservation' not found"),
            update_reservation: find_template_by_label(&templates, &config.update_reservation_name).cloned().expect("Default template 'update_reservation' not found"),
            delete_reservation: find_template_by_label(&templates, &config.delete_reservation_name).cloned().expect("Default template 'delete_reservation' not found"),
        };

        screen = Screen::Configuration{
                editor_text: String::new(),
                editor_body: String::new(),
                editor_label: String::new(),
                default_template: default_template.clone(),
                };

        match screen {
            Screen::Configuration { .. } => {
                Self {
                    screen,
                    config: config.clone(),
                }
            }
            Screen::Home => {
                Self {
                    screen,
                    config: config.clone(),
                }
            }
        }
        
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
            Screen::Configuration { editor_text, editor_body, editor_label, default_template, .. } => {
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
                                ui.label("Panel lewy");
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