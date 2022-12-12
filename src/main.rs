use conf::Conf;

mod conf;
fn main() {
    let conf: Conf = conf::get_conf();
}
