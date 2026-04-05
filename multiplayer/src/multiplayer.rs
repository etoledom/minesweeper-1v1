use crate::Player;
use minesweeper_core::*;
use std::cmp::{self, Ordering};

pub struct Multiplayer {
    pub players: Vec<Player>,
    pub game: Game,
}

impl Multiplayer {
    pub fn new(player_names: [&str; 2], difficulty: Difficulty) -> Multiplayer {
        let mut players: Vec<Player> = player_names.iter().map(|name| Player::new(name.to_string())).collect();

        players[0].is_active = true;

        Multiplayer { players, game: Game::new(difficulty) }
    }

    pub fn get_board(&self) -> &Board {
        self.game.get_board()
    }

    pub fn get_difficulty(&self) -> &Difficulty {
        &self.game.difficulty
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        self.players.iter_mut().find(|player| player.is_active).unwrap()
    }

    pub fn current_player(&self) -> &Player {
        self.players.iter().find(|player| player.is_active).unwrap()
    }

    pub fn get_board_dimentions(&self) -> Size {
        self.game.board.get_size()
    }

    pub fn player_selected(&mut self, coordinates: Point) {
        if let Some(selected_cell) = self.game.board.cell_at(coordinates).cloned() {
            self.game.selected_at(coordinates);
            if selected_cell.is_mine() && !selected_cell.is_cleared() {
                self.current_player_mut().mines_found.push(coordinates);
            } else if !selected_cell.is_cleared() {
                self.switch_active_player()
            }
        }
    }

    fn switch_active_player(&mut self) {
        self.players.iter_mut().for_each(|player| player.is_active = !player.is_active);
    }

    fn did_game_finish(&self) -> bool {
        let half_mines = (self.game.total_mines as f32 / 2.).round() as i32;
        let player_1 = self.players[0].mines_found.len() as i32;
        let player_2 = self.players[1].mines_found.len() as i32;

        half_mines == player_1 || half_mines == player_2
    }

    pub fn player_winning(&self) -> Option<&Player> {
        let first = &self.players[0];
        let second = &self.players[1];

        match first.mines_found.len().cmp(&second.mines_found.len()) {
            Ordering::Greater => Some(first),
            Ordering::Less => Some(second),
            Ordering::Equal => None,
        }
    }

    pub fn remaining_to_win(&self) -> i32 {
        let player_1 = self.players[0].mines_found.len();
        let player_2 = self.players[1].mines_found.len();
        let half_mines = (self.game.total_mines as f32 / 2.).round() as i32;
        let max = cmp::max(player_1, player_2) as i32;

        half_mines - max
    }

    #[allow(clippy::needless_return)]
    pub fn winner(&self) -> Option<&Player> {
        if !self.did_game_finish() {
            return None;
        }
        if self.players[0].score() > self.players[1].score() {
            return Some(&self.players[0]);
        } else {
            return Some(&self.players[1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_board_size() {
        let mult = Multiplayer::new(["1", "2"], Difficulty::Easy);

        assert_eq!(mult.get_board_dimentions(), Size { width: 10, height: 10 });
    }

    #[test]
    fn test_switch_player_after_selecting_non_mine() {
        let mut mult = Multiplayer::new(["1", "2"], Difficulty::Easy);
        assert_eq!(mult.current_player().name, "1");

        let mine = coordinates_for_non_mine(&mult.game.board);
        mult.player_selected(mine);

        assert_eq!(mult.current_player().name, "2");
    }

    #[test]
    fn test_does_not_switch_player_after_selecting_mine() {
        let mut mult = Multiplayer::new(["1", "2"], Difficulty::Easy);
        assert_eq!(mult.current_player().name, "1");

        let mine = coordinates_for_mine(&mult.game.board);
        mult.player_selected(mine);

        assert_eq!(mult.current_player().name, "1");
    }

    #[test]
    fn test_remaining_to_win() {
        let mut mult = Multiplayer::new(["1", "2"], Difficulty::Easy);
        assert_eq!(mult.current_player().name, "1");

        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));

        let to_win = mult.remaining_to_win();

        assert_eq!(to_win, 3);
    }

    #[test]
    fn test_is_win() {
        let mut mult = Multiplayer::new(["1", "2"], Difficulty::Easy);
        assert_eq!(mult.current_player().name, "1");

        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));

        assert!(!mult.did_game_finish());

        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));

        assert!(mult.did_game_finish());
    }

    #[test]
    fn test_is_win_second_player() {
        let mut mult = Multiplayer::new(["1", "2"], Difficulty::Easy);
        assert_eq!(mult.current_player().name, "1");

        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_non_mine(&mult.game.board));

        assert!(!mult.did_game_finish());

        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));

        assert!(!mult.did_game_finish());

        mult.player_selected(coordinates_for_mine(&mult.game.board));

        assert!(mult.did_game_finish());
    }

    #[test]
    fn test_player_winning() {
        let mut mult = Multiplayer::new(["1", "2"], Difficulty::Easy);
        assert!(mult.player_winning().is_none());

        mult.player_selected(coordinates_for_mine(&mult.game.board));

        assert_eq!(mult.player_winning().unwrap().name, "1");

        mult.player_selected(coordinates_for_non_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));

        assert_eq!(mult.player_winning().unwrap().name, "2");
    }

    fn coordinates_for_non_mine(board: &Board) -> Point {
        let mut coordinates_to_select = Point::zero();
        board.iter().any(|(coordinates, cell)| {
            if !cell.is_mine() {
                coordinates_to_select = coordinates;
            }
            return !cell.is_mine();
        });
        coordinates_to_select
    }

    fn coordinates_for_mine(board: &Board) -> Point {
        let mut coordinates_to_select = Point::zero();
        board.iter().any(|(coordinates, cell)| {
            if cell.is_mine() && !cell.is_cleared() {
                coordinates_to_select = coordinates;
                return true;
            }
            false
        });
        coordinates_to_select
    }
}
