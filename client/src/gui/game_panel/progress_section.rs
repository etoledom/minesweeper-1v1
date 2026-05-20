use eframe::egui::{Response, RichText, Ui, Widget};
use minesweeper_multiplayer::Player;

use crate::gui::colors::ColorScheme;
use crate::gui::strings::Strings;

use super::progress_row::ProgressRow;

pub struct ProgressSection<'a> {
    local_player: &'a Player,
    remote_player: &'a Player,
    mines_to_win: usize,
    colors: &'a ColorScheme,
}

impl<'a> ProgressSection<'a> {
    pub fn new(
        local_player: &'a Player,
        remote_player: &'a Player,
        mines_to_win: usize,
        colors: &'a ColorScheme,
    ) -> Self {
        Self {
            local_player,
            remote_player,
            mines_to_win,
            colors,
        }
    }
}

impl<'a> Widget for ProgressSection<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.label(RichText::new(Strings::progress()).color(self.colors.text_primary).size(14.0));

            ui.add_space(8.0);

            ui.add(ProgressRow::new(
                &self.local_player.name,
                self.local_player.mines_found.len(),
                self.mines_to_win,
                self.colors.player_self,
                self.colors,
            ));

            ui.add_space(8.0);

            ui.add(ProgressRow::new(
                &self.remote_player.name,
                self.remote_player.mines_found.len(),
                self.mines_to_win,
                self.colors.player_opponent,
                self.colors,
            ));
        })
        .response
    }
}
