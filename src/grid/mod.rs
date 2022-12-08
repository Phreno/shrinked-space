// import cell and grid
mod cell;
pub mod conf;
mod grid_core;
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
        1.0,
        0,
        1.0,
        (1.0, 2.0),
        1.0,
        "output.svg".to_string(),
    )
}

pub fn to_svg(grid: Grid) -> String {
    let mut svg = String::new();

    // the svg has a white background
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100%\" height=\"100%\">");
    // do a group for rows

    // do a white background
    //svg.push_str("<rect width=\"100%\" height=\"100%\" style=\"fill:white;stroke-width:0\" />");
    svg.push_str("<g style=\"fill: white; background-color: white;\">");
    svg.push_str("<g style=\"fill:none;stroke:black;stroke-width:2\">");
    for row in grid.get_rows() {
        // for each row, link all the points inside a polyline
        svg.push_str("<polyline points=\"");
        for point in row {
            svg.push_str(&format!("{},{} ", point.x, point.y));
        }
        svg.push_str("\"  />");
    }
    svg.push_str("</g>");

    // do a group for lines
    svg.push_str("<g style=\"fill:none;stroke:black;stroke-width:1\">");
    for line in grid.get_lines() {
        // for each row, link all the points inside a polyline
        svg.push_str("<polyline points=\"");
        for point in line {
            svg.push_str(&format!("{},{} ", point.x, point.y));
        }
        svg.push_str("\" />");
    }
    svg.push_str("</g>");

    svg.push_str("</g>");
    svg.push_str("</svg>");
    svg
}

// Tests
#[cfg(test)]
mod tests {
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
