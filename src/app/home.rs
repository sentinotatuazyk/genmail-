use eframe::egui;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen{
    Main,
}

pub struct Home {
    pub screen: Screen,
}

impl Home {
    pub fn new() -> Self{
        Self {
            screen: Screen::Main,
        }
    }
}


pub fn show(ctx: &egui::Context, _state: &mut Home) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Home Screen");
    });

}
