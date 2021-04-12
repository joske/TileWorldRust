use std::cell::RefCell;
use std::rc::Rc;
use super::grid::Direction;
use super::grid::Grid;
use super::grid::Location;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Eq, Hash, Clone)]
struct Node {
    location: Location,
    fscore: u32,
    path : Vec<Direction>,
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

pub fn astar(reference: Rc<RefCell<Grid>>, from: Location, to: Location) -> Option<Vec<Direction>> {
    let grid = reference.borrow();
    let mut open_list : BinaryHeap<Rc<RefCell<Reverse<Node>>>> = BinaryHeap::new();
    let mut closed_list: HashSet<Rc<Location>> = HashSet::new();
    let from_node = Node {
        location: from,
        fscore: 0,
        path : Vec::new(),
    };
    open_list.push(Rc::new(RefCell::new(Reverse(from_node))));
    while let Some(current_node) = open_list.pop() {
        let ref cur_node = *current_node.borrow();
        let cur_location = cur_node.0.location;
        if cur_location == to {
            return Some(cur_node.0.path.clone());
        }
        closed_list.insert(Rc::new(cur_location));
        'outer: 
        for d in [Direction::Up, Direction::Down, Direction::Left, Direction::Right,].iter()
        {
            if cur_location.is_valid(*d) {
                let next_location = cur_location.next_location(*d);
                if next_location == to || grid.is_free(&next_location) {
                    let h = next_location.distance(to);
                    let g = cur_node.0.path.len() as u32;
                    let mut new_path = cur_node.0.path.clone();
                    new_path.push(*d);
                    let child = Node {
                        location: next_location,
                        path : new_path,
                        fscore: g + h,
                    };
                    if !closed_list.contains(&next_location) {
                        for i in open_list.iter() {
                            let n = &i.borrow().0;
                            if n.location == child.location && n.fscore < child.fscore {
                                continue 'outer;
                            }
                        }
                        open_list.push(Rc::new(RefCell::new(Reverse(child))));
                    }
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let grid = Grid::new();
        let from = Location { col: 0, row: 0 };
        let to = Location { col: 1, row: 1 };
        let path = astar(Rc::new(RefCell::new(grid)), from, to);
        let p = path.unwrap();
        assert_eq!(p.len(), 2);
        assert_eq!(p[0], Direction::Down);
        assert_eq!(p[1], Direction::Right);
    }

    #[test]
    fn test_path2() {
        let grid = Grid::new();
        let from = Location { col: 0, row: 0 };
        let to = Location { col: 0, row: 1 };
        let path = astar(Rc::new(RefCell::new(grid)), from, to);
        let p = path.unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0], Direction::Down);
    }

    #[test]
    fn test_path3() {
        let grid = Grid::new();
        let from = Location { col: 0, row: 0 };
        let to = Location { col: 2, row: 2 };
        let path = astar(Rc::new(RefCell::new(grid)), from, to);
        let p = path.unwrap();
        assert_eq!(p.len(), 4);
        assert_eq!(p[0], Direction::Down);
        assert_eq!(p[1], Direction::Down);
        assert_eq!(p[2], Direction::Right);
        assert_eq!(p[3], Direction::Right);
    }

    #[test]
    fn test_path_obstacle() {
        let mut grid = Grid::new();
        let from = Location { col: 0, row: 0 };
        let to = Location { col: 1, row: 1 };
        let obst_location = Location{ col: 1, row: 0};
        let obst = crate::grid::GridObject { 
            id :0,
            object_type : crate::grid::Type::Obstacle,
            location : obst_location,
            score : 0,
            has_tile : false,
            state : crate::grid::State::Idle,
            tile : None,
            hole : None,  
        };
        grid.set_object(Rc::new(RefCell::new(obst)), &obst_location);
        let path = astar(Rc::new(RefCell::new(grid)), from, to);
        let p = path.unwrap();
        assert_eq!(p.len(), 2);
        assert_eq!(p[0], Direction::Down);
        assert_eq!(p[1], Direction::Right);
    }

    #[test]
    fn test_big_grid() {
        let grid = Grid::new();
        let from = Location { col: 0, row: 0 };
        let to = Location { col: 9, row: 9 };
        let path = astar(Rc::new(RefCell::new(grid)), from, to);
        assert!(path.is_some());
        println!("{:?}", path);
        let p = path.unwrap();
        assert_eq!(p.len(), 18);
    }
}
