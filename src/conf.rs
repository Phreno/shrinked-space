mod builder;
mod checker;
mod default;
mod from_argmatches;
pub mod getters;
mod matches;

#[derive(Debug, Clone)]
pub struct Conf {
    size: i32,
    octaves: i32,
    persistence: f64,
    lacunarity: f64,
    frequency: f64,
    amplitude: f64,
    seed: i32,
    zoom: f64,
    scale: (f64, f64),
    bias: f64,
    output: String,
}

pub fn get_conf() -> Conf {
    Conf::from(matches::get_matches())
}
