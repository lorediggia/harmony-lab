use eframe::egui;
use std::f32::consts::PI;
use crate::core::theory::{
    ChordPattern, ACCIDENTALS, CIRCLE_OF_FIFTHS, NOTE_NAMES, SCALES,
};

pub fn top_controls_panel(
    ctx: &egui::Context,
    root: &mut usize,
    scale_idx: &mut usize,
    chord_pattern: &mut ChordPattern,
    chord_deg: &mut Option<usize>,
) {
    egui::TopBottomPanel::top("top_panel_controls")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(22, 22, 26)).inner_margin(16.0))
        .show(ctx, |ui| {
            ui.columns(3, |cols| {
                cols[0].vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Root").strong().size(14.0).color(egui::Color32::from_gray(160)));
                    ui.add_space(4.0);
                    let changed = egui::ComboBox::from_id_source("rt_shared")
                        .width(120.0)
                        .selected_text(NOTE_NAMES[*root])
                        .show_ui(ui, |ui| {
                            (0..12).any(|i| ui.selectable_value(root, i, NOTE_NAMES[i]).changed())
                        }).inner.unwrap_or(false);
                    if changed { *chord_deg = None; }
                });

                cols[1].vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Scale").strong().size(14.0).color(egui::Color32::from_gray(160)));
                    ui.add_space(4.0);
                    let changed = egui::ComboBox::from_id_source("sc_shared")
                        .width(220.0)
                        .selected_text(SCALES[*scale_idx].name)
                        .show_ui(ui, |ui| {
                            SCALES.iter().enumerate().any(|(i, s)| {
                                ui.selectable_value(scale_idx, i, s.name).changed()
                            })
                        }).inner.unwrap_or(false);
                    if changed { *chord_deg = None; }
                });

                cols[2].vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Chord Pattern").strong().size(14.0).color(egui::Color32::from_gray(160)));
                    ui.add_space(4.0);
                    egui::ComboBox::from_id_source("pat_shared")
                        .width(220.0)
                        .selected_text(chord_pattern.name())
                        .show_ui(ui, |ui| {
                            let gray = |s| egui::RichText::new(s).color(egui::Color32::from_gray(120));
                            let mut opt = |ui: &mut egui::Ui, p: ChordPattern| { 
                                ui.selectable_value(chord_pattern, p, p.name());
                            };

                            ui.label(gray("Built in Thirds"));
                            opt(ui, ChordPattern::Triad);
                            opt(ui, ChordPattern::Seventh);
                            opt(ui, ChordPattern::Ninth);
                            opt(ui, ChordPattern::Eleventh);
                            opt(ui, ChordPattern::Thirteenth);
                            ui.separator();
                            ui.label(gray("Suspended"));
                            opt(ui, ChordPattern::Sus2);
                            opt(ui, ChordPattern::Sus4);
                            opt(ui, ChordPattern::SevenSus2);
                            opt(ui, ChordPattern::SevenSus4);
                            ui.separator();
                            ui.label(gray("Added"));
                            opt(ui, ChordPattern::Add9);
                            opt(ui, ChordPattern::Add11);
                            opt(ui, ChordPattern::Add13);
                            ui.separator();
                            ui.label(gray("Alternative Structures"));
                            opt(ui, ChordPattern::PowerChord);
                            opt(ui, ChordPattern::Quartal3);
                            opt(ui, ChordPattern::Quartal4);
                            opt(ui, ChordPattern::Cluster);
                        });
                });
            });
        });
}

fn roman(deg: usize, d3: usize, d5: usize) -> (&'static str, &'static str) {
    let maj = ["I","II","III","IV","V","VI","VII"];
    let min = ["i","ii","iii","iv","v","vi","vii"];
    let dim = ["i°","ii°","iii°","iv°","v°","vi°","vii°"];
    let aug = ["I+","II+","III+","IV+","V+","VI+","VII+"];
    let r = deg.min(6);
    match (d3, d5) {
        (4, 7) => (maj[r], "M"),
        (3, 7) => (min[r], "m"),
        (3, 6) => (dim[r], "dim"),
        (4, 8) => (aug[r], "aug"),
        _      => (maj[r], "?"),
    }
}

pub fn theory_panel(
    ctx: &egui::Context,
    root: &mut usize,
    scale_idx: &mut usize,
    chord_deg: &mut Option<usize>,
    palette: &[egui::Color32; 12],
) {
    egui::TopBottomPanel::bottom("theory_panel_shared")
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(18, 18, 22)).inner_margin(24.0))
        .exact_height(380.0)
        .show(ctx, |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical_centered(|ui| {
                    let (resp, ptr) = ui.allocate_painter(egui::vec2(320.0, 320.0), egui::Sense::click());
                    let c = resp.rect.center();

                    let positions: Vec<_> = (0..12).map(|i| {
                        let a = i as f32 * (PI / 6.0) - PI / 2.0;
                        (
                            c + egui::vec2(a.cos() * 110.0, a.sin() * 110.0), // major
                            c + egui::vec2(a.cos() * 65.0,  a.sin() * 65.0),  // minor
                            c + egui::vec2(a.cos() * 145.0, a.sin() * 145.0), // accidental label
                        )
                    }).collect();

                    let stroke_bg = egui::Stroke::new(1.0, egui::Color32::from_gray(40));
                    for i in 0..12 {
                        let j = (i + 1) % 12;
                        ptr.line_segment([positions[i].0, positions[j].0], stroke_bg);
                        ptr.line_segment([positions[i].1, positions[j].1], stroke_bg);
                        ptr.line_segment([positions[i].0, positions[i].1], stroke_bg);
                    }

                    ptr.text(c, egui::Align2::CENTER_CENTER, "Circle of Fifths",
                        egui::FontId::proportional(14.0), egui::Color32::from_gray(120));

                    let clicked = resp.interact_pointer_pos()
                        .filter(|_| ui.input(|i| i.pointer.primary_clicked()));

                    for (i, &maj_idx) in CIRCLE_OF_FIFTHS.iter().enumerate() {
                        let min_idx = (maj_idx + 9) % 12;
                        let (pos_maj, pos_min, pos_acc) = positions[i];

                        ptr.text(pos_acc, egui::Align2::CENTER_CENTER, ACCIDENTALS[i],
                            egui::FontId::proportional(12.0), egui::Color32::from_gray(100));

                        let bg_maj = if *root == maj_idx && *scale_idx == 1 { palette[maj_idx] } else { egui::Color32::from_gray(30) };
                        ptr.circle_filled(pos_maj, 18.0, bg_maj);
                        ptr.circle_stroke(pos_maj, 18.0, egui::Stroke::new(1.0, egui::Color32::from_gray(60)));
                        ptr.text(pos_maj, egui::Align2::CENTER_CENTER, NOTE_NAMES[maj_idx],
                            egui::FontId::proportional(14.0), egui::Color32::WHITE);

                        let bg_min = if *root == min_idx && *scale_idx == 6 { palette[min_idx] } else { egui::Color32::from_gray(20) };
                        ptr.circle_filled(pos_min, 14.0, bg_min);
                        ptr.circle_stroke(pos_min, 14.0, egui::Stroke::new(1.0, egui::Color32::from_gray(50)));
                        ptr.text(pos_min, egui::Align2::CENTER_CENTER, format!("{}m", NOTE_NAMES[min_idx]),
                            egui::FontId::proportional(11.0), egui::Color32::WHITE);

                        if let Some(mp) = clicked {
                            if mp.distance(pos_maj) < 18.0 { *root = maj_idx; *scale_idx = 1; *chord_deg = None; }
                            if mp.distance(pos_min) < 14.0 { *root = min_idx; *scale_idx = 6; *chord_deg = None; }
                        }
                    }
                });

                cols[1].vertical(|ui| {
                    ui.heading(egui::RichText::new("Degrees and Chords").size(20.0).strong());
                    ui.add_space(16.0);

                    if *scale_idx == 0 {
                        ui.label(egui::RichText::new("Select a scale to calculate chords.").color(egui::Color32::from_gray(150)));
                        return;
                    }

                    let scale: Vec<usize> = SCALES[*scale_idx].intervals.iter()
                        .map(|&i| (*root + i) % 12)
                        .collect();

                    ui.horizontal_wrapped(|ui| {
                        for deg in 0..scale.len() {
                            let root_note = scale[deg];
                            let d3 = (scale[(deg + 2) % scale.len()] + 12 - root_note) % 12;
                            let d5 = (scale[(deg + 4) % scale.len()] + 12 - root_note) % 12;
                            let (numeral, suffix) = roman(deg, d3, d5);

                            let active = *chord_deg == Some(deg);
                            let bg = if active { palette[root_note] } else { egui::Color32::from_rgb(30, 30, 35) };

                            let btn = egui::Button::new(
                                egui::RichText::new(format!("{}\n{}{}", numeral, NOTE_NAMES[root_note], suffix))
                                    .size(16.0)
                                    .color(if active { egui::Color32::BLACK } else { egui::Color32::WHITE }),
                            )
                            .fill(bg)
                            .min_size(egui::vec2(75.0, 75.0))
                            .rounding(12.0);

                            if ui.add(btn).clicked() {
                                *chord_deg = if active { None } else { Some(deg) };
                            }
                        }
                    });
                });
            });
        });
}