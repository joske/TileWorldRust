use crate::{
    COLS, ROWS,
    location::Location,
    objects::{AgentState, GO, HoleState, Object, State, TileState},
};
use rand::Rng;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default, Debug)]
pub struct Grid {
    objects: HashMap<Location, Object>,
}

impl Grid {
    pub fn new() -> Self {
        Grid::default()
    }

    pub fn object(&self, l: Location) -> Option<&Object> {
        self.objects.get(&l)
    }

    pub fn move_object(&mut self, o: Object, old: Location, new: Location) {
        self.objects.remove(&old);
        self.objects.insert(new, o);
    }

    pub fn is_free(&self, location: Location) -> bool {
        !self.objects.contains_key(&location)
    }

    pub fn random_location(&self) -> Location {
        let mut rng = rand::rng();
        let mut c: u16 = rng.random_range(0..COLS);
        let mut r: u16 = rng.random_range(0..ROWS);

        let mut new_loc = Location::new(c, r);
        while !self.is_free(new_loc) {
            c = rng.random_range(0..COLS);
            r = rng.random_range(0..ROWS);
            new_loc = Location::new(c, r);
        }
        new_loc
    }

    pub fn update(&mut self, agents: &[Object], tiles: &[Object], holes: &[Object]) {
        for a in agents {
            if let GO::Agent(ref mut agent) = *a.borrow_mut() {
                agent.update(self, a.clone(), tiles, holes);
            }
        }
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
        let mut rng = rand::rng();
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
            agents.push(r.clone());
            self.objects.insert(l, r);
        }
        for _i in 1..=num_tiles {
            let l = self.random_location();
            let tile = TileState {
                location: l,
                score: rng.random_range(1..6),
            };
            let r = Rc::new(RefCell::new(GO::Tile(tile)));
            tiles.push(r.clone());
            self.objects.insert(l, r);
        }
        for _i in 1..=num_holes {
            let l = self.random_location();
            let r = Rc::new(RefCell::new(GO::Hole(HoleState { location: l })));
            holes.push(r.clone());
            self.objects.insert(l, r);
        }
        for _i in 1..=num_obstacles {
            let l = self.random_location();
            let r = Rc::new(RefCell::new(GO::Obstacle(l)));
            self.objects.insert(l, r);
        }
        (agents, tiles, holes)
    }
}
