use super::configshandler::{Template, Placeholder, render_template};
use eframe::egui::{self, RichText};

pub enum SectorType {
    Editor {
        template: Template,
        extra_placeholders: Vec<Placeholder>,
    },
    Calendar {
        selected_date: String,
    },
    Finished {
        template: Template,
        extra_placeholders: Vec<Placeholder>,
    },
    TextWithButton {
        text: String,
        text_color: egui::Color32,
        text_size: f32,
        button_text: String,
        button_color: egui::Color32,
    },
}

pub struct Sector {
    pub sector_type: SectorType,
    pub rect: egui::Rect,
}

impl Sector {
    pub fn finished(rect: egui::Rect, template: Template, extra_placeholders: Vec<Placeholder>) -> Self {
        Self {
            sector_type: SectorType::Finished { template, extra_placeholders },
            rect,
        }
    }

    pub fn editor(rect: egui::Rect, template: Template, extra_placeholders: Vec<Placeholder>) -> Self {
        Self {
            sector_type: SectorType::Editor { template, extra_placeholders },
            rect,
        }
    }

    pub fn calendar(rect: egui::Rect) -> Self {
        Self {
            sector_type: SectorType::Calendar { selected_date: String::new() },
            rect,
        }
    }

    pub fn text_with_button(rect: egui::Rect, text: String, button_text: String) -> Self {
        Self {
            sector_type: SectorType::TextWithButton {
                text,
                text_color: egui::Color32::WHITE,
                text_size: 20.0,
                button_text,
                button_color: egui::Color32::GRAY,
            },
            rect,
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.painter().rect_filled(
            self.rect,
            egui::Rounding::same(10.0),
            egui::Color32::from_rgb(10, 10, 10),
        );

        match &self.sector_type {
            SectorType::Editor { template, .. } => {
                ui.allocate_ui_at_rect(self.rect, |ui| {
                    ui.label(RichText::new(&template.label).font(egui::FontId::proportional(20.0)).color(egui::Color32::WHITE));
                });
            }
            SectorType::Calendar { selected_date } => {
                ui.allocate_ui_at_rect(self.rect, |ui| {
                    ui.label(RichText::new(format!("Data: {}", selected_date)).color(egui::Color32::WHITE));
                });
            }
            SectorType::Finished { template, extra_placeholders } => {
                let email = render_template(template, extra_placeholders);
                ui.allocate_ui_at_rect(self.rect, |ui| {
                    ui.label(RichText::new("Finished Sector").font(egui::FontId::proportional(20.0)).color(egui::Color32::WHITE));
                    ui.label(RichText::new(email.subject).font(egui::FontId::proportional(14.0)).color(egui::Color32::LIGHT_GRAY));
                    ui.label(RichText::new(email.body).font(egui::FontId::proportional(14.0)).color(egui::Color32::LIGHT_GRAY));
                });
            }
            SectorType::TextWithButton { text, text_color, text_size, button_text, .. } => {
                ui.allocate_ui_at_rect(self.rect, |ui| {
                    ui.label(RichText::new(text).font(egui::FontId::proportional(*text_size)).color(*text_color));
                    if ui.button(button_text).clicked() {
                        println!("Button in TextWithButton sector clicked!");
                    }
                });
            }
        }
    }
}