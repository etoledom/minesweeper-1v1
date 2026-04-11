use eframe::egui::{Response, Ui, Widget};
use minesweeper_multiplayer::Player;

use crate::gui::colors::ColorScheme;

use super::player_row::{player_row, PlayerKind};

pub struct PlayersSection<'a> {
    local_player: &'a Player,
    remote_player: &'a Player,
    colors: &'a ColorScheme,
}

impl<'a> PlayersSection<'a> {
    pub fn new(local_player: &'a Player, remote_player: &'a Player, colors: &'a ColorScheme) -> Self {
        Self {
            local_player,
            remote_player,
            colors,
        }
    }
}

impl<'a> Widget for PlayersSection<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            player_row(ui, self.local_player, PlayerKind::Local, self.colors);

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            player_row(ui, self.remote_player, PlayerKind::Remote, self.colors);
        })
        .response
    }
}
