use clap::ArgMatches;

use crate::grid::conf::{Conf, ConfBuilder};

pub fn check_or_default(matches: clap::ArgMatches, default_conf: &mut Conf) {
    check_size_or_default(&matches, default_conf);
    check_octaves_or_default(&matches, default_conf);
    check_persistence_or_default(&matches, default_conf);
    check_lacunarity_or_default(&matches, default_conf);
    check_frequency_or_default(&matches, default_conf);
    check_amplitude_or_default(&matches, default_conf);
    check_seed_or_default(&matches, default_conf);
    check_zoom_or_default(&matches, default_conf);
    check_scale_or_default(&matches, default_conf);
    check_bias_or_default(&matches, default_conf);
    check_output_or_default(&matches, default_conf);
}
fn check_size_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(size) = matches.value_of("size") {
        conf.with_size(size.parse().unwrap());
    }
}
fn check_octaves_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(octaves) = matches.value_of("octaves") {
        let octaves: i32 = octaves.parse().unwrap();
        conf.octaves = octaves;
    }
}

fn check_persistence_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(persistence) = matches.value_of("persistence") {
        let persistence: f64 = persistence.parse().unwrap();
        conf.persistence = persistence;
    }
}
fn check_lacunarity_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(lacunarity) = matches.value_of("lacunarity") {
        let lacunarity: f64 = lacunarity.parse().unwrap();
        conf.lacunarity = lacunarity;
    }
}

fn check_frequency_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(frequency) = matches.value_of("frequency") {
        let frequency: f64 = frequency.parse().unwrap();
        conf.frequency = frequency;
    }
}
fn check_amplitude_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(amplitude) = matches.value_of("amplitude") {
        let amplitude: f64 = amplitude.parse().unwrap();
        conf.amplitude = amplitude;
    }
}
fn check_seed_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(seed) = matches.value_of("seed") {
        let seed: i32 = seed.parse().unwrap();
        conf.seed = seed;
    }
}

fn check_zoom_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(zoom) = matches.value_of("zoom") {
        let zoom: f64 = zoom.parse().unwrap();
        conf.zoom = zoom;
    }
}

fn check_scale_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(scale) = matches.value_of("scale") {
        let scale: String = scale.parse().unwrap();
        let scale: String = scale.replace('(', "");
        let scale: String = scale.replace(')', "");
        let scale: Vec<&str> = scale.split(',').collect();
        let x: f64 = scale[0].parse().unwrap();
        let y: f64 = scale[1].parse().unwrap();
        conf.scale = (x, y);
    }
}

fn check_bias_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(bias) = matches.value_of("bias") {
        let bias: f64 = bias.parse().unwrap();
        conf.bias = bias;
    }
}
fn check_output_or_default(matches: &ArgMatches, conf: &mut Conf) {
    if let Some(output) = matches.value_of("output") {
        let output: String = output.parse().unwrap();
        conf.output = output;
    }
}
