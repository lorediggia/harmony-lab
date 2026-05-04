mod app;
mod core;
mod ui;

use app::MainApp;
use ui::{piano, guitar};
use rodio::OutputStream;

fn main() -> Result<(), eframe::Error> {
    // audio
    let (stream, handle) = OutputStream::try_default().expect("Failed to find audio output device");
    
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 850.0])
            .with_title("Harmony Lab"),
        ..Default::default()
    };

    eframe::run_native(
        "Harmony Lab",
        options,
        Box::new(|cc| {
            // font setup / style
            piano::setup_custom_fonts(&cc.egui_ctx);
            
            // instruments app
            let piano_app = piano::AppPiano::new(handle.clone());
            let guitar_app = guitar::AppGuitar::new(handle);

            // mainapp / cc injection
            Box::new(MainApp::new(cc, piano_app, guitar_app, stream))
        }),
    )
}