use super::{
    grid::Grid,
    location::{Direction, Location},
};
use priority_queue::PriorityQueue;
use std::{
    cmp::{Ordering, Reverse},
    collections::HashSet,
    hash::{Hash, Hasher},
};

#[derive(Debug, Eq, Clone)]
struct Node {
    location: Location,
    fscore: u16,
    path: Vec<Direction>,
}

impl Node {
    fn new(l: Location, f: u16, p: Vec<Direction>) -> Node {
        Node {
            location: l,
            fscore: f,
            path: p,
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.hash(state);
        self.fscore.hash(state);
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fscore.cmp(&other.fscore)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.fscore.cmp(&other.fscore))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

pub(crate) fn astar(grid: &Grid, from: Location, to: Location) -> Option<Vec<Direction>> {
    let mut open_list: PriorityQueue<Node, Reverse<u16>> = PriorityQueue::new();
    let mut closed_list: HashSet<Location> = HashSet::new();
    let from_node = Node::new(from, 0, Vec::new());
    open_list.push(from_node, Reverse(0));
    while let Some(current_node) = open_list.pop() {
        // this should be the most promising path to the destination
        let cur_node = &current_node.0;
        let cur_location = cur_node.location;
        if cur_location == to {
            // if the cur_location is the destination, we're guaranteed to have found the /best/ path
            return Some(cur_node.path.clone());
        }
        closed_list.insert(cur_location);
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if cur_location.is_valid_move(d) {
                let next_location = cur_location.next_location(d);
                if next_location == to || grid.is_free(next_location) {
                    let h = next_location.distance(to);
                    let g = cur_node.path.len() as u16 + 1;
                    let mut new_path = cur_node.path.clone();
                    new_path.push(d);
                    let child = Node::new(next_location, g + h, new_path);
                    if !closed_list.contains(&next_location)
                        && !open_list
                            .iter()
                            .any(|n| n.0.location == child.location && n.0.fscore < child.fscore)
                    {
                        // this is now the best way to reach next location
                        open_list.push(child, Reverse(g + h));
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    fn test_path() {
        let grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(1, 1);
        let path = astar(&grid, from, to);
        let p = path.unwrap();
        assert_eq!(p.len(), 2);
        assert_eq!(p[0], Direction::Down);
        assert_eq!(p[1], Direction::Right);
    }

    #[test]
    fn test_path2() {
        let grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(0, 1);
        let path = astar(&grid, from, to);
        let p = path.unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0], Direction::Down);
    }

    #[test]
    fn test_path3() {
        let grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(2, 2);
        let path = astar(&grid, from, to);
        let p = path.unwrap();
        debug!("{:?}", p);
        assert_eq!(p.len(), 4);
        assert_eq!(p[0], Direction::Down);
        assert_eq!(p[1], Direction::Right);
        assert_eq!(p[2], Direction::Right);
        assert_eq!(p[3], Direction::Down);
    }

    // #[test]
    // fn test_path_obstacle() {
    //     let mut grid = Grid::new();
    //     let from = Location::new(0, 0);
    //     let to = Location::new(1, 1);
    //     let obst_location = Location { col: 1, row: 0 };
    //     let obst = crate::grid::GridObject {
    //         id: 0,
    //         object_type: crate::grid::Type::Obstacle,
    //         location: obst_location,
    //         score: 0,
    //         has_tile: false,
    //         state: crate::grid::State::Idle,
    //         tile: None,
    //         hole: None,
    //     };
    //     grid.set_object(Rc::new(RefCell::new(obst)), &obst_location, &obst_location);
    //     let path = astar(Rc::new(RefCell::new(grid)), from, to);
    //     let p = path.unwrap();
    //     assert_eq!(p.len(), 2);
    //     assert_eq!(p[0], Direction::Down);
    //     assert_eq!(p[1], Direction::Right);
    // }

    #[test]
    fn test_big_grid() {
        let grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(9, 9);
        let path = astar(&grid, from, to);
        assert!(path.is_some());
        let p = path.unwrap();
        assert_eq!(p.len(), 18);
    }

    #[test]
    fn test_can_not_reach() {
        let grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(100, 100); // these are outside of the grid, no way to find a path
        let path = astar(&grid, from, to);
        assert!(path.is_none());
    }
}
