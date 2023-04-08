use glutin_window::GlutinWindow;
use grid::Grid;
use objects::{AgentInfo, GO};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::{CloseEvent, EventSettings, Events, RenderEvent, Window, WindowSettings};
use rusttype::Font;
use std::{process::exit, thread::sleep, time::Duration};

mod astar;
mod grid;
mod location;
mod objects;

const COLS: u16 = 40;
const ROWS: u16 = 40;
const MAG: f64 = 20.0;
const DELAY: u64 = 150;

fn main() {
    env_logger::init();

    #[cfg(target_os = "macos")]
    let opengl = OpenGL::V3_2;
    #[cfg(target_os = "linux")]
    let opengl = OpenGL::V2_1;

    let settings = WindowSettings::new("TileWorld", (COLS as f64 * MAG + 200.0, ROWS as f64 * MAG))
        .automatic_close(true)
        .graphics_api(opengl)
        .vsync(true);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let font_data: &[u8] = include_bytes!("../UbuntuMono-R.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).expect("failed to load font");
    let glyphs = &mut GlyphCache::from_font(font, (), texture_settings);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    window.should_close();
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);
    let mut g = Grid::new();
    let (agents, tiles, holes) = g.create_objects(6, 20, 20, 20);
    while let Some(e) = events.next(&mut window) {
        if e.close_args().is_some() {
            break;
        }
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |ctx, glgraphics| {
                use graphics::clear;

                clear([1.0; 4], glgraphics);
                g.update(&agents, &tiles, &holes);
                g.print();
                let copy: Vec<AgentInfo> = agents
                    .iter()
                    .flat_map(|go| {
                        if let GO::Agent(ref a) = *go.borrow() {
                            Some(AgentInfo::from(a))
                        } else {
                            // can never happen, but rustc wants that we check all possibilities
                            None
                        }
                    })
                    .collect();
                g.draw(&copy, glyphs, &ctx, glgraphics);
                sleep(Duration::from_millis(DELAY));
            });
        }
    }
    exit(0);
}
