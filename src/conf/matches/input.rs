use clap::Arg;

pub struct Input<'a> {
    pub size: Arg<'a>,
    pub octaves: Arg<'a>,
    pub persistence: Arg<'a>,
    pub lacunarity: Arg<'a>,
    pub frequency: Arg<'a>,
    pub amplitude: Arg<'a>,
    pub seed: Arg<'a>,
    pub zoom: Arg<'a>,
    pub scale: Arg<'a>,
    pub bias: Arg<'a>,
    pub output: Arg<'a>,
}

impl Default for Input<'_> {
    fn default() -> Self {
        Input {
            size: Arg::with_name("size")
                .short('s')
                .long("size")
                .takes_value(true)
                .help("Sets the size of the output image"),
            octaves: Arg::with_name("octaves")
                .short('o')
                .long("octaves")
                .takes_value(true)
                .help("Sets the number of octaves"),
            persistence: Arg::with_name("persistence")
                .short('p')
                .long("persistence")
                .takes_value(true)
                .help("Sets the persistence"),
            lacunarity: Arg::with_name("lacunarity")
                .short('l')
                .long("lacunarity")
                .takes_value(true)
                .help("Sets the lacunarity"),
            frequency: Arg::with_name("frequency")
                .short('f')
                .long("frequency")
                .takes_value(true)
                .help("Sets the frequency"),
            amplitude: Arg::with_name("amplitude")
                .short('a')
                .long("amplitude")
                .takes_value(true)
                .help("Sets the amplitude"),
            seed: Arg::with_name("seed")
                .short('e')
                .long("seed")
                .takes_value(true)
                .help("Sets the seed"),
            zoom: Arg::with_name("zoom")
                .short('z')
                .long("zoom")
                .takes_value(true)
                .help("Sets the zoom"),
            scale: Arg::with_name("scale")
                .short('c')
                .long("scale")
                .takes_value(true)
                .help("Sets the scale (x,y)"),
            bias: Arg::with_name("bias")
                .short('b')
                .long("bias")
                .takes_value(true)
                .help("Sets the bias"),
            output: Arg::with_name("output")
                .short('O')
                .long("output")
                .takes_value(true)
                .help("Sets the output file"),
        }
    }
}
