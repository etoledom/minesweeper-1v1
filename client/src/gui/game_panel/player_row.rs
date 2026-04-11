use eframe::egui::{self, Color32, Layout, RichText, Ui};
use minesweeper_multiplayer::Player;

use crate::gui::colors::ColorScheme;
use crate::gui::game_panel::widgets::avatar_row::AvatarRow;
use crate::gui::strings::Strings;

pub enum PlayerKind {
    Local,
    Remote,
}

impl PlayerKind {
    fn color(&self, colors: &ColorScheme) -> Color32 {
        match self {
            PlayerKind::Local => colors.player_self,
            PlayerKind::Remote => colors.player_opponent,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            PlayerKind::Local => Strings::player_local_label(),
            PlayerKind::Remote => Strings::player_remote_label(),
        }
    }
}

pub fn player_row(ui: &mut Ui, player: &Player, kind: PlayerKind, colors: &ColorScheme) {
    let color = kind.color(colors);
    let mines = player.mines_found.len();

    ui.horizontal(|ui| {
        ui.add(AvatarRow::new(&player.name, &kind.label(), color, colors));

        // Mine count (right-aligned)
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            ui.vertical(|ui| {
                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label(RichText::new(mines.to_string()).color(color).size(28.0));
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label(
                        RichText::new(Strings::mines_label())
                            .color(colors.text_secondary)
                            .size(12.0),
                    );
                });
            });
        });
    });
}
