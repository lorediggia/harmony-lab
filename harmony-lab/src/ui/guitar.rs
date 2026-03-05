use eframe::egui;
use rodio::{OutputStreamHandle, Sink};

use crate::core::theory::{ChordPattern, NOTE_NAMES, SCALES};
use crate::core::audio::GuitarWave;
use crate::ui::shared::{theory_panel, top_controls_panel};

pub struct AppGuitar {
    handle: OutputStreamHandle,
    root: usize,
    scale_idx: usize,
    chord_deg: Option<usize>,
    chord_pattern: ChordPattern,
}

impl AppGuitar {
    pub fn new(handle: OutputStreamHandle) -> Self {
        Self {
            handle,
            root: 0,
            scale_idx: 1,
            chord_deg: None,
            chord_pattern: ChordPattern::Triad,
        }
    }

    fn play(&self, freq: f32) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            sink.append(GuitarWave::new(freq));
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

        top_controls_panel(ctx, &mut self.root, &mut self.scale_idx, &mut self.chord_pattern, &mut self.chord_deg);
        theory_panel(ctx, &mut self.root, &mut self.scale_idx, &mut self.chord_deg, palette);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(10, 10, 12)).inner_margin(24.0))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                
                egui::ScrollArea::horizontal()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        
                        let fret_count = 24;
                        let string_spacing = 42.0;
                        
                        // spacing
                        let scale_length = 1600.0;
                        let get_fret_x = |f: usize| -> f32 {
                            scale_length * (1.0 - 2.0_f32.powf(-(f as f32) / 12.0))
                        };
                        
                        let board_width = get_fret_x(fret_count) + 60.0;
                        let board_height = string_spacing * 5.0; 
                        
                        let (resp, ptr) = ui.allocate_painter(egui::vec2(board_width, board_height + 40.0), egui::Sense::click());
                        let offset = resp.rect.min + egui::vec2(40.0, 20.0);

                        let strings = [
                            (4, 329.63, "E"),  // 1a
                            (11, 246.94, "B"), // 2a
                            (7, 196.00, "G"),  // 3a
                            (2, 146.83, "D"),  // 4a
                            (9, 110.00, "A"),  // 5a
                            (4, 82.41, "E"),   // 6a
                        ];

                        let scale_notes = self.current_scale();
                        let chord_notes = self.active_chord();

                        // bg
                        let board_rect = egui::Rect::from_min_max(
                            egui::pos2(offset.x, offset.y - string_spacing * 0.5),
                            egui::pos2(offset.x + get_fret_x(fret_count), offset.y + string_spacing * 5.5),
                        );
                        ptr.rect_filled(board_rect, 0.0, egui::Color32::from_rgb(20, 20, 24));
                        
                        // frets and inlays
                        let marker_color = egui::Color32::from_rgb(40, 40, 45);
                        for fret in 1..=fret_count {
                            let x = offset.x + get_fret_x(fret);
                            let prev_x = offset.x + get_fret_x(fret - 1);
                            let center_x = (x + prev_x) / 2.0;

                            // fret
                            ptr.line_segment(
                                [egui::pos2(x, offset.y), egui::pos2(x, offset.y + board_height)],
                                egui::Stroke::new(2.0, egui::Color32::from_gray(80)),
                            );

                            // inlays
                            if [3, 5, 7, 9, 15, 17, 19, 21].contains(&fret) {
                                ptr.circle_filled(egui::pos2(center_x, offset.y + board_height / 2.0), 8.0, marker_color);
                            } else if [12, 24].contains(&fret) {
                                ptr.circle_filled(egui::pos2(center_x, offset.y + string_spacing * 1.5), 8.0, marker_color);
                                ptr.circle_filled(egui::pos2(center_x, offset.y + string_spacing * 3.5), 8.0, marker_color);
                            }
                        }

                        ptr.line_segment(
                            [offset, egui::pos2(offset.x, offset.y + board_height)],
                            egui::Stroke::new(6.0, egui::Color32::from_gray(120)),
                        );

                        // strings
                        for (s_idx, _) in strings.iter().enumerate() {
                            let y = offset.y + s_idx as f32 * string_spacing;
                            let thickness = 1.0 + (s_idx as f32 * 0.5); 
                            ptr.line_segment(
                                [egui::pos2(offset.x - 30.0, y), egui::pos2(offset.x + get_fret_x(fret_count), y)],
                                egui::Stroke::new(thickness, egui::Color32::from_gray(100)),
                            );
                        }

                        // notes
                        for (s_idx, &(open_idx, base_freq, _)) in strings.iter().enumerate() {
                            let y = offset.y + s_idx as f32 * string_spacing;

                            for fret in 0..=fret_count {
                                let c_idx = (open_idx + fret) % 12;
                                let freq = base_freq * 2.0_f32.powf(fret as f32 / 12.0);

                                let is_active = if self.scale_idx == 0 {
                                    true
                                } else if self.chord_deg.is_some() {
                                    chord_notes.contains(&c_idx)
                                } else {
                                    scale_notes.contains(&c_idx)
                                };

                                if is_active {
                                    let node_x = if fret == 0 {
                                        offset.x - 20.0 // open strings
                                    } else {
                                        offset.x + (get_fret_x(fret - 1) + get_fret_x(fret)) / 2.0
                                    };
                                    
                                    let pos = egui::pos2(node_x, y);
                                    
                                    let col = if self.scale_idx == 0 { 
                                        egui::Color32::from_gray(80) 
                                    } else { 
                                        palette[c_idx] 
                                    };

                                    let hit_w = if fret == 0 { 24.0 } else { (get_fret_x(fret) - get_fret_x(fret-1)) * 0.8 };
                                    let interact_rect = egui::Rect::from_center_size(pos, egui::vec2(hit_w, string_spacing * 0.8));
                                    
                                    let interaction = ui.interact(interact_rect, ui.id().with(s_idx).with(fret), egui::Sense::click());

                                    let render_col = if interaction.hovered() {
                                        egui::Color32::from_rgb(col.r().saturating_add(50), col.g().saturating_add(50), col.b().saturating_add(50))
                                    } else {
                                        col
                                    };

                                    if interaction.clicked() { self.play(freq); }

                                    ptr.circle_filled(pos, 12.0, render_col);
                                    ptr.circle_stroke(pos, 12.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
                                    
                                    ptr.text(
                                        pos,
                                        egui::Align2::CENTER_CENTER,
                                        NOTE_NAMES[c_idx],
                                        egui::FontId::proportional(11.0),
                                        if render_col.r() as u32 + render_col.g() as u32 + render_col.b() as u32 > 400 { egui::Color32::BLACK } else { egui::Color32::WHITE },
                                    );
                                }
                            }
                        }
                    });
            });
    }
}