fn rotate_x(point: OPoint<f64, Const<3>>, pi: f64) -> OPoint<f64, Const<3>> {
    let x = point.x;
    let y = point.y * pi.cos() - point.z * pi.sin();
    let z = point.y * pi.sin() + point.z * pi.cos();
    Point3::new(x, y, z)
}

fn rotate_y(point: OPoint<f64, Const<3>>, pi: f64) -> OPoint<f64, Const<3>> {
    let x = point.x * pi.cos() + point.z * pi.sin();
    let y = point.y;
    let z = -point.x * pi.sin() + point.z * pi.cos();
    Point3::new(x, y, z)
}

fn rotate_z(point: OPoint<f64, Const<3>>, pi: f64) -> OPoint<f64, Const<3>> {
    let x = point.x * pi.cos() - point.y * pi.sin();
    let y = point.x * pi.sin() + point.y * pi.cos();
    let z = point.z;
    Point3::new(x, y, z)
}
