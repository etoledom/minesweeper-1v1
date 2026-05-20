use eframe::egui::{self, RichText, Ui};
use minesweeper_multiplayer::Difficulty;

use crate::gui::colors::ColorScheme;
use crate::gui::game_panel::lobby::game_info_card::GameInfoCard;
use crate::gui::strings::Strings;

pub struct WaitingScreen<'a> {
    game_info: Option<(&'a str, &'a Difficulty)>,
    colors: &'a ColorScheme,
}

impl<'a> WaitingScreen<'a> {
    pub fn new(game_info: Option<(&'a str, &'a Difficulty)>, colors: &'a ColorScheme) -> Self {
        Self { game_info, colors }
    }

    pub fn show(self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading(RichText::new(Strings::waiting_for_opponent()).color(self.colors.text_primary));
                ui.add_space(24.0);
                if let Some((creator_name, difficulty)) = self.game_info {
                    egui::Frame::NONE.show(ui, |ui| {
                        ui.set_max_width(320.0);
                        GameInfoCard::new(creator_name, difficulty, self.colors).show(ui);
                    });
                }
                ui.add_space(16.0);
                ui.add(egui::Spinner::new().size(20.0));
            });
        });
    }
}
