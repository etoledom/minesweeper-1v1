use eframe::egui::{self, Align2, Color32, CornerRadius, Frame, Margin, RichText, Ui, Vec2, Window};
use minesweeper_multiplayer::Difficulty;

use crate::gui::colors::ColorScheme;
use crate::gui::game_panel::widgets::game_card::LabelRepresentable;
use crate::gui::strings::Strings;

pub enum NewGameFormAction {
    None,
    Create { name: String, difficulty: Difficulty },
    Cancel,
}

#[derive(Default)]
pub struct NewGameForm {
    name: String,
    selected_difficulty: Difficulty,
}

impl NewGameForm {
    pub fn show(&mut self, ui: &mut Ui, colors: &ColorScheme) -> NewGameFormAction {
        let mut action = NewGameFormAction::None;
        let mut open = true;

        Window::new(Strings::new_game_title())
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size(Vec2::new(340.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.add_space(8.0);

                // Name input
                ui.label(
                    RichText::new(Strings::your_name())
                        .color(colors.text_secondary)
                        .size(13.0),
                );
                ui.add_space(4.0);
                ui.text_edit_singleline(&mut self.name);

                ui.add_space(16.0);

                // Difficulty selector
                ui.label(
                    RichText::new(Strings::difficulty_label())
                        .color(colors.text_secondary)
                        .size(13.0),
                );
                ui.add_space(6.0);

                ui.columns(3, |ui| {
                    difficulty_option(
                        &mut ui[0],
                        Difficulty::Easy,
                        &mut self.selected_difficulty,
                        colors.difficulty_easy,
                        "10×10 · 11 mines",
                        colors,
                    );
                    difficulty_option(
                        &mut ui[1],
                        Difficulty::Medium,
                        &mut self.selected_difficulty,
                        colors.difficulty_medium,
                        "16×16 · 41 mines",
                        colors,
                    );
                    difficulty_option(
                        &mut ui[2],
                        Difficulty::Hard,
                        &mut self.selected_difficulty,
                        colors.difficulty_hard,
                        "30×16 · 99 mines",
                        colors,
                    );
                });

                ui.add_space(16.0);

                // Create button
                let can_create = !self.name.trim().is_empty();
                ui.add_enabled_ui(can_create, |ui| {
                    if ui
                        .button(RichText::new(Strings::create_game_button()).size(14.0))
                        .clicked()
                    {
                        action = NewGameFormAction::Create {
                            name: self.name.trim().to_string(),
                            difficulty: self.selected_difficulty,
                        };
                    }
                });
            });

        if !open {
            action = NewGameFormAction::Cancel;
        }

        // Clear state on close
        if !matches!(action, NewGameFormAction::None) {
            self.name = Default::default();
            self.selected_difficulty = Default::default();
        }

        action
    }
}

fn difficulty_option(
    ui: &mut Ui,
    difficulty: Difficulty,
    selected: &mut Difficulty,
    color: Color32,
    description: &str,
    colors: &ColorScheme,
) {
    let is_selected = *selected == difficulty;

    let stroke = if is_selected {
        egui::Stroke::new(2.0, color)
    } else {
        egui::Stroke::new(0.5, colors.separator)
    };

    let response = Frame::NONE
        .fill(colors.background_secondary)
        .corner_radius(CornerRadius::same(6))
        .stroke(stroke)
        .inner_margin(Margin::symmetric(8, 8))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(difficulty.label()).color(colors.text_primary).size(14.0));
                ui.label(RichText::new(description).color(colors.text_muted).size(11.0));
            });
        })
        .response;

    let response = ui.interact(response.rect, response.id, egui::Sense::click());

    if response.clicked() {
        *selected = difficulty;
    }
}
