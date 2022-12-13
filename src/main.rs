use conf::Conf;
mod conf;
mod grid;
fn main() {
    let conf: Conf = conf::get_conf();
}
