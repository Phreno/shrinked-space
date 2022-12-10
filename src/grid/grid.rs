// import cell
use crate::grid::cell::Cell;
extern crate perlin;
use perlin::Perlin;
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
    perlin: Perlin,
}

impl Grid {
    /// Creates a new [`Grid`].
    pub(crate) fn new(width: i32, height: i32) -> Self {
        let cells = render_grid(width, height);
        let perlin = Perlin::new();
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

    fn get_cell_index(&self, y: i32, x: i32) -> usize {
        (y * self.width as i32 + x) as usize
    }

    pub(crate) fn perlin_noise_at(&self, cell: Cell) -> f64 {
        // compute the noise value from x and y
        let noise = perlin::noise2(cell.x, cell.y);
    }
}

fn render_grid(width: i32, height: i32) -> Vec<Cell> {
    let mut cells = vec![];
    for x in 0..width {
        for y in 0..height {
            cells.push(Cell::new(x, y));
        }
    }
    cells
}

#[cfg(test)]
mod tests {
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
        let value = grid.perlin_noise_at(0.0, 0.0);
        assert_eq!(value, 0.0);
    }
}
