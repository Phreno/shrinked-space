// import cell and grid
mod cell;
mod grid;
use crate::grid::cell::Cell;
use crate::grid::grid::Grid;

pub fn get_new_grid(size: i32) -> Grid {
    Grid::new(size, size)
}

pub fn to_svg(grid: Grid) -> String {
    let mut svg = String::new();
    svg.push_str(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"100%\" height=\"100%\">",
    );
    for line in grid.get_lines() {
        svg.push_str("<path d=\"");
        for point in line {
            svg.push_str(&format!("M{} {} ", point.x, point.y));
        }
        svg.push_str("\" stroke=\"black\" stroke-width=\"1\" fill=\"none\" />");
    }
    svg.push_str("</svg>");
    svg
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_should_have_a_constructor() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
    }

    #[test]
    fn get_should_return_a_cell() {
        let grid = Grid::new(10, 10);
        let cell = grid.get(0, 0);
        assert_eq!(cell, Cell::new(0, 0));
        assert_eq!(cell.x, 0);
        assert_eq!(cell.y, 0);
    }
}
