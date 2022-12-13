use clap::{App, ArgMatches};

use self::input::Input;
mod input;
impl From<Input<'_>> for clap::ArgMatches {
    fn from(input: Input) -> Self {
        App::new("shrinked~space")
            .arg(input.size)
            .arg(input.octaves)
            .arg(input.persistence)
            .arg(input.lacunarity)
            .arg(input.frequency)
            .arg(input.amplitude)
            .arg(input.seed)
            .arg(input.zoom)
            .arg(input.scale)
            .arg(input.bias)
            .arg(input.output)
            .get_matches()
    }
}

pub fn get_matches() -> clap::ArgMatches {
    ArgMatches::from(Input::default())
}
