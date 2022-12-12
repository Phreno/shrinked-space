mod default_values;
mod matches;
use crate::grid::conf::Conf;
use default_values::*;
use matches::*;

pub fn parse_args_or_default() -> Conf {
    let mut conf = Conf::default();
    let matches = get_matches();
    check_or_default(matches, &mut conf);
    println!("{:?}", conf);
    conf
}
