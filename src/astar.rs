use super::grid::Direction;
use super::grid::Grid;
use super::grid::Location;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

#[derive(Eq, Hash, Clone)]
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

pub fn astar(grid: Grid, from: Location, to: Location) -> Option<Vec<Direction>> {
    let mut open_list = BinaryHeap::new();
    let mut closed_list: HashSet<Node> = HashSet::new();
    let from_node = Node {
        location: from,
        fscore: 0,
        path : Vec::new(),
    };
    open_list.push(Reverse(from_node));
    while let Some(current_node) = open_list.pop() {
        if current_node.0.location == to {
            return Some(current_node.0.path);
        }
        closed_list.insert(current_node.0);
        for d in [Direction::Up, Direction::Down, Direction::Left, Direction::Right,].iter()
        {
            if current_node.0.location.is_valid(d.clone()) {
                let next_location = current_node.0.location.next_location(d.clone());
                if next_location == to || grid.is_free(next_location) {
                    let h = next_location.distance(to);
                    let g = next_location.distance(from);
                    let mut new_path = current_node.0.path.clone();
                    new_path.insert(0, d.clone());
                    let child = Node {
                        location: next_location,
                        path : new_path,
                        fscore: g + h,
                    };
                    if !closed_list.contains(&child) {
                        for i in open_list.iter() {
                            if i == &Reverse(child) {
                                continue;
                            }
                        }
                        open_list.push(Reverse(child));
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
        let path = astar(grid, from, to);
        assert_eq!(path.unwrap().len(), 2);
    }
}
