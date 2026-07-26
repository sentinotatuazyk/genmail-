use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path,PathBuf};
use regex::Regex;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum DBTypes{
    None,
    Access,
    SQLite,
}

impl std::fmt::Display for DBTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            DBTypes::None => "Brak",
            DBTypes::Access => "MS Access",
            DBTypes::SQLite => "SQLite",
        };
        write!(f, "{}", s)
    } 
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TemplateTypes {
    None,
    NewRes,
    UpdateRes,
    CancelRes,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ConfigVar {
    TemplatesFilePath(String),
    NewReservationName(String),
    UpdateReservationName(String),
    DeleteReservationName(String),
    CustomTemplatesName(Vec<String>),
    Placeholders(Vec<Placeholder>),
    DBtype(DBTypes), 
    DBpath(Option<PathBuf>),
}

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
    pub db_type: DBTypes, 
    pub db_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Template {
    pub ttype: TemplateTypes, 
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
            ttype: TemplateTypes::None,
            label: String::new(),
            subject: String::new(),
            body: String::new(),
            placeholders: Vec::new(),
        }
    }
    pub fn new(Ttype: TemplateTypes ,Label: String, Subject: String, Body: String, Placeholders: Vec<Placeholder>) -> Self {
        Self {
            ttype: Ttype,
            label: Label,
            subject: Subject,
            body: Body,
            placeholders: Placeholders,
        }
    }
}

impl Placeholder {
    pub fn new(Name: String,Truevalue: String, Example:String) -> Self {
        Self {
            name: Name,
            truevalue: Truevalue,
            example: Example,
        }
    }
}

pub fn configs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs")
}

fn ensure_parent_dir(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}
pub fn create_default_config(path: &Path) -> std::io::Result<()> {
    let default_config = Config {
        templates_file_path: String::from("templates.json"),
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
        db_type: DBTypes::None,
        db_path: None,
    };

    ensure_parent_dir(path)?;
    let config_json = serde_json::to_string_pretty(&default_config).expect("Failed to serialize default config");
    fs::write(path, config_json).expect("Failed to write default config to file");
    Ok(())
}

pub fn set_default_template(ttype: TemplateTypes, label: String) -> std::io::Result<()> {
    let config_path = configs_dir().join("config.json");
    let mut config = load_config(&config_path)?;

    match ttype {
        TemplateTypes::NewRes =>config.new_reservation_name = label,
        TemplateTypes::UpdateRes => config.update_reservation_name = label,
        TemplateTypes::CancelRes => config.delete_reservation_name = label,
        TemplateTypes::None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Nie mozna ustawić domyślnego szablonu dla typu None",
            ));
        }
    }

    change_config(&config_path, &config)?;
    Ok(())
}

pub fn change_config(path: &Path, new_config: &Config) -> std::io::Result<()> {
    ensure_parent_dir(path)?;
    let config_json = serde_json::to_string_pretty(new_config).expect("Failed to serialize new config");
    fs::write(path, config_json).expect("Failed to write new config to file");
    Ok(())
}

pub fn change_config_field(val: ConfigVar) ->std::io::Result<()> {
    let config_path = configs_dir().join("config.json");
    let mut config = load_config(&config_path)?;
    
    match val {
        ConfigVar::TemplatesFilePath(v) => config.templates_file_path = v,
        ConfigVar::NewReservationName(v) => config.new_reservation_name = v,
        ConfigVar::UpdateReservationName(v) => config.update_reservation_name = v,
        ConfigVar::DeleteReservationName(v) => config.delete_reservation_name = v,
        ConfigVar::CustomTemplatesName(v) => config.custom_templates_name = v,
        ConfigVar::Placeholders(v) => config.placeholders = v,
        ConfigVar::DBtype(v) => config.db_type = v,
        ConfigVar::DBpath(v) => config.db_path = v,
    }
    change_config(&config_path, &config)?;
    Ok(())
}

pub fn load_config(path: &Path) -> std::io::Result<Config> {
    ensure_parent_dir(path)?;
    let config_json = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_json).expect("Failed to deserialize config");
    Ok(config)
}

pub fn load_templates(path: &Path) -> std::io::Result<Vec<Template>> {
    ensure_parent_dir(path)?;
    let templates_json = fs::read_to_string(path)?;
    let template_configs: TemplateConfigs = serde_json::from_str(&templates_json)
        .expect("Failed to deserialize templates");
    Ok(template_configs.templates)
}


pub fn find_template_by_label<'a>(templates: &'a Vec<Template>, label: &str) -> Option<&'a Template> {
    templates.iter().find(|template| template.label == label)
}

pub fn find_templates_by_label<'a>(templates: &'a Vec<Template>, label: &str) -> Option<Vec<&'a Template>> {
    let pasujace_szablony: Vec<&Template> = templates.iter()
        .filter(|template| template.label.to_lowercase().contains(&label.to_lowercase()))
        .collect();

    let wynik: Option<Vec<&Template>> = if pasujace_szablony.is_empty() { None } else { Some(pasujace_szablony) };
    wynik
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

pub fn find_placeholders_in_body (body:&str) -> Vec<Placeholder> {
    let re_ph = Regex::new(r"\{\{([^}]+)\}\}").unwrap();

    re_ph.captures_iter(body)
        .map(|caps|{
            let name = caps[1].to_string();
            Placeholder::new(name, String::new(), String::new())
        })
        .collect()
}

pub fn add_template(body: String, ttype: TemplateTypes) -> std::io::Result<String>{
    let path = configs_dir().join("templates.json");

    let mut templates = if path.exists() {
        load_templates(&path)?
    } else {
        Vec::new()
    };
    
    let max_number = templates.iter()
        .filter_map(|t| t.label.strip_prefix("custom"))
        .filter_map(|numer_str| numer_str.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
        
    let label = format!("custom{}",max_number+1);
    let placeholders = find_placeholders_in_body(&body);
    let new_template = Template::new(ttype, label.clone(), String::new(), body.clone(),placeholders);

    templates.push(new_template);
    
    let template_configs = TemplateConfigs {templates};
    ensure_parent_dir(&path)?;
    let json = serde_json::to_string_pretty(&template_configs)
        .expect("Failed to serialize templates");
    fs::write(&path,json)?;

    Ok(label)
}
