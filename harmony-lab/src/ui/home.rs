use eframe::egui;
use crate::app::AppState;
use crate::core::theory::{DEFAULT_COLORS, NOTE_NAMES};

pub fn show(ctx: &egui::Context, state: &mut AppState, current_palette: &mut [egui::Color32; 12]) {
    let mut vis = egui::Visuals::dark();
    vis.panel_fill = egui::Color32::from_rgb(10, 10, 12);
    ctx.set_visuals(vis);

    let mut style = (*ctx.style()).clone();
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(20.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(20.0);
    ctx.set_style(style);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add_space(ui.available_height() * 0.20);

            ui.heading(
                egui::RichText::new("Harmony by Ren")
                    .family(egui::FontFamily::Proportional)
                    .size(72.0)
                    .strong()
                    .extra_letter_spacing(12.0)
                    .color(egui::Color32::WHITE),
            );

            ui.add_space(16.0);

            ui.label(
                egui::RichText::new("Select an instrument")
                    .family(egui::FontFamily::Monospace)
                    .size(18.0)
                    .color(egui::Color32::from_gray(140))
                    .extra_letter_spacing(4.0),
            );

            ui.add_space(60.0);

            let btn_style = |text: &str| {
                egui::Button::new(
                    egui::RichText::new(text)
                        .family(egui::FontFamily::Monospace)
                        .size(18.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                )
                .min_size(egui::vec2(220.0, 60.0))
                .rounding(30.0)
                .fill(egui::Color32::from_rgb(18, 18, 22))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(60)))
            };

            ui.allocate_ui_with_layout(
                egui::vec2(480.0, 60.0),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    if ui.add(btn_style("P I A N O")).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        *state = AppState::Piano;
                    }
                    ui.add_space(40.0);
                    if ui.add(btn_style("G U I T A R")).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                        *state = AppState::Guitar;
                    }
                },
            );

            ui.add_space(100.0);

            ui.label(
                egui::RichText::new("C U S T O M   P A L E T T E")
                    .family(egui::FontFamily::Monospace)
                    .size(13.0)
                    .color(egui::Color32::from_gray(80))
                    .extra_letter_spacing(4.0),
            );

            ui.add_space(24.0);

            ui.allocate_ui_with_layout(
                egui::vec2(700.0, 80.0),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 18.0;
                    for i in 0..12 {
                        ui.vertical(|ui| {
                            ui.set_width(40.0);
                            ui.label(
                                egui::RichText::new(NOTE_NAMES[i])
                                    .family(egui::FontFamily::Monospace)
                                    .size(10.0)
                                    .color(egui::Color32::from_gray(120)),
                            );
                            ui.add_space(4.0);
                            let mut color = current_palette[i];
                            if ui.color_edit_button_srgba(&mut color).changed() {
                                current_palette[i] = color;
                            }
                        });
                    }
                }
            );

            ui.add_space(40.0);

            let reset_btn = egui::Button::new(
                egui::RichText::new("⟲ RESET TO DEFAULT")
                    .family(egui::FontFamily::Monospace)
                    .size(11.0)
                    .color(egui::Color32::from_gray(140))
            )
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(40)))
            .rounding(15.0)
            .min_size(egui::vec2(160.0, 30.0));

            if ui.add(reset_btn).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                *current_palette = DEFAULT_COLORS;
            }
        });
    });
}