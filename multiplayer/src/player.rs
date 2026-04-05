use minesweeper_core::Point;
use uuid::Uuid;

#[derive(Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub mines_found: Vec<Point>,
}

impl Player {
    pub fn new(name: impl Into<String>) -> Player {
        Player {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            is_active: false,
            mines_found: vec![],
        }
    }

    pub fn score(&self) -> i32 {
        self.mines_found.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_score() {
        let mut player = Player::new("name");
        assert_eq!(player.score(), 0);
        player.mines_found.push(Point::zero());
        assert_eq!(player.score(), 1);
        player.mines_found.push(Point::zero());
        assert_eq!(player.score(), 2);
    }
}
