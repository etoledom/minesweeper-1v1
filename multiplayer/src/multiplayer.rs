use crate::Player;
use minesweeper_core::*;
use std::cmp::{self, Ordering};

#[derive(Debug)]
pub struct Multiplayer {
    pub local_player: Player,
    pub remote_player: Player,
    pub game: Game,
}

impl Multiplayer {
    pub fn new(local_player: &str, remote_player: &str, difficulty: Difficulty) -> Multiplayer {
        let mut local_player = Player::new(local_player);
        let remote_player = Player::new(remote_player);

        local_player.is_active = true;

        Multiplayer {
            local_player,
            remote_player,
            game: Game::new(difficulty),
        }
    }

    pub fn new_with_game(inner_game: Game, local_player: &str, remote_player: &str) -> Self {
        let mut local_player = Player::new(local_player);
        let remote_player = Player::new(remote_player);

        local_player.is_active = true;

        Multiplayer {
            local_player,
            remote_player,
            game: inner_game,
        }
    }

    pub fn get_board(&self) -> &Board {
        self.game.get_board()
    }

    pub fn get_difficulty(&self) -> &Difficulty {
        &self.game.difficulty
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        if self.local_player.is_active {
            &mut self.local_player
        } else {
            &mut self.remote_player
        }
    }

    pub fn current_player(&self) -> &Player {
        if self.local_player.is_active {
            &self.local_player
        } else {
            &self.remote_player
        }
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
        self.local_player.is_active = !self.local_player.is_active;
        self.remote_player.is_active = !self.remote_player.is_active;
    }

    fn did_game_finish(&self) -> bool {
        let half_mines = (self.game.total_mines as f32 / 2.).round() as i32;
        let local_player = self.local_player.mines_found.len() as i32;
        let remote_player = self.remote_player.mines_found.len() as i32;

        half_mines == local_player || half_mines == remote_player
    }

    pub fn player_winning(&self) -> Option<&Player> {
        match self.local_player.mines_found.len().cmp(&self.remote_player.mines_found.len()) {
            Ordering::Greater => Some(&self.local_player),
            Ordering::Less => Some(&self.remote_player),
            Ordering::Equal => None,
        }
    }

    pub fn remaining_to_win(&self) -> usize {
        let local_player = self.local_player.mines_found.len();
        let remote_player = self.remote_player.mines_found.len();
        let half_mines = (self.game.total_mines as f32 / 2.).round() as usize;
        let max = cmp::max(local_player, remote_player);

        half_mines - max
    }

    pub fn local_to_win(&self) -> usize {
        let local_player = self.local_player.mines_found.len();
        let half_mines = (self.game.total_mines as f32 / 2.).round() as usize;

        half_mines - local_player
    }

    pub fn total_mines_to_win(&self) -> usize {
        1 + self.game.total_mines / 2
    }

    #[allow(clippy::needless_return)]
    pub fn winner(&self) -> Option<&Player> {
        if !self.did_game_finish() {
            return None;
        }
        if self.local_player.score() > self.remote_player.score() {
            return Some(&self.local_player);
        } else {
            return Some(&self.remote_player);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_board_size() {
        let mult = Multiplayer::new("1", "2", Difficulty::Easy);

        assert_eq!(mult.get_board_dimentions(), Size { width: 10, height: 10 });
    }

    #[test]
    fn test_total_mines_to_win() {
        let mult = Multiplayer::new("1", "2", Difficulty::Easy);
        assert_eq!(mult.total_mines_to_win(), 6);

        let mult = Multiplayer::new("1", "2", Difficulty::Medium);
        assert_eq!(mult.total_mines_to_win(), 21);

        let mult = Multiplayer::new("1", "2", Difficulty::Hard);
        assert_eq!(mult.total_mines_to_win(), 50);
    }

    #[test]
    fn test_switch_player_after_selecting_non_mine() {
        let mut mult = Multiplayer::new("1", "2", Difficulty::Easy);
        assert_eq!(mult.current_player().name, "1");

        let mine = coordinates_for_non_mine(&mult.game.board);
        mult.player_selected(mine);

        assert_eq!(mult.current_player().name, "2");
    }

    #[test]
    fn test_does_not_switch_player_after_selecting_mine() {
        let mut mult = Multiplayer::new("1", "2", Difficulty::Easy);
        assert_eq!(mult.current_player().name, "1");

        let mine = coordinates_for_mine(&mult.game.board);
        mult.player_selected(mine);

        assert_eq!(mult.current_player().name, "1");
    }

    #[test]
    fn test_remaining_to_win() {
        let mut mult = Multiplayer::new("1", "2", Difficulty::Easy);
        assert_eq!(mult.current_player().name, "1");

        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));
        mult.player_selected(coordinates_for_mine(&mult.game.board));

        let to_win = mult.remaining_to_win();

        assert_eq!(to_win, 3);
    }

    #[test]
    fn test_is_win() {
        let mut mult = Multiplayer::new("1", "2", Difficulty::Easy);
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
        let mut mult = Multiplayer::new("1", "2", Difficulty::Easy);
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
        let mut mult = Multiplayer::new("1", "2", Difficulty::Easy);
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
