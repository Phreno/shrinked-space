pub fn render_polyline(svg: &mut String, row: Vec<nalgebra::OPoint<f64, nalgebra::Const<2>>>) {
    svg.push_str("<polyline points=\"");
    for point in row {
        svg.push_str(&format!("{},{} ", point.x, point.y));
    }
    svg.push_str("\"  />");
}

pub fn render_bezier_cubic(svg: &mut String, row: Vec<nalgebra::OPoint<f64, nalgebra::Const<2>>>) {
    svg.push_str("<path d=\"");

    let mut first = true;
    for point in row {
        if first {
            svg.push_str(&format!("M{},{} ", point.x, point.y));
            first = false;
        } else {
            svg.push_str(&format!("L{},{} ", point.x, point.y));
        }
    }
    svg.push_str("\"  />");
}
