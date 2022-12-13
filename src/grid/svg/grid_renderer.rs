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
