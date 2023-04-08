use crate::{grid::Grid, location::Location};
use log::debug;
use paste::paste;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    MoveToTile,
    MoveToHole,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AgentState {
    pub location: Location,
    pub id: u8,
    pub score: u32,
    pub tile: Option<Object>,
    pub hole: Option<Object>,
    pub has_tile: bool,
    pub state: State,
}

pub struct AgentInfo {
    pub id: u8,
    pub score: u32,
}

impl From<&AgentState> for AgentInfo {
    fn from(value: &AgentState) -> Self {
        AgentInfo {
            id: value.id,
            score: value.score,
        }
    }
}

fn get_closest(collection: &[Object], loc: Location) -> Option<Object> {
    let mut closest: Option<Object> = None;
    let mut dist = u16::MAX;
    for tile_ref in collection.iter() {
        let t = tile_ref.borrow();
        if t.location().distance(loc) < dist {
            closest = Some(Rc::clone(tile_ref));
            dist = t.location().distance(loc);
        }
    }
    closest
}

// 95% of move_to_tile and move_to_hole are the same, so we use a macro to generate both functions
macro_rules! move_to {
    ($dest:ident, $arrived:ident) => {
        paste!{
            /// move towards the destination
            /// always get the closest $dest again
            /// check if this we are at the destination already
            /// otherwise calculate a path to the destination and move
            fn [<move_to_ $dest>](&mut self, g: &mut Grid, go: Object, tiles: &[Object], holes: &[Object]) {
                let mut list = tiles;
                if stringify!($dest) == "hole" {
                    list = holes;
                }
                let agent_location = self.location;
                let best = get_closest(list, agent_location).unwrap(); // guaranteed to
                // be a best $dest
                self.$dest = Some(best.clone());
                if agent_location == *best.borrow().location() {
                    // arrived!
                    self. $arrived(g, go, tiles, holes, agent_location, best);
                    return;
                }
                if let Some(mut path) = crate::astar::astar(g, agent_location, *best.borrow().location()) {
                    if !path.is_empty() {
                        debug!("path: {:?}", path);
                        let next_direction = path.remove(0);
                        let next_location = agent_location.next_location(next_direction);
                        debug!("next location: {:?}", next_location);
                        if g.is_free(&next_location) || next_location == *best.borrow().location()
                        {
                            debug!("allowed, moving");
                            self.location = next_location;
                            g.move_object(go, agent_location, next_location);
                        } else {
                            debug!("can't move!");
                        }
                    }
                };
            }
        }
    }
}

impl AgentState {
    pub fn update(&mut self, g: &mut Grid, go: Object, tiles: &[Object], holes: &[Object]) {
        debug!("agent {:?}", self);
        match self.state {
            State::Idle => self.idle(tiles),
            State::MoveToTile => self.move_to_tile(g, go, tiles, holes),
            State::MoveToHole => self.move_to_hole(g, go, tiles, holes),
        }
    }

    fn idle(&mut self, tiles: &[Object]) {
        let agent_location = self.location;
        debug!("current location: {:?}", agent_location);
        if let Some(best_tile) = get_closest(tiles, agent_location) {
            debug!("best tile: {:?}", best_tile);
            self.tile = Some(Rc::clone(&best_tile));
            self.state = State::MoveToTile;
        } else {
            debug!("no best tile found");
        }
    }

    /// code to call when the agent arrives at a tile
    fn pick_tile(
        &mut self,
        g: &mut Grid,
        go: Object,
        _tiles: &[Object],
        holes: &[Object],
        agent_location: Location,
        best_tile: Object,
    ) {
        self.has_tile = true;
        if let Some(best_hole) = get_closest(holes, agent_location) {
            self.hole = Some(Rc::clone(&best_hole));
            self.state = State::MoveToHole;
        }
        g.remove(&agent_location);
        let new_location = g.random_location();
        best_tile.borrow_mut().set_location(new_location);
        g.move_object(best_tile, agent_location, new_location);
        self.location = agent_location;
        g.move_object(go, agent_location, agent_location);
        self.state = State::MoveToHole;
    }

    move_to!(tile, pick_tile);

    /// code to call when the agent arrives at a hole
    fn dump_tile(
        &mut self,
        g: &mut Grid,
        go: Object,
        tiles: &[Object],
        _holes: &[Object],
        agent_location: Location,
        best_hole: Object,
    ) {
        self.has_tile = false;
        if let Some(t) = &self.tile.clone() {
            if let GO::Tile(ref tstate) = *t.borrow() {
                self.score += tstate.score;
            }
        }
        g.remove(&agent_location);
        let new_location = g.random_location();
        best_hole.borrow_mut().set_location(new_location);
        g.move_object(best_hole, agent_location, new_location);
        self.location = agent_location;
        g.move_object(go, agent_location, agent_location);
        if let Some(best_tile) = get_closest(tiles, agent_location) {
            self.tile = Some(Rc::clone(&best_tile));
            self.state = State::MoveToTile;
        }
    }

    move_to!(hole, dump_tile);
}

#[derive(Debug, PartialEq, Eq)]
pub struct TileState {
    pub location: Location,
    pub score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HoleState {
    pub location: Location,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GO {
    Agent(AgentState),
    Tile(TileState),
    Hole(HoleState),
    Obstacle(Location),
}

impl GO {
    pub fn location(&self) -> &Location {
        match self {
            GO::Agent(ref a) => &a.location,
            GO::Tile(ref t) => &t.location,
            GO::Hole(ref h) => &h.location,
            GO::Obstacle(ref o) => o,
        }
    }

    fn set_location(&mut self, l: Location) {
        match self {
            GO::Agent(ref mut a) => a.location = l,
            GO::Tile(ref mut t) => t.location = l,
            GO::Hole(ref mut h) => h.location = l,
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn score(&self) -> u32 {
        match self {
            GO::Agent(ref a) => a.score,
            GO::Tile(ref t) => t.score,
            _ => 0,
        }
    }
}

pub type Object = Rc<RefCell<GO>>;
