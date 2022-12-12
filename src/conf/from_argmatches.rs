use crate::conf::Conf;
use clap::ArgMatches;

use super::checker::*;

impl From<ArgMatches> for Conf {
    fn from(arg_matches: ArgMatches) -> Self {
        let mut conf_checker = ConfChecker::new(arg_matches);
        conf_checker.from_arg_size_or_default();
        conf_checker.from_arg_octaves_or_default();
        conf_checker.from_arg_persistence_or_default();
        conf_checker.from_arg_lacunarity_or_default();
        conf_checker.from_arg_frequency_or_default();
        conf_checker.from_arg_amplitude_or_default();
        conf_checker.from_arg_seed_or_default();
        conf_checker.from_arg_zoom_or_default();
        conf_checker.from_arg_scale_or_default();
        conf_checker.from_arg_bias_or_default();
        conf_checker.from_arg_output_or_default();
        conf_checker.conf
    }
}
