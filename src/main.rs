use conf::getters::ConfGetter;
use grid::Grid;
mod conf;
mod grid;
fn main() {
    let grid = Grid::from(conf::get_conf());
    let svg = grid::svg::to_svg(grid);
    std::fs::write(conf::get_conf().get_output(), svg).expect("Unable to write file");
}
