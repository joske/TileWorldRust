use crate::{
    location::Location,
    objects::{AgentInfo, AgentState, HoleState, Object, State, TileState, GO},
    COLS, MAG, ROWS,
};
use graphics::{
    color::{BLACK, BLUE, GREEN, RED},
    types::Color,
    CharacterCache, CircleArc, Context, Ellipse, Graphics, Rectangle, Text,
};
use log::{debug, info};
use rand::{thread_rng, Rng};
use std::{cell::RefCell, collections::HashMap, f64::consts::PI, rc::Rc};

#[derive(Default, Debug)]
pub struct Grid {
    objects: HashMap<Location, Object>,
}

impl Grid {
    pub fn new() -> Self {
        Grid::default()
    }

    pub fn object(&self, l: &Location) -> Option<&Object> {
        self.objects.get(l)
    }

    pub fn move_object(&mut self, o: Object, old: Location, new: Location) {
        self.objects.remove(&old);
        self.objects.insert(new, o);
    }

    pub fn remove(&mut self, l: &Location) {
        self.objects.remove(l);
    }

    pub fn is_free(&self, location: &Location) -> bool {
        !self.objects.contains_key(location)
    }

    pub fn random_location(&self) -> Location {
        let mut rng = rand::thread_rng();
        let mut c: u16 = rng.gen_range(0..COLS);
        let mut r: u16 = rng.gen_range(0..ROWS);

        let mut new_loc = Location::new(c, r);
        while !self.is_free(&new_loc) {
            c = rng.gen_range(0..COLS);
            r = rng.gen_range(0..ROWS);
            new_loc = Location::new(c, r);
        }
        debug!("random location: {:?}", new_loc);
        new_loc
    }

    pub fn update(&mut self, agents: &[Object], tiles: &[Object], holes: &[Object]) {
        for a in agents.iter() {
            if let GO::Agent(ref mut agent) = *a.borrow_mut() {
                agent.update(self, a.clone(), tiles, holes);
            }
        }
    }

    pub fn print(&self) {
        for r in 0..ROWS {
            let line = &mut ['.'; ROWS as usize];
            for c in 0..COLS {
                let l = Location::new(c, r);
                if !self.is_free(&l) {
                    if let Some(go) = self.object(&l) {
                        match *go.borrow() {
                            GO::Agent(ref _a) => line[c as usize] = 'A',
                            GO::Hole(ref _h) => line[c as usize] = 'H',
                            GO::Tile(ref t) => {
                                line[c as usize] = t.score.to_string().chars().next().unwrap()
                            }
                            GO::Obstacle(ref _o) => line[c as usize] = '#',
                        }
                    }
                }
            }
            let to_print: String = line.iter().cloned().collect();
            info!("{}", to_print);
        }
        info!("");
    }

    pub fn create_objects(
        &mut self,
        num_agents: u8,
        num_tiles: u8,
        num_holes: u8,
        num_obstacles: u8,
    ) -> (Vec<Object>, Vec<Object>, Vec<Object>) {
        let mut agents = vec![];
        let mut tiles = vec![];
        let mut holes = vec![];
        let mut rng = thread_rng();
        for i in 1..=num_agents {
            let l = self.random_location();
            let agent = AgentState {
                location: l,
                id: i,
                score: 0,
                hole: None,
                tile: None,
                has_tile: false,
                state: State::Idle,
            };
            let r = Rc::new(RefCell::new(GO::Agent(agent)));
            info!("created: {:?}", &r);
            agents.push(r.clone());
            self.objects.insert(l, r);
        }
        for _i in 1..=num_tiles {
            let l = self.random_location();
            let tile = TileState {
                location: l,
                score: rng.gen_range(1..6),
            };
            let r = Rc::new(RefCell::new(GO::Tile(tile)));
            info!("created: {:?}", &r);
            tiles.push(r.clone());
            self.objects.insert(l, r);
        }
        for _i in 1..=num_holes {
            let l = self.random_location();
            let r = Rc::new(RefCell::new(GO::Hole(HoleState { location: l })));
            info!("created: {:?}", &r);
            holes.push(r.clone());
            self.objects.insert(l, r);
        }
        for _i in 1..=num_obstacles {
            let l = self.random_location();
            let r = Rc::new(RefCell::new(GO::Obstacle(l)));
            info!("created: {:?}", &r);
            self.objects.insert(l, r);
        }
        (agents, tiles, holes)
    }

    pub fn draw<G: Graphics, C>(
        &self,
        agents: &[AgentInfo],
        glyphs: &mut C,
        ctx: &Context,
        g: &mut G,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        let width = f64::from(u16::saturating_mul(COLS, MAG as u16));
        let height = f64::from(u16::saturating_mul(ROWS, MAG as u16));
        Rectangle::new_border(BLACK, 1.0).draw(
            [0.0, 0.0, width, height],
            &ctx.draw_state,
            ctx.transform,
            g,
        );
        for r in 0..ROWS {
            for c in 0..COLS {
                let x = f64::from(u16::saturating_mul(c, MAG as u16));
                let y = f64::from(u16::saturating_mul(r, MAG as u16));
                let l = Location::new(c, r);
                if !self.is_free(&l) {
                    if let Some(go) = self.object(&l) {
                        match *go.borrow() {
                            GO::Agent(ref a) => {
                                let color = Grid::get_color(a.id - 1);
                                Rectangle::new_border(color, 1.0).draw(
                                    [x, y, MAG, MAG],
                                    &ctx.draw_state,
                                    ctx.transform,
                                    g,
                                );
                                if a.has_tile {
                                    let score = a.tile.as_ref().unwrap().borrow().score();
                                    CircleArc::new(color, 1.0, 0.0, 2f64 * PI).draw(
                                        [x, y, MAG, MAG],
                                        &ctx.draw_state,
                                        ctx.transform,
                                        g,
                                    );
                                    Text::new_color(color, 14)
                                        .draw_pos(
                                            &score.to_string(),
                                            [x + MAG / 4f64, y + MAG - 4.0],
                                            glyphs,
                                            &ctx.draw_state,
                                            ctx.transform,
                                            g,
                                        )
                                        .unwrap();
                                }
                            }
                            GO::Hole(_) => {
                                Ellipse::new(BLACK).draw_from_to(
                                    [x, y],
                                    [x + MAG, y + MAG],
                                    &ctx.draw_state,
                                    ctx.transform,
                                    g,
                                );
                            }
                            GO::Tile(ref t) => {
                                let score = t.score.to_string();
                                CircleArc::new(BLACK, 1.0, 0.0, 2f64 * PI).draw(
                                    [x, y, MAG, MAG],
                                    &ctx.draw_state,
                                    ctx.transform,
                                    g,
                                );
                                Text::new_color(BLACK, 14)
                                    .draw_pos(
                                        score.as_str(),
                                        [x + MAG / 4f64, y + MAG - 4.0],
                                        glyphs,
                                        &ctx.draw_state,
                                        ctx.transform,
                                        g,
                                    )
                                    .unwrap();
                            }
                            GO::Obstacle(_) => {
                                Rectangle::new(BLACK).draw(
                                    [x, y, MAG, MAG],
                                    &ctx.draw_state,
                                    ctx.transform,
                                    g,
                                );
                            }
                        }
                    }
                }
            }
        }
        let x = COLS as f64 * MAG + 50_f64;
        let y = 50f64;

        for agent in agents {
            let id = agent.id;
            let score = agent.score;
            let color = Grid::get_color(id - 1);
            let text = format!("Agent({id}): {score}");
            Text::new_color(color, 12)
                .draw_pos(
                    text.as_str(),
                    [x, y + id as f64 * MAG],
                    glyphs,
                    &ctx.draw_state,
                    ctx.transform,
                    g,
                )
                .unwrap();
        }
    }

    fn get_color(num: u8) -> Color {
        match num {
            0 => BLUE,
            1 => RED,
            2 => GREEN,
            3 => [0.5, 0.5, 0., 1.],
            4 => [0., 0.5, 0.5, 1.],
            5 => [0.5, 0., 0.5, 1.],
            _ => [0.5, 0.5, 0.5, 1.],
        }
    }
}
