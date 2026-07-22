pub mod configshandler;

use crate::dbadapt;
use std::{default, fs};
use eframe::egui::{self, Pos2};
use configshandler::{Config, Template, DefaultTemplate, load_config, load_templates, find_template_by_label, configs_dir, add_template, TemplateTypes};


#[derive(Clone ,Copy, PartialEq, )]
enum SelectedPanel {
    NONE,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, )]
enum ConfStage {
    First,
    Second,
}

enum Screen {
    Home,
    FirstConfiguration{
        stage: ConfStage,
        editor_text: String,
        editor_body: String,
        editor_label: String,
        default_template: DefaultTemplate,
        selected_panel: SelectedPanel, 
    },
}

pub struct App{
    pub screen: Screen,
    pub config: Config,
    pub db: Box<dyn dbadapt::DBConnection>,
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

        let db: Box<dyn dbadapt::DBConnection> = match &config.db_path {
            Some(path) if path.exists() => {
                dbadapt::connect(&config).unwrap_or_else(|_| dbadapt::empty())
            }
            _ => dbadapt::empty(),
        };

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
                stage: ConfStage::First,
                editor_text: String::new(),
                editor_body: String::new(),
                editor_label: String::new(),
                default_template,
                selected_panel: SelectedPanel::NONE,
            }
        };

        Self { screen, config, db }
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
            Screen::FirstConfiguration { editor_text, stage, editor_body, editor_label, default_template, selected_panel, .. } => {
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
                        let left_stroke = if *selected_panel == SelectedPanel::Left{
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(100,180,255))
                        }
                        else{
                            egui::Stroke::NONE
                        };
                   // Panel lewy - podgląd/lista
                        let left_frame = egui::Frame::none()
                           .fill(egui::Color32::from_rgb(30, 30, 30))
                            .stroke(left_stroke)
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.set_min_width(200.0);
                                ui.label(format!("Tytuł:\n{}\n\nTreść:\n{}",default_template.new_reservation.subject, default_template.new_reservation.body));
                            });
                        let left_response = ui.interact(
                            left_frame.response.rect,
                            ui.id().with("left_panel"),
                            egui::Sense::click(),
                        );

                        if left_response.clicked() {
                            *selected_panel = SelectedPanel::Left;
                        }

                        if left_response.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }

                        let right_stroke = if *selected_panel == SelectedPanel::Right {
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(100,180,255))
                        }
                        else {
                            egui::Stroke::NONE
                        };
                      // Panel prawy - edytor tekstu
                        let right_frame = egui::Frame::none()
                            .fill(egui::Color32::from_rgb(30, 30, 30))
                            .stroke(right_stroke)
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::same(12.0))
                            .show(ui, |ui| {
                                ui.set_min_width(300.0);
                                ui.label("Twoj szablon:\n {{tag}} -> Symbol wycieczki\n {{date}} -> Data Wycieczki\n {{clients}} -> Klienci\n {{creds}} -> Imie i Nazwisko piszacego\n {{signature}} -> Stopka firmy");
                                ui.add(
                                 egui::TextEdit::multiline(editor_text)
                                     .desired_width(f32::INFINITY)
                                     .desired_rows(15),
                                )
                            });

                        let right_response = right_frame.inner;

                        if right_response.clicked() || right_response.gained_focus()
                        {
                            *selected_panel = SelectedPanel::Right;
                        }

                        if right_response.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                    });

                    egui::Area::new(egui::Id::new("bottom_right_button"))
                        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-12.0, -12.0))
                        .show(ctx, |ui| {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgb(30, 30, 30))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::same(12.0))
                                .show(ui, |ui| {
                                    if ui.add_enabled(*selected_panel != SelectedPanel::NONE, egui::Button::new("Zapisz")).clicked(){
                                        match *selected_panel{
                                            SelectedPanel::Left =>{
                                                *stage = ConfStage::Second;
                                            },
                                            SelectedPanel::Right => {
                                                add_template(editor_text.clone(), TemplateTypes::NewRes ).expect("Failed to add new template");
                                                *stage = ConfStage::Second;  
                                            },
                                            SelectedPanel::NONE => {
                                                println!("JAK");
                                            },
                                        }
                                    }
                                });
                        });
                });
            }
        }
    }
}
