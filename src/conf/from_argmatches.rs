use crate::conf::Conf;
use clap::ArgMatches;

use super::checker::*;

impl From<ArgMatches> for Conf {
    fn from(arg_matches: ArgMatches) -> Self {
        ConfChecker::new(arg_matches).from_args()
    }
}
