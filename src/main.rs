mod grid;

use grid::build_grid;

fn main() {
    let mut grid = build_grid();
    grid.init();
    grid.print();
}
