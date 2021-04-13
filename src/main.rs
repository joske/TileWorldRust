extern crate cairo;
extern crate gio;
extern crate glib;
extern crate gtk;

mod astar;
mod grid;
mod view;

use crate::grid::GridObject;
use gtk::Application;

pub const COLS: u32 = 40;
pub const ROWS: u32 = 40;
const MAG: u32 = 20;

fn main() {
    let application = Application::new(Some("be.sourcery.tileworld"), Default::default())
        .expect("failed to initialize TileWorld");
    view::start_grid(application);

}
