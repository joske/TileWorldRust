extern crate cairo;
extern crate gio;
extern crate glib;
extern crate gtk;

mod astar;
mod grid;

use gio::prelude::*;
use grid::Location;
use gtk::*;

pub const COLS: u32 = 40;
pub const ROWS: u32 = 40;
const MAG: u32 = 20;

fn main() {
    let mut grid = grid::Grid::new();
    grid.init();
    grid.print();
    grid.update();
    grid.print();

    let application = Application::new(Some("be.sourcery.tileworld"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size((COLS * MAG) as i32 + 100, (ROWS * MAG) as i32);
        let frame = &Frame::new(None);
        window.add(frame);
        let area = DrawingArea::new();
        frame.add(&area);
        area.connect_draw(|_, cr| {
            use std::f64::consts::PI;
            cr.set_source_rgb(1., 1., 1.);
            cr.paint();
            cr.set_line_width(2.);
            for r in 0..ROWS {
                for c in 0..COLS {
                    let x = (c * MAG) as f64;
                    let y = (r * MAG) as f64;
                    cr.set_source_rgb(0., 0., 0.);
                    let o = grid.object(&Location { col: c, row: r });
                    match o {
                        None => (),
                        Some(ob) => match ob.object_type {
                            grid::Type::Agent => {
                                cr.rectangle(x, y, MAG as f64, MAG as f64);
                                cr.stroke();
                            }
                            grid::Type::Tile => {
                                cr.arc(x, y, MAG as f64, 0.0, 2.0 * PI);
                                cr.stroke();
                            }
                            grid::Type::Hole => {
                                cr.arc(x, y, MAG as f64, 0.0, 2.0 * PI);
                                cr.fill();
                            }
                            _ => (),
                        },
                    }
                }
            }
            cr.stroke();
            cr.stroke();
            Inhibit(false)
        });
        window.show_all();
    });

    application.run(&[]);
}
