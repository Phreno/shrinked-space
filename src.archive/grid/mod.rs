// import cell and grid
mod cell;
pub mod conf;
mod grid_core;
mod perspective;
pub mod svg;
use crate::grid::conf::Conf;
use crate::grid::grid_core::Grid;

pub fn get_new_grid(conf: Conf) -> Grid {
    Grid::new(conf)
}
