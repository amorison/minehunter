use minehunter::MineHunterApp;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Mine Hunter",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(MineHunterApp::new(cc))),
    )
}
