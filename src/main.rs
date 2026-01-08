use glutin_window::GlutinWindow;
use grid::Grid;
use objects::{AgentInfo, GO};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::{
    CloseEvent, EventSettings, Events, OpenGLWindow, RenderEvent, UpdateEvent, WindowSettings,
};
use rusttype::Font;
use std::process::exit;

mod astar;
mod draw;
mod grid;
mod location;
mod objects;

const COLS: u16 = 40;
const ROWS: u16 = 40;
const MAG: f64 = 20.0;
const UPDATES_PER_SECOND: u64 = 7; // ~143ms between updates (similar to old 150ms delay)

fn main() {
    #[cfg(target_os = "macos")]
    let opengl = OpenGL::V3_2;
    #[cfg(target_os = "linux")]
    let opengl = OpenGL::V2_1;

    let settings = WindowSettings::new(
        "TileWorld",
        (f64::from(COLS) * MAG + 200.0, f64::from(ROWS) * MAG),
    )
    .automatic_close(true)
    .graphics_api(opengl)
    .vsync(true);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let font_data: &[u8] = include_bytes!("../UbuntuMono-R.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).expect("failed to load font");
    let glyphs = &mut GlyphCache::from_font(font, (), texture_settings);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    // Configure event loop: separate update rate from render rate
    let mut event_settings = EventSettings::new();
    event_settings.ups = UPDATES_PER_SECOND; // Game logic updates per second
    event_settings.max_fps = 60; // Render up to 60 fps for smooth visuals
    let mut events = Events::new(event_settings);

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    let mut gl = GlGraphics::new(opengl);
    let mut g = Grid::new();
    let (agents, tiles, holes) = g.create_objects(6, 20, 20, 20);

    // Cache agent info for rendering (updated on game update, used on render)
    let mut agent_info: Vec<AgentInfo> = agents
        .iter()
        .filter_map(|go| {
            if let GO::Agent(ref a) = *go.borrow() {
                Some(AgentInfo::from(a))
            } else {
                None
            }
        })
        .collect();

    while let Some(e) = events.next(&mut window) {
        if e.close_args().is_some() {
            break;
        }

        // Handle game logic updates (at UPDATES_PER_SECOND rate)
        if e.update_args().is_some() {
            g.update(&agents, &tiles, &holes);

            // Update cached agent info for rendering
            agent_info = agents
                .iter()
                .filter_map(|go| {
                    if let GO::Agent(ref a) = *go.borrow() {
                        Some(AgentInfo::from(a))
                    } else {
                        None
                    }
                })
                .collect();
        }

        // Handle rendering (at up to max_fps rate)
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |ctx, glgraphics| {
                use graphics::clear;
                clear([1.0; 4], glgraphics);
                draw::draw(&g, &agent_info, glyphs, &ctx, glgraphics);
            });
        }
    }
    exit(0);
}
