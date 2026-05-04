use eframe::egui;
use rodio::OutputStream;
use crate::ui::{home, piano::AppPiano, guitar::AppGuitar};
use crate::core::theory::DEFAULT_COLORS;

// serializable app state
#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum AppState {
    Home,
    Piano,
    Guitar,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // fallback to default if storage is empty/corrupt
pub struct MainApp {
    pub state: AppState,
    pub palette: [egui::Color32; 12],
    
    // field skipping
    #[serde(skip)] 
    pub piano_app: Option<AppPiano>, 
    #[serde(skip)]
    pub guitar_app: Option<AppGuitar>,
    #[serde(skip)]
    _stream: Option<OutputStream>,
}


impl Default for MainApp {
    fn default() -> Self {
        Self {
            state: AppState::Home,
            palette: DEFAULT_COLORS,
            piano_app: None,
            guitar_app: None,
            _stream: None,
        }
    }
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>, piano: AppPiano, guitar: AppGuitar, stream: OutputStream) -> Self {
        // load from disk
        let mut app: Self = cc.storage
            .and_then(|s| eframe::get_value(s, eframe::APP_KEY))
            .unwrap_or_default();
        
        // runtime re-injection
        app.piano_app = Some(piano);
        app.guitar_app = Some(guitar);
        app._stream = Some(stream);
        
        app
    }
}

impl eframe::App for MainApp {
    // auto-save on exit
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.state {
            AppState::Home => {
                home::show(ctx, &mut self.state, &mut self.palette);
            }
            AppState::Piano => {
                show_nav_bar(ctx, &mut self.state);
                if let Some(ref mut piano) = self.piano_app {
                    piano.update(ctx, frame, &self.palette);
                }
            }
            AppState::Guitar => {
                show_nav_bar(ctx, &mut self.state);
                if let Some(ref mut guitar) = self.guitar_app {
                    guitar.update(ctx, frame, &self.palette);
                }
            }
        }
    }
}

fn show_nav_bar(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("nav_bar")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 18)).inner_margin(8.0))
        .show(ctx, |ui| {
            let btn = egui::Button::new(
                egui::RichText::new("⏴  HOME")
                    .color(egui::Color32::from_gray(200))
                    .strong()
                    .extra_letter_spacing(1.0)
            )
            .fill(egui::Color32::TRANSPARENT)
            .frame(false);

            if ui.add(btn).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                *state = AppState::Home;
            }
        });
}