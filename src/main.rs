mod window;
mod app;
mod dbadapt;


use app::App;
use eframe::egui;
use window::Window;

fn main() -> eframe::Result<()> {

    let window = Window::default();

    let option = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([window.width as f32, window.height as f32]),
        ..Default::default()
    };

    eframe::run_native(
        &window.title,
        option,
        Box::new(|_cc| Box::<App>::default()),
    )

}

