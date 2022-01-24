use crate::grid::World;

use super::grid::Location;
use super::grid::Type;

use cairo::{Context, FontSlant, FontWeight};
use glib::clone;
use gtk::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use gio::prelude::ApplicationExt;
use gio::prelude::ApplicationExtManual;
use gtk::prelude::GtkWindowExt;
use gtk::prelude::ContainerExt;
use gtk::prelude::WidgetExt;
use glib::Continue;


use super::{COLS, MAG, ROWS, DELAY};

pub fn start_grid(world: World, application: gtk::Application) {
    let workspace = Rc::new(RefCell::new(world.grid));
    let wrapped_agents= Rc::new(RefCell::new(world.agents));
    let wrapped_tiles= Rc::new(RefCell::new(world.tiles));
    let wrapped_holes= Rc::new(RefCell::new(world.holes));
    application.connect_activate(move |app| {
    let window = ApplicationWindow::new(app);
    window.set_title("TileWorld");
    window.set_default_size((COLS * MAG) as i32 + 200, (ROWS * MAG) as i32);
    let frame = &Frame::new(None);
    window.add(frame);
    let area = DrawingArea::new();
    frame.add(&area);
    area.connect_draw(clone!(@weak workspace, @weak wrapped_agents => @default-return Inhibit(false), move |_, cr| {
        let ref grid = *workspace.borrow();
        use std::f64::consts::PI;
        cr.set_source_rgb(1., 1., 1.);
        cr.paint().map_err(|err| println!("{:?}", err)).ok();
        cr.set_line_width(2.);
        for r in 0..ROWS {
            for c in 0..COLS {
                let x = (c * MAG) as f64;
                let y = (r * MAG) as f64;
                cr.set_source_rgb(0., 0., 0.);
                let o = grid.object(&Location { col: c, row: r });
                match o {
                    None => (),
                    Some(ob) => match ob.borrow().object_type {
                        Type::Agent => {
                            let (r, g, b) = get_color(ob.borrow().id - 1);
                            cr.set_source_rgb(r, g, b);
                            cr.rectangle(x, y, MAG as f64, MAG as f64);
                            cr.new_sub_path();
                            if ob.borrow().has_tile {
                                cr.arc(x + MAG as f64 / 2., y + MAG as f64 / 2., MAG as f64 / 2.0, 0.0, 2.0 * PI);
                                if let Some(t) = &ob.borrow().tile {
                                    draw_text(cr, x + 6.,  y + 13., &t.borrow().score.to_string());
                                }
                            }
                            cr.stroke().map_err(|err| println!("{:?}", err)).ok();
                        }
                        Type::Tile => {
                            cr.arc(x + MAG as f64 / 2., y + MAG as f64 / 2., MAG as f64 / 2.0, 0.0, 2.0 * PI);
                            cr.stroke().map_err(|err| println!("{:?}", err)).ok();
                            draw_text(cr, x + 6.,  y + 13., &ob.borrow().score.to_string());
                        }
                        Type::Hole => {
                            cr.arc(x + MAG as f64 / 2., y + MAG as f64 / 2., MAG as f64 / 2.0, 0.0, 2.0 * PI);
                            cr.fill().map_err(|err| println!("{:?}", err)).ok();
                        }
                        Type::Obstacle => {
                            cr.rectangle(x, y, MAG as f64, MAG as f64);
                            cr.fill().map_err(|err| println!("{:?}", err)).ok();
                        }
                    },
                }
            }
            let x = COLS as f64 * MAG as f64 + 50 as f64;
            let y = 20 as f64;
            let agents = wrapped_agents.borrow();
             for a in agents.iter() {
                 let id = a.borrow().id as f64;
                 let score = a.borrow().score as f64;
                  let (r, b, g) = get_color(id as u8 - 1);
                  cr.set_source_rgb(r, g, b);
                  let text = format!("Agent({}): {}", id, score);
                  draw_text(cr, x, y + id * MAG as f64, &String::from(text));
            }
        
        }
        Inhibit(false)
    }));
    glib::timeout_add_local(Duration::from_millis(DELAY), glib::clone!(@weak workspace, @weak wrapped_agents, @weak wrapped_tiles, @weak wrapped_holes => @default-return Continue(true), move || {
        area.queue_draw_area(0, 0, (COLS * MAG) as i32 + 200, (ROWS * MAG) as i32);
        let mut agents = wrapped_agents.borrow_mut();
        let tiles = wrapped_tiles.borrow_mut();
        let holes = wrapped_holes.borrow_mut();
        for a in agents.iter_mut() {
            crate::grid::update_agent(Rc::clone(&workspace), Rc::clone(&a), &tiles, &holes);
        }
        workspace.borrow().print();
        glib::Continue(true)
    }));
    window.show_all();
});

    application.run();
}

fn draw_text(cr: &Context, x: f64, y: f64, text: &String) {
    cr.select_font_face("Arial", FontSlant::Normal, FontWeight::Normal);
    cr.set_font_size(14.);

    cr.move_to(x, y);
    cr.show_text(text).map_err(|err| println!("{:?}", err)).ok();
    cr.stroke().map_err(|err| println!("{:?}", err)).ok();
}

fn get_color(num: u8) -> (f64, f64, f64) {
    match num {
        0 => (0., 0., 1.),
        1 => (1., 0., 0.),
        2 => (0., 1., 0.),
        3 => (0.5, 0.5, 0.),
        4 => (0., 0.5, 0.5),
        5 => (0.5, 0., 0.5),
        _ => (0.5, 0.5, 0.5),
    }
}

