extern crate cairo;
extern crate gio;
extern crate glib;
extern crate gtk;

mod astar;
mod grid;

use std::time::Duration;
use crate::grid::Grid;
use crate::grid::GridObject;
use std::cell::RefCell;
use std::rc::Rc;

pub const COLS: u32 = 10;
pub const ROWS: u32 = 10;
const MAG: u32 = 20;

fn main() {
    let mut grid = Grid::new();
    let mut agents = Vec::new();
    let mut tiles = Vec::new();
    let mut holes = Vec::new();
    let mut obstacles = Vec::new();
    for i in 1..=1 {
        let l = grid.random_location();
        let a = Rc::new(RefCell::new(GridObject {
            location: l,
            object_type: crate::grid::Type::Agent,
            id: i,
            score: 0,
        }));
        grid.set_object(Rc::clone(&a), &l);
        agents.push(a);
    }
    for i in 1..=5 {
        let l = grid.random_location();
        let t = Rc::new(RefCell::new(GridObject {
            location: l,
            object_type: crate::grid::Type::Tile,
            id: i,
            score: 0, //rng.gen_range(1..6),
        }));
        grid.set_object(Rc::clone(&t), &l);
        tiles.push(t);
    }
    for i in 1..=5 {
        let l = grid.random_location();
        let h = Rc::new(RefCell::new(GridObject {
            location: l,
            object_type: crate::grid::Type::Hole,
            id: i,
            score: 0,
        }));
        grid.set_object(Rc::clone(&h), &l);
        holes.push(h);
    }
    for i in 1..=5 {
        let l = grid.random_location();
        let o = Rc::new(RefCell::new(GridObject {
            location: l,
            object_type: crate::grid::Type::Obstacle,
            id: i,
            score: 0,
        }));
        grid.set_object(Rc::clone(&o), &l);
        obstacles.push(o);
    }
    let g = Rc::new(RefCell::new(grid));
    loop {
        for a in agents.iter_mut() {
            let l = a.borrow().location;
            println!("current location: {:?}", l);
            if let Some(best_tile) = get_closest(&tiles, l) {
                if let Some(mut path) =
                    crate::astar::astar(Rc::clone(&g), l, best_tile.borrow().location)
                {
                    println!("path: {:?}", path);
                    let next_direction = path.remove(0);
                    let next_location = l.next_location(next_direction);
                    println!("next location: {:?}", next_location);
                    if g.borrow().is_free(&next_location)
                        || next_location == best_tile.borrow().location
                    {
                        println!("allowed, moving");
                        g.borrow_mut().set_object(Rc::clone(&a), &next_location);
                        a.borrow_mut().location = next_location;
                    }
                }
            }
        }
        let delay = Duration::from_millis(2000);
        std::thread::sleep(delay);
        g.borrow().print();
    }
    // let application = Application::new(Some("be.sourcery.tileworld"), Default::default())
    //     .expect("failed to initialize TileWorld");
    // view::start_grid(application);
}

pub fn get_closest<'a>(
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
