use crate::SerializableCell;
use minesweeper_core::{Board, Size, Vec2};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SerializableBoard {
    pub data: Vec<SerializableCell>,
    pub height: usize,
    pub width: usize,
}

impl SerializableBoard {
    pub fn new_from_json(str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(str)
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl From<Board> for SerializableBoard {
    fn from(board: Board) -> Self {
        let size = board.get_size();
        let data: Vec<SerializableCell> = board.cells.iter().map(|(_, cell)| SerializableCell(*cell)).collect();

        SerializableBoard {
            data,
            height: size.height,
            width: size.width,
        }
    }
}

impl From<SerializableBoard> for Board {
    fn from(s_board: SerializableBoard) -> Self {
        let width = s_board.width;
        let height = s_board.height;
        let data = s_board.data.iter().map(|s_cell| s_cell.0).collect();

        let cells = Vec2::new(data, Size::new(height, width));
        Board::new_with_cells(cells)
    }
}
