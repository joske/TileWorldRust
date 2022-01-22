use super::grid::Grid;
use super::grid::Location;
use super::grid::Type;
use super::grid::WrappedGridObject;
use super::GridObject;

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

use rand::thread_rng;
use rand::Rng;

use super::{AGENTS, OBJECTS, COLS, MAG, ROWS, DELAY};

pub fn start_grid(application: gtk::Application) {
    let grid = Grid::new();
    let workspace = Rc::new(RefCell::new(grid));
    let (agents, tiles, holes) = create_objects(AGENTS, OBJECTS, OBJECTS, OBJECTS, workspace.clone());
    let wrapped_agents = Rc::new(RefCell::new(agents));
    let wrapped_tiles = Rc::new(RefCell::new(tiles));
    let wrapped_holes = Rc::new(RefCell::new(holes));
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
                            let (r, g, b) = get_color(ob.borrow().id);
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
                  let (r, b, g) = get_color(id as u8);
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
        _ => (0.5, 0., 0.5),
    }
}

fn create_objects(
    num_agents: u8,
    num_tiles: u8,
    num_holes: u8,
    num_obstacles: u8,
    wgrid: Rc<RefCell<Grid>>,
) -> (
    Vec<WrappedGridObject>,
    Vec<WrappedGridObject>,
    Vec<WrappedGridObject>,
) {
    let mut grid = wgrid.borrow_mut();
    let mut agents = Vec::new();
    let mut tiles = Vec::new();
    let mut holes = Vec::new();
    let mut obstacles = Vec::new();
    for i in 1..=num_agents {
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
    for i in 1..=num_tiles {
        let l = grid.random_location();
        let mut rng = thread_rng();
        let t = Rc::new(RefCell::new(GridObject {
            location: l,
            object_type: crate::grid::Type::Tile,
            id: i,
            score: rng.gen_range(1..6),
            tile: None,
            hole: None,
            has_tile: false,
            state: crate::grid::State::Idle,
        }));
        grid.set_object(Rc::clone(&t), &l, &l);
        tiles.push(t);
    }
    for i in 1..=num_holes {
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
    for i in 1..=num_obstacles {
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
    return (agents, tiles, holes);
}
