use eframe::egui::{self, CornerRadius, Frame, Layout, Margin, RichText, Ui};
use minesweeper_multiplayer::Difficulty;

use crate::gui::colors::ColorScheme;
use crate::gui::game_panel::widgets::avatar_row::AvatarRow;
use crate::gui::game_panel::widgets::game_card::{ColorRepresentable, LabelRepresentable};
use crate::gui::strings::Strings;

pub struct GameInfoCard<'a> {
    creator_name: &'a str,
    difficulty: &'a Difficulty,
    colors: &'a ColorScheme,
}

impl<'a> GameInfoCard<'a> {
    pub fn new(creator_name: &'a str, difficulty: &'a Difficulty, colors: &'a ColorScheme) -> Self {
        Self {
            creator_name,
            difficulty,
            colors,
        }
    }

    pub fn show(self, ui: &mut Ui) {
        let difficulty_color = self.difficulty.color(self.colors);
        let config = self.difficulty.configuration();

        Frame::NONE
            .fill(self.colors.background_secondary)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.add(
                    AvatarRow::new(
                        self.creator_name,
                        Strings::waiting_for_opponent(),
                        difficulty_color,
                        self.colors,
                    )
                    .circle_size(36.0),
                );

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                detail_row_difficulty(ui, self.difficulty, self.colors);

                detail_row_text(
                    ui,
                    Strings::board_label(),
                    &format!("{} x {}", config.size.width, config.size.height),
                    self.colors,
                );

                detail_row_text(ui, Strings::mines_label(), &config.mines_count.to_string(), self.colors);

                let mines_to_win = config.mines_count / 2 + 1;
                detail_row_text(ui, Strings::first_to_label(), &mines_to_win.to_string(), self.colors);
            });
    }
}

fn detail_row(ui: &mut Ui, label: &str, colors: &ColorScheme, right: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(colors.text_secondary).size(13.0));
        ui.with_layout(Layout::right_to_left(egui::Align::Center), right);
    });
}

fn detail_row_text(ui: &mut Ui, label: &str, value: &str, colors: &ColorScheme) {
    detail_row(ui, label, colors, |ui| {
        ui.label(RichText::new(value).color(colors.text_primary).size(13.0));
    });
}

fn detail_row_difficulty(ui: &mut Ui, difficulty: &Difficulty, colors: &ColorScheme) {
    let difficulty_color = difficulty.color(colors);
    detail_row(ui, Strings::difficulty_label(), colors, |ui| {
        Frame::NONE
            .fill(difficulty_color.linear_multiply(0.2))
            .corner_radius(CornerRadius::same(4))
            .inner_margin(Margin::symmetric(8, 2))
            .show(ui, |ui| {
                ui.label(RichText::new(difficulty.label()).color(difficulty_color).size(12.0));
            });
    });
}
