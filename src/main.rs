mod grid;

use grid::Grid;

fn main() {
    let grid = Grid {
        agents: Vec::with_capacity(5),
        tiles: Vec::with_capacity(5),
        holes: Vec::with_capacity(5),
        obstacles: Vec::with_capacity(5),
        objects : [[None; 5]; 5],
    };
}
