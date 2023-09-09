use crate::{COLS, ROWS};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Location {
    pub col: u16,
    pub row: u16,
}

impl Location {
    pub fn new(c: u16, r: u16) -> Location {
        Location { col: c, row: r }
    }
    pub fn next_location(self, d: Direction) -> Location {
        match d {
            Direction::Up => {
                if self.row > 0 {
                    Location::new(self.col, self.row - 1)
                } else {
                    self
                }
            }
            Direction::Down => {
                if self.row < COLS - 1 {
                    Location::new(self.col, self.row + 1)
                } else {
                    self
                }
            }
            Direction::Left => {
                if self.col > 0 {
                    Location::new(self.col - 1, self.row)
                } else {
                    self
                }
            }
            Direction::Right => {
                if self.col < ROWS - 1 {
                    Location::new(self.col + 1, self.row)
                } else {
                    self
                }
            }
        }
    }
    pub fn is_valid_move(self, d: Direction) -> bool {
        match d {
            Direction::Up => self.row > 0,
            Direction::Down => self.row < ROWS - 1,
            Direction::Left => self.col > 0,
            Direction::Right => self.col < COLS - 1,
        }
    }
    pub fn distance(self, other: Location) -> u16 {
        let col_diff = if self.col > other.col {
            self.col - other.col
        } else {
            other.col - self.col
        };
        let row_diff = if self.row > other.row {
            self.row - other.row
        } else {
            other.row - self.row
        };
        col_diff + row_diff
    }
}
