pub mod configshandler;
mod first_configuration;
mod home;

use crate::dbadapt;
use std::fs;
use eframe::egui;
use configshandler::{Config, Template, load_config, load_templates, find_template_by_label};
use first_configuration::FirstConfigurationState;
use home::Home;

enum Screen {
    Home(Home),
    FirstConfiguration(FirstConfigurationState),
}

pub struct App {
    pub screen: Screen,
    pub config: Config,
    pub db: Box<dyn dbadapt::DBConnection>,
}

impl Default for App {
    fn default() -> Self {
        let config_path = configshandler::configs_dir().join("config.json");
        let templates_path = configshandler::configs_dir().join("templates.json");

        let config_existed = fs::metadata(&config_path).is_ok();

        if !config_existed {
            configshandler::create_default_config(&config_path)
                .expect("Failed to create default config");
        }

        let config: Config = load_config(&config_path).expect("Failed to load config");
        let templates: Vec<Template> =
            load_templates(&templates_path).expect("Failed to load templates");

        let db: Box<dyn dbadapt::DBConnection> = match &config.db_path {
            Some(path) if path.exists() => {
                dbadapt::connect(&config).unwrap_or_else(|_| dbadapt::empty())
            }
            _ => dbadapt::empty(),
        };

        let default_template = configshandler::DefaultTemplate {
            new_reservation: find_template_by_label(&templates, &config.new_reservation_name)
                .cloned()
                .expect("Default template 'new_reservation' not found"),
            update_reservation: find_template_by_label(&templates, &config.update_reservation_name)
                .cloned()
                .expect("Default template 'update_reservation' not found"),
            delete_reservation: find_template_by_label(&templates, &config.delete_reservation_name)
                .cloned()
                .expect("Default template 'delete_reservation' not found"),
        };

        let screen = if config_existed {
            Screen::Home(Home::new())
        } else {
            Screen::FirstConfiguration(FirstConfigurationState::new(default_template))
        };

        Self { screen, config, db }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut should_go_home = false;
        match &mut self.screen {
            Screen::Home(state) => {
                home::show(ctx, state);
            }
            Screen::FirstConfiguration(state) => {
                first_configuration::show(ctx, state);

                should_go_home = state.finished;
            }
        }
        if should_go_home {
            self.screen = Screen::Home(Home::new());
        }
    }
}
