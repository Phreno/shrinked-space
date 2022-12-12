use clap::ArgMatches;

use super::{builder::ConfBuilder, Conf};

pub struct ConfChecker {
    pub conf: Conf,
    arg_matches: ArgMatches,
}

pub trait new {
    fn new(arg_matches: ArgMatches) -> Self;
}

impl new for ConfChecker {
    fn new(arg_matches: ArgMatches) -> Self {
        Self {
            conf: Conf::default(),
            arg_matches,
        }
    }
}

pub trait FromArgsOrDefault {
    fn from_args(&mut self);
    fn from_arg_size_or_default(&mut self);
    fn from_arg_octaves_or_default(&mut self);
    fn from_arg_persistence_or_default(&mut self);
    fn from_arg_lacunarity_or_default(&mut self);
    fn from_arg_frequency_or_default(&mut self);
    fn from_arg_amplitude_or_default(&mut self);
    fn from_arg_seed_or_default(&mut self);
    fn from_arg_zoom_or_default(&mut self);
    fn from_arg_scale_or_default(&mut self);
    fn from_arg_bias_or_default(&mut self);
    fn from_arg_output_or_default(&mut self);
}

impl FromArgsOrDefault for ConfChecker {
    fn from_args(&mut self) {
        self.from_arg_size_or_default();
        self.from_arg_octaves_or_default();
        self.from_arg_persistence_or_default();
        self.from_arg_lacunarity_or_default();
        self.from_arg_frequency_or_default();
        self.from_arg_amplitude_or_default();
        self.from_arg_seed_or_default();
        self.from_arg_zoom_or_default();
        self.from_arg_scale_or_default();
        self.from_arg_bias_or_default();
        self.from_arg_output_or_default();
    }
    fn from_arg_size_or_default(&mut self) {
        if let Some(size) = self.arg_matches.value_of("size") {
            self.conf.with_size(size.parse().unwrap());
        }
    }

    fn from_arg_octaves_or_default(&mut self) {
        if let Some(octaves) = self.arg_matches.value_of("octaves") {
            self.conf.with_octaves(octaves.parse().unwrap());
        }
    }

    fn from_arg_persistence_or_default(&mut self) {
        if let Some(persistence) = self.arg_matches.value_of("persistence") {
            self.conf.with_persistence(persistence.parse().unwrap());
        }
    }

    fn from_arg_lacunarity_or_default(&mut self) {
        if let Some(lacunarity) = self.arg_matches.value_of("lacunarity") {
            self.conf.with_lacunarity(lacunarity.parse().unwrap());
        }
    }

    fn from_arg_frequency_or_default(&mut self) {
        if let Some(frequency) = self.arg_matches.value_of("frequency") {
            self.conf.with_frequency(frequency.parse().unwrap());
        }
    }

    fn from_arg_amplitude_or_default(&mut self) {
        if let Some(amplitude) = self.arg_matches.value_of("amplitude") {
            self.conf.with_amplitude(amplitude.parse().unwrap());
        }
    }

    fn from_arg_seed_or_default(&mut self) {
        if let Some(seed) = self.arg_matches.value_of("seed") {
            self.conf.with_seed(seed.parse().unwrap());
        }
    }

    fn from_arg_zoom_or_default(&mut self) {
        if let Some(zoom) = self.arg_matches.value_of("zoom") {
            self.conf.with_zoom(zoom.parse().unwrap());
        }
    }

    fn from_arg_scale_or_default(&mut self) {
        if let Some(scale) = self.arg_matches.value_of("scale") {
            let scale: String = scale.parse().unwrap();
            let scale: String = scale.replace('(', "");
            let scale: String = scale.replace(')', "");
            let scale: Vec<&str> = scale.split(',').collect();
            let x: f64 = scale[0].parse().unwrap();
            let y: f64 = scale[1].parse().unwrap();
            self.conf.with_scale((x, y));
        }
    }

    fn from_arg_bias_or_default(&mut self) {
        if let Some(bias) = self.arg_matches.value_of("bias") {
            self.conf.with_bias(bias.parse().unwrap());
        }
    }

    fn from_arg_output_or_default(&mut self) {
        if let Some(output) = self.arg_matches.value_of("output") {
            self.conf.with_output(output.parse().unwrap());
        }
    }
}
