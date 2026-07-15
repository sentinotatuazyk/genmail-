use eframe::egui;

pub struct Window{
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: String::from("My Window"),
            width: 800,
            height: 600,
        }
    }
}

