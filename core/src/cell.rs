#[derive(Debug, Copy, Clone, Default)]
pub struct Cell {
    pub kind: CellKind,
    pub state: CellState,
}

impl Cell {
    pub fn new(kind: CellKind) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }
    pub fn new_number(number: u8) -> Self {
        let kind = match number {
            0 => CellKind::Empty,
            n => CellKind::Number(n),
        };

        Self {
            kind,
            ..Default::default()
        }
    }
    pub fn is_mine(&self) -> bool {
        matches!(self.kind, CellKind::Mine)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.kind, CellKind::Empty)
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self.state, CellState::Hidden)
    }

    pub fn is_cleared(&self) -> bool {
        matches!(self.state, CellState::Cleared)
    }

    pub fn is_flagged(&self) -> bool {
        matches!(self.state, CellState::Flagged)
    }

    pub fn new_mine() -> Cell {
        Cell {
            kind: CellKind::Mine,
            ..Default::default()
        }
    }

    pub fn toggle_flagged(&mut self) {
        self.state = match self.state {
            CellState::Hidden => CellState::Flagged,
            CellState::Flagged => CellState::Hidden,
            _ => self.state,
        };
    }

    pub fn clear(&mut self) {
        self.state = CellState::Cleared
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum CellState {
    #[default]
    Hidden,
    Cleared,
    Flagged,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum CellKind {
    #[default]
    Empty,
    Number(u8),
    Mine,
}
