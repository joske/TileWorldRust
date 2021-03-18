mod grid;

use grid::Grid;

fn main() {
    let mut grid = Grid::new();
    grid.init();
    grid.print();
}
