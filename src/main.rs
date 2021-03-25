extern crate cairo;
extern crate gio;
extern crate glib;
extern crate gtk;

mod astar;
mod grid;
mod view;

use std::cell::RefCell;
use gtk::*;
use std::rc::Rc;

pub const COLS: u32 = 40;
pub const ROWS: u32 = 40;
const MAG: u32 = 20;

fn main() {
    let grid = grid::Grid::new(5, 20, 20, 20);
    let workspace = Rc::new(RefCell::new(grid));

    let application = Application::new(Some("be.sourcery.tileworld"), Default::default())
        .expect("failed to initialize GTK application");
    view::start_grid(workspace.clone(), application);
}
