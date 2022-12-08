#[derive(Debug, Clone)]
pub struct Conf {
    pub size: i32,
    pub octaves: i32,
    pub persistence: f64,
    pub lacunarity: f64,
    pub frequency: f64,
    pub amplitude: f64,
    pub seed: i32,
    pub zoom: f64,
    pub scale: (f64, f64),
    pub bias: f64,
    pub output: String,
}
impl Conf {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        size: i32,
        octaves: i32,
        persistence: f64,
        lacunarity: f64,
        frequency: f64,
        amplitude: f64,
        seed: i32,
        zoom: f64,
        scale: (f64, f64),
        bias: f64,
        output: String,
    ) -> Self {
        Conf {
            size,
            octaves,
            persistence,
            lacunarity,
            frequency,
            amplitude,
            seed,
            zoom,
            scale,
            bias,
            output,
        }
    }
}
