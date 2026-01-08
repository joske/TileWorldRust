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
                if self.row < ROWS - 1 {
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
                if self.col < COLS - 1 {
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
        let col_diff = self.col.abs_diff(other.col);
        let row_diff = self.row.abs_diff(other.row);
        col_diff + row_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_new() {
        let loc = Location::new(5, 10);
        assert_eq!(loc.col, 5);
        assert_eq!(loc.row, 10);
    }

    #[test]
    fn test_distance_same_location() {
        let loc = Location::new(5, 5);
        assert_eq!(loc.distance(loc), 0);
    }

    #[test]
    fn test_distance_horizontal() {
        let a = Location::new(0, 0);
        let b = Location::new(5, 0);
        assert_eq!(a.distance(b), 5);
        assert_eq!(b.distance(a), 5);
    }

    #[test]
    fn test_distance_vertical() {
        let a = Location::new(0, 0);
        let b = Location::new(0, 7);
        assert_eq!(a.distance(b), 7);
    }

    #[test]
    fn test_distance_diagonal() {
        let a = Location::new(0, 0);
        let b = Location::new(3, 4);
        assert_eq!(a.distance(b), 7); // Manhattan distance
    }

    #[test]
    fn test_next_location_up() {
        let loc = Location::new(5, 5);
        let next = loc.next_location(Direction::Up);
        assert_eq!(next, Location::new(5, 4));
    }

    #[test]
    fn test_next_location_down() {
        let loc = Location::new(5, 5);
        let next = loc.next_location(Direction::Down);
        assert_eq!(next, Location::new(5, 6));
    }

    #[test]
    fn test_next_location_left() {
        let loc = Location::new(5, 5);
        let next = loc.next_location(Direction::Left);
        assert_eq!(next, Location::new(4, 5));
    }

    #[test]
    fn test_next_location_right() {
        let loc = Location::new(5, 5);
        let next = loc.next_location(Direction::Right);
        assert_eq!(next, Location::new(6, 5));
    }

    #[test]
    fn test_next_location_boundary_top() {
        let loc = Location::new(5, 0);
        let next = loc.next_location(Direction::Up);
        assert_eq!(next, loc); // Should stay in place
    }

    #[test]
    fn test_next_location_boundary_bottom() {
        let loc = Location::new(5, ROWS - 1);
        let next = loc.next_location(Direction::Down);
        assert_eq!(next, loc); // Should stay in place
    }

    #[test]
    fn test_next_location_boundary_left() {
        let loc = Location::new(0, 5);
        let next = loc.next_location(Direction::Left);
        assert_eq!(next, loc); // Should stay in place
    }

    #[test]
    fn test_next_location_boundary_right() {
        let loc = Location::new(COLS - 1, 5);
        let next = loc.next_location(Direction::Right);
        assert_eq!(next, loc); // Should stay in place
    }

    #[test]
    fn test_is_valid_move_center() {
        let loc = Location::new(5, 5);
        assert!(loc.is_valid_move(Direction::Up));
        assert!(loc.is_valid_move(Direction::Down));
        assert!(loc.is_valid_move(Direction::Left));
        assert!(loc.is_valid_move(Direction::Right));
    }

    #[test]
    fn test_is_valid_move_top_left_corner() {
        let loc = Location::new(0, 0);
        assert!(!loc.is_valid_move(Direction::Up));
        assert!(loc.is_valid_move(Direction::Down));
        assert!(!loc.is_valid_move(Direction::Left));
        assert!(loc.is_valid_move(Direction::Right));
    }

    #[test]
    fn test_is_valid_move_bottom_right_corner() {
        let loc = Location::new(COLS - 1, ROWS - 1);
        assert!(loc.is_valid_move(Direction::Up));
        assert!(!loc.is_valid_move(Direction::Down));
        assert!(loc.is_valid_move(Direction::Left));
        assert!(!loc.is_valid_move(Direction::Right));
    }
}
