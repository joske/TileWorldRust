mod astar;
mod grid;
mod view;

use grid::World;
#[cfg(feature = "gui")]
use gtk::Application;

pub const COLS: u32 = 40;
pub const ROWS: u32 = 40;
const AGENTS: u8 = 6;
const OBJECTS: u8 = 20;
#[cfg(feature = "gui")]
const MAG: u32 = 20;
#[cfg(feature = "gui")]
const DELAY: u64 = 100;

fn main() {
    env_logger::init();
    start_view();
}

#[cfg(feature = "gui")]
fn start_view() {
    let world = World::new();
    let application = Application::new(Some("be.sourcery.tileworld"), Default::default());
    view::start_grid(world, application);
}

#[cfg(not(feature = "gui"))]
fn start_view() {
    use std::{
        cell::RefCell,
        rc::Rc,
        thread::{self},
        time::Duration,
    };

    let world = World::new();
    let grid = Rc::new(RefCell::new(world.grid));
    let mut agents = world.agents;
    let tiles = world.tiles;
    let holes = world.holes;
    loop {
        for a in agents.iter_mut() {
            crate::grid::update_agent(Rc::clone(&grid), Rc::clone(a), &tiles, &holes);
        }
        grid.borrow().print(&agents);
        thread::sleep(Duration::from_millis(200));
    }
}
