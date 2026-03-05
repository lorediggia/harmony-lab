use eframe::egui;
use rodio::{OutputStreamHandle, Sink};

use crate::core::theory::{ChordPattern, NOTE_NAMES, SCALES};
use crate::core::audio::PianoWave;
use crate::ui::shared::{theory_panel, top_controls_panel};

struct KeyData {
    name: String,
    chromatic_idx: usize,
    freq: f32,
    is_black: bool,
    white_idx: usize,
}

fn generate_keys() -> Vec<KeyData> {
    let mut keys = Vec::with_capacity(88);
    let is_black = [false, true, false, true, false, false, true, false, true, false, true, false];
    let mut w_idx = 0;
    
    for i in 0..88 {
        let c_idx = (i + 9) % 12;
        let black = is_black[c_idx];
        keys.push(KeyData {
            name: format!("{}{}", NOTE_NAMES[c_idx], (i + 9) / 12),
            chromatic_idx: c_idx,
            freq: 27.5 * 2.0_f32.powf(i as f32 / 12.0),
            is_black: black,
            white_idx: if black { w_idx - 1 } else { w_idx },
        });
        if !black { w_idx += 1; }
    }
    keys
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let fonts = egui::FontDefinitions::default(); 
    ctx.set_fonts(fonts);
    
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(12.0, 12.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.visuals.window_rounding = egui::Rounding::same(12.0);
    ctx.set_style(style);
}

pub struct AppPiano {
    handle: OutputStreamHandle,
    keys: Vec<KeyData>,
    root: usize,
    scale_idx: usize,
    chord_deg: Option<usize>,
    chord_pattern: ChordPattern,
}

impl AppPiano {
    pub fn new(handle: OutputStreamHandle) -> Self {
        Self {
            handle,
            keys: generate_keys(),
            root: 0,
            scale_idx: 1,
            chord_deg: None,
            chord_pattern: ChordPattern::Triad,
        }
    }

    fn play(&self, freq: f32) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            sink.append(PianoWave::new(freq));
            sink.detach();
        }
    }

    fn current_scale(&self) -> Vec<usize> {
        SCALES[self.scale_idx]
            .intervals
            .iter()
            .map(|&i| (self.root + i) % 12)
            .collect()
    }

    fn active_chord(&self) -> Vec<usize> {
        if let Some(deg) = self.chord_deg {
            let scale = self.current_scale();
            let n = scale.len();
            
            self.chord_pattern
                .intervals()
                .iter()
                .map(|&offset| scale[(deg + offset) % n])
                .collect()
        } else {
            vec![]
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _f: &mut eframe::Frame, palette: &[egui::Color32; 12]) {
        let mut vis = egui::Visuals::dark();
        vis.panel_fill = egui::Color32::from_rgb(15, 15, 18);
        vis.override_text_color = Some(egui::Color32::from_gray(230));
        ctx.set_visuals(vis);

        // UI
        top_controls_panel(ctx, &mut self.root, &mut self.scale_idx, &mut self.chord_pattern, &mut self.chord_deg);
        theory_panel(ctx, &mut self.root, &mut self.scale_idx, &mut self.chord_deg, palette);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(10, 10, 12)).inner_margin(24.0))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                
                egui::ScrollArea::horizontal()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let w_w = 48.0;
                        let w_h = 280.0;
                        let b_w = w_w * 0.6;
                        let b_h = w_h * 0.65;
                        
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(52.0 * w_w, w_h), egui::Sense::hover());

                        let scale_notes = self.current_scale();
                        let chord_notes = self.active_chord();

                        // white keys
                        for k in self.keys.iter().filter(|k| !k.is_black) {
                            let r = egui::Rect::from_min_size(
                                egui::pos2(rect.min.x + k.white_idx as f32 * w_w, rect.min.y),
                                egui::vec2(w_w, w_h),
                            );

                            let mut col = if self.scale_idx == 0 {
                                egui::Color32::from_gray(245)
                            } else if self.chord_deg.is_some() {
                                if chord_notes.contains(&k.chromatic_idx) { palette[k.chromatic_idx] } else { egui::Color32::from_gray(40) }
                            } else if scale_notes.contains(&k.chromatic_idx) {
                                palette[k.chromatic_idx]
                            } else {
                                egui::Color32::from_gray(60)
                            };

                            let resp = ui.interact(r, ui.id().with(&k.name), egui::Sense::click());
                            if resp.hovered() { col = hl(col); }
                            if resp.clicked() { self.play(k.freq); }

                            ui.painter().rect(
                                r.shrink(1.0),
                                egui::Rounding { nw: 2., ne: 2., sw: 6., se: 6. },
                                col,
                                egui::Stroke::new(1.0, egui::Color32::from_gray(20))
                            );
                        }

                        // black keys
                        for k in self.keys.iter().filter(|k| k.is_black) {
                            let r = egui::Rect::from_min_size(
                                egui::pos2(
                                    rect.min.x + k.white_idx as f32 * w_w + (w_w - b_w / 2.0),
                                    rect.min.y,
                                ),
                                egui::vec2(b_w, b_h),
                            );

                            let mut col = if self.scale_idx == 0 {
                                egui::Color32::from_rgb(25, 25, 25)
                            } else if self.chord_deg.is_some() {
                                if chord_notes.contains(&k.chromatic_idx) { palette[k.chromatic_idx] } else { egui::Color32::from_rgb(15, 15, 15) }
                            } else if scale_notes.contains(&k.chromatic_idx) {
                                palette[k.chromatic_idx]
                            } else {
                                egui::Color32::from_rgb(15, 15, 15)
                            };

                            let resp = ui.interact(r, ui.id().with(&k.name), egui::Sense::click());
                            if resp.hovered() { col = hl_dark(col); }
                            if resp.clicked() { self.play(k.freq); }

                            ui.painter().rect(
                                r,
                                egui::Rounding { nw: 0., ne: 0., sw: 4., se: 4. },
                                col,
                                egui::Stroke::new(1.0, egui::Color32::BLACK),
                            );
                        }
                    });
            });
    }
}

// helpers
fn hl(c: egui::Color32) -> egui::Color32 {
    egui::Color32::from_rgb(c.r().saturating_add(30), c.g().saturating_add(30), c.b().saturating_add(30))
}

fn hl_dark(c: egui::Color32) -> egui::Color32 {
    egui::Color32::from_rgb(c.r().saturating_add(50), c.g().saturating_add(50), c.b().saturating_add(50))
}