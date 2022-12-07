// import cell
use crate::grid::cell::Cell;
use nalgebra::{Const, Matrix3, OPoint, Point2, Point3, Vector3};
use perlin2d::PerlinNoise2D;
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
    perlin: PerlinNoise2D,
}

impl Grid {
    /// Creates a new [`Grid`].
    pub(crate) fn new(width: i32, height: i32) -> Self {
        let cells = render_grid(width, height);
        let perlin = PerlinNoise2D::new(8, 1.0, 2.0, 0.5, 0.7, (1.0, 0.2), 3.0, 0);
        Grid {
            width: width as u32,
            height: height as u32,
            cells,
            perlin,
        }
    }

    pub(crate) fn get(&self, x: i32, y: i32) -> Cell {
        self.cells[self.get_cell_index(y, x)]
    }

    pub(crate) fn to_3_points_perspective(&self, get: Cell) -> Point2<f64> {
        let point = self.get_3d_point_at(get);
        self.get_3_points_perspective_at(point)
    }

    fn get_cell_index(&self, y: i32, x: i32) -> usize {
        (y * self.width as i32 + x) as usize
    }

    fn perlin_noise_at(&self, cell: Cell) -> f64 {
        let noise = self
            .perlin
            .get_noise(cell.get_x_as_f64(), cell.get_y_as_f64());
        noise
    }

    fn get_3d_point_at(&self, get: Cell) -> OPoint<f64, Const<3>> {
        let point = Point3::new(
            get.get_x_as_f64(),
            get.get_y_as_f64(),
            self.perlin_noise_at(get),
        );
        point
    }

    fn get_3_points_perspective_at(&self, point: OPoint<f64, Const<3>>) -> Point2<f64> {
        let vx = point.x / 2.0;
        let vy = point.y / (point.z / 10.0);

        // Calculate the coordinates of the projected cube
        let px = point.x / (point.z / 10.0);
        let py = point.y / (point.z / 10.0);

        // Return the projected coordinates
        Point2::new(px, py)
    }
}

fn render_grid(width: i32, height: i32) -> Vec<Cell> {
    let mut cells = vec![];
    for y in 0..width {
        for x in 0..height {
            cells.push(Cell::new(x, y));
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;

    use super::*;

    #[test]
    fn new_should_have_a_constructor() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100);
    }

    #[test]
    fn get_should_return_a_cell() {
        let grid = Grid::new(10, 10);
        let cell = grid.get(0, 0);
        assert_eq!(cell, Cell::new(0, 0));
        assert_eq!(cell.x, 0);
        assert_eq!(cell.y, 0);
    }

    #[test]
    fn perlin_noise_at_should_return_a_value() {
        let grid = Grid::new(10, 10);
        let value = grid.perlin_noise_at(grid.get(0, 1));
        assert_eq!(value, 2.890339412503229);
    }

    #[test]
    fn get_3d_point_at_should_return_a_3Dpoint() {
        let grid = Grid::new(10, 10);
        let point = grid.get_3d_point_at(grid.get(0, 1));
        assert_eq!(point, Point3::new(0.0, 1.0, 2.890339412503229));
    }

    #[test]
    fn get_3_points_perspective_at_should_return_a_2Dpoint() {
        let grid = Grid::new(10, 10);
        let point = grid.get_3d_point_at(grid.get(0, 1));
        let projection = grid.get_3_points_perspective_at(point);
        assert_eq!(projection, Point2::new(0.0, 3.4598012803414413));
    }

    #[test]
    fn to_3_points_perspective_should_return_a_2Dpoint() {
        let grid = Grid::new(10, 10);
        let projection = grid.to_3_points_perspective(grid.get(0, 1));
        assert_eq!(projection, Point2::new(0.0, 3.4598012803414413));
    }
}
