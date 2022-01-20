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
const AGENTS: u8 = 6;
const OBJECTS: u8 = 20;
const DELAY: u64 = 100;

fn main() {
    let application = Application::new(Some("be.sourcery.tileworld"), Default::default());
    view::start_grid(application);
}
