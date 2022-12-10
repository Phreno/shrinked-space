// import cell and grid
mod cell;
pub mod conf;
mod grid_core;
pub mod svg;
use crate::grid::conf::Conf;
use crate::grid::grid_core::Grid;

pub fn get_new_grid(conf: Conf) -> Grid {
    Grid::new(conf)
}

pub fn default_conf() -> Conf {
    Conf::new(
        50,
        1,
        1.0,
        1.0,
        1.0,
        0.05,
        0,
        1.0,
        (1.0, 2.0),
        1.0,
        "output.svg".to_string(),
    )
}

// Tests
#[cfg(test)]
mod tests {
    use crate::grid::cell::Cell;

    use super::*;
    #[test]
    fn new_should_have_a_constructor() {
        let grid = Grid::new(default_conf());
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
    }

    #[test]
    fn get_should_return_a_cell() {
        let grid = Grid::new(default_conf());
        let cell = grid.get(0, 0);
        assert_eq!(cell, Cell::new(0, 0));
        assert_eq!(cell.x, 0);
        assert_eq!(cell.y, 0);
    }
}
