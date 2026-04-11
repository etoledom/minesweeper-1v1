use egui::{Color32, RichText, Widget};

use crate::gui::{colors::ColorScheme, game_panel::widgets::avatar_circle::AvatarCircle};

pub struct AvatarRow<'a> {
    label: &'a str,
    sublabel: &'a str,
    circle_color: Color32,
    colors: &'a ColorScheme,
    circle_size: f32,
}

impl<'a> AvatarRow<'a> {
    pub fn new(label: &'a str, sublabel: &'a str, circle_color: Color32, colors: &'a ColorScheme) -> Self {
        Self {
            label,
            sublabel,
            circle_color,
            colors,
            circle_size: 40.,
        }
    }

    pub fn circle_size(self, circle_size: f32) -> Self {
        Self { circle_size, ..self }
    }
}

impl<'a> Widget for AvatarRow<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            let initial = self.label.chars().next().unwrap_or('?');
            ui.add(AvatarCircle::new(initial, self.circle_color, self.circle_size));

            ui.vertical(|ui| {
                ui.label(RichText::new(self.label).color(self.colors.text_primary).size(16.));
                ui.label(RichText::new(self.sublabel).color(self.colors.text_secondary).size(12.));
            });
        })
        .response
    }
}
