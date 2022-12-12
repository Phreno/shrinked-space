use clap::{App, Arg};
pub fn get_matches() -> clap::ArgMatches {
    let matches = App::new("shrinked~space")
        .arg(get_amplitude_match())
        .arg(get_bias_match())
        .arg(get_frequency_match())
        .arg(get_lacunarity_match())
        .arg(get_octaves_match())
        .arg(get_output_match())
        .arg(get_persistence_match())
        .arg(get_scale_match())
        .arg(get_seed_match())
        .arg(get_size_match())
        .arg(get_zoom_match())
        .get_matches();
    matches
}
fn get_size_match() -> Arg<'static> {
    Arg::with_name("size")
        .short('s')
        .long("size")
        .takes_value(true)
        .help("Sets the size of the output image")
}
fn get_octaves_match() -> Arg<'static> {
    Arg::with_name("octaves")
        .short('o')
        .long("octaves")
        .takes_value(true)
        .help("Sets the number of octaves")
}
fn get_persistence_match() -> Arg<'static> {
    Arg::with_name("persistence")
        .short('p')
        .long("persistence")
        .takes_value(true)
        .help("Sets the persistence")
}
fn get_lacunarity_match() -> Arg<'static> {
    Arg::with_name("lacunarity")
        .short('l')
        .long("lacunarity")
        .takes_value(true)
        .help("Sets the lacunarity")
}
fn get_frequency_match() -> Arg<'static> {
    Arg::with_name("frequency")
        .short('f')
        .long("frequency")
        .takes_value(true)
        .help("Sets the frequency")
}
fn get_amplitude_match() -> Arg<'static> {
    Arg::with_name("amplitude")
        .short('a')
        .long("amplitude")
        .takes_value(true)
        .help("Sets the amplitude")
}
fn get_seed_match() -> Arg<'static> {
    Arg::with_name("seed")
        .short('e')
        .long("seed")
        .takes_value(true)
        .help("Sets the seed")
}
fn get_zoom_match() -> Arg<'static> {
    Arg::with_name("zoom")
        .short('z')
        .long("zoom")
        .takes_value(true)
        .help("Sets the zoom")
}
fn get_scale_match() -> Arg<'static> {
    Arg::with_name("scale")
        .short('c')
        .long("scale")
        .takes_value(true)
        .help("Sets the scale (x,y)")
}
fn get_bias_match() -> Arg<'static> {
    Arg::with_name("bias")
        .short('b')
        .long("bias")
        .takes_value(true)
        .help("Sets the bias")
}
fn get_output_match() -> Arg<'static> {
    Arg::with_name("output")
        .short('O')
        .long("output")
        .takes_value(true)
        .help("Sets the output file")
}
