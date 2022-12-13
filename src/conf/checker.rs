use clap::ArgMatches;

use super::{builder::ConfBuilder, Conf};

pub struct ConfChecker {
    conf: Conf,
    arg_matches: ArgMatches,
}

impl ConfChecker {
    pub fn new(arg_matches: ArgMatches) -> Self {
        Self {
            conf: Conf::default(),
            arg_matches,
        }
    }
    fn get_conf(&self) -> &Conf {
        &self.conf
    }
}

pub trait FromArgsOrDefault {
    fn from_args(&mut self) -> Conf;
    fn with_size_or_default(&mut self) -> &mut ConfChecker;
    fn with_octaves_or_default(&mut self) -> &mut ConfChecker;
    fn with_persistence_or_default(&mut self) -> &mut ConfChecker;
    fn with_lacunarity_or_default(&mut self) -> &mut ConfChecker;
    fn with_frequency_or_default(&mut self) -> &mut ConfChecker;
    fn with_amplitude_or_default(&mut self) -> &mut ConfChecker;
    fn with_seed_or_default(&mut self) -> &mut ConfChecker;
    fn with_zoom_or_default(&mut self) -> &mut ConfChecker;
    fn with_scale_or_default(&mut self) -> &mut ConfChecker;
    fn with_bias_or_default(&mut self) -> &mut ConfChecker;
    fn with_output_or_default(&mut self) -> &mut ConfChecker;
}

impl FromArgsOrDefault for ConfChecker {
    fn from_args(&mut self) -> Conf {
        self.with_size_or_default()
            .with_octaves_or_default()
            .with_persistence_or_default()
            .with_lacunarity_or_default()
            .with_frequency_or_default()
            .with_amplitude_or_default()
            .with_seed_or_default()
            .with_zoom_or_default()
            .with_scale_or_default()
            .with_bias_or_default()
            .with_output_or_default()
            .get_conf()
            .to_owned()
    }
    fn with_size_or_default(&mut self) -> &mut ConfChecker {
        if let Some(size) = self.arg_matches.value_of("size") {
            self.conf.with_size(size.parse().unwrap());
        }
        self
    }

    fn with_octaves_or_default(&mut self) -> &mut ConfChecker {
        if let Some(octaves) = self.arg_matches.value_of("octaves") {
            self.conf.with_octaves(octaves.parse().unwrap());
        }
        self
    }

    fn with_persistence_or_default(&mut self) -> &mut ConfChecker {
        if let Some(persistence) = self.arg_matches.value_of("persistence") {
            self.conf.with_persistence(persistence.parse().unwrap());
        }
        self
    }

    fn with_lacunarity_or_default(&mut self) -> &mut ConfChecker {
        if let Some(lacunarity) = self.arg_matches.value_of("lacunarity") {
            self.conf.with_lacunarity(lacunarity.parse().unwrap());
        }
        self
    }

    fn with_frequency_or_default(&mut self) -> &mut ConfChecker {
        if let Some(frequency) = self.arg_matches.value_of("frequency") {
            self.conf.with_frequency(frequency.parse().unwrap());
        }
        self
    }

    fn with_amplitude_or_default(&mut self) -> &mut ConfChecker {
        if let Some(amplitude) = self.arg_matches.value_of("amplitude") {
            self.conf.with_amplitude(amplitude.parse().unwrap());
        }
        self
    }

    fn with_seed_or_default(&mut self) -> &mut ConfChecker {
        if let Some(seed) = self.arg_matches.value_of("seed") {
            self.conf.with_seed(seed.parse().unwrap());
        }
        self
    }

    fn with_zoom_or_default(&mut self) -> &mut ConfChecker {
        if let Some(zoom) = self.arg_matches.value_of("zoom") {
            self.conf.with_zoom(zoom.parse().unwrap());
        }
        self
    }

    fn with_scale_or_default(&mut self) -> &mut ConfChecker {
        if let Some(scale) = self.arg_matches.value_of("scale") {
            let scale: String = scale.parse().unwrap();
            let scale: String = scale.replace('(', "");
            let scale: String = scale.replace(')', "");
            let scale: Vec<&str> = scale.split(',').collect();
            let x: f64 = scale[0].parse().unwrap();
            let y: f64 = scale[1].parse().unwrap();
            self.conf.with_scale((x, y));
        }
        self
    }

    fn with_bias_or_default(&mut self) -> &mut ConfChecker {
        if let Some(bias) = self.arg_matches.value_of("bias") {
            self.conf.with_bias(bias.parse().unwrap());
        }
        self
    }

    fn with_output_or_default(&mut self) -> &mut ConfChecker {
        if let Some(output) = self.arg_matches.value_of("output") {
            self.conf.with_output(output.parse().unwrap());
        }
        self
    }
}
