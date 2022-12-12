use nalgebra::{Const, OPoint, Point2, Point3};
use perlin2d::PerlinNoise2D;

use super::{cell::Cell, conf::Conf};

// Define a static perlin noise generator
static mut PERLIN: Option<PerlinNoise2D> = None;

pub fn set_perlin_noise(conf: Conf) {
    let perlin = to_perlin_noise(&conf);
    unsafe {
        PERLIN = Some(perlin);
    }
}

pub fn to_3_points_perspective(cell: Cell) -> Point2<f64> {
    let point = get_3d_point_at(cell);
    get_3_points_perspective_at(point)
}

fn get_3d_point_at(get: Cell) -> OPoint<f64, Const<3>> {
    Point3::new(get.get_x_as_f64(), get.get_y_as_f64(), perlin_noise_at(get))
}

fn get_3_points_perspective_at(point: OPoint<f64, Const<3>>) -> Point2<f64> {
    // rotate the point around the x axis
    //let point = rotate_z(point, std::f64::consts::PI / 4.0);

    //let vx = point.x * 0.8;
    //let vy = point.y / (point.z * 0.2);
    //let px = vx / (point.z / 10.0);
    //let py = vy / (point.z / 10.0);

    let px = point.x / (point.z / 10.0);
    let py = point.y / (point.z / 10.0);
    // Return the projected coordinates
    Point2::new(px, py)
}

fn perlin_noise_at(cell: Cell) -> f64 {
    // Get the perlin noise generator
    let perlin = unsafe { PERLIN.as_ref().unwrap() };
    perlin.get_noise(cell.get_x_as_f64(), cell.get_y_as_f64())
}

fn to_perlin_noise(conf: &Conf) -> PerlinNoise2D {
    let perlin = PerlinNoise2D::new(
        conf.octaves,
        conf.amplitude,
        conf.frequency,
        conf.persistence,
        conf.lacunarity,
        conf.scale,
        conf.bias,
        conf.seed,
    );
    perlin
}
