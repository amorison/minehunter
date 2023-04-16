use eframe::epaint::vec2;
use minehunter::MineHunterApp;

fn main() -> Result<(), eframe::Error> {
    let opts = eframe::NativeOptions {
        initial_window_size: Some(vec2(1080.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Mine Hunter",
        opts,
        Box::new(|cc| Box::new(MineHunterApp::new(cc))),
    )
}
