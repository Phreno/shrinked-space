use clap::{App, Arg};

// import Conf
use crate::grid::{conf::Conf, default_conf};

pub fn parse_args_or_default() -> Conf {
    let mut conf = default_conf();
    let matches = App::new("My App")
        .arg(
            Arg::with_name("size")
                .short('s')
                .long("size")
                .takes_value(true)
                .help("Sets the size of the output image"),
        )
        .arg(
            Arg::with_name("octaves")
                .short('o')
                .long("octaves")
                .takes_value(true)
                .help("Sets the number of octaves"),
        )
        .arg(
            Arg::with_name("persistence")
                .short('p')
                .long("persistence")
                .takes_value(true)
                .help("Sets the persistence"),
        )
        .arg(
            Arg::with_name("lacunarity")
                .short('l')
                .long("lacunarity")
                .takes_value(true)
                .help("Sets the lacunarity"),
        )
        .arg(
            Arg::with_name("frequency")
                .short('f')
                .long("frequency")
                .takes_value(true)
                .help("Sets the frequency"),
        )
        .arg(
            Arg::with_name("amplitude")
                .short('a')
                .long("amplitude")
                .takes_value(true)
                .help("Sets the amplitude"),
        )
        .arg(
            Arg::with_name("seed")
                .short('e')
                .long("seed")
                .takes_value(true)
                .help("Sets the seed"),
        )
        .arg(
            Arg::with_name("zoom")
                .short('z')
                .long("zoom")
                .takes_value(true)
                .help("Sets the zoom"),
        )
        .arg(
            Arg::with_name("scale")
                .short('c')
                .long("scale")
                .takes_value(true)
                .help("Sets the scale (x,y)"),
        )
        .arg(
            Arg::with_name("bias")
                .short('b')
                .long("bias")
                .takes_value(true)
                .help("Sets the bias"),
        )
        .arg(
            Arg::with_name("output")
                .short('O')
                .long("output")
                .takes_value(true)
                .help("Sets the output file"),
        )
        .get_matches();

    if let Some(size) = matches.value_of("size") {
        let size: i32 = size.parse().unwrap();
        conf.size = size;
    }

    if let Some(octaves) = matches.value_of("octaves") {
        let octaves: i32 = octaves.parse().unwrap();
        conf.octaves = octaves;
    }

    if let Some(persistence) = matches.value_of("persistence") {
        let persistence: f64 = persistence.parse().unwrap();
        conf.persistence = persistence;
    }

    if let Some(lacunarity) = matches.value_of("lacunarity") {
        let lacunarity: f64 = lacunarity.parse().unwrap();
        conf.lacunarity = lacunarity;
    }

    if let Some(frequency) = matches.value_of("frequency") {
        let frequency: f64 = frequency.parse().unwrap();
        conf.frequency = frequency;
    }

    if let Some(amplitude) = matches.value_of("amplitude") {
        let amplitude: f64 = amplitude.parse().unwrap();
        conf.amplitude = amplitude;
    }

    if let Some(seed) = matches.value_of("seed") {
        let seed: i32 = seed.parse().unwrap();
        conf.seed = seed;
    }

    if let Some(zoom) = matches.value_of("zoom") {
        let zoom: f64 = zoom.parse().unwrap();
        conf.zoom = zoom;
    }

    if let Some(scale) = matches.value_of("scale") {
        // scale is a string of the form "(x,y)"
        let scale: String = scale.parse().unwrap();
        // remove the parenthesis if they exist
        let scale: String = scale.replace("(", "");
        let scale: String = scale.replace(")", "");
        let scale: Vec<&str> = scale.split(',').collect();
        let x: f64 = scale[0].parse().unwrap();
        let y: f64 = scale[1].parse().unwrap();
        conf.scale = (x, y);
    }

    if let Some(bias) = matches.value_of("bias") {
        let bias: f64 = bias.parse().unwrap();
        conf.bias = bias;
    }

    if let Some(output) = matches.value_of("output") {
        let output: String = output.parse().unwrap();
        conf.output = output;
    }

    print!("{:?}", conf);
    conf
}
