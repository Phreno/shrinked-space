mod args;
mod grid;

fn main() {
    // update the conf with the command line arguments
    let conf = args::parse_args_or_default();
    let output = conf.output.clone();
    // make a grid
    let grid = grid::get_new_grid(conf);
    let svg = grid::svg::to_svg(grid);
    // write to file
    std::fs::write(output, svg).expect("Unable to write file");
}
