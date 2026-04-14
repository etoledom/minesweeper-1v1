use core::fmt;

use crate::board::Board;
use crate::graphics::*;

pub struct GameConfiguration {
    pub mines_count: usize,
    pub size: Size,
}

impl GameConfiguration {
    pub fn easy() -> GameConfiguration {
        GameConfiguration {
            mines_count: 21,
            size: Size { height: 10, width: 10 },
        }
    }

    pub fn medium() -> GameConfiguration {
        GameConfiguration {
            mines_count: 101,
            size: Size { height: 16, width: 16 },
        }
    }

    pub fn hard() -> GameConfiguration {
        GameConfiguration {
            mines_count: 250,
            size: Size { height: 24, width: 20 },
        }
    }

    fn configuration_for(difficulty: &Difficulty) -> GameConfiguration {
        match difficulty {
            Difficulty::Easy => GameConfiguration::easy(),
            Difficulty::Medium => GameConfiguration::medium(),
            Difficulty::Hard => GameConfiguration::hard(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Difficulty {
    #[default]
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Difficulty {
    pub fn configuration(&self) -> GameConfiguration {
        GameConfiguration::configuration_for(self)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Game {
    pub board: Board,
    pub total_mines: usize,
    pub difficulty: Difficulty,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Game {
        let config = GameConfiguration::configuration_for(&difficulty);
        Game {
            board: Board::new(config.mines_count, config.size),
            total_mines: config.mines_count,
            difficulty,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn remaining_mines(&self) -> u32 {
        // let mut mines_count: u32 = 0;
        // self.board.iter().for_each(|(_, cell)| {
        //     if cell.is_mine() && !(cell.cleared || cell.flagged) {
        //         mines_count += 1;
        //     }
        // });

        self.board.iter().fold(0, |count, (_, cell)| {
            if cell.is_mine() && cell.is_hidden() {
                return count + 1;
            }
            count
        })

        // mines_count
    }

    pub fn toggle_flagged(&mut self, coordinates: Point) {
        let Some(cell) = self.board.cell_mut_at(coordinates) else {
            return;
        };

        if cell.is_cleared() {
            return;
        }

        cell.toggle_flagged();
    }

    pub fn selected_at(&mut self, coordinates: Point) {
        // Do not select flagged cells
        if self.board.cell_at(coordinates).is_some_and(|cell| cell.is_flagged()) {
            return;
        }
        let mut stack: Vec<Point> = vec![coordinates];
        let mut visited: Vec<Point> = vec![];

        while let Some(point) = stack.pop() {
            if visited.contains(&point) {
                continue;
            }
            visited.push(point);

            let Some(cell) = self.board.cell_mut_at(point) else {
                continue;
            };
            // Do not clear flagged cells
            if cell.is_flagged() {
                continue;
            }
            cell.clear();
            if cell.is_empty() {
                self.board
                    .neighbors(point)
                    .filter(|(point, _)| !visited.contains(point))
                    .for_each(|(point, _)| stack.push(point));
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.board.iter().any(|(_, cell)| cell.is_mine() && cell.is_cleared())
    }

    pub fn is_win(&self) -> bool {
        if self.remaining_mines() > 0 {
            return false;
        }
        !self.board.iter().any(|(_, cell)| cell.is_mine() && cell.is_cleared())
    }

    pub fn clear_all_non_mines(&mut self) {
        self.board.iter_mut().for_each(|(_, cell)| {
            if !cell.is_cleared() && !cell.is_mine() {
                cell.clear();
            }
        });
    }

    pub fn clear_all(&mut self) {
        self.board.iter_mut().for_each(|(_, cell)| {
            if !cell.is_cleared() {
                cell.clear();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::Cell;

    use super::*;

    #[test]
    fn test_is_gameover() {
        let mut game = Game::new(Difficulty::Easy);
        game.board.iter_mut().for_each(|(_, cell)| {
            if cell.is_mine() {
                cell.clear();
            }
        });
        assert!(game.is_game_over());
    }

    #[test]
    fn test_is_win() {
        let mut game = Game::new(Difficulty::Easy);
        game.board.iter_mut().for_each(|(_, cell)| {
            if cell.is_mine() {
                cell.toggle_flagged();
            }
        });
        assert!(game.is_win());
    }

    #[test]
    fn test_clear_white_cells() {
        let mut board = Board::new_empty(Size { width: 5, height: 5 });
        let mine_coordinates = Point { x: 2, y: 2 };
        board.replace_cell(Cell::new_mine(), mine_coordinates);
        board.add_cell_numbers();

        let mut game = Game {
            board,
            total_mines: 1,
            difficulty: Difficulty::Easy,
        };

        game.selected_at(Point { x: 0, y: 4 });

        game.board.iter().for_each(|(_, cell)| {
            if cell.is_mine() {
                assert!(!cell.is_cleared(), "Mine should NOT be cleared");
            } else {
                assert!(cell.is_cleared(), "Non mines should be cleared");
            }
        });
    }

    #[test]
    fn test_difficulty_to_string() {
        let diff = Difficulty::Easy;
        assert_eq!(diff.to_string(), "Easy");
    }
}
