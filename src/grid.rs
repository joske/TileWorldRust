use super::{COLS, ROWS};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub state: State,
}

impl PartialEq for GridObject {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.location == other.location
    }
}

pub type WrappedGridObject = Rc<RefCell<GridObject>>;

pub struct Grid {
    objects: HashMap<Location, WrappedGridObject>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            objects: HashMap::new(),
        }
    }

    pub fn object(&self, l: &Location) -> Option<Rc<RefCell<GridObject>>> {
        let o = self.objects.get(l);
        match o {
            Some(go) => Some(Rc::clone(go)),
            None => None,
        }
    }

    pub fn set_object(&mut self, o: Rc<RefCell<GridObject>>, old_loc: &Location, l: &Location) {
        self.objects.remove(old_loc);
        self.objects.insert(*l, Rc::clone(&o));
    }

    pub fn remove(&mut self, l: &Location) {
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

pub fn update_agent(
    g: Rc<RefCell<Grid>>,
    a: Rc<RefCell<GridObject>>,
    tiles: &Vec<Rc<RefCell<GridObject>>>,
    holes: &Vec<Rc<RefCell<GridObject>>>,
) {
    println!("agent {:?}", a.borrow());
    let state = a.borrow().state;
    match state {
        crate::grid::State::Idle => idle_agent(Rc::clone(&a), &tiles),
        crate::grid::State::MoveToTile => move_to_tile(g, Rc::clone(&a), &holes),
        crate::grid::State::MoveToHole => move_to_hole(g, Rc::clone(&a), &holes),
    }
}

fn idle_agent(a: Rc<RefCell<GridObject>>, tiles: &Vec<Rc<RefCell<GridObject>>>) {
    let mut go = a.borrow_mut();
    let l = go.location;
    println!("current location: {:?}", l);
    if let Some(best_tile) = get_closest(&tiles, l) {
        println!("best tile: {:?}", best_tile);
        go.tile = Some(Rc::clone(&best_tile));
        go.state = crate::grid::State::MoveToTile;
    } else {
        println!("no best tile found");
    }
}

fn move_to_tile(
    g: Rc<RefCell<Grid>>,
    a: Rc<RefCell<GridObject>>,
    holes: &Vec<Rc<RefCell<GridObject>>>,
) {
    let mut agent = a.borrow_mut();
    if let Some(best_tile) = agent.tile.clone() {
        let l = agent.location;
        if l == best_tile.borrow().location {
            // arrived!
            agent.has_tile = true;
            if let Some(best_hole) = get_closest(&holes, l) {
                agent.hole = Some(Rc::clone(&best_hole));
                agent.state = State::MoveToHole;
            }
            g.borrow_mut().remove(&l); // remove tile
            let new_location = g.borrow().random_location();
            g.borrow_mut()
                .set_object(Rc::clone(&best_tile), &l, &new_location); // set the tile in a new location
            best_tile.borrow_mut().location = new_location;
            agent.state = State::MoveToHole;
        }
        if let Some(o) = g.borrow().object(&best_tile.borrow().location) {
            if *best_tile.borrow() != *o.borrow() {
                // our tile is gone
                agent.state = State::Idle;
                return;
            }
        }
        if let Some(mut path) = crate::astar::astar(Rc::clone(&g), l, best_tile.borrow().location) {
            if path.len() != 0 {
                println!("path: {:?}", path);
                let next_direction = path.remove(0);
                let next_location = l.next_location(next_direction);
                println!("next location: {:?}", next_location);
                if g.borrow().is_free(&next_location)
                    || next_location == best_tile.borrow().location
                {
                    println!("allowed, moving");
                    g.borrow_mut().set_object(Rc::clone(&a), &l, &next_location);
                    agent.location = next_location;
                } else {
                    println!("can't move!");
                }
            }
        }
    }
}

fn move_to_hole(
    g: Rc<RefCell<Grid>>,
    a: Rc<RefCell<GridObject>>,
    holes: &Vec<Rc<RefCell<GridObject>>>,
) {
    let mut agent = a.borrow_mut();
    if let Some(best_hole) = agent.hole.clone() {
        let l = agent.location;
        if l == best_hole.borrow().location {
            // arrived!
            agent.has_tile = false;
            agent.state = crate::grid::State::Idle;
            if let Some(t) = &agent.tile.clone() {
                agent.score += t.borrow().score;
            }
            g.borrow_mut().remove(&l); // remove hole
            let new_location = g.borrow().random_location();
            g.borrow_mut()
                .set_object(Rc::clone(&best_hole), &l, &new_location); //create in new location
            best_hole.borrow_mut().location = new_location;
        }
        if let Some(o) = g.borrow().object(&best_hole.borrow().location) {
            if *best_hole.borrow() != *o.borrow() {
                // our hole is gone, find a new one
                if let Some(best_hole) = get_closest(&holes, l) {
                    agent.hole = Some(Rc::clone(&best_hole));
                    agent.state = State::MoveToHole;
                }
                return;
            }
        }
        if let Some(mut path) = crate::astar::astar(Rc::clone(&g), l, best_hole.borrow().location) {
            if path.len() != 0 {
                println!("path: {:?}", path);
                let next_direction = path.remove(0);
                let next_location = l.next_location(next_direction);
                println!("next location: {:?}", next_location);
                if g.borrow().is_free(&next_location)
                    || next_location == best_hole.borrow().location
                {
                    println!("allowed, moving");
                    g.borrow_mut().set_object(Rc::clone(&a), &l, &next_location);
                    agent.location = next_location;
                }
            }
        }
    }
}

pub fn get_closest(
    collection: &Vec<Rc<RefCell<GridObject>>>,
    loc: crate::grid::Location,
) -> Option<Rc<RefCell<GridObject>>> {
    let mut closest: Option<Rc<RefCell<GridObject>>> = None;
    let mut dist = 1000000000;
    for tile_ref in collection.iter() {
        let t = tile_ref.borrow();
        if t.location.distance(loc) < dist {
            closest = Some(Rc::clone(tile_ref));
            dist = t.location.distance(loc);
        }
    }
    closest
}
