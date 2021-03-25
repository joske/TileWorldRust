use std::cell::RefCell;
use super::grid::Grid;
use super::grid::Type;
use glib::clone;
use gio::prelude::*;
use super::grid::Location;
use gtk::*;
use std::rc::Rc;

use super::{COLS, ROWS, MAG};

pub fn start_grid(workspace: Rc<RefCell<Grid>>, application : gtk::Application) {
application.connect_activate(move |app| {
    let window = ApplicationWindow::new(app);
    window.set_title("TileWorld");
    window.set_default_size((COLS * MAG) as i32 + 100, (ROWS * MAG) as i32);
    let frame = &Frame::new(None);
    window.add(frame);
    let area = DrawingArea::new();
    frame.add(&area);
    area.connect_draw(clone!(@weak workspace => @default-return Inhibit(false), move |_, cr| {
        let ref grid = *workspace.borrow();
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
                        Type::Agent => {
                            let (r, g, b) = get_color(ob.id);
                            cr.set_source_rgb(r, g, b);
                            cr.rectangle(x, y, MAG as f64, MAG as f64);
                            cr.stroke();
                        }
                        Type::Tile => {
                            cr.arc(x + MAG as f64 / 2., y + MAG as f64 / 2., MAG as f64 / 2.0, 0.0, 2.0 * PI);
                            cr.stroke();
                        }
                        Type::Hole => {
                            cr.arc(x + MAG as f64 / 2., y + MAG as f64 / 2., MAG as f64 / 2.0, 0.0, 2.0 * PI);
                            cr.fill();
                        }
                        Type::Obstacle => {
                            cr.rectangle(x, y, MAG as f64, MAG as f64);
                            cr.fill();
                        }
                    },
                }
            }
        }
        Inhibit(false)
    }));
    glib::timeout_add_local(200, clone!(@weak workspace => @default-return Continue(true), move || {
        area.queue_draw_area(0, 0, (COLS * MAG) as i32, (ROWS * MAG) as i32);
        workspace.borrow_mut().update();
        glib::Continue(true)
    }));
    window.show_all();
});

application.run(&[]);
}

fn get_color(num : u8) -> (f64, f64, f64) {
    match num {
        0 => (0., 0., 1.),
        1 => (1., 0., 0.),
        2 => (0., 1., 0.),
        3 => (0.5, 0.5, 0.),
        4 => (0., 0.5, 0.5),
        _ => (0.5, 0., 0.5),
    }
}