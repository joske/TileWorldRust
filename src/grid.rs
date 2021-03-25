extern crate rand;
use super::{COLS, ROWS};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::thread;
use std::time::Duration;

pub struct Grid {
    agents: Vec<GridObject>,
    tiles: Vec<GridObject>,
    holes: Vec<GridObject>,
    obstacles: Vec<GridObject>,
    objects: [[Option<GridObject>; COLS as usize]; ROWS as usize],
}

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
        } else {
            if self.col == other.col - 1 {
                return Direction::Left;
            } else {
                return Direction::Right;
            }
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
pub struct GridObject {
    pub id: u8,
    pub object_type: Type,
    pub location: Location,
    pub score: u32,
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Grid {
            agents: Vec::new(),
            tiles: Vec::new(),
            holes: Vec::new(),
            obstacles: Vec::new(),
            objects: [[None; COLS as usize]; ROWS as usize],
        };
        grid.init(3, 5, 5, 5);
        return grid;
    }

    pub fn init(&mut self, num_agents: u8, num_tiles: u8, num_holes: u8, num_obstacles: u8) {
        let mut rng = rand::thread_rng();
        for i in 1..=num_agents {
            let a = GridObject {
                location: self.random_location(),
                object_type: Type::Agent,
                id: i,
                score: 0,
            };
            self.agents.push(a);
            self.set_object(&a);
        }
        for i in 1..=num_tiles {
            let t = GridObject {
                location: self.random_location(),
                object_type: Type::Tile,
                id: i,
                score: rng.gen_range(1..6),
            };
            self.tiles.push(t);
            self.set_object(&t);
        }
        for i in 1..=num_holes {
            let h = GridObject {
                location: self.random_location(),
                object_type: Type::Hole,
                id: i,
                score: 0,
            };
            self.holes.push(h);
            self.set_object(&h);
        }
        for i in 1..=num_obstacles {
            let o = GridObject {
                location: self.random_location(),
                object_type: Type::Obstacle,
                id: i,
                score: 0,
            };
            self.obstacles.push(o);
            self.set_object(&o);
        }
    }

    pub fn object(&self, l: &Location) -> &Option<GridObject> {
        &self.objects[l.col as usize][l.row as usize]
    }

    pub fn set_object<'grid>(&mut self, o: &'grid GridObject) {
        self.objects[o.location.col as usize][o.location.row as usize] = Some(*o);
    }

    pub fn is_free(&self, location: Location) -> bool {
        let o = self.objects[location.col as usize][location.row as usize];
        match o {
            None => true,
            Some(_) => false,
        }
    }

    fn random_location(&self) -> Location {
        let mut rng = rand::thread_rng();
        let mut c: u32 = rng.gen_range(1..COLS);
        let mut r: u32 = rng.gen_range(1..ROWS);

        let mut l = Location { col: c, row: r };
        while !self.is_free(l) {
            c = rng.gen_range(1..COLS);
            r = rng.gen_range(1..ROWS);
            l = Location { col: c, row: r };
        }
        return l;
    }

    pub fn print(&self) {
        for c in 0..COLS {
            for r in 0..ROWS {
                let l = Location { col: c, row: r };
                if !self.is_free(l) {
                    let o = &self.objects[l.col as usize][l.row as usize];
                    match o.unwrap().object_type {
                        Type::Agent => print!("A"),
                        Type::Hole => print!("H"),
                        Type::Tile => print!("{}", o.unwrap().score.to_string()),
                        Type::Obstacle => print!("#"),
                    }
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        for a in self.agents.iter() {
            let score = a.score;
            print!("Agent {} : {}\n", a.id, score.to_string());
        }
    }

    pub fn update(&mut self) {
        for mut a in self.agents.iter_mut() {
            let d: Direction = rand::random();
            let l = a.location;
            let new_loc = l.next_location(d);
            self.objects[l.col as usize][l.row as usize] = None;
            a.location = new_loc;
            self.objects[new_loc.col as usize][new_loc.row as usize] = Some(*a);
            //let path = super::astar::astar(reference.clone(), l, Location{col:1, row:1});
            print!("Move Agent {:?} to {:?}\n", a, l.next_location(d));
        }
        self.print();
    }
}
