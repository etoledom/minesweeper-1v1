use eframe::egui::{self, Align2, Layout, RichText, Vec2, Window};
use egui::Ui;

use crate::gui::colors::ColorScheme;
use crate::gui::gameplay::OpenGame;
use crate::gui::strings::Strings;

use super::game_info_card::GameInfoCard;

pub enum JoinGameAction {
    None,
    Join(String, String),
    Cancel,
}

#[derive(Default)]
pub struct JoinGameForm {
    name: String,
    game: Option<OpenGame>,
}

impl JoinGameForm {
    pub fn open(&mut self, game: &OpenGame) {
        self.game = Some(game.clone());
        self.name.clear();
    }

    pub fn is_open(&self) -> bool {
        self.game.is_some()
    }

    pub fn reset(&mut self) {
        self.name.clear();
        self.game = None;
    }

    pub fn show(&mut self, ui: &Ui, colors: &ColorScheme) -> JoinGameAction {
        let Some(game) = &self.game else {
            return JoinGameAction::None;
        };

        let mut action = JoinGameAction::None;
        let mut open = true;

        Window::new(Strings::join_game_title())
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size(Vec2::new(340.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.add_space(4.0);

                GameInfoCard::new(&game.name, &game.difficulty, colors).show(ui);

                ui.add_space(16.0);

                // Name input
                ui.label(
                    RichText::new(Strings::join_as_label())
                        .color(colors.text_secondary)
                        .size(13.0),
                );
                ui.add_space(4.0);
                ui.text_edit_singleline(&mut self.name);

                ui.add_space(16.0);

                // Buttons
                let can_join = !self.name.trim().is_empty();

                ui.horizontal(|ui| {
                    if ui.button(Strings::cancel_button()).clicked() {
                        action = JoinGameAction::Cancel;
                    }

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_enabled_ui(can_join, |ui| {
                            if ui.button(RichText::new(Strings::join_button()).size(14.0)).clicked() {
                                action = JoinGameAction::Join(game.game_id.clone(), self.name.clone());
                            }
                        });
                    });
                });
            });

        if !open {
            action = JoinGameAction::Cancel;
        }

        if !matches!(action, JoinGameAction::None) {
            self.reset();
        }

        action
    }
}
