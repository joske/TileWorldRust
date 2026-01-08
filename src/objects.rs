use crate::{grid::Grid, location::{Direction, Location}};
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
    // Cached path to current target (avoids recalculating A* every frame)
    cached_path: Vec<Direction>,
    // Location of target when path was calculated (to detect if target moved)
    cached_target_loc: Option<Location>,
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
    for tile_ref in collection {
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
            /// Move towards the destination using cached path when possible.
            /// Only recalculates path when:
            /// - No cached path exists
            /// - Target has moved (was collected by another agent)
            /// - Path is blocked
            fn [<move_to_ $dest>](&mut self, g: &mut Grid, go: Object, tiles: &[Object], holes: &[Object]) {
                let list = if stringify!($dest) == "hole" { holes } else { tiles };
                let agent_location = self.location;

                // Get or update target
                let target = if let Some(ref cached) = self.$dest {
                    // Check if cached target is still the closest (it might have moved)
                    let cached_loc = *cached.borrow().location();
                    if Some(cached_loc) != self.cached_target_loc {
                        // Target moved, need to find new closest and recalculate path
                        self.cached_path.clear();
                        self.cached_target_loc = None;
                    }
                    Rc::clone(cached)
                } else {
                    // No cached target, find closest
                    let best = get_closest(list, agent_location).unwrap();
                    self.$dest = Some(best.clone());
                    self.cached_path.clear();
                    best
                };

                let target_loc = *target.borrow().location();

                // Check if we've arrived
                if agent_location == target_loc {
                    self.cached_path.clear();
                    self.cached_target_loc = None;
                    self.$arrived(g, go, tiles, holes, agent_location, target);
                    return;
                }

                // Use cached path or calculate new one
                if self.cached_path.is_empty() || self.cached_target_loc != Some(target_loc) {
                    if let Some(path) = crate::astar::astar(g, agent_location, target_loc) {
                        self.cached_path = path;
                        self.cached_target_loc = Some(target_loc);
                    } else {
                        // No path found, clear cache and try again next frame
                        self.cached_path.clear();
                        self.cached_target_loc = None;
                        return;
                    }
                }

                // Follow the cached path
                if !self.cached_path.is_empty() {
                    let next_direction = self.cached_path[0];
                    let next_location = agent_location.next_location(next_direction);
                    debug!("next location: {next_location:?}");

                    if g.is_free(next_location) || next_location == target_loc {
                        debug!("allowed, moving");
                        self.cached_path.remove(0);
                        self.location = next_location;
                        g.move_object(go, agent_location, next_location);
                    } else {
                        // Path is blocked, recalculate next frame
                        debug!("blocked, will recalculate");
                        self.cached_path.clear();
                        self.cached_target_loc = None;
                    }
                }
            }
        }
    }
}

impl AgentState {
    pub fn new(location: Location, id: u8) -> Self {
        AgentState {
            location,
            id,
            score: 0,
            hole: None,
            tile: None,
            has_tile: false,
            state: State::Idle,
            cached_path: Vec::new(),
            cached_target_loc: None,
        }
    }

    /// Clear cached path when changing targets
    fn clear_path_cache(&mut self) {
        self.cached_path.clear();
        self.cached_target_loc = None;
    }

    pub fn update(&mut self, g: &mut Grid, go: Object, tiles: &[Object], holes: &[Object]) {
        debug!("agent {self:?}");
        match self.state {
            State::Idle => self.idle(tiles),
            State::MoveToTile => self.move_to_tile(g, go, tiles, holes),
            State::MoveToHole => self.move_to_hole(g, go, tiles, holes),
        }
    }

    fn idle(&mut self, tiles: &[Object]) {
        let agent_location = self.location;
        debug!("current location: {agent_location:?}");
        if let Some(best_tile) = get_closest(tiles, agent_location) {
            debug!("best tile: {best_tile:?}");
            self.tile = Some(Rc::clone(&best_tile));
            self.clear_path_cache(); // New target, clear cached path
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
        self.clear_path_cache(); // New target, clear cached path
        if let Some(best_hole) = get_closest(holes, agent_location) {
            self.hole = Some(Rc::clone(&best_hole));
            self.state = State::MoveToHole;
        }
        // Teleport the tile to a new random location (respawn)
        if let Some(new_location) = g.random_location() {
            best_tile.borrow_mut().set_location(new_location);
            g.move_object(best_tile, agent_location, new_location);
        }
        self.location = agent_location;
        g.move_object(go, agent_location, agent_location);
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
        if let Some(t) = &self.tile.clone()
            && let GO::Tile(ref tstate) = *t.borrow()
        {
            self.score += tstate.score;
        }
        // Teleport the hole to a new random location (respawn)
        if let Some(new_location) = g.random_location() {
            best_hole.borrow_mut().set_location(new_location);
            g.move_object(best_hole, agent_location, new_location);
        }
        self.location = agent_location;
        g.move_object(go, agent_location, agent_location);
        self.clear_path_cache(); // New target, clear cached path
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
            GO::Agent(a) => &a.location,
            GO::Tile(t) => &t.location,
            GO::Hole(h) => &h.location,
            GO::Obstacle(o) => o,
        }
    }

    fn set_location(&mut self, l: Location) {
        match self {
            GO::Agent(a) => a.location = l,
            GO::Tile(t) => t.location = l,
            GO::Hole(h) => h.location = l,
            GO::Obstacle(_) => {}
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            GO::Agent(a) => a.score,
            GO::Tile(t) => t.score,
            _ => 0,
        }
    }
}

pub type Object = Rc<RefCell<GO>>;
