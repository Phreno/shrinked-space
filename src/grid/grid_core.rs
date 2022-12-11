// import cell
use crate::grid::cell::Cell;
use crate::grid::conf::Conf;
use nalgebra::{Const, OPoint};

use super::perspective::{set_perlin_noise, to_3_points_perspective};

pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
}

impl Grid {
    /// Creates a new [`Grid`].
    pub(crate) fn new(conf: Conf) -> Self {
        let width = conf.size;
        let height = conf.size;
        let cells = render_grid(conf.size, conf.size);
        set_perlin_noise(conf);
        Grid {
            width,
            height,
            cells,
        }
    }

    pub(crate) fn get(&self, x: i32, y: i32) -> Cell {
        self.cells[self.get_cell_index(y, x)]
    }

    fn get_cell_index(&self, y: i32, x: i32) -> usize {
        (y * self.width as i32 + x) as usize
    }

    pub(crate) fn get_lines(&self) -> Vec<Vec<OPoint<f64, Const<2>>>> {
        let walk_row = |row| {
            let walk_col = |col| to_3_points_perspective(self.get(col as i32, row as i32));
            (0..self.width).map(walk_col).collect()
        };
        (0..self.height).map(walk_row).collect()
    }

    pub(crate) fn get_rows(&self) -> Vec<Vec<OPoint<f64, Const<2>>>> {
        let walk_col = |col| {
            let walk_row = |row| to_3_points_perspective(self.get(col as i32, row as i32));
            (0..self.width).map(walk_row).collect()
        };
        (0..self.height).map(walk_col).collect()
    }
}

fn render_grid(width: i32, height: i32) -> Vec<Cell> {
    (0..width)
        .flat_map(|y| (0..height).map(move |x: i32| Cell::new(x, y)))
        .collect()
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;

    use crate::grid::default_conf;

    use super::*;

    #[test]
    fn new_should_have_a_constructor() {
        let grid = Grid::new(default_conf());
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100);
    }

    #[test]
    fn get_should_return_a_cell() {
        let grid = Grid::new(default_conf());
        let cell = grid.get(0, 0);
        assert_eq!(cell, Cell::new(0, 0));
        assert_eq!(cell.x, 0);
        assert_eq!(cell.y, 0);
    }

    #[test]
    fn perlin_noise_at_should_return_a_value() {
        let grid = Grid::new(default_conf());
        let value = grid.perlin_noise_at(grid.get(0, 1));
        assert_eq!(value, 2.890339412503229);
    }

    #[test]
    fn get_3d_point_at_should_return_a_3Dpoint() {
        let grid = Grid::new(default_conf());
        let point = grid.get_3d_point_at(grid.get(0, 1));
        assert_eq!(point, Point3::new(0.0, 1.0, 2.890339412503229));
    }

    #[test]
    fn get_3_points_perspective_at_should_return_a_2Dpoint() {
        let grid = Grid::new(default_conf());
        let point = grid.get_3d_point_at(grid.get(0, 1));
        let projection = grid.get_3_points_perspective_at(point);
        assert_eq!(projection, Point2::new(0.0, 3.4598012803414413));
    }

    #[test]
    fn to_3_points_perspective_should_return_a_2Dpoint() {
        let grid = Grid::new(default_conf());
        let projection = grid.to_3_points_perspective(grid.get(0, 1));
        assert_eq!(projection, Point2::new(0.0, 3.4598012803414413));
    }

    #[test]
    fn get_lines_should_return_a_vector_of_lines() {
        let grid = Grid::new(default_conf());
        let lines = grid.get_lines();
        assert_eq!(lines.len(), 10);
    }

    #[test]
    fn get_rows_should_return_a_vector_of_rows() {
        let grid = Grid::new(default_conf());
        let rows = grid.get_rows();
        assert_eq!(rows.len(), 10);
    }
}
