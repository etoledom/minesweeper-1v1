use eframe::egui::{self, Color32, CornerRadius, Frame, Layout, Margin, RichText, Ui};
use minesweeper_multiplayer::Difficulty;

use crate::gui::colors::ColorScheme;
use crate::gui::game_panel::widgets::avatar_row::AvatarRow;
use crate::gui::strings::Strings;

pub enum GameCardAction {
    None,
    Join,
}

pub struct GameCard<'a> {
    creator_name: &'a str,
    difficulty: Difficulty,
    colors: &'a ColorScheme,
}

pub trait LabelRepresentable {
    fn label(&self) -> String;
}

impl LabelRepresentable for Difficulty {
    fn label(&self) -> String {
        match self {
            Difficulty::Easy => Strings::difficulty_easy().into(),
            Difficulty::Medium => Strings::difficulty_medium().into(),
            Difficulty::Hard => Strings::difficulty_hard().into(),
        }
    }
}

pub trait ColorRepresentable {
    fn color(&self, colros: &ColorScheme) -> Color32;
}

impl ColorRepresentable for Difficulty {
    fn color(&self, colors: &ColorScheme) -> Color32 {
        match self {
            Difficulty::Easy => colors.difficulty_easy,
            Difficulty::Medium => colors.difficulty_medium,
            Difficulty::Hard => colors.difficulty_hard,
        }
    }
}

impl<'a> GameCard<'a> {
    pub fn new(creator_name: &'a str, difficulty: Difficulty, colors: &'a ColorScheme) -> Self {
        Self {
            creator_name,
            difficulty,
            colors,
        }
    }
}

impl<'a> GameCard<'a> {
    pub fn show(self, ui: &mut Ui) -> GameCardAction {
        let difficulty_color = self.difficulty.color(self.colors);
        let difficulty_label = self.difficulty.label();
        let mut action = GameCardAction::None;

        Frame::NONE
            .fill(self.colors.background_secondary)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let name = format!("{}'s game", self.creator_name);
                    ui.add(AvatarRow::new(
                        &name,
                        Strings::waiting_for_opponent(),
                        difficulty_color,
                        self.colors,
                    ));

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(Strings::join_button()).clicked() {
                            action = GameCardAction::Join;
                        }

                        // Difficulty badge
                        Frame::NONE
                            .fill(difficulty_color.linear_multiply(0.2))
                            .corner_radius(CornerRadius::same(4))
                            .inner_margin(Margin::symmetric(8, 3))
                            .show(ui, |ui| {
                                ui.label(RichText::new(difficulty_label).color(difficulty_color).size(12.0));
                            });
                    });
                });
            });

        action
    }
}
