use eframe::egui::{self, Color32, CornerRadius, Layout, Rect, RichText, Ui, Vec2};

use crate::gui::colors::ColorScheme;

pub struct ProgressRow<'a> {
    name: &'a str,
    current: usize,
    target: usize,
    color: Color32,
    colors: &'a ColorScheme,
}

impl<'a> ProgressRow<'a> {
    pub fn new(name: &'a str, current: usize, target: usize, color: Color32, colors: &'a ColorScheme) -> Self {
        Self {
            name,
            current,
            target,
            color,
            colors,
        }
    }
}

impl<'a> egui::Widget for ProgressRow<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            // Name and fraction label
            ui.horizontal(|ui| {
                ui.label(RichText::new(self.name).color(self.color).size(14.0));
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{}/{}", self.current, self.target))
                            .color(self.colors.text_primary)
                            .size(14.0),
                    );
                });
            });

            // Progress bar
            let bar_height = 8.0;
            let available_width = ui.available_width();
            let (rect, response) = ui.allocate_exact_size(Vec2::new(available_width, bar_height), egui::Sense::hover());

            let corner_radius = CornerRadius::same(bar_height as u8 / 2);

            // Background track
            ui.painter()
                .rect_filled(rect, corner_radius, self.colors.background_cell_hidden);

            // Filled portion
            if self.target > 0 {
                let fraction = (self.current as f32 / self.target as f32).min(1.0);
                let filled_width = rect.width() * fraction;

                if filled_width > 0.0 {
                    let filled_rect = Rect::from_min_size(rect.min, Vec2::new(filled_width, bar_height));
                    ui.painter().rect_filled(filled_rect, corner_radius, self.color);
                }
            }

            response
        })
        .inner
    }
}
