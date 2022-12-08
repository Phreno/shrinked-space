mod grid;
use grid::*;
fn main() {
    // make a grid
    let grid = grid::get_new_grid(50);
    let svg = grid::to_svg(grid);
    // write to file
    std::fs::write("grid.svg", svg).expect("Unable to write file");
}
