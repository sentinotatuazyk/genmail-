use eframe::epaint::tessellator::path;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Placeholder {
    pub name: String,
    pub truevalue: String,
    pub example: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub templates_file_path: String,
    pub new_reservation_name: String,
    pub update_reservation_name: String,
    pub delete_reservation_name: String,
    pub custom_templates_name: Vec<String>,
    pub placeholders: Vec<Placeholder>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    pub label: String,
    pub subject: String,
    pub body: String,
    pub placeholders: Vec<Placeholder>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DefaultTemplate {
    pub new_reservation: Template,
    pub update_reservation: Template,
    pub delete_reservation: Template,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TemplateConfigs {
    pub templates: Vec<Template>,
}

impl Template {
    pub fn none() -> Self {
        Self {
            label: String::new(),
            subject: String::new(),
            body: String::new(),
            placeholders: Vec::new(),
        }
    }
}

pub fn create_default_config(path: & str) -> std::io::Result<()> {
    let default_config = Config {
        templates_file_path: String::from("configs/templates.json"),
        new_reservation_name: String::from("rezerwacja preset"),
        update_reservation_name: String::from("aktualizacja preset"),
        delete_reservation_name: String::from("odwołanie preset"),
        custom_templates_name: vec![],
        placeholders: vec![
            Placeholder {
                name: String::from("creds"),
                truevalue: String::from(""),
                example: String::from("Jan Kowalski"),
            },
            Placeholder {
                name: String::from("signature"),
                truevalue: String::from(""),
                example: String::from("Twoja firma"),
            }
        ],
    };

    let config_json = serde_json::to_string_pretty(&default_config).expect("Failed to serialize default config");
    fs::write(path, config_json).expect("Failed to write default config to file");
    Ok(())
}

pub fn change_config(path: &str, new_config: &Config) -> std::io::Result<()> {
    let config_json = serde_json::to_string_pretty(new_config).expect("Failed to serialize new config");
    fs::write(path, config_json).expect("Failed to write new config to file");
    Ok(())
}

pub fn load_config(path: & str) -> std::io::Result<Config> {
    let config_json = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_json).expect("Failed to deserialize config");
    Ok(config)
}

pub fn load_templates(path: &str) -> std::io::Result<Vec<Template>> {
    let templates_json = fs::read_to_string(path)?;
    let template_configs: TemplateConfigs = serde_json::from_str(&templates_json)
        .expect("Failed to deserialize templates");
    Ok(template_configs.templates)
}


pub fn find_template_by_label<'a>(templates: &'a Vec<Template>, label: &str) -> Option<&'a Template> {
    templates.iter().find(|template| template.label == label)
}

pub struct RenderedEmail {
    pub subject: String,
    pub body: String,
}

pub fn render_template(template: &Template, extra_placeholders: &[Placeholder]) -> RenderedEmail {
    let mut all_placeholders: Vec<&Placeholder> = template.placeholders.iter().collect();
    all_placeholders.extend(extra_placeholders.iter());

    let mut subject = template.subject.clone();
    let mut body = template.body.clone();

    for placeholder in all_placeholders {
        let pattern = format!("{{{{{}}}}}", placeholder.name); // {{name}}

        let value = if !placeholder.truevalue.is_empty() {
            &placeholder.truevalue
        } else {
            &placeholder.example
        };

        subject = subject.replace(&pattern, value);
        body = body.replace(&pattern, value);
    }

    RenderedEmail { subject, body }
}