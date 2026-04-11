mod serializable_board;
mod serializable_point;
use minesweeper_core::{Cell, CellKind, CellState, Difficulty, Game};

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};
pub use serializable_board::*;
pub use serializable_point::*;

#[derive(Debug, Clone)]
pub struct SerializableCell(pub Cell);

impl Serialize for SerializableCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let cell = &self.0;
        let mut s = serializer.serialize_struct("Cell", 2)?;
        s.serialize_field("kind", &SerializableCellKind::new(cell.kind))?;
        s.serialize_field("state", &SerializableCellState::new(cell.state))?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for SerializableCell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CellFields {
            kind: SerializableCellKind,
            state: SerializableCellState,
        }

        let fields = CellFields::deserialize(deserializer)?;
        Ok(SerializableCell(Cell {
            kind: fields.kind.into_inner(),
            state: fields.state.into_inner(),
        }))
    }
}

#[derive(Serialize, Deserialize)]
enum SerializableCellState {
    Hidden,
    Cleared,
    Flagged,
}

impl SerializableCellState {
    fn new(state: CellState) -> Self {
        match state {
            CellState::Hidden => Self::Hidden,
            CellState::Cleared => Self::Cleared,
            CellState::Flagged => Self::Flagged,
        }
    }
    fn into_inner(&self) -> CellState {
        match self {
            Self::Cleared => CellState::Cleared,
            Self::Flagged => CellState::Flagged,
            Self::Hidden => CellState::Hidden,
        }
    }
}

#[derive(Serialize, Deserialize)]
enum SerializableCellKind {
    Empty,
    Number(u8),
    Mine,
}

impl SerializableCellKind {
    fn new(kind: CellKind) -> Self {
        match kind {
            CellKind::Empty => Self::Empty,
            CellKind::Number(n) => Self::Number(n),
            CellKind::Mine => Self::Mine,
        }
    }
    fn into_inner(&self) -> CellKind {
        match self {
            Self::Empty => CellKind::Empty,
            Self::Number(n) => CellKind::Number(*n),
            Self::Mine => CellKind::Mine,
        }
    }
}

impl From<Cell> for SerializableCell {
    fn from(cell: Cell) -> Self {
        SerializableCell(cell)
    }
}

impl From<SerializableCell> for Cell {
    fn from(cell: SerializableCell) -> Self {
        cell.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum SerializableDifficulty {
    Easy,
    Medium,
    Hard,
}

impl From<Difficulty> for SerializableDifficulty {
    fn from(value: Difficulty) -> Self {
        match value {
            Difficulty::Easy => SerializableDifficulty::Easy,
            Difficulty::Medium => SerializableDifficulty::Medium,
            Difficulty::Hard => SerializableDifficulty::Hard,
        }
    }
}

impl From<SerializableDifficulty> for Difficulty {
    fn from(value: SerializableDifficulty) -> Self {
        match value {
            SerializableDifficulty::Easy => Difficulty::Easy,
            SerializableDifficulty::Medium => Difficulty::Medium,
            SerializableDifficulty::Hard => Difficulty::Hard,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GameDefinition {
    pub name: String,
    pub id: String,
    pub difficulty: SerializableDifficulty,
}

impl GameDefinition {
    pub fn new(id: impl Into<String>, name: impl Into<String>, difficulty: Difficulty) -> Self {
        GameDefinition {
            name: name.into(),
            id: id.into(),
            difficulty: difficulty.into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SerializableGame {
    board: SerializableBoard,
    total_mines: usize,
    difficulty: SerializableDifficulty,
}

impl From<Game> for SerializableGame {
    fn from(value: Game) -> Self {
        SerializableGame {
            board: value.board.into(),
            total_mines: value.total_mines,
            difficulty: value.difficulty.into(),
        }
    }
}

impl From<SerializableGame> for Game {
    fn from(value: SerializableGame) -> Self {
        Game {
            board: value.board.into(),
            total_mines: value.total_mines,
            difficulty: value.difficulty.into(),
        }
    }
}
