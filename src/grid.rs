use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::{COLS, ROWS};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(1..=4) {
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Location {
    pub col: u32,
    pub row: u32,
}

impl Location {
    pub fn next_location(&self, d: Direction) -> Location {
        match d {
            Direction::Up => {
                if self.row > 0 {
                    Location {
                        col: self.col,
                        row: self.row - 1,
                    }
                } else {
                    *self
                }
            }
            Direction::Down => {
                if self.row < COLS - 1 {
                    Location {
                        col: self.col,
                        row: self.row + 1,
                    }
                } else {
                    *self
                }
            }
            Direction::Left => {
                if self.col > 0 {
                    Location {
                        col: self.col - 1,
                        row: self.row,
                    }
                } else {
                    *self
                }
            }
            Direction::Right => {
                if self.col < ROWS - 1 {
                    Location {
                        col: self.col + 1,
                        row: self.row,
                    }
                } else {
                    *self
                }
            }
        }
    }

    pub fn direction(&self, other: &Self) -> Direction {
        if self.col == other.col {
            if self.row == other.row - 1 {
                return Direction::Up;
            } else {
                return Direction::Down;
            }
        } else if self.col == other.col - 1 {
            return Direction::Left;
        } else {
            return Direction::Right;
        }
    }

    pub fn is_valid(&self, d: Direction) -> bool {
        match d {
            Direction::Up => self.row > 0,
            Direction::Down => self.row < ROWS - 1,
            Direction::Left => self.col > 0,
            Direction::Right => self.col < COLS - 1,
        }
    }

    pub fn distance(&self, other: Location) -> u32 {
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

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Agent,
    Tile,
    Hole,
    Obstacle,
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Idle,
    MoveToTile,
    MoveToHole,
}

#[derive(Debug)]
pub struct GridObject {
    pub id: u8,
    pub object_type: Type,
    pub location: Location,
    pub score: u32,
    pub tile: Option<Rc<RefCell<GridObject>>>,
    pub hole: Option<Rc<RefCell<GridObject>>>,
    pub has_tile: bool,
    pub state : State,
}

pub struct Grid {
    objects: HashMap<Location, Rc<RefCell<GridObject>>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            objects: HashMap::new()
        }
    }

    pub fn object(&self, l: &Location) -> Option<Rc<RefCell<GridObject>>> {
        let o = self.objects.get(l);
        match o {
            Some(go) => Some(Rc::clone(go)),
            None => None,
        }
    }

    pub fn set_object(&mut self, o: Rc<RefCell<GridObject>>, l : &Location) {
        let old_loc = &o.borrow().location;
        self.objects.remove(old_loc);
        self.objects.insert(*l, Rc::clone(&o));
    }

    pub fn remove(&mut self, l : &Location) {
        self.objects.remove(l);
    }

    pub fn is_free(&self, location: &Location) -> bool {
        let o = self.objects.get(location);
        match o {
            None => true,
            Some(_) => false,
        }
    }

    pub fn random_location(&self) -> Location {
        let mut rng = rand::thread_rng();
        let mut c: u32 = rng.gen_range(1..COLS);
        let mut r: u32 = rng.gen_range(1..ROWS);

        let mut l = Location { col: c, row: r };
        while !self.is_free(&l) {
            c = rng.gen_range(1..COLS);
            r = rng.gen_range(1..ROWS);
            l = Location { col: c, row: r };
        }
        println!("random location: {:?}", l);
        return l;
    }

    pub fn print(&self) {
        for r in 0..ROWS {
            for c in 0..COLS {
                let l = Location { col: c, row: r };
                if !self.is_free(&l) {
                    let o = self.objects.get(&l);
                    match o.unwrap().borrow().object_type {
                        Type::Agent => print!("A"),
                        Type::Hole => print!("H"),
                        Type::Tile => print!("{}", o.unwrap().borrow().score.to_string()),
                        Type::Obstacle => print!("#"),
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

}


