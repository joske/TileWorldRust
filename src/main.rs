extern crate cairo;
extern crate gio;
extern crate glib;
extern crate gtk;

mod astar;
mod grid;
mod view;

use crate::grid::Grid;
use crate::grid::GridObject;
use gtk::Application;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

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
            tile: None,
            hole: None,
            has_tile: false,
            state: crate::grid::State::Idle,
        }));
        grid.set_object(Rc::clone(&a), &l, &l);
        agents.push(a);
    }
    for i in 1..=5 {
        let l = grid.random_location();
        let t = Rc::new(RefCell::new(GridObject {
            location: l,
            object_type: crate::grid::Type::Tile,
            id: i,
            score: 0, //rng.gen_range(1..6),
            tile: None,
            hole: None,
            has_tile: false,
            state: crate::grid::State::Idle,
        }));
        grid.set_object(Rc::clone(&t), &l, &l);
        tiles.push(t);
    }
    for i in 1..=5 {
        let l = grid.random_location();
        let h = Rc::new(RefCell::new(GridObject {
            location: l,
            object_type: crate::grid::Type::Hole,
            id: i,
            score: 0,
            tile: None,
            hole: None,
            has_tile: false,
            state: crate::grid::State::Idle,
        }));
        grid.set_object(Rc::clone(&h), &l, &l);
        holes.push(h);
    }
    for i in 1..=5 {
        let l = grid.random_location();
        let o = Rc::new(RefCell::new(GridObject {
            location: l,
            object_type: crate::grid::Type::Obstacle,
            id: i,
            score: 0,
            tile: None,
            hole: None,
            has_tile: false,
            state: crate::grid::State::Idle,
        }));
        grid.set_object(Rc::clone(&o), &l, &l);
        obstacles.push(o);
    }
    let g = Rc::new(RefCell::new(grid));
    // let application = Application::new(Some("be.sourcery.tileworld"), Default::default())
    //     .expect("failed to initialize TileWorld");
    // view::start_grid(Rc::clone(&g), application);

    loop {
        g.borrow().print();
        for a in agents.iter_mut() {
            update_agent(Rc::clone(&g), Rc::clone(&a), &tiles, &holes);
        }
        let delay = Duration::from_millis(200);
        std::thread::sleep(delay);
    }

    fn update_agent(
        g: Rc<RefCell<Grid>>,
        a: Rc<RefCell<GridObject>>,
        tiles: &Vec<Rc<RefCell<GridObject>>>,
        holes: &Vec<Rc<RefCell<GridObject>>>,
    ) {
        println!("agent {:?}", a.borrow());
        let state = a.borrow().state;
        match state {
            crate::grid::State::Idle => idle_agent(Rc::clone(&a), &tiles),
            crate::grid::State::MoveToTile => move_to_tile(g, Rc::clone(&a), &tiles, &holes),
            crate::grid::State::MoveToHole => move_to_hole(g, Rc::clone(&a), &tiles, &holes),
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
        tiles: &Vec<Rc<RefCell<GridObject>>>,
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
                    agent.state = crate::grid::State::MoveToHole;
                }
                g.borrow_mut().remove(&l); // remove tile
                let new_location = g.borrow().random_location();
                g.borrow_mut().set_object(Rc::clone(&best_tile), &l, &new_location); // set the tile in a new location
                best_tile.borrow_mut().location = new_location;
                agent.state = crate::grid::State::MoveToHole;
            }
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
                    g.borrow_mut().set_object(Rc::clone(&a), &l, &next_location);
                    agent.location = next_location;
                } else {
                    println!("can't move!");
                }
            }
        }
    }

    fn move_to_hole(
        g: Rc<RefCell<Grid>>,
        a: Rc<RefCell<GridObject>>,
        tiles: &Vec<Rc<RefCell<GridObject>>>,
        holes: &Vec<Rc<RefCell<GridObject>>>,
    ) {
        let mut agent = a.borrow_mut();
        if let Some(best_hole) = agent.hole.clone() {
            let l = agent.location;
            if l == best_hole.borrow().location {
                // arrived!
                agent.has_tile = false;
                agent.state = crate::grid::State::Idle;
                g.borrow_mut().remove(&l); // remove hole
                let new_location = g.borrow().random_location();
                g.borrow_mut().set_object(Rc::clone(&best_hole), &l, &new_location); //create in new location
                best_hole.borrow_mut().location = new_location;
            }
            if let Some(mut path) =
                crate::astar::astar(Rc::clone(&g), l, best_hole.borrow().location)
            {
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
