use super::{
    grid::Grid,
    location::{Direction, Location},
};
use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap};

/// Reconstructs the path by walking back through the came_from map
fn reconstruct_path(
    came_from: &HashMap<Location, (Location, Direction)>,
    mut current: Location,
    start: Location,
) -> Vec<Direction> {
    let mut path = Vec::new();
    while current != start {
        if let Some(&(parent, direction)) = came_from.get(&current) {
            path.push(direction);
            current = parent;
        } else {
            break;
        }
    }
    path.reverse();
    path
}

pub(crate) fn astar(grid: &Grid, from: Location, to: Location) -> Option<Vec<Direction>> {
    // Early return if start == goal
    if from == to {
        return Some(Vec::new());
    }

    // Priority queue: (Reverse(f_score), g_score, location)
    // Using Reverse for min-heap behavior
    let mut open_heap: BinaryHeap<(Reverse<u16>, u16, Location)> = BinaryHeap::new();

    // Maps location -> best known g_score (O(1) lookup)
    let mut g_scores: HashMap<Location, u16> = HashMap::new();

    // Maps location -> (parent_location, direction_taken)
    let mut came_from: HashMap<Location, (Location, Direction)> = HashMap::new();

    // Initialize with start node
    g_scores.insert(from, 0);
    let h = from.distance(to);
    open_heap.push((Reverse(h), 0, from));

    while let Some((_, current_g, current_loc)) = open_heap.pop() {
        // Check if we've reached the goal
        if current_loc == to {
            return Some(reconstruct_path(&came_from, to, from));
        }

        // Skip if we've already found a better path to this node
        if let Some(&best_g) = g_scores.get(&current_loc) {
            if current_g > best_g {
                continue;
            }
        }

        // Explore neighbors
        for d in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if current_loc.is_valid_move(d) {
                let next_loc = current_loc.next_location(d);

                // Check if passable (or is the destination)
                if next_loc == to || grid.is_free(next_loc) {
                    let tentative_g = current_g + 1;

                    // Only proceed if this is a better path
                    let dominated = g_scores
                        .get(&next_loc)
                        .is_some_and(|&best| tentative_g >= best);
                    if !dominated {
                        // This is the best path to next_loc so far
                        g_scores.insert(next_loc, tentative_g);
                        came_from.insert(next_loc, (current_loc, d));

                        let h = next_loc.distance(to);
                        let f = tentative_g + h;
                        open_heap.push((Reverse(f), tentative_g, next_loc));
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

    /// Helper to verify a path leads from start to end
    fn verify_path(start: Location, end: Location, path: &[Direction]) -> bool {
        let mut current = start;
        for &d in path {
            current = current.next_location(d);
        }
        current == end
    }

    #[test]
    fn test_path() {
        let grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(1, 1);
        let path = astar(&grid, from, to);
        let p = path.unwrap();
        assert_eq!(p.len(), 2); // Optimal path length
        assert!(verify_path(from, to, &p));
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
        assert_eq!(p.len(), 4); // Optimal path length
        assert!(verify_path(from, to, &p));
    }

    #[test]
    fn test_path_around_obstacle() {
        let mut grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(1, 1);
        // Place obstacle at (1, 0), blocking the direct right path
        grid.add_obstacle(Location::new(1, 0));
        let path = astar(&grid, from, to);
        let p = path.unwrap();
        assert_eq!(p.len(), 2); // Still optimal length
        assert!(verify_path(from, to, &p));
    }

    #[test]
    fn test_path_around_multiple_obstacles() {
        let mut grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(2, 0);
        // Block the direct path at (1, 0)
        grid.add_obstacle(Location::new(1, 0));
        let path = astar(&grid, from, to);
        let p = path.unwrap();
        // Must go around the obstacle
        assert_eq!(p.len(), 4);
        assert!(verify_path(from, to, &p));
    }

    #[test]
    fn test_path_blocked_completely() {
        let mut grid = Grid::new();
        let from = Location::new(1, 1);
        let to = Location::new(1, 3);
        // Surround the target with obstacles
        grid.add_obstacle(Location::new(0, 3));
        grid.add_obstacle(Location::new(2, 3));
        grid.add_obstacle(Location::new(1, 2));
        grid.add_obstacle(Location::new(1, 4));
        let path = astar(&grid, from, to);
        assert!(path.is_none());
    }

    #[test]
    fn test_same_location() {
        let grid = Grid::new();
        let loc = Location::new(5, 5);
        let path = astar(&grid, loc, loc);
        let p = path.unwrap();
        assert!(p.is_empty());
    }

    #[test]
    fn test_big_grid() {
        let grid = Grid::new();
        let from = Location::new(0, 0);
        let to = Location::new(9, 9);
        let path = astar(&grid, from, to);
        assert!(path.is_some());
        let p = path.unwrap();
        assert_eq!(p.len(), 18);
        assert!(verify_path(from, to, &p));
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
