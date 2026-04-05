mod board;
mod cell;
mod game;
mod graphics;

pub use board::Board;
pub use cell::*;
pub use game::{Difficulty, Game};
pub use graphics::{Point, Size, Vec2};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawing_hard() {
        let mut game = Game::new(Difficulty::Hard);
        game.clear_all();
        println!("{}", game.board);
    }

    #[test]
    fn game() {
        let mut game = Game::new(Difficulty::Easy);
        assert!(!game.is_game_over());

        for x in 0..game.board.get_width() {
            if game.is_game_over() {
                break;
            }
            for y in 0..game.board.get_height() {
                if game.board.cell_at(Point { x, y }).unwrap().is_mine() {
                    game.selected_at(Point { x, y });
                    break;
                }
            }
        }
        let game_over = game.is_game_over();
        assert!(game_over);
    }
}
