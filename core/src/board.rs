use std::fmt::Display;

use crate::{
    graphics::{Vec2Iter, Vec2IterMut},
    Cell, Point, Size, Vec2,
};

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec2<Cell>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_row = 0;
        for (coordinates, cell) in self.iter() {
            if last_row != coordinates.x {
                last_row = coordinates.x;
                writeln!(f)?;
            }
            let symbol = match cell.state {
                crate::cell::CellState::Hidden => "[ \u{25AE}]".to_string(),
                crate::cell::CellState::Cleared => match cell.kind {
                    crate::cell::CellKind::Empty => "[  ]".to_string(),
                    crate::cell::CellKind::Number(number) => format!("[ {}]", number),
                    crate::cell::CellKind::Mine => "[ *]".to_string(),
                },
                crate::cell::CellState::Flagged => "[ \u{1F6A9}]".to_string(),
            };

            write!(f, "{}", symbol)?;
        }
        writeln!(f)
    }
}

impl Board {
    pub fn new_empty(size: Size) -> Self {
        let cells = vec![Cell { ..Default::default() }; size.width * size.height];
        Board::new_with_cells(Vec2::new(cells, size))
    }

    pub fn new(mines: usize, size: Size) -> Self {
        let mut board = Board::new_empty(size);
        board.add_mines(mines).add_cell_numbers();
        board
    }

    pub fn new_with_cells(cells: Vec2<Cell>) -> Board {
        Board { cells }
    }

    // Populate cells:

    fn add_mines(&mut self, mines: usize) -> &mut Self {
        if mines == 0 {
            return self;
        }
        let coordinates = Point::random_between(0..self.get_width(), 0..self.get_height());

        let Some(cell) = self.cell_at(coordinates) else {
            return self.add_mines(mines);
        };

        if cell.is_mine() {
            return self.add_mines(mines);
        }

        let mine = Cell::new_mine();
        self.replace_cell(mine, coordinates);

        self.add_mines(mines - 1)
    }

    pub fn add_cell_numbers(&mut self) -> &mut Self {
        let points: Vec<Point> = self
            .cells
            .iter()
            .filter(|(_, cell)| !cell.is_mine())
            .map(|(point, _)| point)
            .collect();
        for point in points {
            let mines_count = self.count_mines_around_cell_at(point);
            self.replace_cell(Cell::new_number(mines_count), point);
        }
        self
    }

    fn count_mines_around_cell_at(&mut self, coordinates: Point) -> u8 {
        self.neighbors(coordinates).fold(0u8, |count, (_, cell)| {
            if cell.is_mine() {
                return count + 1;
            }
            count
        })
    }

    pub fn cell_at(&self, coordinates: Point) -> Option<&Cell> {
        self.cells.get_element(coordinates)
    }

    pub fn cell_mut_at(&mut self, coordinates: Point) -> Option<&mut Cell> {
        self.cells.get_element_mut(coordinates)
    }

    pub fn get_width(&self) -> usize {
        self.cells.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.cells.get_height()
    }

    pub fn get_size(&self) -> Size {
        Size {
            width: self.get_width(),
            height: self.get_height(),
        }
    }

    pub fn replace_cell(&mut self, new_cell: Cell, coordinates: Point) {
        self.cells.replace_at(new_cell, coordinates);
    }

    pub fn iter<'a>(&'a self) -> Vec2Iter<'a, Cell> {
        self.cells.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> Vec2IterMut<'a, Cell> {
        self.cells.iter_mut()
    }

    pub fn neighbors(&self, coordinates: Point) -> impl Iterator<Item = (Point, &Cell)> {
        self.cells.neighbors(coordinates)
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::CellKind;

    use super::*;

    #[test]
    fn test_add_mines() {
        let mut board = Board::new_empty(Size { width: 2, height: 2 });
        board.add_mines(4).add_cell_numbers();
        let mine = board.cell_at(Point::zero());

        assert!(mine.unwrap().is_mine())
    }

    #[test]
    fn test_top_left_cell() {
        let board = get_board_with_number_top_left();
        let Some(cell) = board.cell_at(Point::zero()) else {
            panic!()
        };
        assert!(matches!(cell.kind, CellKind::Number(_)));
    }

    #[test]
    fn test_get_cells_around_top_left_cell() {
        let board = Board::new(1, Size { width: 3, height: 3 });
        let cells = board.neighbors(Point::zero());
        assert_eq!(cells.count(), 3);
    }

    #[test]
    fn test_get_cells_around_central_cell() {
        let board = Board::new(1, Size { width: 3, height: 3 });
        let cells = board.neighbors(Point { x: 1, y: 1 });
        assert_eq!(cells.count(), 8);
    }

    #[test]
    fn test_get_cells_around_bottom_right_cell() {
        let board = Board::new(1, Size { width: 3, height: 3 });
        let cells = board.neighbors(Point { x: 2, y: 2 });
        assert_eq!(cells.count(), 3);
    }

    /// Get a board where the top-left cell must be a number.
    fn get_board_with_number_top_left() -> Board {
        let board = Board::new(1, Size { width: 2, height: 2 });
        if let Some(cell) = board.cell_at(Point::zero()) {
            if cell.is_mine() {
                return get_board_with_number_top_left();
            }
        }
        board
    }
}
