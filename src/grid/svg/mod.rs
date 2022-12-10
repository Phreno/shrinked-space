mod grid_renderer;
use grid_renderer::render_bezier_cubic;

use super::grid_core::Grid;

pub fn to_svg(grid: Grid) -> String {
    let mut svg = String::new();
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100%\" height=\"100%\" viewBox=\"0 0 1169 1654\">");
    append_content(&mut svg, grid);
    svg.push_str("</svg>");
    svg
}

fn append_content(svg: &mut String, grid: Grid) {
    svg.push_str("<g style=\"fill: white; background-color: white;\">");
    append_rows(svg, &grid);
    append_lines(svg, &grid);
    svg.push_str("</g>");
}

fn append_rows(svg: &mut String, grid: &Grid) {
    svg.push_str("<g style=\"fill:none;stroke:black;stroke-width:2\">");
    for row in grid.get_rows() {
        render_bezier_cubic(svg, row);
    }
    svg.push_str("</g>");
}

fn append_lines(svg: &mut String, grid: &Grid) {
    svg.push_str("<g style=\"fill:none;stroke:black;stroke-width:1\">");
    for line in grid.get_lines() {
        render_bezier_cubic(svg, line);
    }
    svg.push_str("</g>");
}
