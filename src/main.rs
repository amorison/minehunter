use eframe::{egui::ViewportBuilder, epaint::vec2};
use minehunter::MineHunterApp;

fn main() -> Result<(), eframe::Error> {
    let opts = eframe::NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(vec2(1080.0, 720.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Mine Hunter",
        opts,
        Box::new(|cc| Ok(Box::new(MineHunterApp::new(cc)))),
    )
}
