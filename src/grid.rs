mod cell;
mod perspective;
pub mod svg;
use crate::{
    conf::{getters::ConfGetter, Conf},
    grid::cell::Cell,
};
use nalgebra::{Const, OPoint};

use self::perspective::{set_perlin_noise, to_3_points_perspective};

pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
}

impl From<Conf> for Grid {
    fn from(conf: Conf) -> Self {
        let width = conf.get_size();
        let height = conf.get_size();
        let cells = render_grid(conf.get_size(), conf.get_size());
        set_perlin_noise(conf);
        Grid {
            width,
            height,
            cells,
        }
    }
}

impl Grid {
    /// Creates a new [`Grid`].
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
