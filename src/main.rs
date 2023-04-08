use minehunter::MineHunterApp;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mine Hunter",
        native_options,
        Box::new(|cc| Box::new(MineHunterApp::new(cc))),
    )
}