mod grid;
mod astar;

fn main() {
    let mut grid = grid::Grid::new();
    grid.init();
    grid.print();
    grid.update();
    grid.print();
}
