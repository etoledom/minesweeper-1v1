use eframe::egui::{RichText, Ui};

use crate::gui::colors::ColorScheme;
use crate::gui::game_panel::widgets::game_card::{GameCard, GameCardAction};
use crate::gui::gameplay::OpenGame;
use crate::gui::strings::Strings;

pub enum GameListAction<'a> {
    None,
    Join(&'a OpenGame),
}

pub struct GameList<'a> {
    games: &'a [OpenGame],
    colors: &'a ColorScheme,
}

impl<'a> GameList<'a> {
    pub fn new(games: &'a [OpenGame], colors: &'a ColorScheme) -> Self {
        Self { games, colors }
    }

    pub fn show(self, ui: &'a mut Ui) -> GameListAction<'a> {
        let mut action = GameListAction::None;

        ui.vertical(|ui| {
            if self.games.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(24.0);
                    ui.label(
                        RichText::new(Strings::no_games_available())
                            .color(self.colors.text_muted)
                            .size(14.0),
                    );
                    ui.add_space(24.0);
                });
            } else {
                for game in self.games {
                    let card_action = GameCard::new(&game.name, game.difficulty, self.colors).show(ui);

                    if let GameCardAction::Join = card_action {
                        action = GameListAction::Join(game)
                    }

                    ui.add_space(8.0);
                }
            }
        });

        action
    }
}
