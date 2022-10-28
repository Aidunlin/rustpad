use rustpad::RustpadApp;

use eframe::{run_native, NativeOptions};

fn main() {
    run_native(
        "Rustpad",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(RustpadApp::default())),
    );
}
