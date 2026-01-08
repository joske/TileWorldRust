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

    #[cfg(test)]
    pub fn add_obstacle(&mut self, location: Location) {
        let r = Rc::new(RefCell::new(GO::Obstacle(location)));
        self.objects.insert(location, r);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new_is_empty() {
        let grid = Grid::new();
        let loc = Location::new(0, 0);
        assert!(grid.is_free(loc));
        assert!(grid.object(loc).is_none());
    }

    #[test]
    fn test_add_obstacle() {
        let mut grid = Grid::new();
        let loc = Location::new(5, 5);
        assert!(grid.is_free(loc));

        grid.add_obstacle(loc);

        assert!(!grid.is_free(loc));
        assert!(grid.object(loc).is_some());
        if let GO::Obstacle(l) = *grid.object(loc).unwrap().borrow() {
            assert_eq!(l, loc);
        } else {
            panic!("Expected obstacle");
        }
    }

    #[test]
    fn test_is_free() {
        let mut grid = Grid::new();
        let loc1 = Location::new(1, 1);
        let loc2 = Location::new(2, 2);

        assert!(grid.is_free(loc1));
        assert!(grid.is_free(loc2));

        grid.add_obstacle(loc1);

        assert!(!grid.is_free(loc1));
        assert!(grid.is_free(loc2));
    }

    #[test]
    fn test_move_object() {
        let mut grid = Grid::new();
        let old_loc = Location::new(1, 1);
        let new_loc = Location::new(2, 2);

        grid.add_obstacle(old_loc);
        let obj = grid.object(old_loc).unwrap().clone();

        grid.move_object(obj, old_loc, new_loc);

        assert!(grid.is_free(old_loc));
        assert!(!grid.is_free(new_loc));
    }

    #[test]
    fn test_random_location_returns_free_location() {
        let mut grid = Grid::new();
        // Add some obstacles
        for i in 0..10 {
            grid.add_obstacle(Location::new(i, 0));
        }

        // random_location should return a free location
        let loc = grid.random_location();
        assert!(grid.is_free(loc));
    }

    #[test]
    fn test_create_objects_counts() {
        let mut grid = Grid::new();
        let (agents, tiles, holes) = grid.create_objects(3, 5, 4, 2);

        assert_eq!(agents.len(), 3);
        assert_eq!(tiles.len(), 5);
        assert_eq!(holes.len(), 4);
        // Total objects: 3 + 5 + 4 + 2 = 14
    }

    #[test]
    fn test_create_objects_agent_ids() {
        let mut grid = Grid::new();
        let (agents, _, _) = grid.create_objects(3, 0, 0, 0);

        for (i, agent) in agents.iter().enumerate() {
            if let GO::Agent(ref a) = *agent.borrow() {
                assert_eq!(a.id, (i + 1) as u8);
                assert_eq!(a.score, 0);
                assert!(!a.has_tile);
                assert_eq!(a.state, State::Idle);
            } else {
                panic!("Expected agent");
            }
        }
    }

    #[test]
    fn test_create_objects_tiles_have_scores() {
        let mut grid = Grid::new();
        let (_, tiles, _) = grid.create_objects(0, 10, 0, 0);

        for tile in &tiles {
            if let GO::Tile(ref t) = *tile.borrow() {
                assert!(t.score >= 1 && t.score <= 5);
            } else {
                panic!("Expected tile");
            }
        }
    }

    #[test]
    fn test_create_objects_all_unique_locations() {
        let mut grid = Grid::new();
        let (agents, tiles, holes) = grid.create_objects(5, 5, 5, 5);

        let mut locations = std::collections::HashSet::new();

        for agent in &agents {
            locations.insert(*agent.borrow().location());
        }
        for tile in &tiles {
            locations.insert(*tile.borrow().location());
        }
        for hole in &holes {
            locations.insert(*hole.borrow().location());
        }

        // All objects should have unique locations (20 total, not counting obstacles in returned vecs)
        assert_eq!(locations.len(), 15); // 5 agents + 5 tiles + 5 holes
    }
}
