use egui::{RichText, Sense, Ui, Vec2};

use crate::gui::{colors::ColorScheme, strings::Strings};

pub struct TurnIndicator<'a> {
    pub is_my_turn: bool,
    pub opponent_name: &'a str,
    pub colors: &'a ColorScheme,
}

impl<'a> TurnIndicator<'a> {
    pub fn ui(&self, ui: &mut Ui) {
        let (color, text) = if self.is_my_turn {
            (self.colors.player_self, Strings::your_turn().to_string())
        } else {
            (self.colors.player_opponent, Strings::opponents_turn(self.opponent_name))
        };

        ui.vertical_centered(|ui| {
            ui.add_space(12.0);
            ui.label(RichText::new(text.to_uppercase()).size(11.0).color(color));
            ui.add_space(4.0);

            let (rect, _) = ui.allocate_exact_size(Vec2::new(8.0, 8.0), Sense::hover());
            ui.painter().circle_filled(rect.center(), 4.0, color);

            ui.add_space(12.0);
        });
    }
}
