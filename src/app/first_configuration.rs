use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};
use eframe::egui;
use egui_extras::{TableBuilder, Column};
use egui::Color32;
use crate::{app::configshandler, dbadapt::{self, DBError}};

use super::configshandler::{DefaultTemplate, TemplateTypes, add_template, set_default_template, DBTypes};

#[derive(Clone, Copy, PartialEq)]
pub enum SelectedPanel {
    None,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ConfStage {
    First,
    Second,
    Third,
}


/// Stan ekranu pierwszej konfiguracji - wszystko czego potrzebuje ten ekran do działania
pub struct FirstConfigurationState {
    pub stage: ConfStage,
    pub editor_text: String,
    //pub editor_body: String,
    //pub editor_label: String,
    pub default_template: DefaultTemplate,
    pub selected_panel: SelectedPanel,
    pub selected_db_type: DBTypes,
    pub selected_db_path: Option<PathBuf>,
    pub preview_headers: Vec<String>,
    pub preview_rows: Vec<Vec<String>>,
    pub preview_error: Option<String>,
    pub preview_receiver: Option<Receiver<Result<(Vec<String>, Vec<Vec<String>>), String>>>,
    pub finished: bool,
}

impl FirstConfigurationState {
    pub fn new(default_template: DefaultTemplate) -> Self {
        Self {
            stage: ConfStage::First,
            editor_text: String::new(),
            //editor_body: String::new(),
            //editor_label: String::new(),
            default_template,
            selected_panel: SelectedPanel::None,
            selected_db_type: DBTypes::None,
            selected_db_path: None,
            preview_headers: Vec::new(),
            preview_rows: Vec::new(),
            preview_error: None, 
            preview_receiver: None,
            finished: false,
        }
    }
}

fn load_preview(
    db_type: &DBTypes,
    path: &std::path::Path,
) -> Result<(Vec<String>, Vec<Vec<String>>), DBError>  { 
    let conn: Box<dyn dbadapt::DBConnection> = match db_type {
        DBTypes::Access => Box::new(dbadapt::AccessConnection::connect(path)?),
        DBTypes::SQLite => Box::new(dbadapt::SqliteConnection::connect(path)?),
        DBTypes::None => return Err(dbadapt::DBError::ConnectToNone),
    };

    let tables = conn.list_tables()?;
    let first_table = tables.first().ok_or(dbadapt::DBError::ConnectToNone)?;

    let sql = format!("SELECT * FROM {}", first_table);
    conn.query_rows(&sql)
}

/// Renderuje cały ekran pierwszej konfiguracji, wywoływane co klatkę z app.rs
pub fn show(ctx: &egui::Context, state: &mut FirstConfigurationState) {
    match state.stage {
        ConfStage::First => show_first_stage(ctx, state),
        ConfStage::Second => show_second_stage(ctx, state),
        ConfStage::Third => show_third_stage(ctx, state),
    }
}

fn show_first_stage(ctx: &egui::Context, state: &mut FirstConfigurationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hej, wybierz szablon rezerwacji, który chcesz wybrać jako domyślny");

        egui::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.label("To jest zawartość w szarym panelu");
            });

        ui.horizontal(|ui| {
            let left_stroke = if state.selected_panel == SelectedPanel::Left {
                egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(100, 180, 255))
            } else {
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
                    ui.label(format!(
                        "Tytuł:\n{}\n\nTreść:\n{}",
                        state.default_template.new_reservation.subject,
                        state.default_template.new_reservation.body
                    ));
                });

            let left_response = ui.interact(
                left_frame.response.rect,
                ui.id().with("left_panel"),
                egui::Sense::click(),
            );

            if left_response.clicked() {
                state.selected_panel = SelectedPanel::Left;
            }
            if left_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            let right_stroke = if state.selected_panel == SelectedPanel::Right {
                egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(100, 180, 255))
            } else {
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
                        egui::TextEdit::multiline(&mut state.editor_text)
                            .desired_width(f32::INFINITY)
                            .desired_rows(15),
                    )
                });

            let right_response = right_frame.inner;

            if right_response.clicked() || right_response.gained_focus() {
                state.selected_panel = SelectedPanel::Right;
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
                        if ui
                            .add_enabled(
                                state.selected_panel != SelectedPanel::None,
                                egui::Button::new("Zapisz"),
                            )
                            .clicked()
                        {
                            match state.selected_panel {
                                SelectedPanel::Left => {
                                    state.stage = ConfStage::Second;
                                }
                                SelectedPanel::Right => {
                                    let label = add_template(state.editor_text.clone(), TemplateTypes::NewRes)
                                        .expect("Failed to add new template");
                                    set_default_template(TemplateTypes::NewRes, label);
                                    state.stage = ConfStage::Second;
                                }
                                SelectedPanel::None=> {
                                    println!("JAK");
                                }
                            }
                        }
                    });
            });
    });
}

fn show_second_stage(ctx: &egui::Context, state: &mut FirstConfigurationState) {
    if let Some(rx) = &state.preview_receiver {
        if let Ok(result)  = rx.try_recv() {
            match result {
                Ok((headers, rows)) => {
                    state.preview_headers = headers;
                    state.preview_rows = rows;
                    state.preview_error = None;
                }
                Err(e) => {
                    state.preview_error = Some(e);
                    state.preview_headers.clear();
                    state.preview_rows.clear();
                }
            }
            state.preview_receiver = None;
        }
    }
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Drugi etap konfiguracji");

        ui.separator();

        ui.label("Wybierz typ bazy danych:");

        egui::ComboBox::from_label("Typ bazy danych")
            .selected_text(format!("{}", state.selected_db_type))   
            .show_ui(ui, |ui| {
               ui.selectable_value(&mut state.selected_db_type, DBTypes::None, "Brak");
               ui.selectable_value(&mut state.selected_db_type, DBTypes::Access, "MS Access");
               ui.selectable_value(&mut state.selected_db_type, DBTypes::SQLite, "SQLite");
            });

        if state.selected_db_type != DBTypes::None {
            ui.separator();

            egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 30))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.label("Wybierz plik bazy danych:");

                    ui.horizontal(|ui| {
                        let path_text = state
                            .selected_db_path
                            .as_ref()
                            .map(|p| p.display().to_string())
                            .unwrap_or_else(|| "Nie wybrano pliku".to_string());
                        
                        ui.label(path_text);

        
                        if ui.button("Wybierz plik...").clicked() {
                            let dialog = match state.selected_db_type {
                                DBTypes::SQLite => rfd::FileDialog::new().add_filter("SQLite", &["db","sqlite"]),
                                DBTypes::Access=> rfd::FileDialog::new().add_filter("MS Access", &["accdb","mdb"]),
                                DBTypes::None => rfd::FileDialog::new(),
                            };

                            if let Some(path) = dialog.pick_file() {
                                state.selected_db_path = Some(path.clone());
                                state.preview_error = None;

                                let (tx, rx) = channel();
                                state.preview_receiver = Some(rx);

                                let db_type = state.selected_db_type.clone();

                                std::thread::spawn(move || {
                                    let result = load_preview(&db_type, &path)
                                        .map_err(|e| e.to_string());
                                    let _ = tx.send(result);
                                });
                            }
                        }
                    });
                });
        }
        
        if let Some(err) = &state.preview_error {
            ui.colored_label(egui::Color32::RED, format!("Błąd podglądu: {err}"));
        } else if !state.preview_rows.is_empty() {
            ui.separator();
            ui.label(egui::RichText::new("Podgląd bazy danych:").strong().size(16.0));

            egui::Frame::none()
                .fill(Color32::from_rgb(25, 25, 25))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(8.0))  
                .show(ui, |ui| {
                    egui::ScrollArea::both()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .resizable(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .columns(Column::auto().at_least(80.0).clip(true), state.preview_headers.len())
                                .header(24.0, |mut header| {
                                    for h in &state.preview_headers {
                                        header.col(|ui| {
                                            ui.label(egui::RichText::new(h).strong().color(Color32::from_rgb(100,180,255)));
                                        });
                                    }
                                })
                                .body(|mut body| {
                                    for row in &state.preview_rows {
                                        body.row(22.0, |mut table_row| {
                                            for cell in row {
                                                table_row.col(|ui|{
                                                    ui.label(cell);
                                                });
                                            }
                                        });
                                    }
                                });
                        });
                });

        } else if state.preview_receiver.is_some() {
            ui.spinner();
            ui.label("Ładowanie podglądu bazy danych...");
        }
        
        if state.preview_receiver.is_some() {
            ctx.request_repaint();
        }

        egui::Area::new(egui::Id::new("bottom_right_bottom"))
            .anchor(egui::Align2::RIGHT_BOTTOM,egui::vec2(-12.0, -12.0))
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(30,30,30))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(8.0))
                    .show(ui, |ui| {
                        if ui.add_enabled(state.selected_db_path.is_some(), egui::Button::new("Zapisz")).clicked() {
                            configshandler::change_config_field(
                                configshandler::ConfigVar::DBtype(state.selected_db_type.clone())
                            ).expect("Failed to save db type");
                            if let Some(path) = &state.selected_db_path {
                                configshandler::change_config_field(
                                    configshandler::ConfigVar::DBpath(Some(path.clone()))
                                ).expect("failed to save db path");
                            }
                            state.stage = ConfStage::Third;
                        }
                    });
            });
    });
}


fn show_third_stage(ctx: &egui::Context, state: &mut FirstConfigurationState ) {
    egui::CentralPanel::default().show(ctx, |ui| {

        ui.heading("Koniec pierwszej konfiguracji!");
        ui.separator();

        ui.label(egui::RichText::new("Gratulacje! Przeszedłeś pierwszą konfiguracje programu\nPAMIETAJ, zawsze bedziesz mógł zmienic te ustawienia (dodać nowe szablony itp.) w panelu Ustawienia"));

        egui::Area::new(egui::Id::new("bottom_right_bottom"))
            .anchor(egui::Align2::RIGHT_BOTTOM,egui::vec2(-12.0, -12.0))
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(30,30,30))
                    .rounding(egui::Rounding::same(8.0))
                    .inner_margin(egui::Margin::same(8.0))
                    .show(ui, |ui| {
                        if ui.button("Zakończ").clicked() {
                            state.finished = true;
                        }
                    });
            });

    });
}

