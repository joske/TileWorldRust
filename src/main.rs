extern crate cairo;
extern crate gio;
extern crate glib;
extern crate gtk;

mod astar;
mod grid;
mod view;

use gtk::*;
use std::rc::Rc;

pub const COLS: u32 = 10;
pub const ROWS: u32 = 10;
const MAG: u32 = 20;

fn main() {
    let mut grid = grid::Grid::new();
    grid.init();
    grid.print();
    grid.update();
    grid.print();

    let workspace = Rc::new(grid);
    let application = Application::new(Some("be.sourcery.tileworld"), Default::default())
        .expect("failed to initialize GTK application");
    view::start_grid(workspace, application);
}
